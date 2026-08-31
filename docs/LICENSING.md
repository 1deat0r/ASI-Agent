# Licensing decision gate

Release status: BLOCKED

Owner-ratified SPDX identifier: PENDING

An explicit owner decision is still required.

ASI Agent intentionally has no `LICENSE` file yet. Do not publish, redistribute, or describe the repository as open source until the owner explicitly selects and ratifies a license. This document records an engineering release gate, not legal advice.

The practical candidates are:

- `Apache-2.0`: explicit patent terms and contributor protections, with more notice obligations.
- `MIT`: short and permissive, with no express patent grant.
- `MIT OR Apache-2.0`: lets downstream users choose either set of terms, at the cost of maintaining both notices.

The owner should also decide whether imported adapters, documentation, fixtures, branding, and future model artifacts share one license or require separate notices. Third-party harnesses and skills are dependencies or untrusted inputs; their licenses are not absorbed into ASI Agent merely because the system can inspect or invoke them.

## Mechanical guardrail

`node scripts/release-readiness.mjs` requires much more than a non-empty root `LICENSE`: this document must record `Release status: APPROVED` and an owner-ratified SPDX identifier; all sixteen gates need evidence; the machine review disposition must approve public release; third-party notices, an SPDX SBOM, protected owner approval, and reproducibility evidence must exist. `node scripts/package.mjs` runs that check before creating `dist/`. CI invokes readiness in `--expect-blocked` mode, proving that v0.2 remains deliberately unreleasable. A license by itself cannot unlock packaging.
