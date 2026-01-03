#!/bin/bash
# Run All Nova Tests
#
# This runs the same checks that CI runs.
# Use this before pushing to catch issues early.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$REPO_ROOT/bootstrap"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║                    Nova Test Suite                         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════╝${NC}"

# Track results
PASSED=0
FAILED=0

run_check() {
    local name="$1"
    local cmd="$2"

    echo -e "\n${YELLOW}▶ $name${NC}"
    if eval "$cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}"
        PASSED=$((PASSED + 1))
    else
        echo -e "${RED}✗ FAILED${NC}"
        FAILED=$((FAILED + 1))
    fi
}

# Core checks
run_check "Formatting (cargo fmt)" "cargo fmt -- --check"
run_check "Linting (cargo clippy)" "cargo clippy -- -D warnings"
run_check "Build (cargo build)" "cargo build"
run_check "Tests (cargo test)" "cargo test"

# Security checks
run_check "Span Security Tests" "cargo test span_attack"
run_check "Token Security Tests" "cargo test token_attack"

# Size guarantees
run_check "Token Size = 12 bytes" "cargo test token_is_12_bytes"
run_check "TokenKind Size = 1 byte" "cargo test token_kind_is_1_byte"
run_check "Span Size = 8 bytes" "cargo test span_is_8_bytes"

# Documentation
run_check "Documentation builds" "cargo doc --no-deps"

# Summary
echo -e "\n${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}Passed: $PASSED${NC}  ${RED}Failed: $FAILED${NC}"

if [ $FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All checks passed! Ready to commit/push.${NC}"
    exit 0
else
    echo -e "\n${RED}Some checks failed. Please fix before committing.${NC}"
    exit 1
fi
