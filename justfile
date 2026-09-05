set ignore-comments
set script-interpreter := ['bash', '-eu']
set unstable

mise := "mise exec --"

[default]
_default:
    @just --list

#
# configuration group recipes
#

# Install pinned tools and dependencies
[group('configuration')]
deps:
    mise install
    mise deps

# Format markdown and config files, then the justfile
[group('configuration')]
format:
    {{ mise }} prettier --write .
    mise fmt
    just --fmt

# Remove installed dependencies
[group('configuration')]
clean:
    rm -rf node_modules

# Report tools and dependencies with newer versions available
[group('configuration')]
outdated:
    -mise outdated --local --bump
    -bun outdated

# Upgrade pinned tools and dependencies to their latest versions
[group('configuration')]
upgrade:
    mise upgrade --local --bump --yes
    bun update --latest

#
# checks group recipes
#

# Verify formatting without writing changes
[group('checks')]
format-check:
    {{ mise }} prettier --check .
    mise fmt --check
    just --fmt --check

# Lint markdown structure
[group('checks')]
lint:
    {{ mise }} markdownlint-cli2 "**/*.md"

# Validate release-note fragment format in .changes/
[group('checks')]
lint-changes:
    bun scripts/validate-changes.mjs

# Lint commit messages in a range (defaults to auto-detected base..HEAD)
[group('checks')]
[script]
commitlint from="" to="HEAD":
    set -euo pipefail
    base="{{ from }}"
    if [ -z "$base" ]; then
      if git rev-parse HEAD~1 >/dev/null 2>&1; then
        base=HEAD~1
      else
        base=$(git rev-list --max-parents=0 HEAD)
      fi
    fi
    bun x commitlint --from "$base" --to {{ to }} --verbose

# Run every gate: formatting, markdown lint, link check
[group('checks')]
check: format-check lint lint-changes test

#
# tests group recipes
#

# Check markdown files for broken links
[group('tests')]
test:
    bun x linkinator "*.md" --markdown
