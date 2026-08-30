#!/usr/bin/env bash
# pdf-engine-presentation bootstrap — installs git hooks and fetches dependencies.
set -euo pipefail

SCM_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCM_DIR/.." && pwd)"

echo "==> Installing git hooks"
git -C "$REPO_ROOT" config core.hooksPath scm/scripts/hooks
echo "    core.hooksPath -> scm/scripts/hooks (pre-commit, commit-msg)"

echo "==> Fetching dependencies"
(cd "$SCM_DIR" && cargo fetch --locked)

echo "Bootstrap complete."
