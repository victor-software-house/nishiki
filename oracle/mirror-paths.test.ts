// Tests of `mirroredPaths`, run by `mise run test:scripts`.
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

import { mirroredPaths } from "./mirror-paths.ts";

/** A mirror with one input, the mirror's own files, and `.git` made by
 * `plantGit`. */
function plantedMirror(plantGit: (mirror: string) => void): string {
  const mirror = mkdtempSync(join(tmpdir(), "nishiki-sources-"));
  const input = join(mirror, "owner", "repo", "0123abcd");
  mkdirSync(input, { recursive: true });
  writeFileSync(join(input, "page.html"), "<p>a</p>\n");
  for (const own of ["README.md", "AGENTS.md", "manifest.toml", ".gitattributes"])
    writeFileSync(join(mirror, own), "");
  mkdirSync(join(mirror, ".github", "workflows"), { recursive: true });
  writeFileSync(join(mirror, ".github", "workflows", "check.yml"), "");
  plantGit(mirror);
  return mirror;
}

test("a clone's .git directory is left out", () => {
  const mirror = plantedMirror((root) => {
    mkdirSync(join(root, ".git", "objects"), { recursive: true });
    writeFileSync(join(root, ".git", "HEAD"), "ref: refs/heads/main\n");
  });
  expect(mirroredPaths(mirror)).toEqual(["owner/repo/0123abcd/page.html"]);
});

test("a worktree's .git file is left out", () => {
  const mirror = plantedMirror((root) => {
    writeFileSync(join(root, ".git"), "gitdir: /elsewhere/.git/worktrees/mirror\n");
  });
  expect(mirroredPaths(mirror)).toEqual(["owner/repo/0123abcd/page.html"]);
});

test("a file whose name starts with .git is still listed", () => {
  const mirror = plantedMirror((root) => {
    writeFileSync(join(root, ".git"), "gitdir: /elsewhere\n");
    writeFileSync(join(root, "owner", "repo", "0123abcd", ".gitignore"), "");
  });
  expect(mirroredPaths(mirror).sort()).toEqual([
    "owner/repo/0123abcd/.gitignore",
    "owner/repo/0123abcd/page.html",
  ]);
});
