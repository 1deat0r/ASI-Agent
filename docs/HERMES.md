# Hermes worker installation record

ASI Agent's Hermes adapter is validated against Nous Research Hermes Agent `0.20.0` at commit `b9aa9289a8083f2e9d248ad6837b2938f5ee92d7` from `https://github.com/NousResearch/hermes-agent.git`.

`scripts/install-hermes.sh` refuses a different commit, origin, dirty source tree, `uv` build, or Python patch release. It requires exactly `uv 0.12.5 (x86_64-unknown-linux-musl)` and Python `3.13.15`, then performs `uv sync --locked --no-dev` into an ASI-owned, commit-addressed environment under `~/.local/share/asi-agent/hermes/`. An isolated `XDG_CONFIG_HOME` prevents user-level uv settings from changing resolution while preserving the checked-in Hermes project settings that produced `uv.lock`. Hermes explicitly rejects wheel builds, so its own project is installed in the supported editable mode; dependencies remain lockfile-pinned. The user-facing `~/.local/bin/hermes` is a direct symlink into that environment and does not call `mise use -g` or mutate global runtime selection.

Because the editable environment imports the pinned checkout, later source-tree modification would alter the running worker without changing the launcher fingerprint. The installer rechecks the exact commit and a completely clean worktree every time; the signed genome is a checkpoint, not continuous file-integrity monitoring.

The installation intentionally skips interactive setup, browser installation, gateways, services, and provider configuration. `hermes --help` is the non-interactive health check. A model call is not part of installation acceptance because credentials and provider choice are operator-owned.

The adapter requests `--safe-mode`, ignores user configuration and rules, limits turns, and selects Hermes' `safe` toolset. That toolset excludes terminal access but includes provider-backed web, vision, and image capabilities. Bubblewrap makes the host filesystem read-only and gives Hermes ephemeral state writes, but provider egress remains unfiltered. Therefore Hermes is a worker behind the Heart, not a trusted security boundary.

For the same reason, Hermes is explicit-use in v0.2 and is not eligible for `--harness auto`. Automatic routing can be reconsidered only after a narrower conformance-tested tool profile or independent network policy exists.
