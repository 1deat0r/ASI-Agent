import { createHash } from "node:crypto";
import {
  copyFileSync,
  existsSync,
  lstatSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function run(program, args) {
  const result = spawnSync(program, args, { cwd: root, encoding: "utf8", env: process.env });
  if (result.error) throw result.error;
  if (result.status !== 0) {
    throw new Error(`${program} ${args.join(" ")} failed\n${result.stdout}\n${result.stderr}`);
  }
}

function requirePlainPath(path, kind) {
  if (!existsSync(path)) return;
  const metadata = lstatSync(path);
  if (metadata.isSymbolicLink()) throw new Error(`refusing symbolic-link ${kind}: ${path}`);
  if (kind === "directory" && !metadata.isDirectory()) {
    throw new Error(`expected release directory: ${path}`);
  }
  if (kind === "file" && !metadata.isFile()) throw new Error(`expected release file: ${path}`);
}

function copyPlainFile(source, destination) {
  requirePlainPath(source, "file");
  if (!existsSync(source)) throw new Error(`required release file is missing: ${source}`);
  requirePlainPath(destination, "file");
  copyFileSync(source, destination);
}

// This must be the first stateful boundary: no dist directory is created until
// licensing, review, gate, SBOM, approval, and reproducibility controls pass.
run(process.execPath, [join(root, "scripts", "release-readiness.mjs")]);

const cargo = process.env.ASI_CARGO ?? "cargo";
run(cargo, ["build", "--locked", "--release"]);

const cargoManifest = readFileSync(join(root, "Cargo.toml"), "utf8");
const version = cargoManifest.match(/^version = "([^"]+)"$/m)?.[1];
if (!version) throw new Error("cannot determine package version from Cargo.toml");

const packageName = `asi-agent-v${version}-${process.platform}-${process.arch}`;
const dist = join(root, "dist");
const staging = join(dist, packageName);
const archive = join(dist, `${packageName}.tar.gz`);
requirePlainPath(dist, "directory");
mkdirSync(dist, { recursive: true });
requirePlainPath(staging, "directory");
requirePlainPath(archive, "file");
requirePlainPath(`${archive}.sha256`, "file");
rmSync(staging, { recursive: true, force: true });
rmSync(archive, { force: true });
rmSync(`${archive}.sha256`, { force: true });
mkdirSync(join(staging, "docs"), { recursive: true });
mkdirSync(join(staging, "docs", "reviews", "v0.2"), { recursive: true });
mkdirSync(join(staging, "artifacts"), { recursive: true });
copyPlainFile(join(root, "target", "release", "asi"), join(staging, "asi"));
for (const file of ["README.md", "LICENSE", "ASI-Agent-Astronomical-Plan.md", "Cargo.lock"]) {
  copyPlainFile(join(root, file), join(staging, file));
}
for (const file of [
  "ARCHITECTURE.md",
  "THREAT-MODEL.md",
  "VERIFICATION-v0.2.md",
  "LICENSING.md",
  "THIRD-PARTY-NOTICES.md",
  "REPRODUCIBILITY.md",
]) {
  copyPlainFile(join(root, "docs", file), join(staging, "docs", file));
}
for (const file of [
  "ai-research-scientist.md",
  "ai-research-scientist.json",
  "senior-llm-engineer.md",
  "senior-llm-engineer.json",
  "generative-ai-engineer.md",
  "generative-ai-engineer.json",
  "caio.md",
  "caio.json",
  "ai-solutions-architect.md",
  "ai-solutions-architect.json",
  "integrated-disposition.md",
  "review-disposition.json",
]) {
  copyPlainFile(
    join(root, "docs", "reviews", "v0.2", file),
    join(staging, "docs", "reviews", "v0.2", file),
  );
}
copyPlainFile(join(root, "artifacts", "sbom.spdx.json"), join(staging, "artifacts", "sbom.spdx.json"));
copyPlainFile(
  join(root, ".asi", "release", "approval.json"),
  join(staging, "docs", "RELEASE-APPROVAL.json"),
);
run("tar", ["-C", dist, "-czf", archive, packageName]);
rmSync(staging, { recursive: true, force: true });

const digest = createHash("sha256").update(readFileSync(archive)).digest("hex");
writeFileSync(`${archive}.sha256`, `${digest}  ${packageName}.tar.gz\n`, { flag: "wx" });
process.stdout.write(`${archive}\n`);
