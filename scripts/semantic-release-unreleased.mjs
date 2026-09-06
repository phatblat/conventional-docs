import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';

const CHANGELOG = 'CHANGELOG.md';
const UNRELEASED = '## [Unreleased]';
const VERSION_HEADING_RE = /^## \[/;
const LINK_DEF_RE = /^\[[^\]]+\]:\s/;

function changelogPath(context) {
  return path.join(context.cwd ?? process.cwd(), CHANGELOG);
}

/** `https://host/owner/repo` from any of git's URL spellings. */
function webUrl(repositoryUrl) {
  const stripped = repositoryUrl
    .replace(/^git\+/, '')
    .replace(/^ssh:\/\//, '')
    .replace(/^git@([^:/]+):/, 'https://$1/')
    .replace(/\.git$/, '');
  return stripped.startsWith('http') ? stripped : `https://${stripped}`;
}

function trimBlank(lines) {
  let start = 0;
  let end = lines.length;
  while (start < end && lines[start].trim() === '') start += 1;
  while (end > start && lines[end - 1].trim() === '') end -= 1;
  return lines.slice(start, end);
}

/** Head, `[Unreleased]` body, released sections, and reference definitions. */
function parse(content) {
  const lines = content.replace(/\n+$/, '').split('\n');
  const defsStart = lines.findIndex((line) => LINK_DEF_RE.test(line));
  const prose = defsStart === -1 ? lines : lines.slice(0, defsStart);
  const defs = defsStart === -1 ? [] : lines.slice(defsStart);

  const unreleased = prose.findIndex((line) => line.trim() === UNRELEASED);
  if (unreleased === -1) {
    throw new Error(`${CHANGELOG}: no "${UNRELEASED}" heading`);
  }

  let next = prose.length;
  for (let i = unreleased + 1; i < prose.length; i += 1) {
    if (VERSION_HEADING_RE.test(prose[i])) {
      next = i;
      break;
    }
  }

  return {
    head: trimBlank(prose.slice(0, unreleased)),
    body: trimBlank(prose.slice(unreleased + 1, next)),
    released: trimBlank(prose.slice(next)),
    defs: trimBlank(defs),
  };
}

/**
 * semantic-release `generateNotes` step: this release's notes are the curated
 * `[Unreleased]` section, so the changelog is the source and the release notes
 * are drawn from it.
 */
export async function generateNotes(pluginConfig, context) {
  const { body } = parse(await readFile(changelogPath(context), 'utf8'));
  return body.join('\n');
}

/**
 * semantic-release `prepare` step: rename `[Unreleased]` to the new version in
 * both the heading and its reference link, and open a fresh empty
 * `[Unreleased]` pointing at `HEAD`. Must run before `@semantic-release/git`'s
 * `prepare` so the rewrite is staged in the release commit — keep this plugin
 * earlier than `@semantic-release/git` in `.releaserc.json`'s `plugins` array.
 */
export async function prepare(pluginConfig, context) {
  const { nextRelease, lastRelease, options, logger } = context;
  const file = changelogPath(context);
  const { head, body, released, defs } = parse(await readFile(file, 'utf8'));

  const date = new Date().toISOString().slice(0, 10);
  const base = webUrl(options.repositoryUrl);
  const versionLink = lastRelease.gitTag
    ? `${base}/compare/${lastRelease.gitTag}...${nextRelease.gitTag}`
    : `${base}/releases/tag/${nextRelease.gitTag}`;

  const out = [
    ...head,
    '',
    UNRELEASED,
    '',
    `## [${nextRelease.version}] - ${date}`,
    ...(body.length > 0 ? ['', ...body] : []),
    ...(released.length > 0 ? ['', ...released] : []),
    '',
    `[Unreleased]: ${base}/compare/${nextRelease.gitTag}...HEAD`,
    `[${nextRelease.version}]: ${versionLink}`,
    ...defs.filter((line) => !/^\[unreleased\]:/i.test(line)),
    '',
  ];

  await writeFile(file, out.join('\n'));
  logger.log(
    'Promoted %s to %s in %s',
    UNRELEASED,
    nextRelease.version,
    CHANGELOG,
  );
}
