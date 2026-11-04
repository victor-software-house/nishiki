// The languages an engine grammar can hand a region to. The keys are Shiki's
// ids. Design section 14 of `openspec/changes/engine-handoff` describes the
// hand-off.
//
// Most of them nishiki lexes by hand. Sass and CoffeeScript run on the engine
// with tables of their own. A hand-off to one of them runs the engine again,
// on the scratch the hand-off lends it.
//
// A spec names the languages its table hands off with `>`, as in
// `haml>ruby>javascript`. An include of one of their grammars becomes a
// hand-off to the language. The table holds no copy of the grammar's rules.
// The grammar needs no licence entry for that table.

/** A language an engine grammar can hand a region to. */
type HandLexed = {
  /** The root scope of Shiki's grammar for it. */
  scope: string;
  /** Its `Language` variant. */
  variant: string;
  /** The Cargo feature that builds it. */
  feature: string;
};

export const handLexed: Record<string, HandLexed> = {
  coffee: { scope: "source.coffee", variant: "CoffeeScript", feature: "lang-coffee" },
  css: { scope: "source.css", variant: "Css", feature: "lang-css" },
  javascript: { scope: "source.js", variant: "JavaScript", feature: "lang-javascript" },
  markdown: { scope: "text.html.markdown", variant: "Markdown", feature: "lang-markdown" },
  ruby: { scope: "source.ruby", variant: "Ruby", feature: "lang-ruby" },
  sass: { scope: "source.sass", variant: "Sass", feature: "lang-sass" },
};
