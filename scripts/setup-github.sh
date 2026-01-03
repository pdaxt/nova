#!/bin/bash
# Setup GitHub repository with proper branch protection
# Run this after creating the repo with: gh repo create

set -e

OWNER="nova-lang"  # Change to your org/username
REPO="nova"
BRANCH="main"

echo "Setting up branch protection for $OWNER/$REPO..."

# Create branch protection rule via GitHub API
gh api \
  --method PUT \
  -H "Accept: application/vnd.github+json" \
  "/repos/$OWNER/$REPO/branches/$BRANCH/protection" \
  -f required_status_checks='{"strict":true,"contexts":["test"]}' \
  -f enforce_admins=false \
  -f required_pull_request_reviews='{"dismiss_stale_reviews":true,"require_code_owner_reviews":true,"required_approving_review_count":1}' \
  -f restrictions=null \
  -F allow_force_pushes=false \
  -F allow_deletions=false \
  -F required_linear_history=true

echo "Branch protection configured!"
echo ""
echo "Settings applied:"
echo "  ✓ Require PR for all changes"
echo "  ✓ Require CODEOWNER review (you)"
echo "  ✓ Require status checks to pass"
echo "  ✓ Dismiss stale reviews on new commits"
echo "  ✓ Block force pushes"
echo "  ✓ Block branch deletion"
echo "  ✓ Require linear history"
echo ""
echo "You (@pdaxt) are the sole approver via CODEOWNERS."
