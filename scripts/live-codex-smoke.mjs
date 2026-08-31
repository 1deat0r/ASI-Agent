import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const binary = join(root, "target", "debug", "asi");
const task = "Reply exactly: ASI_V02_CHAIN_OK";

function invoke(args, timeout = 180_000) {
  const result = spawnSync(binary, args, {
    cwd: root,
    encoding: "utf8",
    env: process.env,
    timeout,
  });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`asi ${args.join(" ")} failed\n${result.stdout}\n${result.stderr}`);
  }
  return JSON.parse(result.stdout);
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

const temporary = mkdtempSync(join(tmpdir(), "asi-live-codex-v02-"));
try {
  const privateKey = join(temporary, "private.json");
  const publicKey = join(temporary, "public.json");
  const genomePath = join(temporary, "genome.json");
  const ledgerPath = join(temporary, "bloodline.jsonl");
  invoke([
    "key",
    "generate",
    "--private-key",
    privateKey,
    "--public-key",
    publicKey,
    "--json",
  ]);
  invoke(["genome", "sign", "--private-key", privateKey, "--output", genomePath, "--json"]);

  const taskArguments = [
    "--harness",
    "codex",
    "--effect",
    "read-only",
    "--task",
    task,
    "--timeout",
    "60",
  ];
  const planned = invoke(["plan", ...taskArguments, "--json"]);
  const outcome = invoke([
    "run",
    ...taskArguments,
    "--execute",
    "--approved-plan-sha256",
    planned.plan.plan_sha256,
    "--genome",
    genomePath,
    "--genome-public-key",
    publicKey,
    "--ledger",
    ledgerPath,
    "--json",
  ]);
  const ledger = invoke(["ledger", "verify", "--path", ledgerPath, "--json"]);

  assert(outcome.output.trim() === "ASI_V02_CHAIN_OK", "Codex returned the wrong marker");
  assert(outcome.plan_sha256 === planned.plan.plan_sha256, "execution plan digest drifted");
  assert(outcome.isolation.enforced === true, "Codex execution was not isolated");
  assert(outcome.isolation.enforcer === "bubblewrap", "Codex used the wrong enforcer");
  assert(ledger.valid === true && ledger.events === 4, "live Bloodline did not verify");
  process.stdout.write(
    `${JSON.stringify(
      {
        marker: outcome.output.trim(),
        duration_ms: outcome.duration_ms,
        plan_sha256: outcome.plan_sha256,
        genome_sha256: outcome.genome_sha256,
        genome_key_id: outcome.genome_key_id,
        worker_version: planned.plan.harness_version,
        worker_sha256: planned.plan.executable_sha256,
        isolation_enforcer: outcome.isolation.enforcer,
        ledger_events: ledger.events,
        ledger_last_hash: ledger.last_hash,
      },
      null,
      2,
    )}\n`,
  );
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
