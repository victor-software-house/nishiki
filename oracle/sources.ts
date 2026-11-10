// Reads the third-party files the oracle needs from the nishiki-sources
// mirror, and fills and checks that mirror.
//
//   bun run oracle/sources.ts fill <mirror>
//   bun run oracle/sources.ts check <mirror>
//
// The mirror, victor-software-house/nishiki-sources, is public.
// It holds each pinned file at its raw URL's path without the host,
// `<owner>/<repo>/<commit>/<path>`, byte for byte.
// Each file's licence texts sit at their own paths beside it.
// `manifest.toml` has one row per file: its source, licence, size and SHA-256.
//
// `fill` fetches every pinned file and licence text the mirror lacks.
// It never rewrites a file already there.
// `check` fails when a pinned file has no row, when a file differs from its
// row, when a file has no row, or when `keptOut` and the mirror conflict.
//
// Scripts read through `sourceText`, from the directory in `NISHIKI_SOURCES`.
// Nothing falls back to the upstream URL, so a missing file fails the run.
import { bundledLanguagesInfo } from "shiki@4.6.0";
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { inputs, samples, sqlGrammar, standIns } from "./inputs.ts";

const repository = new URL("..", import.meta.url).pathname;
const rawHost = "https://raw.githubusercontent.com/";
const githubHost = "https://github.com/";

/** A file the mirror leaves out, whose text a committed fixture repeats. */
type KeptOut = {
  /** The fixture under `tests/oracle/` whose `L` lines hold the text. */
  fixture: string;
  /** The header line that keeps the file out. */
  header: string;
  /** Why that line keeps it out. */
  reason: string;
};

/** Files that carry their own "All rights reserved" notice inside a
 * repository whose licence cannot cover them, so the public mirror leaves
 * them out (operator, 2026-11-10).
 * Each fixture's `L` lines, joined with line feeds, are the file byte for
 * byte: none has a carriage return or a byte order mark. */
export const keptOut: Record<string, KeptOut> = {
  [`${samples}/shellscript.sample`]: {
    fixture: "shellscript/sample.txt",
    header: "# Copyright 2018 computer-geek64. All rights reserved.",
    reason: "a script copied into Shiki's samples, whose author reserves all rights",
  },
  "https://raw.githubusercontent.com/jeff-hykin/better-objcpp-syntax/5a7eb15eee382dd5aa388bc04fdb60a0d2128e14/language_examples/misc_003.mm":
    {
      fixture: "objective-cpp/grammar-misc-003.txt",
      header: "//  Copyright (c) 2013 Selçuk Kızılkaya. All rights reserved.",
      reason: "an app's source copied into an MIT repository, whose author reserves all rights",
    },
  "https://raw.githubusercontent.com/rometools/rome/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75/rome-modules/src/test/resources/xml/slashdot.xml":
    {
      fixture: "xml/rome-slashdot.txt",
      header:
        "    <dc:rights>Copyright 1997-2005, OSTG - Open Source Technology Group, " +
        "Inc.  All Rights Reserved.</dc:rights>",
      reason: "a Slashdot feed in an Apache-2.0 repository, whose publisher reserves all rights",
    },
};

/** A third-party file the oracle reads, with the licence found at its source. */
type PinnedFile = {
  url: string;
  /** An SPDX expression. */
  licence: string;
  /** Where the licence is stated, as GitHub file URLs at the pinned commit.
   * Each is mirrored as a licence text. */
  licenceSources: string[];
};

/** Shiki's samples are under the MIT licence of their repository: its README
 * puts every file but the grammars and themes under `LICENSE`. */
const sampleLicenceSources = [
  "https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/README.md#L72",
  "https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/LICENSE",
];

/** `qml.sample` states its own licence, KDE's LGPL choice. */
const qmlLicence: Omit<PinnedFile, "url"> = {
  licence: "LGPL-2.1-only OR LGPL-3.0-only OR LicenseRef-KDE-Accepted-LGPL",
  licenceSources: [
    "LGPL-2.1-only.txt",
    "LGPL-3.0-only.txt",
    "LicenseRef-KDE-Accepted-LGPL.txt",
  ].map(
    (name) =>
      `https://github.com/KDE/knewstuff/blob/4ebc0157add67654dd86af21201fba52f48a2dbb/LICENSES/${name}`,
  ),
};

/** The `sql` grammar is MIT, as microsoft/vscode. */
const sqlGrammarLicence: Omit<PinnedFile, "url"> = {
  licence: "MIT",
  licenceSources: [
    "https://github.com/microsoft/vscode/blob/af600487b1e94374d9f48f57cbf2cad24656b07f/LICENSE.txt",
  ],
};

