import { existsSync, lstatSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

function nonEmptyFile(path) {
  if (!existsSync(path)) return false;
  const metadata = lstatSync(path);
  return !metadata.isSymbolicLink() && metadata.isFile() && readFileSync(path, "utf8").trim().length > 0;
}

function checkedGateIds(text) {
  const matches = [...text.matchAll(/^- \[([ x])\] (G\d+):/gm)];
  return matches
    .map((match, index) => ({
      checked: match[1] === "x",
      id: match[2],
      block: text.slice(match.index, matches[index + 1]?.index ?? text.length),
    }))
    .filter((gate) => gate.checked && /^  EVIDENCE: (?!pending\s*$).+/m.test(gate.block))
    .map((gate) => gate.id);
}

export function releaseBlockers(candidateRoot) {
  const root = resolve(candidateRoot);
  const blockers = [];
  const licensePath = join(root, "LICENSE");
  if (!nonEmptyFile(licensePath)) blockers.push("owner-selected LICENSE is missing");

  const licensingPath = join(root, "docs", "LICENSING.md");
  const licensing = nonEmptyFile(licensingPath) ? readFileSync(licensingPath, "utf8") : "";
  if (!/^Release status: APPROVED$/m.test(licensing)) {
    blockers.push("docs/LICENSING.md lacks owner-ratified Release status: APPROVED");
  }
  const SPDX_TOKENS = new Set(["MIT", "Apache-2.0", "OR", "AND", "WITH"]);
  const spdxRaw = licensing.match(/^Owner-ratified SPDX identifier: (.+)$/m)?.[1]?.trim();
  const spdxValid = spdxRaw != null && spdxRaw.split(/\s+/).every((token) => SPDX_TOKENS.has(token));
  if (!spdxValid) {
    blockers.push("docs/LICENSING.md lacks an owner-ratified SPDX identifier");
  }

  if (!nonEmptyFile(join(root, "docs", "THIRD-PARTY-NOTICES.md"))) {
    blockers.push("third-party notices are missing");
  }
  const sbomPath = join(root, "artifacts", "sbom.spdx.json");
  if (!nonEmptyFile(sbomPath)) {
    blockers.push("SPDX SBOM is missing");
  } else {
    try {
      const sbom = JSON.parse(readFileSync(sbomPath, "utf8"));
      if (!sbom.spdxVersion || !Array.isArray(sbom.packages)) blockers.push("SPDX SBOM is incomplete");
    } catch {
      blockers.push("SPDX SBOM is not valid JSON");
    }
  }

  const dispositionPath = join(root, "docs", "reviews", "v0.2", "review-disposition.json");
  if (!nonEmptyFile(dispositionPath)) {
    blockers.push("machine-readable independent-review disposition is missing");
  } else {
    try {
      const disposition = JSON.parse(readFileSync(dispositionPath, "utf8"));
      if (disposition.public_release_status !== "approved") {
        blockers.push("independent-review disposition does not approve public release");
      }
    } catch {
      blockers.push("independent-review disposition is not valid JSON");
    }
  }

  const gatesPath = join(root, "GATES.md");
  if (!nonEmptyFile(gatesPath)) {
    blockers.push("gate evidence is missing");
  } else {
    const gateIds = checkedGateIds(readFileSync(gatesPath, "utf8"));
    const expected = Array.from({ length: 16 }, (_, index) => `G${index + 1}`);
    if (JSON.stringify(gateIds) !== JSON.stringify(expected)) {
      blockers.push("all sixteen gates are not checked with evidence");
    }
  }

  const approvalPath = join(root, ".asi", "release", "approval.json");
  if (!nonEmptyFile(approvalPath)) {
    blockers.push("protected owner release approval is missing");
  } else {
    try {
      const approval = JSON.parse(readFileSync(approvalPath, "utf8"));
      if (
        approval.approved !== true ||
        typeof approval.approved_by !== "string" ||
        !/^[0-9a-f]{64}$/.test(approval.source_sha256 ?? "")
      ) {
        blockers.push("protected owner release approval is incomplete");
      }
    } catch {
      blockers.push("protected owner release approval is not valid JSON");
    }
  }

  if (!nonEmptyFile(join(root, "docs", "REPRODUCIBILITY.md"))) {
    blockers.push("release reproducibility evidence is missing");
  }
  return blockers;
}

function main() {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
  const expectBlocked = process.argv.includes("--expect-blocked");
  const blockers = releaseBlockers(root);
  if (expectBlocked) {
    if (blockers.length === 0) throw new Error("release is ready; remove --expect-blocked");
    process.stdout.write(`release correctly blocked:\n- ${blockers.join("\n- ")}\n`);
    return;
  }
  if (blockers.length > 0) throw new Error(`release blocked:\n- ${blockers.join("\n- ")}`);
  process.stdout.write("release readiness passed: all owner, evidence, review, SBOM, and approval controls are present\n");
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) main();
