import {
  chmodSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  statSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { createHash } from "node:crypto";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";
import { validateAgainstSchema } from "./schema-validator.mjs";
import { releaseBlockers } from "./release-readiness.mjs";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const binary = join(root, "target", "debug", "asi");

function invoke(args, expectedSuccess = true, options = {}) {
  const result = spawnSync(binary, args, {
    cwd: options.cwd ?? root,
    encoding: "utf8",
    env: { ...process.env, ...(options.env ?? {}) },
    timeout: options.timeout ?? 60_000,
  });
  if (result.error) throw result.error;
  const succeeded = result.status === 0;
  if (succeeded !== expectedSuccess) {
    throw new Error(
      `unexpected exit ${result.status} for asi ${args.join(" ")}\nstdout:\n${result.stdout}\nstderr:\n${result.stderr}`,
    );
  }
  return result;
}

function parseJson(text, label) {
  try {
    return JSON.parse(text);
  } catch (error) {
    throw new Error(`${label} was not valid JSON: ${error.message}\n${text}`);
  }
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

function provisionTrust(directory) {
  const privateKey = join(directory, "lineage-private.json");
  const publicKey = join(directory, "lineage-public.json");
  const genome = join(directory, "signed-genome.json");
  invoke([
    "key",
    "generate",
    "--private-key",
    privateKey,
    "--public-key",
    publicKey,
    "--json",
  ]);
  invoke(
    ["genome", "sign", "--private-key", privateKey, "--output", genome, "--json"],
    true,
    { timeout: 120_000 },
  );
  return { privateKey, publicKey, genome };
}

function inspectedPlan(taskArguments, options = {}) {
  return parseJson(
    invoke(["plan", ...taskArguments, "--json"], true, options).stdout,
    "inspected invocation plan",
  );
}

function executeApproved(taskArguments, ledger, trust, options = {}) {
  const inspected = inspectedPlan(taskArguments, options);
  assert(/^[0-9a-f]{64}$/.test(inspected.plan.plan_sha256), "plan lacks an approval digest");
  return parseJson(
    invoke(
      [
        "run",
        ...taskArguments,
        "--execute",
        "--approved-plan-sha256",
        inspected.plan.plan_sha256,
        "--ledger",
        ledger,
        "--genome",
        trust.genome,
        "--genome-public-key",
        trust.publicKey,
        "--json",
      ],
      true,
      { ...options, timeout: options.timeout ?? 180_000 },
    ).stdout,
    "approved run outcome",
  );
}

function doctor() {
  const report = parseJson(invoke(["doctor", "--json"]).stdout, "doctor report");
  const byId = new Map(report.map((entry) => [entry.id, entry]));
  for (const id of ["pi", "oh-my-pi", "codex", "claude"]) {
    assert(byId.get(id)?.installed === true, `${id} was not detected`);
    assert(byId.get(id)?.authority_owner === "asi-agent", `${id} gained authority ownership`);
    assert(
      byId.get(id)?.schema_version === "asi.harness-descriptor.v0.1",
      `${id} descriptor is not versioned`,
    );
    assert(
      byId.get(id)?.supported_version_prefixes?.length > 0,
      `${id} lacks a reviewed compatibility prefix`,
    );
  }
  for (const entry of report) {
    const descriptor = {
      schema_version: entry.schema_version,
      id: entry.id,
      display_name: entry.display_name,
      executable_name: entry.executable_name,
      supported_version_prefixes: entry.supported_version_prefixes,
      supported_effects: entry.supported_effects,
      containment_claims: entry.containment_claims,
      authority_owner: entry.authority_owner,
      integration_status: entry.integration_status,
    };
    validateAgainstSchema(
      join(root, "specs", "harness-adapter.schema.json"),
      descriptor,
      `${entry.id} descriptor`,
    );
  }
  assert(byId.has("hermes"), "Hermes adapter is missing from the genome");
  assert(byId.get("construct-fixture")?.installed === true, "fixture construct is unavailable");
  process.stdout.write("doctor acceptance passed\n");
}

function policy() {
  const denied = invoke(
    [
      "plan",
      "--harness",
      "construct-fixture",
      "--effect",
      "workspace-write",
      "--task",
      "negative control",
      "--json",
    ],
    false,
  );
  assert(
    `${denied.stdout}\n${denied.stderr}`.includes("policy denied [effect-class-denied]"),
    "workspace-write negative control did not fail through policy",
  );
  const allowed = parseJson(
    invoke([
      "plan",
      "--harness",
      "construct-fixture",
      "--effect",
      "none",
      "--task",
      "positive control",
      "--json",
    ]).stdout,
    "allowed plan",
  );
  assert(allowed.executed === false, "plan unexpectedly executed");
  assert(allowed.policy.allowed === true, "none effect was unexpectedly denied");
  assert(!JSON.stringify(allowed).includes("positive control"), "raw task leaked into public plan");

  // Version probes mount /tmp as an empty tmpfs, so keep the adversarial
  // executable under the read-only workspace where the probe can see it.
  const temporary = mkdtempSync(join(root, ".asi-version-contract-"));
  try {
    const supported = join(temporary, "pi-supported");
    writeFileSync(
      supported,
      "#!/bin/sh\nif [ -n \"${ASI_AMBIENT_SECRET:-}\" ]; then printf '%s\\n' 'AMBIENT_SECRET_LEAK'; else printf '%s\\n' '0.84.3'; fi\n",
    );
    chmodSync(supported, 0o755);
    const injected = parseJson(
      invoke(
        [
          "plan",
          "--harness",
          "pi",
          "--effect",
          "none",
          "--task=--dangerously-bypass-approvals-and-sandbox",
          "--json",
        ],
        true,
        { env: { ASI_HARNESS_PI_PATH: supported, ASI_AMBIENT_SECRET: "must-not-reach-probe" } },
      ).stdout,
      "option-injection plan",
    );
    const injectedArgs = injected.plan.arguments;
    assert(injectedArgs.at(-2) === "--", "task is not separated from adapter options");
    assert(injectedArgs.at(-1).startsWith("<task:sha256="), "task was not redacted");
    assert(
      !JSON.stringify(injected).includes("dangerously-bypass"),
      "option-like task leaked into the public plan",
    );
    assert(injected.plan.harness_version === "0.84.3", "Pi plan omitted the reviewed version");
    assert(/^[0-9a-f]{64}$/.test(injected.plan.executable_sha256), "Pi plan omitted its fingerprint");

    const unsupported = join(temporary, "pi-unsupported");
    writeFileSync(unsupported, "#!/bin/sh\nprintf '%s\\n' '99.0.0'\n");
    chmodSync(unsupported, 0o755);
    const incompatible = invoke(
      [
        "plan",
        "--harness",
        "pi",
        "--effect",
        "none",
        "--task",
        "compatibility negative control",
        "--json",
      ],
      false,
      { env: { ASI_HARNESS_PI_PATH: unsupported } },
    );
    assert(
      incompatible.stderr.includes("outside the reviewed compatibility prefixes"),
      `unsupported harness version did not fail closed: ${incompatible.stderr}`,
    );

    const unknown = join(temporary, "pi-unknown");
    writeFileSync(unknown, "#!/bin/sh\nexit 9\n");
    chmodSync(unknown, 0o755);
    const unavailable = invoke(
      [
        "plan",
        "--harness",
        "pi",
        "--effect",
        "none",
        "--task",
        "unknown-version negative control",
        "--json",
      ],
      false,
      { env: { ASI_HARNESS_PI_PATH: unknown } },
    );
    assert(unavailable.stderr.includes("version is unavailable"), "unknown version was accepted");
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("policy acceptance passed\n");
}

function bloodline() {
  const temporary = mkdtempSync(join(tmpdir(), "asi-bloodline-"));
  try {
    const ledger = join(temporary, "bloodline.jsonl");
    const trust = provisionTrust(temporary);
    const taskArguments = [
        "--harness",
        "construct-fixture",
        "--effect",
        "none",
        "--task",
        "acceptance construct",
    ];
    const inspected = inspectedPlan(taskArguments);
    const missingApproval = invoke(
      [
        "run",
        ...taskArguments,
        "--execute",
        "--ledger",
        ledger,
        "--genome",
        trust.genome,
        "--genome-public-key",
        trust.publicKey,
        "--json",
      ],
      false,
    );
    assert(
      missingApproval.stderr.includes("approved-plan-sha256"),
      "execution did not require a separately inspected plan digest",
    );
    const wrongApproval = invoke(
      [
        "run",
        ...taskArguments,
        "--execute",
        "--approved-plan-sha256",
        "0".repeat(64),
        "--ledger",
        ledger,
        "--genome",
        trust.genome,
        "--genome-public-key",
        trust.publicKey,
        "--json",
      ],
      false,
    );
    assert(wrongApproval.stderr.includes("does not match"), "wrong plan approval digest was accepted");
    assert(!existsSync(ledger), "rejected plan created Bloodline state");
    const missingGenome = invoke(
      [
        "run",
        ...taskArguments,
        "--execute",
        "--approved-plan-sha256",
        inspected.plan.plan_sha256,
        "--ledger",
        ledger,
        "--genome",
        join(temporary, "missing-genome.json"),
        "--genome-public-key",
        trust.publicKey,
        "--json",
      ],
      false,
    );
    assert(missingGenome.stderr.includes("requires a signed genome"), "missing signed genome was accepted");
    assert(!existsSync(ledger), "missing-genome rejection created Bloodline state");

    const outcome = executeApproved(taskArguments, ledger, trust);
    assert(outcome.status === "completed", "construct did not complete");
    assert(outcome.plan_sha256 === inspected.plan.plan_sha256, "execution used a different plan");
    assert(/^[0-9a-f]{64}$/.test(outcome.genome_sha256), "run omitted signed-genome evidence");
    assert(outcome.genome_key_id.startsWith("ed25519:"), "run omitted the genome key id");
    const verification = parseJson(
      invoke(["ledger", "verify", "--path", ledger, "--json"]).stdout,
      "ledger verification",
    );
    assert(verification.valid === true, "ledger was not valid");
    assert(verification.events === 4, `expected 4 ledger events, got ${verification.events}`);
    if (process.platform !== "win32") {
      const mode = statSync(ledger).mode & 0o777;
      assert(mode === 0o600, `Bloodline mode was ${mode.toString(8)}, expected 600`);
    }

    const tampered = join(temporary, "tampered.jsonl");
    writeFileSync(
      tampered,
      readFileSync(ledger, "utf8").replace("harness.completed", "harness.corrupted"),
    );
    const negative = invoke(["ledger", "verify", "--path", tampered, "--json"], false);
    assert(negative.stderr.includes("integrity hash mismatch"), "tamper negative control passed");
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("bloodline acceptance passed\n");
}

function isolation() {
  const escapePath = join(root, ".asi-isolation-escape");
  assert(!existsSync(escapePath), `${escapePath} exists before the isolation probe`);

  const plan = parseJson(
    invoke([
      "plan",
      "--harness",
      "isolation-fixture",
      "--effect",
      "none",
      "--task=--attempt-option-injection",
      "--json",
    ]).stdout,
    "isolated plan",
  );
  assert(plan.plan.isolation.enforced === true, "default subprocess isolation was not enforced");
  validateAgainstSchema(
    join(root, "specs", "isolation-report.schema.json"),
    plan.plan.isolation,
    "emitted isolation report",
  );
  assert(plan.plan.isolation.enforcer === "bubblewrap", "Bubblewrap was not the enforcer");
  assert(
    plan.plan.isolation.filesystem.includes("host-root-read-only"),
    "read-only host root was not disclosed",
  );
  assert(
    plan.plan.isolation.network.includes("unmediated"),
    "unmediated provider network was not disclosed",
  );
  assert(
    plan.plan.isolation.limitations.some((item) => item.includes("host-readable")),
    "host read visibility was not disclosed",
  );
  assert(plan.plan.arguments.includes("--ro-bind"), "Bubblewrap plan omitted read-only root bind");
  assert(plan.plan.arguments.includes("--tmpfs"), "Bubblewrap plan omitted ephemeral /tmp");
  assert(
    !JSON.stringify(plan).includes("attempt-option-injection"),
    "option-like task leaked into the public isolation plan",
  );

  const unavailable = invoke(
    [
      "plan",
      "--harness",
      "isolation-fixture",
      "--effect",
      "none",
      "--task",
      "missing bwrap",
      "--json",
    ],
    false,
    { env: { PATH: "" } },
  );
  assert(unavailable.stderr.includes("required isolation unavailable"), "missing bwrap did not fail closed");

  const spoofDirectory = mkdtempSync(join(tmpdir(), "asi-fake-bwrap-"));
  try {
    const spoof = join(spoofDirectory, "bwrap");
    writeFileSync(spoof, "#!/bin/sh\nprintf '%s' 'FAKE_BWRAP_RAN'\n");
    chmodSync(spoof, 0o755);
    const rejectedSpoof = invoke(
      [
        "plan",
        "--harness",
        "isolation-fixture",
        "--effect",
        "none",
        "--task",
        "spoofed enforcer",
        "--json",
      ],
      false,
      { env: { PATH: `${spoofDirectory}:/usr/bin:/bin` } },
    );
    assert(
      rejectedSpoof.stderr.includes("trusted root-owned bwrap"),
      "user-owned bwrap shim was accepted as the security boundary",
    );
    assert(!rejectedSpoof.stdout.includes("FAKE_BWRAP_RAN"), "spoofed bwrap was executed");
  } finally {
    rmSync(spoofDirectory, { recursive: true, force: true });
  }

  const unacknowledged = invoke(
    [
      "plan",
      "--harness",
      "isolation-fixture",
      "--effect",
      "none",
      "--task",
      "unsafe negative control",
      "--isolation",
      "off",
      "--json",
    ],
    false,
  );
  assert(
    unacknowledged.stderr.includes("acknowledge-unsafe-subprocess"),
    "unsafe mode did not require a second signal",
  );
  const acknowledged = parseJson(
    invoke([
      "plan",
      "--harness",
      "isolation-fixture",
      "--effect",
      "none",
      "--task",
      "unsafe positive control",
      "--isolation",
      "off",
      "--acknowledge-unsafe-subprocess",
      "--json",
    ]).stdout,
    "unsafe acknowledged plan",
  );
  assert(acknowledged.plan.isolation.enforced === false, "unsafe mode claimed isolation");
  assert(acknowledged.plan.isolation.enforcer === "none", "unsafe mode named an enforcer");

  const temporary = mkdtempSync(join(tmpdir(), "asi-isolation-"));
  try {
    const trust = provisionTrust(temporary);
    const outcome = executeApproved(
      [
        "--harness",
        "isolation-fixture",
        "--effect",
        "none",
        "--task=--runtime-injection-attempt",
      ],
      join(temporary, "bloodline.jsonl"),
      trust,
      { env: { ASI_AMBIENT_SECRET: "must-not-reach-worker" } },
    );
    assert(outcome.output === "ASI_ISOLATION_OK", `unexpected isolation output: ${outcome.output}`);
    assert(outcome.isolation.enforced === true, "executed subprocess was not isolated");
    assert(!existsSync(escapePath), "isolated subprocess wrote into the workspace");

    const diagnosticArguments = [
      "--harness",
      "isolation-fixture",
      "--effect",
      "none",
      "--task",
      "diagnostic-negative-control",
    ];
    const diagnosticPlan = inspectedPlan(diagnosticArguments);
    const failed = invoke(
      [
        "run",
        ...diagnosticArguments,
        "--execute",
        "--approved-plan-sha256",
        diagnosticPlan.plan.plan_sha256,
        "--ledger",
        join(temporary, "failed-bloodline.jsonl"),
        "--genome",
        trust.genome,
        "--genome-public-key",
        trust.publicKey,
        "--json",
      ],
      false,
    );
    assert(failed.stderr.includes("stderr_sha256="), "failed harness omitted diagnostic digest");
    assert(!failed.stderr.includes("HARNESS_SECRET_DIAGNOSTIC"), "failed harness leaked raw stderr");
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("isolation acceptance passed\n");
}

function lineage() {
  const temporary = mkdtempSync(join(tmpdir(), "asi-lineage-"));
  try {
    const ledger = join(temporary, "bloodline.jsonl");
    const trust = provisionTrust(temporary);
    executeApproved(
      [
        "--harness",
        "construct-fixture",
        "--effect",
        "none",
        "--task",
        "signed lineage acceptance",
      ],
      ledger,
      trust,
    );
    const privateKey = trust.privateKey;
    const publicKey = trust.publicKey;
    const keyOutput = readFileSync(publicKey, "utf8");
    const key = parseJson(keyOutput, "key generation outcome");
    assert(key.algorithm === "Ed25519", "lineage public-key algorithm drifted");
    assert(!keyOutput.includes("secret_key_hex"), "private key material leaked to CLI output");
    if (process.platform !== "win32") {
      const mode = statSync(privateKey).mode & 0o777;
      assert(mode === 0o600, `private key mode was ${mode.toString(8)}, expected 600`);
    }

    const checkpoint = join(temporary, "checkpoint.json");
    invoke([
      "ledger",
      "checkpoint",
      "--path",
      ledger,
      "--private-key",
      privateKey,
      "--output",
      checkpoint,
      "--json",
    ]);
    validateAgainstSchema(
      join(root, "specs", "signed-checkpoint.schema.json"),
      parseJson(readFileSync(checkpoint, "utf8"), "signed checkpoint"),
      "emitted signed checkpoint",
    );
    const verified = parseJson(
      invoke([
        "ledger",
        "verify-checkpoint",
        "--path",
        ledger,
        "--checkpoint",
        checkpoint,
        "--public-key",
        publicKey,
        "--json",
      ]).stdout,
      "checkpoint verification",
    );
    assert(verified.valid === true && verified.events === 4, "signed checkpoint did not verify");

    const tamperedLedger = join(temporary, "tampered-ledger.jsonl");
    writeFileSync(
      tamperedLedger,
      readFileSync(ledger, "utf8").replace("harness.completed", "harness.corrupted"),
    );
    invoke(
      [
        "ledger",
        "verify-checkpoint",
        "--path",
        tamperedLedger,
        "--checkpoint",
        checkpoint,
        "--public-key",
        publicKey,
        "--json",
      ],
      false,
    );

    const tamperedCheckpoint = join(temporary, "tampered-checkpoint.json");
    const checkpointValue = parseJson(readFileSync(checkpoint, "utf8"), "checkpoint fixture");
    checkpointValue.material.events += 1;
    writeFileSync(tamperedCheckpoint, JSON.stringify(checkpointValue, null, 2));
    invoke(
      [
        "ledger",
        "verify-checkpoint",
        "--path",
        ledger,
        "--checkpoint",
        tamperedCheckpoint,
        "--public-key",
        publicKey,
        "--json",
      ],
      false,
    );

    const wrongPrivate = join(temporary, "wrong-private.json");
    const wrongPublic = join(temporary, "wrong-public.json");
    invoke([
      "key",
      "generate",
      "--private-key",
      wrongPrivate,
      "--public-key",
      wrongPublic,
      "--json",
    ]);
    const wrongKey = invoke(
      [
        "ledger",
        "verify-checkpoint",
        "--path",
        ledger,
        "--checkpoint",
        checkpoint,
        "--public-key",
        wrongPublic,
        "--json",
      ],
      false,
    );
    assert(wrongKey.stderr.includes("pinned public key"), "wrong checkpoint key did not fail closed");
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("lineage acceptance passed\n");
}

function genome() {
  const temporary = mkdtempSync(join(tmpdir(), "asi-genome-"));
  try {
    const privateKey = join(temporary, "lineage-private.json");
    const publicKey = join(temporary, "lineage-public.json");
    invoke([
      "key",
      "generate",
      "--private-key",
      privateKey,
      "--public-key",
      publicKey,
      "--json",
    ]);
    const genomePath = join(temporary, "genome.json");
    const signed = parseJson(
      invoke(
        ["genome", "sign", "--private-key", privateKey, "--output", genomePath, "--json"],
        true,
        { timeout: 120_000 },
      ).stdout,
      "signed genome",
    );
    assert(signed.algorithm === "Ed25519", "genome signature algorithm drifted");
    validateAgainstSchema(
      join(root, "specs", "signed-genome.schema.json"),
      signed,
      "emitted signed genome",
    );
    assert(signed.material.entries.length >= 7, "signed genome omitted harness descriptors");
    for (const entry of signed.material.entries) {
      assert(entry.descriptor.authority_owner === "asi-agent", `${entry.descriptor.id} gained authority`);
      if (entry.installed && entry.executable_path) {
        assert(
          /^[0-9a-f]{64}$/.test(entry.executable_sha256),
          `${entry.descriptor.id} lacks an executable fingerprint`,
        );
      }
    }
    const verified = parseJson(
      invoke(
        ["genome", "verify", "--genome", genomePath, "--public-key", publicKey, "--json"],
        true,
        { timeout: 120_000 },
      ).stdout,
      "genome verification",
    );
    assert(verified.valid === true, "genome signature did not verify");
    assert(verified.current_state_matches === true, "current harness state drifted after signing");
    assert(/^[0-9a-f]{64}$/.test(verified.genome_sha256), "genome verification omitted its digest");

    const tamperedPath = join(temporary, "tampered-genome.json");
    const tampered = parseJson(readFileSync(genomePath, "utf8"), "genome fixture");
    tampered.material.entries[0].descriptor.display_name = "tampered descriptor";
    writeFileSync(tamperedPath, JSON.stringify(tampered, null, 2));
    invoke(
      ["genome", "verify", "--genome", tamperedPath, "--public-key", publicKey, "--json"],
      false,
      { timeout: 120_000 },
    );

    const wrongPrivate = join(temporary, "wrong-private.json");
    const wrongPublic = join(temporary, "wrong-public.json");
    invoke([
      "key",
      "generate",
      "--private-key",
      wrongPrivate,
      "--public-key",
      wrongPublic,
      "--json",
    ]);
    const wrongKey = invoke(
      ["genome", "verify", "--genome", genomePath, "--public-key", wrongPublic, "--json"],
      false,
      { timeout: 120_000 },
    );
    assert(wrongKey.stderr.includes("pinned public key"), "wrong genome key did not fail closed");
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("genome acceptance passed\n");
}

function skills() {
  const safe = parseJson(
    invoke(["skill", "inspect", "tests/fixtures/skills/safe", "--json"]).stdout,
    "safe skill report",
  );
  assert(safe.name === "evidence-reader", "safe skill metadata was not parsed");
  assert(["none", "low"].includes(safe.risk_level), "safe skill was overclassified");

  const risky = parseJson(
    invoke(["skill", "inspect", "tests/fixtures/skills/risky", "--json"]).stdout,
    "risky skill report",
  );
  const codes = new Set(risky.findings.map((finding) => finding.code));
  assert(codes.has("instruction-authority"), "immediate execution was not detected");
  assert(codes.has("piped-installer"), "piped installer was not detected");

  const temporary = mkdtempSync(join(tmpdir(), "asi-skill-crypt-"));
  try {
    if (process.platform !== "win32") {
      const outside = join(temporary, "outside.md");
      writeFileSync(outside, "---\nname: escaped\n---\nsecret\n");
      const linkedDirectory = join(temporary, "linked-skill");
      mkdirSync(linkedDirectory);
      symlinkSync(outside, join(linkedDirectory, "SKILL.md"));
      const nestedLink = invoke(["skill", "inspect", linkedDirectory, "--json"], false);
      assert(nestedLink.stderr.includes("symbolic link"), "symlinked SKILL.md escaped inspection");
      const directLink = join(temporary, "direct.SKILL.md");
      symlinkSync(outside, directLink);
      const direct = invoke(["skill", "inspect", directLink, "--json"], false);
      assert(direct.stderr.includes("symbolic link"), "direct skill symlink escaped inspection");
    }
    const outcome = parseJson(
      invoke([
        "skill",
        "assimilate",
        "tests/fixtures/skills/risky",
        "--vault",
        temporary,
        "--json",
      ]).stdout,
      "assimilation outcome",
    );
    assert(outcome.state === "quarantined", "skill was promoted during assimilation");
    const manifest = parseJson(readFileSync(outcome.manifest_path, "utf8"), "SkillSpec");
    validateAgainstSchema(
      join(root, "specs", "skill-spec.schema.json"),
      manifest,
      "emitted SkillSpec",
    );
    assert(manifest.state === "quarantined", "SkillSpec state is not quarantined");
    assert(
      manifest.lineage.transformations.includes("scripts-not-executed"),
      "SkillSpec omitted non-execution lineage",
    );
    if (process.platform !== "win32") {
      const mode = statSync(outcome.quarantined_source_path).mode & 0o777;
      assert((mode & 0o111) === 0, "quarantined source retained executable permission");
    }
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  process.stdout.write("skill acceptance passed\n");
}

function release() {
  const workflowPath = join(root, ".github", "workflows", "ci.yml");
  assert(existsSync(workflowPath), "CI workflow is missing");
  const workflow = readFileSync(workflowPath, "utf8");
  for (const required of [
    "cargo fmt",
    "cargo clippy",
    "cargo test",
    "scripts/acceptance.mjs",
    "release-readiness",
  ]) {
    assert(workflow.includes(required), `CI workflow omitted ${required}`);
  }
  const packageScript = join(root, "scripts", "package.mjs");
  assert(existsSync(packageScript), "release packaging script is missing");
  assert(existsSync(join(root, "LICENSE")), "owner-ratified LICENSE file is missing");
  const currentBlockers = releaseBlockers(root);
  assert(
    !currentBlockers.some((blocker) => blocker.includes("owner-selected LICENSE is missing")),
    "ratified license did not clear the license-missing blocker",
  );
  assert(
    currentBlockers.some((blocker) => blocker.includes("Release status: APPROVED")),
    "license ratification alone did not keep release blocked",
  );
  assert(
    currentBlockers.some((blocker) => blocker.includes("independent-review disposition")),
    "non-approving independent review did not block release",
  );
  const licenseOnly = mkdtempSync(join(tmpdir(), "asi-license-only-"));
  try {
    writeFileSync(join(licenseOnly, "LICENSE"), "placeholder license\n");
    const blockers = releaseBlockers(licenseOnly);
    assert(blockers.length > 1, "a non-empty LICENSE alone unlocked release readiness");
    assert(
      blockers.some((blocker) => blocker.includes("SPDX")) &&
        blockers.some((blocker) => blocker.includes("SBOM")),
      "license-only negative control did not require ratification and SBOM evidence",
    );
  } finally {
    rmSync(licenseOnly, { recursive: true, force: true });
  }
  const blocked = spawnSync(process.execPath, [packageScript], {
    cwd: root,
    encoding: "utf8",
    env: process.env,
    timeout: 30_000,
  });
  assert(blocked.status !== 0, "packaging proceeded while release remains blocked");
  assert(
    `${blocked.stdout}\n${blocked.stderr}`.toLowerCase().includes("licensing"),
    "packaging failure did not explain the licensing blocker",
  );
  assert(!existsSync(join(root, "dist")), "blocked packaging left a release directory behind");
  process.stdout.write("release acceptance passed\n");
}

function hermes() {
  const report = parseJson(invoke(["doctor", "--json"], true, { timeout: 120_000 }).stdout, "doctor report");
  const entry = report.find((candidate) => candidate.id === "hermes");
  assert(entry?.installed === true, "Hermes is not installed");
  assert(entry.executable_path, "Hermes has no direct executable path");
  assert(entry.executable_source === "path", "Hermes was not discovered as a direct PATH executable");
  const launcher = readFileSync(entry.executable_path, "utf8").slice(0, 16_384);
  assert(!launcher.includes("mise use -g"), "Hermes executable mutates global mise state");
  const health = spawnSync(entry.executable_path, ["--help"], {
    cwd: root,
    encoding: "utf8",
    env: process.env,
    timeout: 60_000,
  });
  if (health.error) throw health.error;
  assert(health.status === 0, `Hermes health check failed: ${health.stderr}`);
  assert(`${health.stdout}\n${health.stderr}`.includes("Hermes"), "Hermes help output was not recognized");
  const chatHelp = spawnSync(entry.executable_path, ["chat", "--help"], {
    cwd: root,
    encoding: "utf8",
    env: process.env,
    timeout: 60_000,
  });
  if (chatHelp.error) throw chatHelp.error;
  assert(chatHelp.status === 0, `Hermes chat health check failed: ${chatHelp.stderr}`);
  for (const flag of ["--toolsets", "--max-turns", "--ignore-user-config", "--ignore-rules", "--safe-mode"]) {
    assert(`${chatHelp.stdout}\n${chatHelp.stderr}`.includes(flag), `Hermes chat omitted reviewed flag ${flag}`);
  }
  const plan = inspectedPlan([
    "--harness",
    "hermes",
    "--effect",
    "read-only",
    "--task",
    "Hermes adapter conformance plan",
  ]);
  assert(plan.plan.harness_version.startsWith("Hermes Agent v0.20.0"), "Hermes version contract drifted");
  assert(plan.plan.arguments.includes("--safe-mode"), "Hermes plan omitted safe mode");
  assert(plan.plan.arguments.includes("safe"), "Hermes plan omitted the safe toolset");
  assert(!JSON.stringify(plan).includes("Hermes adapter conformance plan"), "Hermes plan leaked its task");
  const installation = readFileSync(join(root, "docs", "HERMES.md"), "utf8");
  assert(
    installation.includes("b9aa9289a8083f2e9d248ad6837b2938f5ee92d7"),
    "Hermes installation record omitted the pinned source commit",
  );
  const installer = readFileSync(join(root, "scripts", "install-hermes.sh"), "utf8");
  assert(installer.includes("uv 0.12.5 (x86_64-unknown-linux-musl)"), "installer does not pin uv exactly");
  assert(installer.includes("Python 3.13.15"), "installer does not pin Python exactly");
  process.stdout.write("hermes acceptance passed\n");
}

function reviews() {
  const directory = join(root, "docs", "reviews", "v0.2");
  const reviews = [
    ["ai-research-scientist", "AI Research Scientist"],
    ["senior-llm-engineer", "Senior LLM Engineer"],
    ["generative-ai-engineer", "Generative AI Engineer"],
    ["caio", "Chief AI Officer (CAIO)"],
    ["ai-solutions-architect", "AI Solutions Architect"],
  ];
  const records = [];
  const findingIds = new Set();
  const findingSeverity = new Map();
  for (const [slug, role] of reviews) {
    const markdownPath = join(directory, `${slug}.md`);
    const recordPath = join(directory, `${slug}.json`);
    assert(existsSync(markdownPath), `independent review is missing: ${slug}.md`);
    assert(existsSync(recordPath), `machine review record is missing: ${slug}.json`);
    const markdown = readFileSync(markdownPath, "utf8");
    for (const marker of ["independent review", "verdict", "critical", "high", "unsupported assumption"]) {
      assert(markdown.toLowerCase().includes(marker), `${slug}.md omitted ${marker}`);
    }
    assert(
      markdown.toLowerCase().includes("did not read the other reviewers"),
      `${slug}.md lacks independence attestation`,
    );
    const record = parseJson(readFileSync(recordPath, "utf8"), `${slug} review record`);
    validateAgainstSchema(
      join(root, "specs", "independent-review.schema.json"),
      record,
      `${slug} review record`,
    );
    assert(record.role === role, `${slug} role does not match the required reviewer`);
    assert(record.independence_attestation === true, `${slug} is not independently attested`);
    for (const finding of record.critical_findings) {
      assert(!findingIds.has(finding.id), `duplicate independent finding id ${finding.id}`);
      findingIds.add(finding.id);
      findingSeverity.set(finding.id, "critical");
    }
    for (const finding of record.high_findings) {
      assert(!findingIds.has(finding.id), `duplicate independent finding id ${finding.id}`);
      findingIds.add(finding.id);
      findingSeverity.set(finding.id, "high");
    }
    records.push({ slug, record });
  }
  const dispositionPath = join(directory, "integrated-disposition.md");
  assert(existsSync(dispositionPath), "integrated review disposition is missing");
  const disposition = readFileSync(dispositionPath, "utf8");
  assert(disposition.includes("Five independent reviews"), "review count was not integrated");

  const machineDisposition = parseJson(
    readFileSync(join(directory, "review-disposition.json"), "utf8"),
    "machine review disposition",
  );
  validateAgainstSchema(
    join(root, "specs", "review-disposition.schema.json"),
    machineDisposition,
    "machine review disposition",
  );
  const dispositions = new Map();
  for (const finding of machineDisposition.findings) {
    assert(!dispositions.has(finding.id), `duplicate disposition id ${finding.id}`);
    dispositions.set(finding.id, finding);
    assert(
      finding.severity === findingSeverity.get(finding.id),
      `${finding.id} changed severity during disposition`,
    );
    if (finding.status === "scope-blocked") {
      assert(finding.release_blocking === true, `${finding.id} is blocked but did not block release`);
    }
  }
  assert(dispositions.size === findingIds.size, "disposition count does not match independent findings");
  for (const id of findingIds) assert(dispositions.has(id), `finding ${id} has no disposition`);
  assert(
    machineDisposition.reviews.length === records.length &&
      new Set(machineDisposition.reviews.map((review) => review.role)).size === records.length,
    "integrated disposition does not contain five unique review roles",
  );
  for (const { slug, record } of records) {
    const integrated = machineDisposition.reviews.find((review) => review.role === record.role);
    assert(integrated?.verdict === record.verdict, `${record.role} verdict changed during integration`);
    assert(integrated?.record === `${slug}.json`, `${record.role} points to the wrong machine record`);
  }
  const nonApproving = records.some(({ record }) => record.verdict === "do-not-approve");
  if (nonApproving) {
    assert(machineDisposition.adoption_status === "blocked", "non-approval did not block adoption");
    assert(machineDisposition.public_release_status === "blocked", "non-approval did not block release");
  }
  process.stdout.write("reviews acceptance passed\n");
}

function summary() {
  const cargo = readFileSync(join(root, "Cargo.toml"), "utf8");
  const sourceFiles = readdirSync(join(root, "src"))
    .filter((name) => name.endsWith(".rs"))
    .map((name) => `src/${name}`)
    .sort();
  assert(cargo.includes('name = "asi-agent"'), "Cargo package identity is missing");
  for (const path of sourceFiles) {
    const source = readFileSync(join(root, path), "utf8");
    assert(!/\bTODO\b|unimplemented!\s*\(/.test(source), `${path} contains unfinished code`);
  }
  for (const schema of readdirSync(join(root, "specs"))
    .filter((name) => name.endsWith(".json"))
    .map((name) => `specs/${name}`)
    .sort()) {
    parseJson(readFileSync(join(root, schema), "utf8"), schema);
  }
  let rejectedInvalidSchemaDocument = false;
  try {
    validateAgainstSchema(
      join(root, "specs", "isolation-report.schema.json"),
      { schema_version: "wrong" },
      "schema negative control",
    );
  } catch {
    rejectedInvalidSchemaDocument = true;
  }
  assert(rejectedInvalidSchemaDocument, "schema validator accepted an invalid emitted document");
  const gates = readFileSync(join(root, "GATES.md"), "utf8");
  const ids = [...gates.matchAll(/^- \[[ x]\] (G\d+):/gm)].map((match) => match[1]);
  const expectedIds = Array.from({ length: 16 }, (_, index) => `G${index + 1}`);
  assert(JSON.stringify(ids) === JSON.stringify(expectedIds), "gate ids are missing, duplicated, or reordered");
  process.stdout.write("summary acceptance passed\n");
}

function parseGateBlocks(gates) {
  const matches = [...gates.matchAll(/^- \[([ x])\] (G\d+):/gm)];
  return matches.map((match, index) => ({
    checked: match[1] === "x",
    id: match[2],
    block: gates.slice(match.index, matches[index + 1]?.index ?? gates.length),
  }));
}

function evidenceDigest() {
  const files = [
    "Cargo.lock",
    "Cargo.toml",
    "ASI-Agent-Astronomical-Plan.md",
    "PLAN.md",
    "README.md",
    ".github/workflows/ci.yml",
  ];
  for (const directory of ["docs", "scripts", "specs", "src", "tests"]) {
    const visit = (relative) => {
      for (const entry of readdirSync(join(root, relative), { withFileTypes: true })) {
        const child = `${relative}/${entry.name}`;
        if (entry.isDirectory()) visit(child);
        else if (entry.isFile()) files.push(child);
      }
    };
    visit(directory);
  }
  files.sort();
  const digest = createHash("sha256");
  for (const path of files) {
    digest.update(path);
    digest.update("\0");
    digest.update(readFileSync(join(root, path)));
    digest.update("\0");
  }
  return { files: files.length, sha256: digest.digest("hex") };
}

function summaryV02() {
  summary();
  for (const path of [
    ".github/workflows/ci.yml",
    "docs/LICENSING.md",
    "docs/VERIFICATION-v0.2.md",
    "docs/reviews/v0.2/integrated-disposition.md",
    "scripts/install-hermes.sh",
    "scripts/package.mjs",
  ]) {
    assert(existsSync(join(root, path)), `v0.2 artifact is missing: ${path}`);
  }
  const gates = readFileSync(join(root, "GATES.md"), "utf8");
  assert(!gates.includes("ABANDONED"), "gate ledger contains abandoned work");
  const blocks = new Map(parseGateBlocks(gates).map((entry) => [entry.id, entry]));
  for (let id = 1; id <= 15; id += 1) {
    const gate = blocks.get(`G${id}`);
    assert(gate?.checked, `G${id} is not checked before the final gate`);
    assert(/^  EVIDENCE: (?!pending\s*$).+/m.test(gate.block), `G${id} lacks recorded evidence`);
  }
  const evidence = evidenceDigest();
  process.stdout.write(`source_sha256=${evidence.sha256} files=${evidence.files}\n`);
  process.stdout.write("v0.2 summary acceptance passed\n");
}

const command = process.argv[2];
const commands = {
  doctor,
  policy,
  bloodline,
  isolation,
  lineage,
  genome,
  skills,
  release,
  hermes,
  reviews,
  summary,
  "summary-v02": summaryV02,
};
if (!commands[command]) {
  throw new Error(`unknown acceptance command: ${command ?? "<missing>"}`);
}
commands[command]();
