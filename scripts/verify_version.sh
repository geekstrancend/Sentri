#!/usr/bin/env bash
set -euo pipefail

# Script: verify_version.sh
# Purpose: Validate version consistency before release
# Usage: ./scripts/verify_version.sh <tag_ref>

TAG_REF="${1:-${GITHUB_REF:-}}"

if [ -z "$TAG_REF" ]; then
  echo "ERROR: GITHUB_REF not set and no argument provided"
  exit 1
fi

# Extract version from tag ref (e.g., refs/tags/v1.2.3 -> 1.2.3)
TAG_VERSION="${TAG_REF#refs/tags/v}"

if [ "$TAG_VERSION" = "$TAG_REF" ]; then
  echo "ERROR: Invalid tag format: $TAG_REF"
  echo "Expected format: refs/tags/vX.Y.Z or refs/tags/vX.Y.Z-beta"
  exit 1
fi

echo "Tag version: $TAG_VERSION"

# Extract version from Cargo.toml
CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

if [ -z "$CARGO_VERSION" ]; then
  echo "ERROR: Could not extract version from Cargo.toml"
  exit 1
fi

echo "Cargo.toml version: $CARGO_VERSION"

# Verify exact match
if [ "$TAG_VERSION" != "$CARGO_VERSION" ]; then
  echo "ERROR: Version mismatch!"
  echo "  Tag version: $TAG_VERSION"
  echo "  Cargo version: $CARGO_VERSION"
  exit 1
fi

echo "✓ Versions match: $TAG_VERSION"

# Verify semantic versioning format
# Pattern: X.Y.Z or X.Y.Z-beta where X, Y, Z are integers
if ! echo "$TAG_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$'; then
  echo "ERROR: Invalid semantic version format: $TAG_VERSION"
  echo "Expected format: X.Y.Z or X.Y.Z-beta"
  exit 1
fi

echo "✓ Valid semantic version format: $TAG_VERSION"

# Extract major, minor, patch for comparison
MAJOR=$(echo "$TAG_VERSION" | cut -d. -f1)
MINOR=$(echo "$TAG_VERSION" | cut -d. -f2)
PATCH=$(echo "$TAG_VERSION" | cut -d. -f3 | cut -d- -f1)

echo "Version components: MAJOR=$MAJOR MINOR=$MINOR PATCH=$PATCH"

# Check if this is a prerelease
IS_PRERELEASE=false
if echo "$TAG_VERSION" | grep -q "-"; then
  IS_PRERELEASE=true
  PRERELEASE=$(echo "$TAG_VERSION" | cut -d- -f2)
  echo "✓ Prerelease detected: $PRERELEASE"
fi

# Get list of existing tags and verify new version is higher
echo ""
echo "Checking version history..."

# Get all existing version tags (excluding current tag)
EXISTING_VERSIONS=$(git tag -l 'v[0-9]*.[0-9]*.[0-9]*' 2>/dev/null | grep -v "^v${TAG_VERSION}$" || echo "")

if [ -z "$EXISTING_VERSIONS" ]; then
  echo "✓ First release: $TAG_VERSION"
else
  # Find the highest existing version (excluding current tag)
  LATEST_VERSION=$(echo "$EXISTING_VERSIONS" | sed 's/^v//' | sort -V | tail -1)
  LATEST_MAJOR=$(echo "$LATEST_VERSION" | cut -d. -f1)
  LATEST_MINOR=$(echo "$LATEST_VERSION" | cut -d. -f2)
  LATEST_PATCH=$(echo "$LATEST_VERSION" | cut -d. -f3 | cut -d- -f1)
  
  echo "Latest published version: $LATEST_VERSION"
  echo "Current release version: $TAG_VERSION"
  
  # Compare versions (simple numeric comparison)
  if [ "$MAJOR" -lt "$LATEST_MAJOR" ]; then
    echo "ERROR: New version ($TAG_VERSION) is lower than latest ($LATEST_VERSION)"
    exit 1
  elif [ "$MAJOR" -eq "$LATEST_MAJOR" ]; then
    if [ "$MINOR" -lt "$LATEST_MINOR" ]; then
      echo "ERROR: New version ($TAG_VERSION) is lower than latest ($LATEST_VERSION)"
      exit 1
    elif [ "$MINOR" -eq "$LATEST_MINOR" ]; then
      if [ "$PATCH" -le "$LATEST_PATCH" ]; then
        echo "ERROR: New version ($TAG_VERSION) is not higher than latest ($LATEST_VERSION)"
        exit 1
      fi
    fi
  fi
  
  echo "✓ Version is properly incremented"
fi

# Verify required project files exist
echo ""
echo "Checking project files..."

REQUIRED_FILES=(
  "Cargo.toml"
  "Cargo.lock"
  "LICENSE"
  "README.md"
  "CHANGELOG.md"
  "rust-toolchain.toml"
)

for file in "${REQUIRED_FILES[@]}"; do
  if [ ! -f "$file" ]; then
    echo "ERROR: Required file not found: $file"
    exit 1
  fi
  echo "✓ Found: $file"
done

# Verify CHANGELOG has entry for this version
echo ""
echo "Checking CHANGELOG..."

if ! grep -q "## \[$TAG_VERSION\]" CHANGELOG.md; then
  echo "ERROR: No CHANGELOG entry found for version $TAG_VERSION"
  echo "Add an entry with format: ## [$TAG_VERSION] - YYYY-MM-DD"
  exit 1
fi

echo "✓ CHANGELOG entry found for $TAG_VERSION"

# Verify git working directory is clean
echo ""
echo "Checking git status..."

if [ -n "$(git status --porcelain)" ]; then
  echo "ERROR: Working directory is not clean"
  git status
  exit 1
fi

echo "✓ Working directory is clean"

# Verify tag points to current HEAD
CURRENT_COMMIT=$(git rev-parse HEAD)
TAG_COMMIT=$(git rev-list -n 1 "v$TAG_VERSION" 2>/dev/null || echo "")

if [ -n "$TAG_COMMIT" ] && [ "$TAG_COMMIT" != "$CURRENT_COMMIT" ]; then
  echo "WARNING: Tag v$TAG_VERSION already exists at different commit"
  echo "  Tag commit: $TAG_COMMIT"
  echo "  Current commit: $CURRENT_COMMIT"
  echo "  This is normal for CI runs after tag push"
fi

# Final summary
echo ""
echo "========================================="
echo "✓ Version validation successful!"
echo "========================================="
echo "Release version: $TAG_VERSION"
echo "Format: Semantic versioning"
echo "Status: Ready for release"
echo ""

# Export for GitHub Actions
echo "VERSION=$TAG_VERSION" >> "${GITHUB_ENV:-/dev/null}"
exit 0
