// Lists the files of a nishiki-sources checkout that `manifest.toml` must
// have a row for. `sources.ts check` reads it.
import { readdirSync } from "node:fs";
import { join, relative } from "node:path";

/** Every file in the mirror but its own README, guide, manifest, Git
 * attributes, `.github` directory and `.git`.
 * In a clone `.git` is a directory; in a worktree it is a file. */
export function mirroredPaths(mirror: string): string[] {
  const ownFiles = new Set(["README.md", "AGENTS.md", "manifest.toml", ".gitattributes"]);
  const ownDirectories = new Set([".git", ".github"]);
  return readdirSync(mirror, { recursive: true, withFileTypes: true })
    .filter((found) => found.isFile())
    .map((found) => relative(mirror, join(found.parentPath, found.name)))
    .filter((path) => !ownFiles.has(path) && !ownDirectories.has(path.split("/")[0] ?? ""));
}