/** SPDX ids for the licence names `tests/oracle/SOURCES.md` uses. */
const spdxIds: Record<string, string> = {
  "Apache-2.0 with the Runtime Library Exception": "Apache-2.0 WITH Swift-exception",
  "MIT or Apache-2.0": "MIT OR Apache-2.0",
  "Unlicense or MIT": "Unlicense OR MIT",
  PostgreSQL: "PostgreSQL",
  zlib: "Zlib",
};

/** Each row of `tests/oracle/SOURCES.md`, by fixture or language, with its
 * licence and the link that states it. */
function recordedLicences(): Map<string, Omit<PinnedFile, "url">> {
  const text = readFileSync(`${repository}tests/oracle/SOURCES.md`, "utf8");
  const links = new Map<string, string>();
  for (const [, label, url] of text.matchAll(/^\[([^\]]+)\]: (\S+)$/gm)) links.set(label, url);
  const rows = new Map<string, Omit<PinnedFile, "url">>();
  for (const [, key, name, label] of text.matchAll(
    /^\| `?([^`|]+?)`? \| .+ \| \[([^\]]+)\]\[([^\]]+)\] \|$/gm,
  )) {
    const link = links.get(label);
    if (link === undefined)
      throw new Error(`SOURCES.md: ${key} links to [${label}], which has no URL`);
    rows.set(key, { licence: spdxIds[name] ?? name, licenceSources: [link] });
  }
  return rows;
}

/** Every third-party file the oracle reads, once each, with its licence. */
export function pinnedFiles(): PinnedFile[] {
  const recorded = recordedLicences();
  const files = new Map<string, PinnedFile>();
  const add = (url: string, licence: Omit<PinnedFile, "url"> | undefined, where: string) => {
    if (licence === undefined)
      throw new Error(`${url}: no licence in tests/oracle/SOURCES.md for ${where}`);
    files.set(url, { url, ...licence });
  };
  for (const input of inputs) {
    if (!("url" in input)) continue;
    const key = `${input.lang}/${input.name}`;
    if (input.url.startsWith(`${samples}/`)) continue;
    add(input.url, recorded.get(key), key);
  }
  for (const [language, url] of Object.entries(standIns)) {
    if (!files.has(url)) add(url, recorded.get(language), language);
  }
  for (const language of bundledLanguagesInfo) {
    if (language.id in standIns) continue;
    const url = `${samples}/${language.id}.sample`;
    add(
      url,
      language.id === "qml" ? qmlLicence : { licence: "MIT", licenceSources: sampleLicenceSources },
      url,
    );
  }
  for (const input of inputs) {
    if ("url" in input && input.url.startsWith(`${samples}/`) && !files.has(input.url)) {
      throw new Error(`${input.url}: an input sample of a language Shiki does not bundle`);
    }
  }
  add(sqlGrammar, sqlGrammarLicence, "sql-words.ts");
  return [...files.values()];
}

/** A file's path in the mirror: its raw URL without the host. */
export function mirrorPath(url: string): string {
  if (!url.startsWith(rawHost)) throw new Error(`${url}: not a raw.githubusercontent.com URL`);
  return url.slice(rawHost.length);
}

/** A licence text's raw URL, from its GitHub file URL. A line anchor is
 * dropped: the whole file is mirrored. */
function rawLicenceUrl(url: string): string {
  const match = /^https:\/\/github\.com\/([^/]+\/[^/]+)\/blob\/([0-9a-f]{40})\/([^#]+)(#.*)?$/.exec(
    url,
  );
  if (match === null) throw new Error(`${url}: not a GitHub file URL at a commit`);
  return `${rawHost}${match[1]}/${match[2]}/${match[3]}`;
}

/** The mirror's directory, from `NISHIKI_SOURCES`. */
function mirrorDirectory(): string {
  const directory = Bun.env["NISHIKI_SOURCES"];
  if (!directory) {
    throw new Error(
      "set NISHIKI_SOURCES to a checkout of " +
        "victor-software-house/nishiki-sources (oracle/AGENTS.md)",
    );
  }
  return directory;
}

/** The text of the file at `url`: from the mirror, or for a file in
 * `keptOut`, from its fixture. Line ends are as the file has them. */
export async function sourceText(url: string): Promise<string> {
  const kept = keptOut[url];
  if (kept !== undefined) return fixtureText(kept.fixture);
  const path = `${mirrorDirectory()}/${mirrorPath(url)}`;
  if (!existsSync(path))
    throw new Error(`${url}: not in the mirror at ${path}; run oracle/sources.ts fill`);
  return Bun.file(path).text();
}

/** The source text a fixture repeats: its `L` lines joined with line feeds. */
function fixtureText(fixture: string): string {
  return readFileSync(`${repository}tests/oracle/${fixture}`, "utf8")
    .split("\n")
    .filter((line) => line.startsWith("L "))
    .map((line) => line.slice(2))
    .join("\n");
}

/** One row of the mirror's `manifest.toml`. */
type MirroredFile = {
  path: string;
  source: string;
  commit: string;
  /** "input" for a file a script reads, "licence text" for a licence or
   * notice an input's row names. */
  purpose: "input" | "licence text";
  /** Set on an input only. */
  licence?: string;
  /** Set on an input only: the GitHub file URLs that state its licence. */
  licence_sources?: string[];
  bytes: number;
  sha256: string;
};

const sha256 = (bytes: Uint8Array) => new Bun.CryptoHasher("sha256").update(bytes).digest("hex");

function readManifest(mirror: string): Map<string, MirroredFile> {
  const path = `${mirror}/manifest.toml`;
  if (!existsSync(path)) return new Map();
  const parsed = Bun.TOML.parse(readFileSync(path, "utf8")) as { file?: MirroredFile[] };
  return new Map((parsed.file ?? []).map((row) => [row.path, row]));
}

/** Writes the manifest, rows sorted by path. JSON strings are TOML basic
 * strings. */
function writeManifest(mirror: string, rows: Map<string, MirroredFile>) {
  const header = [
    "# One row per mirrored file. README.md says what the mirror is.",
    "# purpose: input, a file nishiki's oracle reads; or licence text, a licence",
    "# or notice that an input's licence_sources names.",
    "",
  ];
  const tables = [...rows.values()]
    .sort((a, b) => (a.path < b.path ? -1 : 1))
    .map((row) =>
      [
        "[[file]]",
        ...Object.entries(row).map(([key, value]) =>
          Array.isArray(value)
            ? `${key} = [${value.map((element) => JSON.stringify(element)).join(", ")}]`
            : `${key} = ${JSON.stringify(value)}`,
        ),
        "",
      ].join("\n"),
    );
  writeFileSync(`${mirror}/manifest.toml`, `${header.join("\n")}\n${tables.join("\n")}`);
}

/** Runs `task` on every job, `width` at a time. */
async function pool<T>(jobs: T[], width: number, task: (job: T) => Promise<void>) {
  let next = 0;
  await Promise.all(
    Array.from({ length: width }, async () => {
      while (next < jobs.length) await task(jobs[next++]);
    }),
  );
}

/** Fetches `url` into the mirror unless its file is there. Returns its row,
 * or undefined when the server has no such file and `mayBeAbsent` is set. */
async function mirrorFile(
  mirror: string,
  rows: Map<string, MirroredFile>,
  url: string,
  fields: Pick<MirroredFile, "purpose" | "licence" | "licence_sources">,
  mayBeAbsent = false,
): Promise<MirroredFile | undefined> {
  const path = mirrorPath(url);
  const file = `${mirror}/${path}`;
  let bytes: Uint8Array;
  if (existsSync(file)) {
    bytes = new Uint8Array(readFileSync(file));
  } else {
    const response = await fetch(url);
    if (response.status === 404 && mayBeAbsent) return undefined;
    if (!response.ok) throw new Error(`${url}: HTTP ${response.status}`);
    bytes = new Uint8Array(await response.arrayBuffer());
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, bytes);
  }
  const recorded = rows.get(path);
  const digest = sha256(bytes);
  if (recorded !== undefined && recorded.sha256 !== digest)
    throw new Error(`${path}: differs from its row; fill never rewrites a file`);
  const commit = path.split("/")[2];
  const mirrored: MirroredFile = {
    path,
    source: url,
    commit,
    ...fields,
    bytes: bytes.length,
    sha256: digest,
  };
  if (mirrored.licence === undefined) delete mirrored.licence;
  if (mirrored.licence_sources === undefined) delete mirrored.licence_sources;
  rows.set(path, mirrored);
  return mirrored;
}

/** The names an Apache-2.0 repository's NOTICE file may have, at the root of its commit. */
const noticeNames = ["NOTICE", "NOTICE.txt", "NOTICE.md"];

async function fill(mirror: string) {
  const rows = readManifest(mirror);
  const toMirror = pinnedFiles().filter((pinned) => !(pinned.url in keptOut));
  const licenceTexts = new Set(
    toMirror.flatMap((pinned) => pinned.licenceSources.map(rawLicenceUrl)),
  );
  await pool([...licenceTexts], 16, async (url) => {
    await mirrorFile(mirror, rows, url, { purpose: "licence text" });
  });
  const apacheCommitDirectories = new Set(
    toMirror
      .filter((pinned) => pinned.licence.includes("Apache-2.0"))
      .map((pinned) => mirrorPath(pinned.url).split("/").slice(0, 3).join("/")),
  );
  const notices = new Map<string, string[]>();
  await pool([...apacheCommitDirectories], 16, async (commitDirectory) => {
    for (const name of noticeNames) {
      const found = await mirrorFile(
        mirror,
        rows,
        `${rawHost}${commitDirectory}/${name}`,
        { purpose: "licence text" },
        true,
      );
      if (found !== undefined)
        notices.set(commitDirectory, [...(notices.get(commitDirectory) ?? []), found.path]);
    }
  });
  await pool(toMirror, 16, async (pinned) => {
    const commitDirectory = mirrorPath(pinned.url).split("/").slice(0, 3).join("/");
    const [owner, name, commit] = commitDirectory.split("/");
    const noticeSources = (notices.get(commitDirectory) ?? []).map(
      (path) =>
        `${githubHost}${owner}/${name}/blob/${commit}/${path.split("/").slice(3).join("/")}`,
    );
    await mirrorFile(mirror, rows, pinned.url, {
      purpose: "input",
      licence: pinned.licence,
      licence_sources: [...pinned.licenceSources, ...noticeSources],
    });
  });
  writeManifest(mirror, rows);
  console.log(`✔ ${rows.size} files in ${mirror}/manifest.toml`);
}

/** Every file in the mirror but its own README, guide, manifest, Git
 * attributes and Git directory. */
function mirroredPaths(mirror: string): string[] {
  const ownFiles = new Set(["README.md", "AGENTS.md", "manifest.toml", ".gitattributes"]);
  return readdirSync(mirror, { recursive: true, withFileTypes: true })
    .filter((found) => found.isFile())
    .map((found) => relative(mirror, join(found.parentPath, found.name)))
    .filter(
      (path) => !ownFiles.has(path) && !path.startsWith(".git/") && !path.startsWith(".github/"),
    );
}

function check(mirror: string) {
  const rows = readManifest(mirror);
  const problems: string[] = [];
  for (const row of rows.values()) {
    const file = `${mirror}/${row.path}`;
    if (!existsSync(file)) {
      problems.push(`${row.path}: listed but missing`);
      continue;
    }
    const bytes = new Uint8Array(readFileSync(file));
    if (bytes.length !== row.bytes || sha256(bytes) !== row.sha256)
      problems.push(`${row.path}: differs from its row`);
    if (row.purpose !== "input") continue;
    for (const source of row.licence_sources ?? []) {
      if (!rows.has(mirrorPath(rawLicenceUrl(source))))
        problems.push(`${row.path}: licence text ${source} is not mirrored`);
    }
  }
  for (const path of mirroredPaths(mirror)) {
    if (!rows.has(path)) problems.push(`${path}: no row in manifest.toml`);
  }
  const pinned = pinnedFiles();
  const pinnedUrls = new Set(pinned.map((file) => file.url));
  for (const file of pinned) {
    if (file.url in keptOut) continue;
    const row = rows.get(mirrorPath(file.url));
    if (row === undefined) {
      problems.push(`${file.url}: pinned but not mirrored; run oracle/sources.ts fill`);
      continue;
    }
    if (row.licence !== file.licence)
      problems.push(`${row.path}: licence ${row.licence}, SOURCES.md says ${file.licence}`);
    for (const source of file.licenceSources) {
      if (!row.licence_sources?.includes(source))
        problems.push(`${row.path}: licence_sources lacks ${source}`);
    }
  }
  for (const [url, kept] of Object.entries(keptOut)) {
    const path = mirrorPath(url);
    if (!pinnedUrls.has(url)) problems.push(`keptOut: ${url} is no longer pinned`);
    if (rows.has(path) || existsSync(`${mirror}/${path}`))
      problems.push(`keptOut: ${path} is in the mirror`);
    if (!fixtureText(kept.fixture).split("\n").includes(kept.header)) {
      problems.push(
        `keptOut: tests/oracle/${kept.fixture} has no line ${JSON.stringify(kept.header)}`,
      );
    }
  }
  if (problems.length > 0) {
    for (const problem of problems) console.error(`✘ ${problem}`);
    process.exit(1);
  }
  console.log(`✔ ${rows.size} mirrored files match manifest.toml and the oracle's pins`);
}

if (import.meta.main) {
  const [command, mirror] = process.argv.slice(2);
  if (command === "fill" && mirror) await fill(mirror);
  else if (command === "check" && mirror) check(mirror);
  else throw new Error("usage: sources.ts fill|check <mirror>");
}
