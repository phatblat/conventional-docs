set ignore-comments
set script-interpreter := ['bash', '-eu']
set unstable

mise := "mise exec --"
hugo := "mise exec -- hugo --source site"
agents_skills_dir := env_var('HOME') + "/.agents/skills"

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
    {{ mise }} cargo fmt
    mise fmt
    just --fmt

# Remove installed dependencies
[group('configuration')]
clean: clean-rust
    rm -rf node_modules
    rm -rf site/public site/public-check site/resources site/.hugo_build.lock

# Remove the Rust build directory
[group('configuration')]
clean-rust:
    {{ mise }} cargo clean

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
# install group recipes
#

# Symlink every skill into ~/.agents/skills for local use
[group('install')]
[script]
link:
    set -euo pipefail
    mkdir -p "{{ agents_skills_dir }}"
    for src in "{{ justfile_directory() }}"/skills/*/; do
        name="$(basename "$src")"
        target="{{ agents_skills_dir }}/$name"
        if [ -e "$target" ] && [ ! -L "$target" ]; then
            echo "Refusing to overwrite non-symlink $target" >&2
            exit 1
        fi
        ln -sfn "${src%/}" "$target"
        echo "linked $name -> ${src%/}"
    done

# Remove the symlinks installed by `just link`
[group('install')]
[script]
unlink:
    set -euo pipefail
    for src in "{{ justfile_directory() }}"/skills/*/; do
        name="$(basename "$src")"
        target="{{ agents_skills_dir }}/$name"
        if [ -L "$target" ]; then
            rm "$target"
            echo "removed $target"
        elif [ -e "$target" ]; then
            echo "Refusing to remove non-symlink $target" >&2
            exit 1
        fi
    done

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

# Validate SKILL.md frontmatter against the Agent Skills spec
[group('checks')]
lint-skills:
    bun scripts/validate-skills.mjs

# Lint Rust sources and verify their formatting
[group('checks')]
lint-rust:
    {{ mise }} cargo fmt --check
    {{ mise }} cargo clippy --all-targets -- -D warnings

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

# Run every gate: formatting, markdown lint, Rust lint, link check, tests
[group('checks')]
check: format-check lint lint-skills lint-rust test test-rust

#
# site group recipes
#

# Serve the site locally with live reload at http://localhost:1313/conventional-docs/
[group('site')]
site-dev:
    {{ hugo }} server --buildDrafts=false --disableFastRender

# Build the deployable site into site/public
[group('site')]
site-build:
    {{ hugo }} --minify

# Build a root-relative copy of the site for link checking
[group('site')]
site-build-check:
    {{ hugo }} --minify --baseURL "/" --destination public-check

#
# tests group recipes
#

# Check markdown files and the built site for broken links
[group('tests')]
test: test-markdown-links test-site-links

[group('tests')]
test-markdown-links:
    bun x linkinator "*.md" "skills/**/*.md" "docs/**/*.md" --markdown

# Run the Rust test suite
[group('tests')]
test-rust:
    {{ mise }} cargo test

[group('tests')]
test-site-links: site-build-check
    bun x linkinator site/public-check --recurse --directory-listing --retry --retry-errors --retry-errors-count 3
