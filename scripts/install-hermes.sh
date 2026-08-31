#!/usr/bin/env bash
set -euo pipefail

readonly ASI_HERMES_PIN="b9aa9289a8083f2e9d248ad6837b2938f5ee92d7"
readonly ASI_HERMES_EXPECTED_ORIGIN="https://github.com/NousResearch/hermes-agent.git"
readonly ASI_UV_EXPECTED_VERSION="uv 0.12.5 (x86_64-unknown-linux-musl)"
readonly ASI_PYTHON_EXPECTED_VERSION="Python 3.13.15"
readonly ASI_HERMES_SOURCE_DIR="${ASI_HERMES_SOURCE:-/run/media/mustbearnold/Projects/Research/hermes-agent-repo}"
readonly ASI_UV_BIN="${ASI_UV:-$(command -v uv || true)}"
readonly ASI_INSTALL_ROOT="${ASI_HERMES_INSTALL_ROOT:-${HOME:?}/.local/share/asi-agent/hermes/${ASI_HERMES_PIN}}"
readonly ASI_HERMES_ENV="${ASI_INSTALL_ROOT}/venv"
readonly ASI_HERMES_LINK="${HOME:?}/.local/bin/hermes"

if [[ -z "${ASI_UV_BIN}" || ! -x "${ASI_UV_BIN}" ]]; then
  printf '%s\n' "error: uv is required; set ASI_UV to an executable pinned uv installation" >&2
  exit 1
fi
actual_uv_version="$("${ASI_UV_BIN}" --version)"
if [[ "${actual_uv_version}" != "${ASI_UV_EXPECTED_VERSION}" ]]; then
  printf 'error: uv is %s, expected exactly %s\n' \
    "${actual_uv_version}" "${ASI_UV_EXPECTED_VERSION}" >&2
  exit 1
fi
if [[ ! -d "${ASI_HERMES_SOURCE_DIR}/.git" ]]; then
  printf 'error: Hermes source checkout not found: %s\n' "${ASI_HERMES_SOURCE_DIR}" >&2
  exit 1
fi

actual_commit="$(git -C "${ASI_HERMES_SOURCE_DIR}" rev-parse HEAD)"
actual_origin="$(git -C "${ASI_HERMES_SOURCE_DIR}" remote get-url origin)"
source_status="$(git -C "${ASI_HERMES_SOURCE_DIR}" status --porcelain=v1)"
if [[ "${actual_commit}" != "${ASI_HERMES_PIN}" ]]; then
  printf 'error: Hermes source is %s, expected pinned commit %s\n' "${actual_commit}" "${ASI_HERMES_PIN}" >&2
  exit 1
fi
if [[ "${actual_origin}" != "${ASI_HERMES_EXPECTED_ORIGIN}" ]]; then
  printf 'error: Hermes origin is %s, expected %s\n' "${actual_origin}" "${ASI_HERMES_EXPECTED_ORIGIN}" >&2
  exit 1
fi
if [[ -n "${source_status}" ]]; then
  printf '%s\n' "error: Hermes source checkout is not clean" >&2
  printf '%s\n' "${source_status}" >&2
  exit 1
fi

mkdir -p "${ASI_INSTALL_ROOT}/uv-config" "$(dirname "${ASI_HERMES_LINK}")"
env -u PYTHONPATH -u PYTHONHOME \
  XDG_CONFIG_HOME="${ASI_INSTALL_ROOT}/uv-config" \
  UV_PROJECT_ENVIRONMENT="${ASI_HERMES_ENV}" \
  "${ASI_UV_BIN}" sync \
    --project "${ASI_HERMES_SOURCE_DIR}" \
    --locked \
    --no-dev \
    --python 3.13.15

if [[ ! -x "${ASI_HERMES_ENV}/bin/hermes" ]]; then
  printf '%s\n' "error: locked Hermes environment did not produce bin/hermes" >&2
  exit 1
fi
actual_python_version="$("${ASI_HERMES_ENV}/bin/python" --version)"
if [[ "${actual_python_version}" != "${ASI_PYTHON_EXPECTED_VERSION}" ]]; then
  printf 'error: Hermes Python is %s, expected exactly %s\n' \
    "${actual_python_version}" "${ASI_PYTHON_EXPECTED_VERSION}" >&2
  exit 1
fi
if [[ -e "${ASI_HERMES_LINK}" || -L "${ASI_HERMES_LINK}" ]]; then
  existing_target="$(readlink -f "${ASI_HERMES_LINK}" || true)"
  case "${existing_target}" in
    "${HOME:?}/.local/share/asi-agent/hermes/"*) ;;
    *)
      printf 'error: refusing to replace unrelated Hermes executable: %s -> %s\n' \
        "${ASI_HERMES_LINK}" "${existing_target}" >&2
      exit 1
      ;;
  esac
fi
ln -sfn "${ASI_HERMES_ENV}/bin/hermes" "${ASI_HERMES_LINK}"

"${ASI_HERMES_LINK}" --help >/dev/null
git -C "${ASI_HERMES_SOURCE_DIR}" diff --quiet
git -C "${ASI_HERMES_SOURCE_DIR}" diff --cached --quiet
test -z "$(git -C "${ASI_HERMES_SOURCE_DIR}" status --porcelain=v1)"

printf 'Hermes Agent installed from %s\n' "${ASI_HERMES_PIN}"
printf 'uv: %s\n' "${actual_uv_version}"
printf 'python: %s\n' "${actual_python_version}"
printf 'environment: %s\n' "${ASI_HERMES_ENV}"
printf 'executable: %s\n' "${ASI_HERMES_LINK}"
