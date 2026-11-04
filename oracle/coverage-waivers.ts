// The oracle-coverage gate's inputs, which oracle/coverage.ts reads.
//
// A grammar is complete when each unit it reaches is exercised by a fixture
// or waived here. Once complete, it stays so: a change that leaves one of its
// units unexercised and unwaived fails `mise run check:oracle-coverage`.
// openspec/changes/oracle-coverage/design.md, section 5.

/** A unit no document can reach, such as a branch that an earlier pattern
 * always takes first. */
export type Waiver = {
  /** Shiki's name for the grammar. */
  grammar: string;
  /** The expression's path, as tests/oracle/COVERAGE.md prints it. */
  path: string;
  /** The capture group, when the waiver is for one group of the expression. */
  group?: number;
  /** Why no document reaches the unit. */
  reason: string;
  /** The oracle/onig.ts expression id whose probe cases check the matcher,
   * required where oracle/fuzz/<grammar>.ts has probe support. */
  probe?: string;
};

export const waivers: Waiver[] = [];

/** The grammars that are complete, by Shiki's name. A grammar joins in the
 * pull request that closes its last gap. */
export const complete: string[] = [];
