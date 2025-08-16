#!/usr/bin/env bash

set -euo pipefail

# ========= Config (edit as needed) =========
TARGET="aarch64-unknown-linux-gnu"
BIN="client"
OUTDIR="thermostat-client"
CLIENT_SERVICE="thermostat_client.service"
REMOTE_HOST="braxt@192.168.0.130"
REMOTE_DIR="~/" # where OUTDIR will be copied on the Pi (remote)
PROFILE="debug" # default, can be overridden by args
RUN_REMOTE=true # default, can be disabled with --no-run
# ==========================================

# Parse command line arguments
for arg in "$@"; do
    case "$arg" in
    release)
        PROFILE="release"
        ;;
    --no-run)
        RUN_REMOTE=false
        ;;
    *)
        echo "[WARN] Unknown argument: $arg"
        ;;
    esac
done

ARTIFACT="target/${TARGET}/${PROFILE}/${BIN}"
if [[ "$PROFILE" == "release" ]]; then
    RELEASE_FLAG="--release"
else
    RELEASE_FLAG=""
fi

# Detect if the script is being sourced
is_sourced() { [[ "${BASH_SOURCE[0]}" != "$0" ]]; }

# "die" should NOT kill the parent shell if sourced
die() {
    echo "[ERROR] $*" >&2
    if is_sourced; then
        return 1 # don't nuke the user’s shell
    else
        exit 1
    fi
}

need_cmd() {
    command -v "$1" >/dev/null 2>&1 || die "'$1' not found. Please install it and ensure it's on PATH."
}

echo "==> Checking required tools..."
need_cmd cargo
need_cmd rustup
need_cmd scp
need_cmd ssh

# Ensure Rust target is installed
if ! rustup target list --installed | grep -q "^${TARGET}\$"; then
    echo "==> Installing Rust target '${TARGET}' ..."
    rustup target add "${TARGET}" || die "Failed to add Rust target ${TARGET}"
fi

# For glibc-based aarch64 target, ensure the system linker exists and tell Cargo to use it
LINKER_ENV=""
if [[ "${TARGET}" == "aarch64-unknown-linux-gnu" ]]; then
    if ! command -v aarch64-linux-gnu-gcc >/dev/null 2>&1; then
        cat >&2 <<EOF
[ERROR] Required linker 'aarch64-linux-gnu-gcc' not found.
        On Ubuntu, install it with:
            sudo apt update && sudo apt install gcc-aarch64-linux-gnu
EOF
        exit 1
    fi
    # Tell Cargo which linker to use for this target without requiring a config file.
    LINKER_ENV="CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc"
fi

echo "==> Building (${PROFILE}) for ${TARGET} with cargo ..."
# shellcheck disable=SC2086
env ${LINKER_ENV} cargo build --bin "${BIN}" ${RELEASE_FLAG} --target "${TARGET}" || die "Build failed."

if [[ ! -f "${ARTIFACT}" ]]; then
    cat >&2 <<EOF
[ERROR] Build artifact not found: "${ARTIFACT}"
        Ensure BIN="${BIN}" matches your binary name.
EOF
    exit 1
fi

echo "==> Preparing local output folder \"${OUTDIR}\" ..."
mkdir -p "${OUTDIR}"

echo "==> Copying \"${ARTIFACT}\" to \"${OUTDIR}/${BIN}\" ..."
cp -f "${ARTIFACT}" "${OUTDIR}/${BIN}" || die "Copy failed."

echo "==> Copying \"${CLIENT_SERVICE}\" to \"${OUTDIR}/${CLIENT_SERVICE}\" ..."
cp -f "${CLIENT_SERVICE}" "${OUTDIR}/${CLIENT_SERVICE}" || die "Copy failed."

echo "==> Renaming remote file with .old"
ssh "${REMOTE_HOST}" "sudo mv -f ./${OUTDIR}/* ${OUTDIR}.old" || echo "Oh well, continuing"

echo "==> Uploading \"${OUTDIR}\" to ${REMOTE_HOST}:${REMOTE_DIR} ..."
# keep REMOTE_DIR unquoted on the remote side so ~ expands there
scp -r "${OUTDIR}" "${REMOTE_HOST}:${REMOTE_DIR}" || die "scp failed."

echo "==> Running \"${CLIENT_SERVICE}\" on ${REMOTE_HOST} ..."
REMOTE_PATH="${REMOTE_DIR%/}/${OUTDIR}"
REMOTE_CMD="mkdir -p ${REMOTE_PATH} && cd ${REMOTE_PATH} && chmod +x ${BIN} && \
    sudo cp ./${CLIENT_SERVICE} /etc/systemd/system/${CLIENT_SERVICE} && \
    sudo systemctl daemon-reload && \
    sudo systemctl enable ${CLIENT_SERVICE} && \
    sudo systemctl restart ${CLIENT_SERVICE}"
REMOTE_ENV="BIN=${BIN}"

set +e
ssh "${REMOTE_HOST}" "${REMOTE_ENV} ${REMOTE_CMD}"
rc=$?
set -e

if $RUN_REMOTE; then
    echo "==> Following \"${CLIENT_SERVICE}\" on ${REMOTE_HOST} ..."

    set +e
    ssh "${REMOTE_HOST}" "sudo journalctl -u ${CLIENT_SERVICE} -f"
    rc=0
    set -e

    echo "==> Remote program exited with code ${rc}."
else
    echo "==> Skipping remote execution (--no-run specified)."
    rc=0
fi

rc=${rc:-0}
if is_sourced; then
    return "$rc"
else
    exit "$rc"
fi
