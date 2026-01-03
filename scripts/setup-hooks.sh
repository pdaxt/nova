#!/bin/bash
# Setup Git Hooks for Nova
#
# This script configures git to use the hooks in .githooks/
# Run this once after cloning the repository.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

echo "Setting up Nova git hooks..."

# Configure git to use our hooks directory
git config core.hooksPath .githooks

# Make hooks executable
chmod +x "$REPO_ROOT/.githooks/pre-commit"
chmod +x "$REPO_ROOT/.githooks/pre-push"

echo ""
echo "Git hooks installed!"
echo ""
echo "What happens now:"
echo "  - pre-commit: Runs fmt, clippy, build, and tests before each commit"
echo "  - pre-push:   Runs security tests and size verification before push"
echo ""
echo "To skip hooks (not recommended):"
echo "  git commit --no-verify"
echo "  git push --no-verify"
echo ""
echo "To uninstall hooks:"
echo "  git config --unset core.hooksPath"
