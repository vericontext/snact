#!/bin/bash
set -e

# Auto-release hook: after a git commit, check if it should trigger a release.
#
# Triggers on commits with messages starting with:
#   feat:  → minor bump (0.1.0 → 0.2.0)
#   fix:   → patch bump (0.1.0 → 0.1.1)
#   release: vX.Y.Z → exact version
#
# Reads current version from the latest git tag, bumps accordingly,
# creates a new tag, and pushes it to trigger the GitHub Actions release.

INPUT=$(cat)

# Extract the command from PostToolUse stdin JSON (structure: {tool_input: {command: "..."}})
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only act on git commit commands
if ! echo "$COMMAND" | grep -q "git commit"; then
    exit 0
fi

# Get the latest commit message
COMMIT_MSG=$(git log -1 --pretty=%s 2>/dev/null || true)
[ -z "$COMMIT_MSG" ] && exit 0

# Determine bump type from conventional commit prefix
BUMP=""
EXACT_VERSION=""

case "$COMMIT_MSG" in
    release:\ v*)
        EXACT_VERSION=$(echo "$COMMIT_MSG" | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1)
        ;;
    feat:*|feat\(*)
        BUMP="minor"
        ;;
    fix:*|fix\(*)
        BUMP="patch"
        ;;
    *)
        # Not a release-triggering commit
        exit 0
        ;;
esac

# Get current version from latest tag
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
CURRENT="${LATEST_TAG#v}"

# Parse semver
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT"
MAJOR=${MAJOR:-0}
MINOR=${MINOR:-0}
PATCH=${PATCH:-0}

# Compute new version
if [ -n "$EXACT_VERSION" ]; then
    NEW_TAG="$EXACT_VERSION"
elif [ "$BUMP" = "minor" ]; then
    NEW_TAG="v${MAJOR}.$((MINOR + 1)).0"
elif [ "$BUMP" = "patch" ]; then
    NEW_TAG="v${MAJOR}.${MINOR}.$((PATCH + 1))"
else
    exit 0
fi

# Check if tag already exists
if git rev-parse "$NEW_TAG" >/dev/null 2>&1; then
    echo "Tag $NEW_TAG already exists, skipping" >&2
    exit 0
fi

# Create and push tag
git tag "$NEW_TAG" >/dev/null 2>&1
echo "Created tag: $NEW_TAG" >&2

if git push origin "$NEW_TAG" >/dev/null 2>&1; then
    echo "Pushed tag: $NEW_TAG → GitHub Actions will build the release" >&2
else
    echo "Tag $NEW_TAG created locally (push failed — run 'git push origin $NEW_TAG' manually)" >&2
fi

exit 0
