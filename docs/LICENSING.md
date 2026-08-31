# Licensing decision gate

Release status: BLOCKED

Owner-ratified SPDX identifier: MIT OR Apache-2.0

Ratified by owner decision, 2026-08-31. The project is licensed under both
`MIT` and `Apache-2.0`; recipients may choose either (`MIT OR Apache-2.0`),
with full texts in `LICENSE-MIT` and `LICENSE-APACHE` and the root grant in
`LICENSE`. Component-assimilation policy: this license covers ASI Agent
source, documentation, specifications, scripts, schemas, and fixtures.
Third-party harnesses, skills, models, and artifacts that the system can
inspect or invoke remain external works under their own licenses and are
not absorbed. Branding is not licensed; trademark rights are reserved.

`Release status` below still governs binary packaging and formal releases
only; publishing the public source repository does not flip it.

The repository is public as of 2026-08-31 under the ratified dual license;
the source tree may be referenced as open source under `MIT OR Apache-2.0`.
This document records an engineering release gate, not legal advice. The
`Apache-2.0` side carries the express patent grant; the `MIT` side minimizes
notice obligations. Third-party harnesses and skills are dependencies or
untrusted inputs; their licenses are not absorbed into ASI Agent merely
because the system can inspect or invoke them.

## Mechanical guardrail

`node scripts/release-readiness.mjs` requires much more than a non-empty root `LICENSE`: this document must record `Release status: APPROVED` and an owner-ratified SPDX identifier; all sixteen gates need evidence; the machine review disposition must approve public release; third-party notices, an SPDX SBOM, protected owner approval, and reproducibility evidence must exist. `node scripts/package.mjs` runs that check before creating `dist/`. CI invokes readiness in `--expect-blocked` mode, proving that v0.2 remains deliberately unreleasable. A license by itself cannot unlock packaging.
