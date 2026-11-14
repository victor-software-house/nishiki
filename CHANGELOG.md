# Changelog

## nishiki 0.1.1

- Every language runs fewer instructions when its lexer makes or finds a scope stack.
  JSON runs about 6% fewer, TOML 4%, XML 3% and Rust 2% ([`5bde9fd`](https://github.com/victor-software-house/nishiki/commit/5bde9fd2e5bf4fc8473b7451f84891e397f020b7)).
- Highlight x86 and x86-64 assembly with nishiki's TextMate engine on Shiki's `asm` grammar, behind the `lang-asm` feature in the opt-in `textmate` tier ([`3ba2e1c`](https://github.com/victor-software-house/nishiki/commit/3ba2e1ce4f40e5efcede123e4fc02f3e7b7747b1)).
- Assembly looks its keywords up without libc's `memcmp`.
  Its instruction count stays the same wherever the words lie in the binary ([`9c2d056`](https://github.com/victor-software-house/nishiki/commit/9c2d05662d90bb36216dfd6fbd1e88dd295808b4)).
- Highlight Assembly with a lexer of its own, enabled by default and in `all`.
  The `lang-asm` feature leaves the opt-in `textmate` tier ([`2bd333f`](https://github.com/victor-software-house/nishiki/commit/2bd333fc544e0dc2482744d4f264759050c73ade)).
- Highlight Astro with a lexer of its own, enabled by default and in `all` and `web`, as `Language::Astro` and the name `astro`.
  Its frontmatter, expressions, scripts and styles go to the TypeScript, TSX, JavaScript, JSON, CSS, PostCSS, SCSS, Less and Sass lexers ([`389dc63`](https://github.com/victor-software-house/nishiki/commit/389dc6370ac75314f42701a97cea3c943b4f09df)).
- Highlight Batch with nishiki's TextMate engine on Shiki's `bat` grammar, behind the `lang-bat` feature in the opt-in `textmate` tier ([`ac0b990`](https://github.com/victor-software-house/nishiki/commit/ac0b990c69cbf7e3d290322e14849786d61ea94b)).
- Markdown's `bat` and `batch` fences are now highlighted as Batch in a default build, where they were plain text before ([`4f98bb2`](https://github.com/victor-software-house/nishiki/commit/4f98bb2f69337b49bd42f681f3633403575efa85)).
- Highlight Batch with a lexer of its own, enabled by default and in `all`.
  The `lang-bat` feature leaves the opt-in `textmate` tier ([`4f98bb2`](https://github.com/victor-software-house/nishiki/commit/4f98bb2f69337b49bd42f681f3633403575efa85)).
- Highlight Blade, Laravel's templates, behind the `lang-blade` feature, enabled by default and in the `all` and `web` tiers.
  The HTML lexer runs its tags, echoes and directives, and a copy of PHP's rules lexes the PHP inside them.
  An echo or a directive inside that PHP opens at any depth, as in Shiki, such as `{{ "a {{ b }} c" }}` ([`159a4a9`](https://github.com/victor-software-house/nishiki/commit/159a4a9edcd6a887552c98e679813d40468baf01)).
- Highlight a Scala import rename and an XML attribute after a run of blanks as Shiki does.
  Sass, PostCSS and Mermaid skip most of each run of blanks again ([`4ac4af6`](https://github.com/victor-software-house/nishiki/commit/4ac4af6d5a6cb28d6d3969e8ef53923f6fb7ef41)).
- A line of only spaces inside a C `asm` statement lexes without a panic ([`fdaf4d0`](https://github.com/victor-software-house/nishiki/commit/fdaf4d06d193af1291baa19545d2e3b4f6a29ac0)).
- Colouring the first line of C costs about 2 million instructions instead of 250 million.
  Each context's pattern tables are built the first time a line reaches it, not all at once ([`ab55847`](https://github.com/victor-software-house/nishiki/commit/ab558471c065ddcbe19bda3e4f0b614f2ff6140c)).
- Lex C with about 8 percent fewer instructions in a default release build, by letting the compiler inline its pattern dispatch across modules ([`1d24762`](https://github.com/victor-software-house/nishiki/commit/1d24762346e393507029b61838dd43df650e0230)).
- Highlight C with the scopes of Shiki's `c` grammar, and the `c` and `h` fences of Markdown ([`b441f29`](https://github.com/victor-software-house/nishiki/commit/b441f2971f609b6378da2b6b8a120678a425f74c)).
- Lex a C line full of `::` and a shell `select` word ending in `\` in linear time, so neither hits the work limit.
  Read a shell name's combining marks as word characters, as Shiki's `\w` does ([`8528845`](https://github.com/victor-software-house/nishiki/commit/8528845f69712d517f1be35345b7aa692d890492)).
- Count every scan the C and shell lexers make ahead of the cursor against the work limit ([`5759e40`](https://github.com/victor-software-house/nishiki/commit/5759e406fd77733c7e852fd4bdb38a98ce247e9d)).
- Make the C lexer 6 to 9 percent faster by keeping its buffers from line to line instead of allocating them for each line ([`2e96842`](https://github.com/victor-software-house/nishiki/commit/2e96842e93982a09ae90e820e962a3e0ea92a3b6)).
- The first document of a language on the rule engine runs fewer instructions.
  Haskell runs about 36% fewer, and the median over all languages falls about 8% ([`f35a9cb`](https://github.com/victor-software-house/nishiki/commit/f35a9cbe363beaeaa99cbe84fb924159078b8d6a)).
- Lex the JavaScript in CoffeeScript backticks as JavaScript, as Shiki does.
  A capture whose rule includes a hand-lexed grammar now hands its text to that lexer ([`69b7346`](https://github.com/victor-software-house/nishiki/commit/69b73466d0002f0ccc481326a31ef2ecf21f14f6)).
- Lex SQL with about 28 percent fewer instructions and Dockerfiles with about 35 percent fewer.
  An ASCII byte is read without validating it as UTF-8 ([`374bf4d`](https://github.com/victor-software-house/nishiki/commit/374bf4dcf9c33af60ee6729c83d1527e91d1c5e3)).
- Highlight CMake with nishiki's TextMate engine on Shiki's `cmake` grammar, behind the `lang-cmake` feature in the opt-in `textmate` tier ([`869f868`](https://github.com/victor-software-house/nishiki/commit/869f868fdd1fcfa2332442e7362e4680a12dac7a)).
- Highlight CMake with a lexer of its own, enabled by default and in `all`.
  The `lang-cmake` feature leaves the opt-in `textmate` tier ([`b28bdfa`](https://github.com/victor-software-house/nishiki/commit/b28bdfafedd9063f809a21036177289b7390a033)).
- CMake matches its word lists and globs in byte loops, and lexes with about 12% fewer instructions ([`ec53958`](https://github.com/victor-software-house/nishiki/commit/ec5395812aac5a97827d18da56be7a87da3f4abd)).
- Highlight CoffeeScript with nishiki's TextMate engine on Shiki's `coffee` grammar, behind the `lang-coffee` feature in the opt-in `textmate` tier.
  Its embedded JavaScript goes to nishiki's JavaScript lexer, and a Markdown fence named `coffee` embeds it ([`3e5c516`](https://github.com/victor-software-house/nishiki/commit/3e5c51673a4d78842947bde7135d264f13c804f9)).
- Highlight CoffeeScript with a lexer of its own, enabled by default and in `all`.
  The `lang-coffee` feature leaves the opt-in `textmate` tier.
  A default build now highlights Markdown's `coffee` fences and the `<script lang="coffee">` of Vue and Svelte, which it left plain before.
  HAML's `:coffee` filter uses the same lexer ([`a0c139e`](https://github.com/victor-software-house/nishiki/commit/a0c139e87db3b512c0fa0fb28d3b6acc695d56c7)).
- Highlight Common Lisp with nishiki's TextMate engine on Shiki's `common-lisp` grammar, behind the `lang-common-lisp` feature in the opt-in `textmate` tier ([`b292a76`](https://github.com/victor-software-house/nishiki/commit/b292a76cc3be48750f3890ecdc860b4cc747d407)).
- Highlight Common Lisp with a lexer of its own, enabled by default and in `all`.
  The `lang-common-lisp` feature leaves the opt-in `textmate` tier ([`efcd1d1`](https://github.com/victor-software-house/nishiki/commit/efcd1d1c6c04e01f0f898ffb90b458b6f8f5e197)).
- Lex C++ slightly faster by keeping the gaps between tokens small enough to return without a copy ([`40876e6`](https://github.com/victor-software-house/nishiki/commit/40876e6a002477c2cae0bffb92b1f5d76361716a)).
- Lex C++ about 5 percent faster by scanning the blanks between tokens once ([`718357e`](https://github.com/victor-software-house/nishiki/commit/718357e0e8d73d2fd28f3302bbfa28eecd323ae6)).
- Lex C++ about 2 percent faster by skipping the preprocessor directive patterns on lines without a `#` ([`dfdceee`](https://github.com/victor-software-house/nishiki/commit/dfdceee869c8d70fac569cfc0443ffd5544aa0c4)).
- Highlight C++ with the scopes of Shiki's `cpp` grammar, and the `cpp`, `c++` and `cxx` fences of Markdown ([`3d57406`](https://github.com/victor-software-house/nishiki/commit/3d5740664906f968f5673e48c9ec434c01f07e2d)).
- Lex C++ about 6 percent faster by skipping the patterns whose required text the rest of the line lacks ([`176d5e8`](https://github.com/victor-software-house/nishiki/commit/176d5e8e61391a97168d3212224b1f2f591ba102)).
- Close a C++ raw string whose delimiter is longer than 16 bytes on that delimiter, as Shiki does, instead of on `)"` alone ([`b5fef8d`](https://github.com/victor-software-house/nishiki/commit/b5fef8db8c311e37076159e318ae1ba41940878f)).
- Lex C++ about 15 percent faster by testing word characters and blanks with the shared byte tables ([`2933eb4`](https://github.com/victor-software-house/nishiki/commit/2933eb44581080f8c9071ac53b74e17eddad4bf9)).
- Lex C++ about 5 percent faster by looking up its keyword and type lists by integer keys ([`c9bcc5a`](https://github.com/victor-software-house/nishiki/commit/c9bcc5a6ad22c4129350cd93941f2e4a3894808f)).
- Make the crate docs the docs.rs landing page: a quick start, themes, limits, cargo features and tiers, and Markdown fences, moved from the README ([`0751174`](https://github.com/victor-software-house/nishiki/commit/0751174f167c3c2f4eec3ff5ad4b11cef62ba114)).
- Lex C#, Go, Swift, Java, Kotlin and Markdown with fewer instructions, most of all C# in a default release build.
  The keyword test is inlined where it pays ([`47b76a3`](https://github.com/victor-software-house/nishiki/commit/47b76a37f047cc593070964fc40c566411d9bc56)).
- Lex C# with about 4 percent fewer instructions in a default release build, by letting the compiler inline its pattern dispatch across modules ([`b98644b`](https://github.com/victor-software-house/nishiki/commit/b98644bbe0b1b258208baaebf13d63c8bc353a3f)).
- Highlight C# with the scopes of Shiki's `csharp` grammar, and the `cs`, `csharp` and `c#` fences of Markdown ([`dfac8b9`](https://github.com/victor-software-house/nishiki/commit/dfac8b987106db8086613935bd578cad8802f3bf)).
- Highlight CSS with the scopes of Shiki's `css` grammar, and the `css` and `css.erb` fences of Markdown ([`d8301b9`](https://github.com/victor-software-house/nishiki/commit/d8301b9668d9fb47817551cc5128cfe866353240)).
- Match CSS and SCSS words written with a character that folds to several letters, such as `ß` for `ss` or the ligature `ﬁ` for `fi`, as Shiki does ([`c8eaae0`](https://github.com/victor-software-house/nishiki/commit/c8eaae0419874724de62d7dfa8a9a6e704f47da3)).
- Start a CSS selector at an upper-case `Mark`, `Font`, `Mask` or other name that could be a property, as Shiki does.
  Such a name is not read as a property name ([`05cea15`](https://github.com/victor-software-house/nishiki/commit/05cea15af7a2939837cb153405b47b65de730e85)).
- Lex a CSS or SCSS number's unit faster.
  The word after the number is looked up once among the grammar's units.
  Before, each of the 54 unit patterns looked it up ([`47988f5`](https://github.com/victor-software-house/nishiki/commit/47988f58a7a9c21cd3455e8aa087ffeaa3d29825)).
- Lex CSS and SCSS numbers faster.
  The 54 unit patterns of a number are now one pattern, tried once ([`00a634c`](https://github.com/victor-software-house/nishiki/commit/00a634c9d1df52a890fd12db60c9bbcf9cf1c49b)).
- Name a CSS or SCSS unit spelled with `ſ`, such as `500mſ`, from its text lowered as Shiki does (`keyword.other.unit.mſ.css`), instead of from the ASCII letters ([`ca35afb`](https://github.com/victor-software-house/nishiki/commit/ca35afb725dd1076a6c38672d8f8711181df4f0d)).
- Highlight CSS and SCSS property values with about 30 percent fewer instructions.
  Each word is looked up once, in one table of the grammar's word lists, instead of once per list ([`7e114a1`](https://github.com/victor-software-house/nishiki/commit/7e114a18b644b05083efa6485af973ca5da88176)).
- `Highlighter::line` leaves the tokens already in its buffer as they were.
  A Markdown, Vue or Svelte line that popped a less fence cut earlier lines' tokens there ([`b182082`](https://github.com/victor-software-house/nishiki/commit/b182082d3670c57a39f7cbd55045ec68f1a24ad5)).
- Highlight Dart with nishiki's TextMate engine on Shiki's `dart` grammar, behind the `lang-dart` feature in the opt-in `textmate` tier, and the `dart` fences of Markdown ([`c8aa89f`](https://github.com/victor-software-house/nishiki/commit/c8aa89f7ad1419ff9c2ab346373971de3437645f)).
- Markdown's `dart` fences are now highlighted as Dart in a default build, where they were plain text before ([`9921b92`](https://github.com/victor-software-house/nishiki/commit/9921b92666ca52af42fa13730017bde2d08b8370)).
- Highlight Dart with a lexer of its own, enabled by default and in `all`.
  The `lang-dart` feature leaves the opt-in `textmate` tier ([`a8a8012`](https://github.com/victor-software-house/nishiki/commit/a8a80127f249d6d52ad7f404ec66eb612f254160)).
- Raise the default depth limit from 256 to 4,096 open contexts.
  A long Ruby spec file keeps Shiki's scopes to its end.
  Shiki leaves one call open for each `call(<<~RUBY)`, because the here document takes its `)`.
  Shiki has no depth limit ([`34d4f11`](https://github.com/victor-software-house/nishiki/commit/34d4f11f91ed29c7bf8b026ae645e8770e3f5f8b)).
- `Limits::depth` leaves out a language's root context in Rust and Markdown, as in the languages lexed by the shared rule engine.
  Under a pinned depth, Rust and Markdown open one more nested context than before ([`d1ae494`](https://github.com/victor-software-house/nishiki/commit/d1ae494df5c2bfe4eababdcae3dac0bb311a9fcc)).
- Highlight diff with the scopes of Shiki's `diff` grammar, and the `diff`, `patch` and `rej` fences of Markdown ([`5b549a4`](https://github.com/victor-software-house/nishiki/commit/5b549a47f9d654417b73e00d0252b5c7dece1c65)).
- Markdown fences of TypeScript, JavaScript, HTML, ERB and html-derivative pay no extra call per line once nishiki has more than fifty languages ([`96c1509`](https://github.com/victor-software-house/nishiki/commit/96c150900ff2c3084c8ebd84441fa0b4e864cb7d)).
- Highlight Dockerfiles with the scopes of Shiki's `docker` grammar, and the `dockerfile` fences of Markdown ([`b3ed28e`](https://github.com/victor-software-house/nishiki/commit/b3ed28e5b875e29bbd7453e5178293589c28941e)).
- A `Document` highlights text as it streams in, delta by delta.
  Each delta may end mid-line or inside a UTF-8 character.
  A completed line is lexed once, and only the last line is lexed again on each delta.
  The last line is provisional, and marks where its last token starts ([`e3cf39b`](https://github.com/victor-software-house/nishiki/commit/e3cf39bfe589aff1425a73c6132b70fae9ac1460)).

  `Document::lex` lexes through a chosen line, so a viewport can come first.
  `Document::take_changes` names the lines to draw again.
- `Document::replace` swaps a byte range for new text, for edits and diffs.
  Lexing then stops at the first line whose end state matches its old one.
  `Document::reset` starts a fresh highlighter when its arena fills ([`517ad8f`](https://github.com/victor-software-house/nishiki/commit/517ad8f7e30adf763072a733752b838595d1e750)).
- A `Document` styles its tokens with its theme.
  `DocumentLine::styled` gives each token's end and style, equal to `Theme::style`.
  `Document::lex` keeps each stack's style, so reading a token costs one lookup.
  `Document::set_theme` switches the theme and restyles every token once ([`b824acf`](https://github.com/victor-software-house/nishiki/commit/b824acf2fa3116581eae813d00135ce6f6f2e5b3)).
- A long last line of a `Document` is lexed again in bounded steps as it grows.
  Past 1 KiB, it waits until it has grown by an eighth, so its cost stays linear.
  `Document::set_tail` and `Tail` change or turn off the rule ([`f2b71a0`](https://github.com/victor-software-house/nishiki/commit/f2b71a0963141f37d9ecc3910b544a15fb763c17)).
- Highlight Elixir with nishiki's TextMate engine on Shiki's `elixir` grammar, behind the `lang-elixir` feature in the opt-in `textmate` tier.
  Its `~H` and `~L` sigils hand HTML to its lexer, and a Markdown fence named `elixir` embeds it ([`26e6697`](https://github.com/victor-software-house/nishiki/commit/26e6697d2690df322c031648b5eb4738d357f852)).
- Highlight Elixir with a lexer of its own, enabled by default and in `all`.
  The `lang-elixir` feature leaves the opt-in `textmate` tier.
  Markdown's `elixir` fences now highlight with the default features ([`b5fff1f`](https://github.com/victor-software-house/nishiki/commit/b5fff1f91b54fc327e80e5bc7e4cd27a74dac794)).
- Markdown and TOML that a Vue block embeds in a list item's or a quote's fence start after the line's start, as in Shiki.
  Their patterns that need the line's start match nowhere there.
  So an HTML block in such Markdown ends with its first line, and a TOML table header there is `invalid.illegal` ([`0980dfe`](https://github.com/victor-software-house/nishiki/commit/0980dfe5925e9e8f920c2ba4cb9623e089193003)).
- Under `(?i)`, nishiki's TextMate engine folds a character into several, as Shiki does.
  `ß` matches `ss`, `class` matches `claß`, and `ffi` matches `ﬃ`.
  A class such as `[\w]` also matches the fold strings of its members ([`e2ee602`](https://github.com/victor-software-house/nishiki/commit/e2ee6026e902fb2d159451a87afea54dc84e7b41)).
- Under `(?i)`, nishiki's TextMate engine matches a character as every character Unicode's simple case folds join it to, as Shiki does.
  `s` matches `ſ`, `k` matches the Kelvin sign, and `[^k]` refuses it ([`8c1a6c0`](https://github.com/victor-software-house/nishiki/commit/8c1a6c0e21105a85eeb20cd330f5855d1236ade6)).
- The TextMate engine allocates the cells of its compiled patterns once per process, on first use.
  It ships none of them as static data.
  A build with every engine language is about 400 KB smaller ([`685afa6`](https://github.com/victor-software-house/nishiki/commit/685afa684cb4d68a7d9f4a4cffc85eb5f271351a)).
- Highlight ERB with the scopes of Shiki's `erb` grammar: HTML whose `<% %>` tags hold Ruby, run on the HTML and Ruby lexers ([`cc11946`](https://github.com/victor-software-house/nishiki/commit/cc119462da8aa5e59f3bfb96058b35c042052b8c)).
- Highlight GDScript with nishiki's TextMate engine on Shiki's `gdscript` grammar, behind the `lang-gdscript` feature in the opt-in `textmate` tier ([`b39e03c`](https://github.com/victor-software-house/nishiki/commit/b39e03cf851082d4b75e15f97168ee6b07896c3d)).
- Highlight GDScript with a lexer of its own, enabled by default and in `all`.
  The `lang-gdscript` feature leaves the opt-in `textmate` tier ([`9a48677`](https://github.com/victor-software-house/nishiki/commit/9a4867755076f9e7ef37a4c79142a22130d3f28a)).
- Highlight Go with the scopes of Shiki's `go` grammar, and the `go` and `golang` fences of Markdown ([`b535ac5`](https://github.com/victor-software-house/nishiki/commit/b535ac57b81265c3a7434391dee02fbe15da7a11)).
- nishiki highlights any TextMate grammar a caller brings, behind the `textmate` feature, which now turns on `json` ([`bdfa9be`](https://github.com/victor-software-house/nishiki/commit/bdfa9be9b2cf11b14b20bcda194972b59ca6694e)).

  - `Grammar::from_json` compiles a grammar in the JSON form Shiki and VS Code use.
  - `Grammar::to_bytes` and `Grammar::from_bytes` let a build script compile a grammar ahead of time.
  - `Grammars` holds the grammars an include can name, and `Grammars::hand_off` names languages to lex by hand.
  - `Highlighter::for_grammar` and `Highlighter::for_grammar_with_limits` build a `Highlighter<Grammar>`.
  - Errors come back as a `GrammarError`.

  Each distinct grammar stays in memory until the process ends.
- A build script can write a grammar as Rust, which a program includes as a `static` that takes no work to load.
  The `codegen` feature gives `Grammar` an implementation of `quote::ToTokens`, and the `format` feature adds `Grammar::to_rust`.
  Both are off by default and not in `all` ([`d89b255`](https://github.com/victor-software-house/nishiki/commit/d89b255ab657ff2cf6ea1e377ee8ee2443f46182)).
- Lex a GraphQL `${...}` interpolation with the JavaScript lexer across lines, as Shiki does.
  It runs up to the `}` that no context of the code is waiting for, instead of counting braces on one line ([`ea206b4`](https://github.com/victor-software-house/nishiki/commit/ea206b44ee0720a3d65352e0edcb1d93d64a7bdd)).
- GraphQL runs about 1.3% fewer instructions on its fixtures ([`6944aba`](https://github.com/victor-software-house/nishiki/commit/6944aba7b6fc0d965fcc986d2762fc00ecfdf66c)).
- Highlight GraphQL with the scopes of Shiki's `graphql` grammar ([`a8678f0`](https://github.com/victor-software-house/nishiki/commit/a8678f00b50772a699f44b60045a92ef5d658375)).
- The GraphQL lexer allocates nothing per line: it reuses the buffer that holds the open rules from the line before ([`2aebeb6`](https://github.com/victor-software-house/nishiki/commit/2aebeb6840564d6a2c6b6eeb6b9603894b45f68c)).
- GraphQL runs on the shared rule engine.
  `Limits::depth` now counts only the contexts open above GraphQL's root, as in the other languages.
  This changes the count for every depth limit, including the default `Some(4096)`.
  Each interpolation's JavaScript spends the line's one work budget ([`fb4d76e`](https://github.com/victor-software-house/nishiki/commit/fb4d76eb94b5a5b110f16de2dd8482c09aa37c23)).
- Highlight Hack with a lexer of its own, enabled by default and in `all`, behind the `lang-hack` feature.
  It embeds HTML around the code, and SQL in strings that start with a SQL keyword ([`16876bd`](https://github.com/victor-software-house/nishiki/commit/16876bd12dfdc14560703f08d009a5d70547522f)).
- Highlight HAML with nishiki's TextMate engine on Shiki's `haml` grammar, behind the `lang-haml` feature in the opt-in `textmate` tier.
  Its filters and Ruby lines go to nishiki's Ruby, JavaScript, CSS and Markdown lexers ([`a2bc755`](https://github.com/victor-software-house/nishiki/commit/a2bc7555978f5fcb73179f709270be7c5042ea5c)).
- Lex HAML's `:sass` and `:coffee` filters as Sass and CoffeeScript, as Shiki does ([`db46778`](https://github.com/victor-software-house/nishiki/commit/db46778ab8f4e1a899079518185bf02de678e21d)).
- Highlight HAML with a lexer of its own, enabled by default and in `all`.
  The `lang-haml` feature leaves the opt-in `textmate` tier.
  It turns on Ruby, JavaScript, CSS, Sass, CoffeeScript and Markdown, which HAML's filters and Ruby lines embed.
  Ruby's `<<~HAML` here documents are now highlighted as HAML in a default build, where they were plain text before ([`9a675eb`](https://github.com/victor-software-house/nishiki/commit/9a675eb9e7f9e47b83ff3f574810564d499db1bd)).
- A grammar's `while` rule can hand each line to a hand lexer, as a Markdown fence does.
  The lexer now sees `\G` at the start of such a line, as in Shiki.
  So in a shell fence, the line after a `\` reads as a new command ([`6010370`](https://github.com/victor-software-house/nishiki/commit/601037096022cea2ee39514d0371d7f97cad796f)).
- A loaded grammar can hand a region to a language when the region's end or the rules beside the include use `\G`.
  Before, the include matched nothing there, or the link failed when the language was named with `Grammars::hand_off` ([`312c220`](https://github.com/victor-software-house/nishiki/commit/312c220f409ab3ba81329e99cc18acf1102779c7)).
- A grammar can hand a region to a hand lexer and keep rules of its own beside the include.
  Those rules now apply only where the lexer has no context of its own open, as in Shiki ([`899c54d`](https://github.com/victor-software-house/nishiki/commit/899c54dd13eb6c45c1ba49c0eda6afcb847ea642)).

  Before, such a rule that matched at the start of a line applied inside the lexer's open contexts too.
  In HTML, a `//` comment line inside a JavaScript object then lost the object's scopes.
- A grammar loaded from JSON can include the grammar of any supported language.
  That include now goes to the language's lexer when the language's feature is on and the `Grammars` set lacks that grammar.
  Before, only eleven languages took such a hand-off, and an include of another language's grammar matched nothing ([`855b8ff`](https://github.com/victor-software-house/nishiki/commit/855b8ff4f91ce67f80d1f0635a263d78d15a28c9)).

  `Grammars::hand_off` now accepts every language.
  So `Highlighter::for_grammar` does not fail with "no grammar can hand a region to" a language.

  A list of rules that includes several such grammars does not fail with "a list of rules includes two hand-offs".
  One language takes the region: the one whose lexer also lexes the others' text, else the first.

  - A Markdown `php` fence goes to PHP's lexer, which reads its HTML too.
  - A Vue template goes to html-derivative's lexer.

  With no language named through `Grammars::hand_off`, a region whose end pattern uses `\G` hands nothing off, and the grammar links.
  A named language still makes such a grammar fail.

  Some lexers, such as Rust's, cannot stop inside a line where the grammar takes the text back.
  Such a language takes a region only when the region has no rules beside the include and no end pattern, as in a Markdown fence.

  - With no language named, any other region hands nothing off.
  - A named language makes such a grammar fail.

  html-derivative's lexer now stops there.
  So a Markdown paragraph's inline code, emphasis and links keep their scopes around its HTML.
- A grammar loaded through `Grammar::from_json` that hands a region to a lexer now gives it `\G` where Shiki does.
  After a begin match that took the line feed, `\G` matches at the start of each later line while the lexer has only its root open.
  A Makefile line that starts with a tab is then an assignment there, as in Shiki ([`f53b298`](https://github.com/victor-software-house/nishiki/commit/f53b298bae0624d1e9c77dab7e4023e6abc3f30d)).

  Before, `\G` never matched at the start of those lines.
- A loaded grammar that hands a region to Less now ends that region where Less stops at a `)` or `}` at its root, as in Shiki.
  The rest of the line takes the scope of the rule around the region.
  This covers a Markdown fence and a Vue `<style lang="less">` body loaded through `Grammar::from_json` ([`868c779`](https://github.com/victor-software-house/nishiki/commit/868c779ad65d9c63a7725400b0d65489c61291a2)).

  Before, Less went on past that character and kept the region open.
- Highlight Haskell with nishiki's TextMate engine on Shiki's `haskell` grammar, behind the `lang-haskell` feature in the opt-in `textmate` tier ([`f25e02f`](https://github.com/victor-software-house/nishiki/commit/f25e02f43e9de9c3e4ee7e4eda7957064222fcb4)).
- Highlight Haskell with a lexer of its own, enabled by default and in `all`.
  The `lang-haskell` feature leaves the opt-in `textmate` tier ([`416a719`](https://github.com/victor-software-house/nishiki/commit/416a7191c9fc5f609e672d0b7074dcc49ddf6223)).
- Highlight HCL with a lexer of its own, enabled by default and in `all`, behind the `lang-hcl` feature ([`9397129`](https://github.com/victor-software-house/nishiki/commit/9397129afdfdc6f9dd058b1a828ad13ac7b497c1)).
- Add `Highlighter::style`, which gives a token's style from its stack.
  It gives the style `Theme::style` gives, and keeps each stack's style once computed.
  It keeps the styles of the two themes used last, so styling each token with a light and a dark theme stays cached.
  Colouring through it runs about 53% fewer instructions than through `Theme::style`, and JSON about 96% fewer ([`06a47ec`](https://github.com/victor-software-house/nishiki/commit/06a47ece3001d2520b5eb9b2e74988a14e4afa62)).
- `Highlighter` takes a type parameter, `S: Syntax`, which defaults to `Language`.
  `Highlighter` alone still names a language highlighter, so existing code compiles as before.
  `Syntax` is sealed.
  It makes room for highlighters that lex a grammar the caller loads ([`001428f`](https://github.com/victor-software-house/nishiki/commit/001428f63459a2cec38de43cd478b03a52161643)).
- Highlight HLSL with a lexer of its own, enabled by default and in `all`, as `Language::Hlsl` and the name `hlsl`.
  It follows the scopes of Shiki's `hlsl` grammar ([`ff872e6`](https://github.com/victor-software-house/nishiki/commit/ff872e6b2c00b1992d52a6f784eb0402ce505187)).
- Highlight html-derivative with the scopes of Shiki's `html-derivative` grammar, the HTML grammar that Vue's templates embed, run on the HTML lexer.
  With `lang-html-derivative` on, Markdown highlights HTML blocks and the HTML in paragraphs, headings and emphasis, as Shiki does ([`2fcf235`](https://github.com/victor-software-house/nishiki/commit/2fcf2351c36475a2d2f5ebf56f02854f30006cb4)).
- An HTML fence in a Markdown grammar loaded through `Grammar::from_json` now names attributes as Shiki does.
  A name such as `close` on an unknown tag is `meta.attribute.unrecognized.close.html` there, as in nishiki's own Markdown.
  This holds with or without the `lang-markdown` feature.
  HTML in a loaded Svelte grammar follows Svelte's order the same way ([`032caac`](https://github.com/victor-software-house/nishiki/commit/032caac517298a52bbdf04e6db4717afb4339678)).

  Before, the loaded grammar's fences read HTML as another grammar embeds it, so some attribute names differed.
- Read HTML's attribute list for an invalid tag, such as `<foo close for>`, in HTML that a Markdown or Svelte document holds, as Shiki does.
  That covers Markdown `html` and `php` fences, and the HTML that PHP and Perl hand on inside them.
  An HTML document still reads the MathML list there ([`3dc0933`](https://github.com/victor-software-house/nishiki/commit/3dc0933fb68268c71bcb5a3d643296b52b106166)).
- Lex the JavaScript of an HTML `<script>` element or event handler with its whole line in view, as Shiki does.
  A `{` right after `<script>` opens an object literal.
  After an event handler's opening quote, a `<` is an operator.
  A JSX `=` that ends an event handler's line is an illegal attribute ([`1714f79`](https://github.com/victor-software-house/nishiki/commit/1714f79cad50379307e402cb1813393865c55337)).
- Highlight HTML with the scopes of Shiki's `html` grammar.
  The CSS of `<style>` elements goes to the CSS lexer.
  The JavaScript of `<script>` elements and event handlers goes to the JavaScript lexer.
  Markdown fences named `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl` or `tpl` are highlighted as HTML; they were plain text before ([`9434dc2`](https://github.com/victor-software-house/nishiki/commit/9434dc2059670289a5584ff7cde2b3d887a2b5c9)).
- Match HTML tag names written with a character that folds to letters, as Shiki does.
  Examples are `ﬁ` for `fi` and `ſ` for `s`.
  This covers `<style>` and `<script>` tags, and the end tags of SVG and MathML elements ([`da17a70`](https://github.com/victor-software-house/nishiki/commit/da17a705354897b5147d71edfdc3f2140269558e)).
- HTML, its derivatives, ERB and PostCSS compare tag names and literals in byte loops instead of libc's `memcmp` ([`3cf3bfc`](https://github.com/victor-software-house/nishiki/commit/3cf3bfcf41769d0392973c8ed29fe89e45029b83)).
- Highlight INI with the scopes of Shiki's `ini` grammar, and the `ini` and `conf` fences of Markdown ([`28992ab`](https://github.com/victor-software-house/nishiki/commit/28992ab5b0603bbe5737e3cb44c623272a210e29)).
- Highlight Java with the scopes of Shiki's `java` grammar, and the `java` fences of Markdown ([`b6342a3`](https://github.com/victor-software-house/nishiki/commit/b6342a3b615f62b3de4057822526926cb3ddae55)).
- Highlight JavaScript, JSX and TSX with the scopes of Shiki's `javascript`, `jsx` and `tsx` grammars, on the TypeScript lexer, and hand `js`, `jsx` and `tsx` fences in Markdown to it ([`5aeefb5`](https://github.com/victor-software-house/nishiki/commit/5aeefb5465561cb830e245238f7261f9dc995748)).
- Highlight Jinja with a lexer of its own, enabled by default and in `all`, as `Language::Jinja` and the name `jinja` ([`c222619`](https://github.com/victor-software-house/nishiki/commit/c2226191a680d0b1340d2fb373a415f70a52ee97)).
- Highlight JSON, JSON with comments and JSON Lines with the scopes of Shiki's `json`, `jsonc` and `jsonl` grammars, and the `json`, `jsonc` and `jsonl` fences of Markdown ([`510f740`](https://github.com/victor-software-house/nishiki/commit/510f7404ae7fe64ae5f3c4c0d8498ad6b87e2678)).
- Highlight Julia with nishiki's TextMate engine on Shiki's `julia` grammar, behind the `lang-julia` feature in the opt-in `textmate` tier, and the `julia` fences of Markdown ([`be0a0cd`](https://github.com/victor-software-house/nishiki/commit/be0a0cda61a05ecb364008e3d6082442fbdd2392)).
- Highlight Julia with a lexer of its own, enabled by default and in `all`.
  The `lang-julia` feature leaves the opt-in `textmate` tier.
  It turns on Python, JavaScript and SQL, which Julia's `py"""`, `js"""` and `sql"""` string macros embed.
  Markdown's Julia fences are now highlighted as Julia in a default build, where they were plain text before ([`a5b07a1`](https://github.com/victor-software-house/nishiki/commit/a5b07a1fef1c913ddca478487ac6e8944df33ae7)).
- A line that ends inside an embedded language does not make the next line open every rule again.
  Svelte runs about 3% fewer instructions, and Ruby, PHP and Perl about 0.1% fewer ([`e8d2e32`](https://github.com/victor-software-house/nishiki/commit/e8d2e32f47f6bed38f13cf05ccab7148f4816e5a)).
- Lex shell and rule-engine lines that follow a kept rule without building the open rules again.
  A shell document 300 levels deep takes two thirds fewer instructions ([`eb22f1f`](https://github.com/victor-software-house/nishiki/commit/eb22f1f7b0b821b3b1ac164b77c7c0df3cb62edb)).
- Highlight Kotlin with the scopes of Shiki's `kotlin` grammar ([`1330cf4`](https://github.com/victor-software-house/nishiki/commit/1330cf41afa081a4b46108e8e15526e0d1cc87ba)).
- List each language's behaviour kept from Shiki on its `Language` variant on docs.rs, moved from the README ([`e4ee4a0`](https://github.com/victor-software-house/nishiki/commit/e4ee4a033149f0db409ceb384042bfd0739c8476)).
- Build only the languages you highlight: each lexer is a `lang-*` feature, `default` turns them all on, and the tiers `config`, `web`, `systems` and `docs` turn on groups. `Language::ALL` and `Language::from_name` cover the languages that are built, and `json` stays the theme-file feature ([`a0b2f36`](https://github.com/victor-software-house/nishiki/commit/a0b2f360f1d6125336f72a0a83dbd035d0c8eb52)).

  A dependency written `default-features = false, features = ["json"]` built every language in 0.1.0 and now builds none. Add the `lang-*` features it uses, or `all`.
- Highlight LaTeX with a lexer of its own, under the `lang-latex` feature, enabled by default and in `all` and `docs`.
  Its code environments, such as `minted` and `pycode`, hand their text to the language they name ([`655ae98`](https://github.com/victor-software-house/nishiki/commit/655ae98401aa2a57c91683a3610290d5f5086023)).

  Markdown `latex` and `tex` fences are now LaTeX, as in Shiki; they were plain text.
- Read Less words that ignore case when they are spelled with a character that folds to letters, as Shiki does.
  This covers counter styles such as `ſquare` or `Katakana` with the Kelvin sign.
  It covers units such as `1ſ`, and special colour keywords such as `tranſparent` ([`7b4f6f5`](https://github.com/victor-software-house/nishiki/commit/7b4f6f59201f79a976d37014f695bb42b41d6f2e)).
- Highlight Less with the scopes of Shiki's `less` grammar, and the `less` fences of Markdown ([`349c143`](https://github.com/victor-software-house/nishiki/commit/349c143b64185a9d5484a63915096719b9a09494)).
- Lex Less faster.
  Each Less pattern is now tried with a byte test or a single tail call ([`129dcbd`](https://github.com/victor-software-house/nishiki/commit/129dcbd676623cb771fe58a1863bf0854697302b)).
- A Svelte or Vue `<style lang="less">` body that starts with `)` or `}` ends there, as in Shiki.
  The rest of the body is plain text, and the element closes at `</style>`.
  Before, the rest of the document was highlighted as Less ([`9be2e22`](https://github.com/victor-software-house/nishiki/commit/9be2e223d061e31c7bc972588ed979faa962bf65)).
- Lex Less words faster.
  The search for a word of the grammar's big lists stops where no listed word can continue.
  A word is searched lowered only when it holds a capital ([`43bed4a`](https://github.com/victor-software-house/nishiki/commit/43bed4ab45430e469e9e62f9e71954f061a18b13)).
- Lex Less values faster.
  A function name is compared with the grammar's names only where `(` follows it.
  A number's dimension is tried only where a number can start ([`d2e6d68`](https://github.com/victor-software-house/nishiki/commit/d2e6d6877e94c48bb6663bfc96f2ab1a8360476e)).
- Highlighting a line pays no call into the language dispatch for diff, Dockerfile, YAML, TOML, JSON, INI, GraphQL and Markdown.
  In a default release build, those languages run 0.3% to 4.8% fewer instructions ([`94c11ac`](https://github.com/victor-software-house/nishiki/commit/94c11ac8344ef4312c2ad7b7b0b4d176dbe17ccc)).
- End a Lua documentation comment's `@enum`, `@cast` or `@diagnostic` tag before the line feed, as Shiki does.
  A tag that ends its line does not run into the next one ([`631800f`](https://github.com/victor-software-house/nishiki/commit/631800f7efa4edde98ed85d52e74ca7126c5b942)).
- Lua highlights the body of an `ffi.cdef[[...]]` long bracket as C, as Shiki does.
  The body ends at the bracket of its own level.
  It needs the `lang-c` feature ([`c6c1bee`](https://github.com/victor-software-house/nishiki/commit/c6c1bee98e2f633d2f857647f86c963a57aa8fda)).
- Highlight Lua with the scopes of Shiki's `lua` grammar, and the `lua` fences of Markdown ([`a695cec`](https://github.com/victor-software-house/nishiki/commit/a695cec995f4884bbe10eba0774bbc028398af40)).
- Count the Makefile lexer's open contexts above the document's own, as every other lexer does.
  A pinned depth limit then opens as many contexts in Makefiles as elsewhere ([`762d14c`](https://github.com/victor-software-house/nishiki/commit/762d14c64ca7c3716f312653b2691386babd26b4)).
- Highlight Makefiles with the scopes of Shiki's `make` grammar, and the `makefile`, `Makefile`, `GNUmakefile` and `OCamlMakefile` fences of Markdown ([`06234d7`](https://github.com/victor-software-house/nishiki/commit/06234d7237a84acc2e61d49342afdb35cc75bfa2)).
- Lex Makefile lines, and lines of the other rule-engine languages that open their rules afresh, without allocating once a document's states are interned ([`a877923`](https://github.com/victor-software-house/nishiki/commit/a877923865e90c2d97296054ef68c90332ebe2ef)).
- Markdown does not clear every open container's saved anchor on each line, and runs about 0.3% fewer instructions on its fixtures ([`97e04da`](https://github.com/victor-software-house/nishiki/commit/97e04da02bd2db535bb8762ecba8db103708c789)).
- Markdown does not copy its table of capture groups for each match, and runs about 1.2% fewer instructions on its fixtures ([`d1cd3f4`](https://github.com/victor-software-house/nishiki/commit/d1cd3f449ff01311d0e0d3ecde83c2d6277decd4)).
- A line of many Markdown code spans now costs work linear in its length ([`e319b15`](https://github.com/victor-software-house/nishiki/commit/e319b152f8d20d50803a3f4c12f23672681489f6)).
- A line of many Markdown italic openers now costs work linear in its length ([`5fb2e56`](https://github.com/victor-software-house/nishiki/commit/5fb2e56bd1bb7f004b91a22163a00a94fbd5e103)).
- Match a Markdown fence's language name with Oniguruma's case folding, as Shiki does.
  The Kelvin sign reads as `k`, the long s as `s`, and the ligatures U+FB00 to U+FB06 as the letters they stand for ([`b5cd806`](https://github.com/victor-software-house/nishiki/commit/b5cd8063312fb72941f9b174ccefd443b96ad6d3)).
- A dot in the Markdown fence names `css.erb`, `Berksfile.lock` and `.textmate_init` now matches any one character, as in Shiki ([`5750034`](https://github.com/victor-software-house/nishiki/commit/575003429c6c3b0d1a40f251b1e8cffd9067bdb7)).
- Read Markdown fences as Shiki does for three more names.
  A `bat` or `batch` fence is lexed as Batch when `lang-bat` is on.
  A `php` fence's body is lexed as HTML under `meta.embedded.block.php`.
  An `xsl` or `xslt` fence is plain, because nishiki does not lex the XSL grammar ([`b1a4bcd`](https://github.com/victor-software-house/nishiki/commit/b1a4bcd3afbfadc8019fb725d19048bc2df3a2a2)).
- Markdown does not allocate on the first line of a document once the highlighter has lexed one ([`6f7e895`](https://github.com/victor-software-house/nishiki/commit/6f7e8952b9c56a63032878291269a5e542276571)).
- Markdown: a line of many image links, such as `![a](` repeated, now lexes in linear time.
  It stays within its work budget ([`d43f0bb`](https://github.com/victor-software-house/nishiki/commit/d43f0bbf074f6c74f7464bc6055898a1466bd94c)).
- In a Markdown `less` fence, a `)` or `}` at the root of the Less now ends the fence's Less body, as in Shiki ([`1f66c5a`](https://github.com/victor-software-house/nishiki/commit/1f66c5a39d015438cb0dc773d934ff25aa0915e7)).
- Highlight Markdown with Shiki's scopes, handing Rust and Markdown fences to their own lexers ([`087a539`](https://github.com/victor-software-house/nishiki/commit/087a539bba4b5c36ce8759729516d4d54479d107)).
- Match Markdown fence names and raw HTML block tags written with `ß` for `ss`, as Shiki does.
  The fence names now take the other folds from the shared table, not from a copy of their own ([`0e10992`](https://github.com/victor-software-house/nishiki/commit/0e109920e967479bd0044e32ff9dcc2091c1cc18)).
- Lex Markdown three to nineteen times faster by trying each rule only where it can start ([`1de67e3`](https://github.com/victor-software-house/nishiki/commit/1de67e39e3fb494ac918aedd2664c0ba7c49b897)).
- A line of many Markdown strikethroughs now costs work linear in its length ([`e3b00bc`](https://github.com/victor-software-house/nishiki/commit/e3b00bc0b62d10ad99cad8b6ed0c12d7c5f946e5)).
- Hand TypeScript fences in Markdown (`ts` and `typescript`) to the TypeScript lexer ([`1f6b5c5`](https://github.com/victor-software-house/nishiki/commit/1f6b5c5068dd27eba56570c5f7e1d76629366be6)).
- Markdown reads `\w` and `\s` from the classes measured on Shiki.
  A combining mark such as U+093C or U+0951 after an emphasis delimiter keeps it from closing, as in Shiki.
  `Theme::style` does not allocate for each scope of the stack it styles ([`b12b6f6`](https://github.com/victor-software-house/nishiki/commit/b12b6f63d4887fdb01ce5edc71102b66f139e810)).
- Markdown counts every byte that a link reference definition's title or a fence's `{.lang}` attributes read against the work limit.
  It checks each in time linear in the line, so no hostile line can make the work quadratic in its length.
  A fence line that runs out of interned nodes resumes the same whether its state is cached or not ([`d1ae494`](https://github.com/victor-software-house/nishiki/commit/d1ae494df5c2bfe4eababdcae3dac0bb311a9fcc)).
- Highlight MATLAB with nishiki's TextMate engine on Shiki's `matlab` grammar, behind the `lang-matlab` feature in the opt-in `textmate` tier ([`cecb54e`](https://github.com/victor-software-house/nishiki/commit/cecb54e7deeaa67152a5c4ee360343d941e13b94)).
- Highlight MATLAB with a lexer of its own, enabled by default and in `all`.
  The `lang-matlab` feature leaves the opt-in `textmate` tier ([`abf2557`](https://github.com/victor-software-house/nishiki/commit/abf2557d8433a0fdee701dbe333234a8c22c9f0d)).
- A Mermaid click's tooltip string ends at the line's last `"`, as in Shiki.
  Text after it, such as a trailing backslash, keeps its own scopes, and a click line with no `"` is no click ([`4ec9ee1`](https://github.com/victor-software-house/nishiki/commit/4ec9ee1460bcecd79abf8cd9f142495208140194)).
- Highlight Mermaid with nishiki's TextMate engine on Shiki's `mermaid` grammar, behind the `lang-mermaid` feature in the opt-in `textmate` tier ([`5f314eb`](https://github.com/victor-software-house/nishiki/commit/5f314ebf659bcbb5e0f59fef76e31a57b2696328)).
- Highlight Mermaid with a lexer of its own, enabled by default and in `all`.
  The `lang-mermaid` feature leaves the opt-in `textmate` tier ([`8b589b1`](https://github.com/victor-software-house/nishiki/commit/8b589b18d02e3a96645b3db8ad0057e094e0fce9)).
- In Dart, Go, Jinja, Kotlin, Nix, PowerShell, Protocol Buffers and Zig, a long run of spaces, tabs and Unicode spaces ends under the work limit.
  These languages pass a run of such mixed blanks in one step.
  Their patterns treat a Unicode space as a space, so the tokens stay the same ([`49dd496`](https://github.com/victor-software-house/nishiki/commit/49dd496a88930b1bcb091a935d0435167b61f872)).
- Lex shell scripts slightly faster by copying the values of six once-per-line searches instead of borrowing them ([`3665e60`](https://github.com/victor-software-house/nishiki/commit/3665e60000e1397549e881ea4b4110579adbbaf2)).
- The TextMate engine stops nesting capture groups at 256 on a line, and reports the depth limit there.
  A line that nested more, such as two thousand Perl heredoc openers, overflowed the stack of a debug build ([`a184aa6`](https://github.com/victor-software-house/nishiki/commit/a184aa61df6be58078cefa3a378fa1b4ed33d053)).
- Highlight Nix with nishiki's TextMate engine on Shiki's `nix` grammar, behind the `lang-nix` feature in the opt-in `textmate` tier ([`1fd6dc0`](https://github.com/victor-software-house/nishiki/commit/1fd6dc0790e55d0b3441d3e6a7a38ea0cccddfc5)).
- Highlight Nix with a lexer of its own, enabled by default and in `all`.
  The `lang-nix` feature leaves the opt-in `textmate` tier ([`7a573d5`](https://github.com/victor-software-house/nishiki/commit/7a573d58783861ff968543ab6b95bf09d1d439dc)).
- Rename the two public types whose names held "kind", and the field and methods that use them ([`2f746cf`](https://github.com/victor-software-house/nishiki/commit/2f746cf6488a8ae9ed89b3b108d9a0f86ee6cd5f)).

  This breaks every caller that names them:

  - `ThemeKind` is now `ColorScheme`.
    `Theme::kind` is now `Theme::color_scheme`, and `ThemeSource`'s `kind` field is now `color_scheme`.
  - `LineKind` is now `Freshness`, and `DocumentLine::kind` is now `DocumentLine::freshness`.

  Each variant keeps its name, and no output changes.
- Give each licence in the third-party notices its own block.
  This restores the diff and C++ licence texts and corrects the Lua grammar's copyright line ([`bd7ac1d`](https://github.com/victor-software-house/nishiki/commit/bd7ac1d8c8dd98d898b202fbf2678041c8e887ae)).
- An Objective-C fence in Markdown tries Markdown's root patterns at each `$base` include of its grammar, as Shiki does.
  After `(`, a paragraph, an HTML block or a link definition now opens inside the parentheses ([`8ebdf91`](https://github.com/victor-software-house/nishiki/commit/8ebdf910bf285e05fe02bb48628532672c5be0bd)).
- Highlight Objective-C with nishiki's TextMate engine on Shiki's `objective-c` grammar, behind the `lang-objective-c` feature in the opt-in `textmate` tier.
  A Markdown fence named `objc`, `objective-c`, `obj-c`, `objectivec`, `mm` or `m` embeds it ([`370d3ea`](https://github.com/victor-software-house/nishiki/commit/370d3eab5c9e9e598cdf058249d56379153b35d7)).
- Highlight Objective-C with a lexer of its own, enabled by default and in `all` and `systems`.
  The `lang-objective-c` feature leaves the opt-in `textmate` tier.
  Markdown's `objc`, `objective-c`, `objectivec`, `obj-c`, `m` and `mm` fences now highlight with the default features ([`f47f699`](https://github.com/victor-software-house/nishiki/commit/f47f6994a327a778fd3367458169bbd4a10712ec)).
- Highlight Objective-C++ with nishiki's TextMate engine on Shiki's `objective-cpp` grammar, behind the `lang-objective-cpp` feature in the opt-in `textmate` tier ([`d9a053d`](https://github.com/victor-software-house/nishiki/commit/d9a053d403b66c650e1238878cfb1cfca21ce125)).
- Highlight Objective-C++ with a lexer of its own, enabled by default and in `all` and `systems`.
  The `lang-objective-cpp` feature leaves the opt-in `textmate` tier and turns on `lang-objective-c`, whose matchers it shares ([`f80e5ad`](https://github.com/victor-software-house/nishiki/commit/f80e5adc602d7dc3f8fde13470775b2b096e451d)).
- CoffeeScript, PHP, Perl, Elixir, GraphQL, HAML and LaTeX keep one copy of the code that hands a line to an embedded language and back.
  The crate's code is smaller, and those languages run up to 0.07% fewer instructions ([`591a21a`](https://github.com/victor-software-house/nishiki/commit/591a21adf7135a7da7cc38c93d7980ee17e58b09)).
- Highlight Pascal, including Delphi and Free Pascal, with nishiki's TextMate engine on Shiki's `pascal` grammar, behind the `lang-pascal` feature in the opt-in `textmate` tier ([`7006f23`](https://github.com/victor-software-house/nishiki/commit/7006f2355ffdb767cadd00ebd04c24a30ec09eb9)).
- Pascal and Common Lisp look their keywords up in a byte loop.
  Pascal runs about 8% fewer instructions, and Common Lisp about 11% fewer.
  Their counts stay the same wherever the words lie ([`0155a83`](https://github.com/victor-software-house/nishiki/commit/0155a836b4f657fbb5493a390cb36ffec0ff2d2c)).
- Highlight Pascal with a lexer of its own, enabled by default and in `all`.
  The `lang-pascal` feature leaves the opt-in `textmate` tier ([`ea40d57`](https://github.com/victor-software-house/nishiki/commit/ea40d572c8bdb61c7e39410f713ab3040a0379d9)).
- Fix scopes that one token took from another on the same stack.
  Two neighbouring escapes in a CSS class name were merged under the first one's scope.
  A Java record's `<super>` could take the `extends` scope of an earlier record ([`a187235`](https://github.com/victor-software-house/nishiki/commit/a187235edc1a2ccdcb482449cd530a01b7d53ef8)).
- Highlight Perl with nishiki's TextMate engine on Shiki's `perl` grammar, behind the `lang-perl` feature in the opt-in `textmate` tier.
  Its heredocs hand HTML, XML, CSS, JavaScript and SQL to their lexers, and a Markdown fence named `perl`, `pl`, `pm`, `pod`, `t`, `psgi` or `vcl` embeds it ([`e39a225`](https://github.com/victor-software-house/nishiki/commit/e39a225bc1ebc7847e50857ee39656fdf577a960)).
- Highlight Perl with a lexer of its own, enabled by default and in `all`.
  The `lang-perl` feature leaves the opt-in `textmate` tier.
  It turns on CSS, HTML, JavaScript, SQL and XML, which Perl's heredocs embed.
  Markdown's `perl` fences are now highlighted as Perl in a default build, where they were plain text before ([`ee1c928`](https://github.com/victor-software-house/nishiki/commit/ee1c9289931bdeffac2ed692348cd5a3f0883dbd)).
- Highlight PHP with nishiki's TextMate engine on Shiki's `php` grammar, behind the `lang-php` feature in the opt-in `textmate` tier.
  Its strings and heredocs hand SQL, HTML, XML, JavaScript, JSON and CSS to their lexers.
  With `lang-php`, a Markdown `php` fence lexes HTML and PHP together, as Shiki does.
  Without it, the fence stays HTML, as before ([`d3453fc`](https://github.com/victor-software-house/nishiki/commit/d3453fcf11b039cbf0f2bcc96666ff2798969742)).
- Highlight PHP with a lexer of its own, enabled by default and in `all`.
  The `lang-php` feature leaves the opt-in `textmate` tier.
  It turns on CSS, HTML, JavaScript, JSON, SQL and XML, which PHP's strings and heredocs embed.
  Markdown's `php` fences are now highlighted as HTML and PHP in a default build, where they were HTML alone before ([`13165c3`](https://github.com/victor-software-house/nishiki/commit/13165c38a705cf946c62329939f244dff7ea1c2d)).
- Highlight PostCSS with a lexer of its own, behind the `lang-postcss` feature in the default `all` tier and in `web` ([`cc6b288`](https://github.com/victor-software-house/nishiki/commit/cc6b2882cea681cc3b4deafc6a4b8d35c2e7ee4d)).
- Highlight PowerShell with the scopes of Shiki's `powershell` grammar, run by nishiki's own TextMate engine, and the `powershell`, `ps1`, `psm1`, `psd1` and `pwsh` fences of Markdown.
  The language is the `lang-powershell` feature, in the new `textmate` tier, which is off by default ([`4d18e6b`](https://github.com/victor-software-house/nishiki/commit/4d18e6b15a8d84672de73acae5f4917f5b21db5a)).
- Highlight PowerShell with a lexer of its own, enabled by default and in `all`.
  The `lang-powershell` feature leaves the opt-in `textmate` tier.
  Markdown's PowerShell fences are now highlighted as PowerShell in a default build, where they were plain text before ([`aa26dda`](https://github.com/victor-software-house/nishiki/commit/aa26ddaf88e182313623ef111931935ba30d66d1)).
- The doc comments read more plainly ([`38cd849`](https://github.com/victor-software-house/nishiki/commit/38cd84997d7a3aabe6ca65d1f2eaf70b955fb534)).
- Highlight Protobuf with nishiki's TextMate engine on Shiki's `proto` grammar, behind the `lang-proto` feature in the opt-in `textmate` tier ([`444670c`](https://github.com/victor-software-house/nishiki/commit/444670c2d91fa27bf9aa910cfd6a4b15bb369bf0)).
- Protocol Buffers runs on matchers generated from its grammar, in place of hand-written ones.
  Its output is unchanged, and it runs about 5% fewer instructions ([`6f0a8c9`](https://github.com/victor-software-house/nishiki/commit/6f0a8c90dc3ccd2b90688541962aa6adf6be96e3)).
- Highlight Protobuf with a lexer of its own, enabled by default and in `all`.
  The `lang-proto` feature leaves the opt-in `textmate` tier ([`efcca0c`](https://github.com/victor-software-house/nishiki/commit/efcca0ccbc9611d2c614b5484d3f4b4887e8a273)).
- End a single-quoted Python docstring at the line end even after a backslash, as Shiki does.
  The next line is code, not more of the string ([`4e70c24`](https://github.com/victor-software-house/nishiki/commit/4e70c24323a4a42eecb42e6fecf8c5f04ef9b2bf)).
- Highlight Python with the scopes of Shiki's `python` grammar, and the `python`, `py` and related fences of Markdown ([`5ee08b8`](https://github.com/victor-software-house/nishiki/commit/5ee08b8d9ee217130c644d0158b9b8215eeaf83a)).
- Highlight R with a lexer of its own, enabled by default and in `all`, behind the `lang-r` feature.
  Markdown's `r`, `s` and `rprofile` fences are now highlighted as R in a default build, where they were plain text before ([`c4d7eef`](https://github.com/victor-software-house/nishiki/commit/c4d7eefbbcf782eea6807fe8b05944fcdeaa20a0)).
- Restructure the README into short sentences, lists and tables, with each language's Shiki quirks as a bullet list ([`e8c9c81`](https://github.com/victor-software-house/nishiki/commit/e8c9c81f37a86eb3005a948b2cb2887cdb18c031)).
- The README's language table names the tier that turns each language on ([`d13c929`](https://github.com/victor-software-house/nishiki/commit/d13c929b4d9db05d4a773fac710b5bcda15a9f99)).
- Lex a Ruby here document named for C++ or Lua, such as `<<~CPP` or `<<~LUA`, in that language, as Shiki does.
  Ruby's interpolation and escapes apply where the language has nothing open ([`7a2d6b3`](https://github.com/victor-software-house/nishiki/commit/7a2d6b3b5091aff6852a9558332cc2fc350065ea)).
- Lex a Ruby here document named for SQL, GraphQL, CSS, C, shell or Ruby, such as `<<~SQL`, in that language, as Shiki does.
  Ruby's interpolation and escapes apply where the language has nothing open.
  They were plain here documents before ([`d2e2953`](https://github.com/victor-software-house/nishiki/commit/d2e2953948e0d96bd11b73a964766471c6678bba)).
- The Ruby docs drop the claim that the text after `__END__` is lexed as HTML.
  That text stays Ruby, as in Shiki ([`569d1ea`](https://github.com/victor-software-house/nishiki/commit/569d1ea89318ebee19893f2fbae058f1ef63be83)).
- Ruby's here documents leave `\G` where each of their lines starts, as Shiki does.
  In a shell here document, a line after one that ends in `\` starts with a command name.
  Before, that line had no command name ([`0e3b260`](https://github.com/victor-software-house/nishiki/commit/0e3b26080335e0d8d163ff053f9e37cffec04fd0)).
- Lex a Ruby here document named for HTML, such as `<<~HTML`, as HTML, as Shiki does.
  Ruby's interpolation and escapes apply where the HTML has nothing open ([`b3568d6`](https://github.com/victor-software-house/nishiki/commit/b3568d665b51364e219d7267acf9ebdd37c74430)).
- Lex a Ruby here document named for JavaScript, such as `<<~JS`, as JavaScript, as Shiki does.
  Ruby's interpolation and escapes apply where the JavaScript has nothing open ([`082877a`](https://github.com/victor-software-house/nishiki/commit/082877a18bf1eed4838a2205e4062b30defde58b)).
- Highlight Ruby with the scopes of Shiki's `ruby` grammar, and the `ruby`, `rb`, `rake`, `gemspec`, `Gemfile` and other Ruby fences of Markdown ([`7d3c2b6`](https://github.com/victor-software-house/nishiki/commit/7d3c2b60c24491072991b45f37a585b3a390b1c3)).
- Ruby closes a percent literal that ends a line, such as `%w`, where the next line ends, as Shiki does.
  The line feed is the literal's delimiter, so the literal holds one line.
  Before, the literal held every line after it ([`b9c112b`](https://github.com/victor-software-house/nishiki/commit/b9c112bc725a8f80d40dcd683e8623e0ec780324)).
- After a Ruby here document opener, a second opener on the same line takes the rest of the line into its embedded language, as Shiki does.
  That text gets the embedded language's scopes.
  Before, it kept Ruby's scopes ([`a8a2946`](https://github.com/victor-software-house/nishiki/commit/a8a29469c712bd0fb80f476ecb893018a542a2f7)).
- Lex a Ruby here document named for XML, such as `<<~XML`, as XML, as Shiki does.
  Ruby's interpolation and escapes apply where the XML has nothing open ([`b11e78d`](https://github.com/victor-software-house/nishiki/commit/b11e78d7c675fc155d8a21cb341fdc82b880f3b7)).
- Lex a Ruby here document named for YAML, such as `<<~YAML`, as YAML.
  The line goes back to Ruby where Shiki hands it back: at an interpolation where a YAML step begins ([`e12c095`](https://github.com/victor-software-house/nishiki/commit/e12c095322b1b99733aba2ffbb53f56b1cbfe367)).
- Lex deeply nested lines faster on the rule engine.
  A line does not visit every open rule to end.
  A Ruby document 300 levels deep takes a quarter fewer instructions ([`0a0b7b1`](https://github.com/victor-software-house/nishiki/commit/0a0b7b1a92413987f61ac2ae682819291353cbd4)).
- Reach the first token of C++, C#, CSS, Go, Java, Kotlin, Less, Lua, Python, Ruby, SCSS and Swift in about half the instructions.
  Each pattern's start is read once, when a rule's tables are built ([`05fc359`](https://github.com/victor-software-house/nishiki/commit/05fc359b1d1c4eefac5099ccf4e0d47e22cc689c)).
- Look up a rule's candidate tables with slightly fewer instructions in C++, C#, CSS, Go, Java, Kotlin, Python, Ruby and Swift.
  Each table is filled through a closure that copies the rule instead of borrowing it ([`6ee21f7`](https://github.com/victor-software-house/nishiki/commit/6ee21f7560810dec7a98e57a80a8e7d605e8a1a7)).
- Lex Rust about 1.3 times faster, in files and in Markdown fences.
  A word is compared with the keyword lists only when its first byte and length allow it ([`64bafaa`](https://github.com/victor-software-house/nishiki/commit/64bafaa4c82279fba7782cb29806629312e2757e)).
- Read the Unicode spaces that Shiki's `\s` reads as blanks in Rust, and not as word characters.
  They are U+0085, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F and U+3000.
  `fn` and `pub` before one are then keywords, as they are in Shiki ([`ddeda8d`](https://github.com/victor-software-house/nishiki/commit/ddeda8d8c383c85e0100ca9780ed0f5c13e71e95)).
- Rust's `\w`, `\b` and `\d` match Unicode letters and digits, as Shiki does.
  An Arabic-Indic digit such as `٣` starts a number, and an emoji such as `🦀` ends a name ([`1db48d4`](https://github.com/victor-software-house/nishiki/commit/1db48d462abce957c32df423427fe22a3dd6fdc8)).
- Highlight Sass, the indented syntax, with nishiki's TextMate engine on Shiki's `sass` grammar, behind the `lang-sass` feature in the opt-in `textmate` tier ([`8480291`](https://github.com/victor-software-house/nishiki/commit/84802910cd0e3ba40e84cd22f1d76fd9831a99c7)).
- Highlight Sass with a lexer of its own, enabled by default and in `all`.
  The `lang-sass` feature leaves the opt-in `textmate` tier.
  Vue's `<style lang="sass">` and HAML's `:sass` filter use the same lexer ([`1487657`](https://github.com/victor-software-house/nishiki/commit/1487657e2a1fabd1a17e8cb5dabf873f2116db30)).
- End a Sass `url(…)` whose `)` follows blanks as Shiki does, so the `)` closes the function ([`07f0931`](https://github.com/victor-software-house/nishiki/commit/07f093142f5abef80eee10a01981190d612df6d1)).
- Highlight Scala with nishiki's TextMate engine on Shiki's `scala` grammar, behind the `lang-scala` feature in the opt-in `textmate` tier, and the `scala` and `sbt` fences of Markdown ([`fb55c04`](https://github.com/victor-software-house/nishiki/commit/fb55c04171f92d097fc4cf26268233b193497db0)).
- Markdown's `scala` and `sbt` fences are now highlighted as Scala in a default build, where they were plain text before ([`a7ea4cf`](https://github.com/victor-software-house/nishiki/commit/a7ea4cf53e0fb8f63c349296930639ccd6deb993)).
- Highlight Scala with a lexer of its own, enabled by default and in `all`.
  The `lang-scala` feature leaves the opt-in `textmate` tier ([`a7ea4cf`](https://github.com/victor-software-house/nishiki/commit/a7ea4cf53e0fb8f63c349296930639ccd6deb993)).
- In a Scala `//> using` directive, leave a backtick plain when its quoted name closes after the first blank, as Shiki does ([`6eb4362`](https://github.com/victor-software-house/nishiki/commit/6eb4362f1e08e3ba71fa96ebd49687d64990bf42)).
- Languages on the shared rule engine skip, past each line feed, the patterns that cannot match there.
  Ruby runs about 36% fewer instructions, C++ about 23%, Go about 32% and Perl about 40% ([`b906232`](https://github.com/victor-software-house/nishiki/commit/b906232d6f01b543bd567465440ff1a0c8e0362a)).
- Languages on the shared rule engine do not try a pattern where its leading lookbehind rules it out.
  Such places are the start of the line and the inside of a word.
  Astro runs about 5% fewer instructions, Scala and Go about 4%, and Swift about 3% ([`d860631`](https://github.com/victor-software-house/nishiki/commit/d86063169b20e99fba32e0ff62a1991e4225b5b6)).
- The languages on the rule engine, and shell, run up to 0.6% fewer instructions.
  Their stack cache stays in place when another language's tables change.
  A highlighter allocates 16 KiB less, because shell's cache is made only when shell lexes a line ([`1ae6378`](https://github.com/victor-software-house/nishiki/commit/1ae63789015f6a6d4ec243ab8ccfac5862392aa5)).
- Highlight SCSS with the scopes of Shiki's `scss` grammar, and the `scss` fences of Markdown.
  The rules the grammar takes from CSS run on the CSS lexer ([`06c5866`](https://github.com/victor-software-house/nishiki/commit/06c5866bdaa16c6879544da6439b6b2d7e19e481)).
- Highlight ShaderLab with a lexer of its own, enabled by default and in `all`, as `Language::ShaderLab` and the names `shaderlab` and `shader`.
  Its `CGPROGRAM` and `HLSLPROGRAM` blocks go to the HLSL lexer, with Unity's own names ([`00d96ef`](https://github.com/victor-software-house/nishiki/commit/00d96ef4976b872cc12ca99aa1431ed64ad29a6c)).
- Every language on one line spends that line's one work budget.
  A fence in Markdown starts no budget of its own.
  Nor do a Ruby heredoc's embedded language, an HTML document's scripts and styles, or the JavaScript of a GraphQL interpolation ([`4e5ff6c`](https://github.com/victor-software-house/nishiki/commit/4e5ff6c575e773b89eaa45785953feecddf5fe4d)).

  Under a pinned `Limits::work`, such a line can reach the limit where it did not before.
  Every language still counts only its own contexts against the depth limit.
  The languages on nishiki's TextMate engine keep their own budget for now.
- Share one rule engine between the Python, Java, Makefile, CSS and Kotlin lexers.
  Their output does not change, and Python, Java, CSS and Makefiles lex as fast or faster ([`77e98b2`](https://github.com/victor-software-house/nishiki/commit/77e98b2f753ed572f1172de16a9f2cecc5d77f22)).
- Keep `\G` at the start of every shell line of a Markdown fence, as Shiki does, so a line after a continuation matches its output ([`77d2709`](https://github.com/victor-software-house/nishiki/commit/77d270915cd7565232aaf17dd3e19af90207e059)).
- End open shell constructs at a Markdown fence line that the list or quote containers take whole, as Shiki does ([`746f6bd`](https://github.com/victor-software-house/nishiki/commit/746f6bd6d2dcffe36fe62a9b3ecf0f9f719609ae)).
- Highlight shell scripts, and the `sh`, `bash`, `shell` and `zsh` fences of Markdown, with the scopes of Shiki's `shellscript` grammar.
  `Scopes` yields names borrowed from the highlighter instead of `&'static str`, so it can name scopes made from source text ([`57e02a8`](https://github.com/victor-software-house/nishiki/commit/57e02a8d7b7d334a3cd2ae4feda0fddfc43ed31e)).
- Lex shell lines inside deeply nested rules without copying the open rules on each line.
  A document 300 levels deep takes about a third fewer instructions ([`7dec9ed`](https://github.com/victor-software-house/nishiki/commit/7dec9edd9b3020653cce00d08c07580e5631bc35)).
- Make the shell lexer 3 to 10 percent faster by answering repeated blank and name lookups from the byte or the last answer ([`70fc32d`](https://github.com/victor-software-house/nishiki/commit/70fc32ded5e871ed6a78f7a79fe80108c2448639)).
- `Limits::depth` leaves out a language's root context in JSON, TOML, YAML, Dockerfile and INI, as in the other languages.
  Under a pinned depth, these open one more nested context than before ([`f8352c6`](https://github.com/victor-software-house/nishiki/commit/f8352c6758149d9abbb396ad7f46e239c3afbbff)).
- TOML runs about 2% fewer instructions in a default release build, at sixteen codegen units ([`9dc613a`](https://github.com/victor-software-house/nishiki/commit/9dc613a23b2ff508214a24b16caa27a3e636f521)).
- Lex SQL with about 7 percent fewer instructions, by comparing the ASCII bytes of a keyword without decoding them ([`3cafe2e`](https://github.com/victor-software-house/nishiki/commit/3cafe2e70311c45a0ee1cd13534d38d6b834499d)).
- Lex SQL with about 5 percent fewer instructions than before its move onto the rule engine, in a default release build.
  The compiler may inline its pattern dispatch across modules ([`f14abd3`](https://github.com/victor-software-house/nishiki/commit/f14abd312ab73d9160e8173a9f75651c0e68c6fa)).
- Highlight SQL with the scopes of Shiki's `sql` grammar, and the `sql`, `ddl` and `dml` fences of Markdown ([`a18bfbd`](https://github.com/victor-software-house/nishiki/commit/a18bfbd56917cc311cae2a488fc8cd0b0ab1bfea)).
- SQL runs about 7% fewer instructions in a default release build, at sixteen codegen units ([`de60d0e`](https://github.com/victor-software-house/nishiki/commit/de60d0e77fc26fe2e89d4f8386ac853e4f3eb40f)).
- Match SQL keywords written with a character that folds to several letters, such as `ß` for `ss` or the ligature `ﬆ` for `st`, as Shiki does ([`3806415`](https://github.com/victor-software-house/nishiki/commit/3806415bc75e7a6d50b163f91a9dfefa9bed7b58)).
- SQL runs on the shared rule engine.
  Lexing a SQL line allocates nothing once the document's states exist.
  A pinned `Limits::depth` leaves out SQL's root context, as in the other languages ([`2768ebc`](https://github.com/victor-software-house/nishiki/commit/2768ebc473d10aeeac1e88bfa293c2cc007c7b88)).
- Svelte opens a `<script>` or `<style>` body after a `>` inside the tag's attribute value, as Shiki does.
  In `<script lang=">let one = 1;</script>`, the text after the `>` is JavaScript, and so are the lines after the tag.
  Before, the text stayed in the attribute's string ([`ff46d4b`](https://github.com/victor-software-house/nishiki/commit/ff46d4bf6422a6cec3a4b6efda11ec1beb2f7e5c)).
- Give Svelte's `$` prefixes their scope in a CoffeeScript body too, as Shiki does: in code, comments, interpolations and the JavaScript between backticks ([`146a8c4`](https://github.com/victor-software-house/nishiki/commit/146a8c4097c3bcbea86b729196cc3724ec20d4f9)).
- In a Svelte `@component` comment, a `<script>` in the comment's Markdown HTML takes Svelte's `$` prefixes, as in Shiki ([`3d22671`](https://github.com/victor-software-house/nishiki/commit/3d22671fc5b68b1b245c660e523c3a52b7af77a6)).
- Svelte lexes the line after an open `{#each}` tag as Shiki does.
  Where that line starts with `{…}` and more text follows, the text keeps the destructuring's scopes.
  Before, the line reached the work limit ([`44f84af`](https://github.com/victor-software-house/nishiki/commit/44f84af5f8d47fd89fccbe3e1591800927d64ee8)).
- Highlight Svelte with a lexer of its own, behind the `lang-svelte` feature.
  The feature is in the default `all` tier and in `web` ([`14d2d41`](https://github.com/victor-software-house/nishiki/commit/14d2d41a4f5c30ebd67d14123cd0cf388e553ef5)).

  - Its expressions and `<script>` bodies go to the TypeScript and JavaScript lexers.
  - Its `<style>` bodies go to the CSS, PostCSS, SCSS, Less and Sass lexers.
  - A `lang="coffee"` body goes to the CoffeeScript lexer when `lang-coffee` is on.
  - An `@component` comment goes to the Markdown lexer when `lang-markdown` is on.
- A Svelte grammar loaded through `Grammar::from_json` now gives the CoffeeScript it hands off Svelte's `$` prefixes too.
  The `$` of `$count` is `punctuation.definition.variable.svelte` there, as in Shiki.
  This holds with or without the `lang-svelte` feature ([`0b6d988`](https://github.com/victor-software-house/nishiki/commit/0b6d988627911f69521fa50017eb757fd590465e)).

  Before, the handed CoffeeScript read `$` as part of the name.
- A Svelte grammar loaded through `Grammar::from_json` now gives the JavaScript and TypeScript it hands off Svelte's `$` prefixes.
  The `$` of `$count` is `punctuation.definition.variable.svelte` there, as in Shiki and in nishiki's own Svelte.
  This holds with or without the `lang-svelte` feature.
  HTML in such a grammar gives its `<script>` the prefixes too ([`af4c53d`](https://github.com/victor-software-house/nishiki/commit/af4c53dc8ddf6db20c001adf49c6022a23f2ec6b)).

  Before, the handed code read `$` as part of the name.
- Lex a Svelte `<script>` or `<style>` body that starts with `/>` as the body, as Shiki does, instead of ending the tag there ([`589b7ca`](https://github.com/victor-software-house/nishiki/commit/589b7ca3d99e998e775459e71e5772f26f60bd20)).
- Highlight Swift as Shiki does where a capture lexed again ends, such as an interpolation inside `#sourceLocation`.
  There an operator at its end is postfix, and a prefix operator is infix ([`a3de3d4`](https://github.com/victor-software-house/nishiki/commit/a3de3d4fc421849595459bdc9c13ed975bfa2651)).
- Highlight Swift with the scopes of Shiki's `swift` grammar, and the `swift` fences of Markdown ([`41a3c51`](https://github.com/victor-software-house/nishiki/commit/41a3c5149b6a79b55d4d0d388fb79d02bab1e4b4)).
- Highlight TeX with a lexer of its own, under the `lang-tex` feature, enabled by default and in `all` and `docs` ([`7d1da77`](https://github.com/victor-software-house/nishiki/commit/7d1da7701e4bf080048f1bd06d0907d5c966870d)).
- Highlight Scala, Zig and PowerShell with about 11 to 19 percent fewer instructions on the TextMate engine.
  Each rule list's automaton cache stays in place on every scan ([`b9fa2b8`](https://github.com/victor-software-house/nishiki/commit/b9fa2b80227291c536ce22371d4d2a5a46a9499b)).
- A loaded grammar's pattern with an Oniguruma callout, such as `(*FAIL)`, never matches.
  The regex parser refuses callouts, where it read one as a group of literal characters ([`42fb704`](https://github.com/victor-software-house/nishiki/commit/42fb7047022334862f8ecd88b685087413f9d386)).
- Colour the first line of a language on nishiki's TextMate engine with 7 to 37 percent fewer instructions.
  Each character class's ASCII members are worked out once from its ranges, instead of testing every character twice ([`85be0f5`](https://github.com/victor-software-house/nishiki/commit/85be0f57a6af4d9bed7378370959d7c32694f049)).
- Search with the TextMate engine's patterns without clearing every capture slot at each position tried ([`9184b13`](https://github.com/victor-software-house/nishiki/commit/9184b130c01ee832f4fa2eb648da55831ddb9b5b)).
- A language on nishiki's TextMate engine inside a Markdown fence now spends that line's one work budget, as the hand-written languages do.
  Under a pinned `Limits::work`, such a line can reach the limit where it did not before ([`89bb55c`](https://github.com/victor-software-house/nishiki/commit/89bb55cfcf70283594556ffe9dc812520be5848b)).
- Lex lines inside deeply nested rules faster with the TextMate engine.
  Each line reuses the open rules the previous line left.
  It touches only the rules it opens or closes and the open `while` rules ([`4424ddb`](https://github.com/victor-software-house/nishiki/commit/4424ddbd29c7039a3a40d2395e0c0f124571aa48)).
- Keep the TextMate engine's caches for each grammar when a document switches between grammars.
  A Markdown document that has fences in two of the engine's languages switches that way.
  Lexing such a document again allocates nothing ([`ac0df22`](https://github.com/victor-software-house/nishiki/commit/ac0df22897db543377796f7dc3e69517bc4c741c)).
- The first token of a language on nishiki's TextMate engine comes sooner.
  A rule list's scanner now parses each pattern once and compiles no program.
  A pattern's program compiles when a search first tries the pattern ([`81a4957`](https://github.com/victor-software-house/nishiki/commit/81a4957de05ee487b5f11abdd1851a61e16c7ca3)).
- The TextMate engine runs a smaller program on a line with no character that a multi-character case fold needs, such as `ß` or `ﬁ`.
  PowerShell, CMake, Batch and the other engine languages with case-insensitive patterns lex faster.
  They also keep fewer automaton caches in memory ([`285b1d4`](https://github.com/victor-software-house/nishiki/commit/285b1d48b5f427e9cc884556d7e6b69f60f421d4)).
- Lex the languages on nishiki's TextMate engine with fewer instructions, by testing a literal's first byte before comparing the rest ([`d00733f`](https://github.com/victor-software-house/nishiki/commit/d00733f52628260781923349a5be4d7440f82f93)).
- The first token of a language on nishiki's TextMate engine comes sooner.
  Each pattern's relaxed expression is now built without cloning the pattern's parsed tree ([`31aca0f`](https://github.com/victor-software-house/nishiki/commit/31aca0f6a3364f7187cd6c8e1c1bfc415fd1ab7d)).
- Reach the first token of a language on nishiki's TextMate engine sooner.
  Each rule list's scanner is built from the patterns' expressions, instead of printing them as source and parsing them back ([`354a40e`](https://github.com/victor-software-house/nishiki/commit/354a40e2e6e6cfe1e5ab20f7ce10432adfdc58b8)).
- The first token of a language on nishiki's TextMate engine comes sooner.
  A rule list's scanner now extracts its prefilter with a tighter limit than `regex-automata`'s.
  A list of literals keeps `regex-automata`'s own extraction ([`3088c2c`](https://github.com/victor-software-house/nishiki/commit/3088c2c9c64ad8e3bf72fde46dd0b9221cc3501c)).
- `Theme::style` allocates nothing for stacks of up to 64 scopes.
  Colouring a document runs about 7% fewer instructions for the median language, and JSON about 31% fewer ([`988e7c0`](https://github.com/victor-software-house/nishiki/commit/988e7c098a5e7a6489e72d046cbd2e636d9cae38)).
- Colouring a document with a theme runs about 18% fewer instructions, lexing included ([`a02262b`](https://github.com/victor-software-house/nishiki/commit/a02262b406ab944968d0a111add74c469c28b599)).
- Colour tokens with TextMate themes in Shiki's format.
  `Theme::style` gives a scope stack the colour and font style Shiki gives it.
  The optional `json` feature reads theme files with `Theme::from_json` ([`ebedaca`](https://github.com/victor-software-house/nishiki/commit/ebedaca99d2b297294359a7acec4e082d9cf3c8d)).
- Highlight TOML with the scopes of Shiki's `toml` grammar ([`4337092`](https://github.com/victor-software-house/nishiki/commit/4337092a3b2520f708086ea6ca083022b5b6f80e)).
- A TypeScript line that reaches the node limit where a context opens stops there, instead of spending its whole work budget on the same step ([`4523bfb`](https://github.com/victor-software-house/nishiki/commit/4523bfb2b41a19f73ce1dfebb9191ddecd81658a)).
- TypeScript, JavaScript, CoffeeScript and Objective-C compare their words and operators in byte loops instead of libc's `memcmp` ([`1c45faf`](https://github.com/victor-software-house/nishiki/commit/1c45faf168de2fc8b4cdca0b0dec83d987577162)).
- End a TypeScript type annotation over the blanks before a brace, so `: void {` after an open parenthesis does not begin a binding pattern ([`0af4c85`](https://github.com/victor-software-house/nishiki/commit/0af4c854488c89de64194cef008f219dc6889dc4)).
- Match `^` in TypeScript, JavaScript, JSX and TSX code only at the start of the line.
  It does not match at the start of a Markdown fence's text inside a list or a quote.
  There, as in Shiki, a type annotation runs on to the next line and a triple-slash directive is a plain comment ([`438f906`](https://github.com/victor-software-house/nishiki/commit/438f90622ad66dd69a489f4b81e2185566b1fa08)).
- Match Shiki in TypeScript and JavaScript on letters and blanks beyond ASCII.
  Names use Oniguruma's `[[:alpha:]]` and `[[:alnum:]]`, and `\b` uses its `\w`.
  A Unicode blank counts as `\s` wherever the grammar tests for one.
  Scans ahead in doc comments, JSX names and block comments count against the work limit, and a doc comment full of `[` is lexed in linear time ([`760f125`](https://github.com/victor-software-house/nishiki/commit/760f125bc33ec8c4398d5c00877010211a513c7f)).
- Match Shiki in TypeScript and JavaScript in two places.
  A line that starts with a field name followed by `}` keeps the field open.
  A parameter modifier such as `readonly` with no name after it becomes the parameter's name ([`bb6d975`](https://github.com/victor-software-house/nishiki/commit/bb6d975bf7e5ed2330580a00b17cc4d088a00310)).
- Match Shiki in TypeScript and JavaScript where a keyword rule's word boundaries differ from its neighbours'.
  `interface` and the doc-comment tags need `\b`, and `export` does not.
  A `namespace` needs a name after it.
  A cast at the start of a line has no blank after `<`.
  An arrow's return type needs a type before `=>` ([`b2c89c1`](https://github.com/victor-software-house/nishiki/commit/b2c89c1858149e367e6b6493425dae297576811c)).
- Highlight TypeScript with the scopes of Shiki's typescript grammar ([`1f6b5c5`](https://github.com/victor-software-house/nishiki/commit/1f6b5c5068dd27eba56570c5f7e1d76629366be6)).
- The TypeScript lexer does not allocate at an object value that may begin an arrow function ([`b914e40`](https://github.com/victor-software-house/nishiki/commit/b914e40a6062cab868d902401ae7f1d29c030b0a)).
- The TypeScript lexer searches the rest of a line for a backtick, a `(` or a `<` once per line.
  It does not search once per text that an embedding language lexes with it.
  A line of many HTML event handlers does not read the line up to each handler.
  The work counted is the bytes the searches read ([`4808ddb`](https://github.com/victor-software-house/nishiki/commit/4808ddbded2eff49118c9d5ad81e7575aaf6a328)).
- The TypeScript lexer does not allocate while lexing doc-comment tags or long chains of names ([`aa017cd`](https://github.com/victor-software-house/nishiki/commit/aa017cd072ebc5b2e1b05bfdf9704adea99bcac3)).
- Lex TypeScript and JavaScript faster.
  A line keeps only the contexts it opens, instead of filling two tables of every nesting depth.
  A deeply nested document does not allocate on each line ([`53868a9`](https://github.com/victor-software-house/nishiki/commit/53868a9576a1bc0693787ada67b7cbe115a78e76)).
- Make the TypeScript lexer a little faster by jumping over the plain text of comments and strings ([`1ac96ee`](https://github.com/victor-software-house/nishiki/commit/1ac96ee56081b83580975f09c7392cfdef4cd318)).
- The TypeScript lexer is 1.3 to 1.8 times faster, with the same output ([`d9ee552`](https://github.com/victor-software-house/nishiki/commit/d9ee552c2204a2795bf9fdfcf7864c5f144ad393)).
- Read the Unicode spaces that Shiki's `\s` reads as blanks in TypeScript, JavaScript, JSX and TSX.
  They are U+0085, U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F and U+3000.
  A keyword or a literal next to one is then highlighted as it is in Shiki ([`651b1e2`](https://github.com/victor-software-house/nishiki/commit/651b1e2044a3970a9909cd24da9bf64cc4192b46)).
- Highlight an object key of non-ASCII digits, such as `٤٢:`, as a number, as Shiki does.
  Shiki's `\d` takes every Unicode decimal digit ([`84ed988`](https://github.com/victor-software-house/nishiki/commit/84ed9883b324f81e7458a3cb79580492ef67e54b)).
- A long run of no-break or other Unicode spaces ends under the work limit.
  The rule-engine languages pass such a run in one step, as they pass spaces and tabs ([`28ed1b2`](https://github.com/victor-software-house/nishiki/commit/28ed1b269345301c93b6aa016b12eacdc8873724)).
- Add `nishiki::unstable`, the rule engine's grammar trait and the types it names, built only with `--cfg nishiki_unstable`.
  The module is exempt from semver: any release can change it, a patch included.
  `RuleSyntax<G>` runs such a grammar in a `Highlighter` ([`c85587c`](https://github.com/victor-software-house/nishiki/commit/c85587ccbbe89de322f5d7c29ff7366fd93f574c)).
- Lex text beyond ASCII faster.
  One shared decoder builds each character from the bits of its bytes.
  It runs no separate validation pass ([`2e76e76`](https://github.com/victor-software-house/nishiki/commit/2e76e76e8b0ffa31298e9e523bfbe1262b63832b)).
- Highlight Visual Basic .NET with nishiki's TextMate engine on Shiki's `vb` grammar, behind the `lang-vb` feature in the opt-in `textmate` tier.
  A Markdown fence named `vb` embeds it ([`f2fdc0c`](https://github.com/victor-software-house/nishiki/commit/f2fdc0c2a7ec7f88730982487c9b6142a146e30c)).
- Highlight Visual Basic with a lexer of its own, enabled by default and in `all`.
  The `lang-vb` feature leaves the opt-in `textmate` tier.
  Markdown's `vb` fences now highlight with the default features ([`ea3dadb`](https://github.com/victor-software-house/nishiki/commit/ea3dadb993ada46fc2febc6089aa99fe1ba8fc6e)).
- Vue lexes blocks in Markdown, JSON, JSONC, TOML and Vue with those languages' lexers, as Shiki does.
  Before, their text was plain.
  A `<!-- @name ... -->` comment holds JSON.
  In Vue's tags, `generic` holds TypeScript type parameters and `style` holds CSS, with Vue's own scopes.
  A directive's dynamic argument, such as `[eventName]` in `v-on:[eventName]`, holds a TypeScript expression ([`5cbd4cc`](https://github.com/victor-software-house/nishiki/commit/5cbd4cc85871fa335c65fc6c501e2f8abe1d2043)).
- Vue reads a tag whose name runs straight into `/>`, such as `<div/>`, as Shiki does.
  The tag closes itself, and the name's last letter reads as an attribute ([`73e8231`](https://github.com/victor-software-house/nishiki/commit/73e82311ba637bdd7a90aef245d47a1bcd3e2ef7)).
- Add Vue.
  The HTML lexer runs single-file components with the scopes of Shiki's `vue` grammar.
  Each block's body runs on the lexer of the language its `lang` attribute names.
  A Markdown `vue` fence holds Vue, as Shiki's `markdown-vue` grammar adds it ([`d960c8e`](https://github.com/victor-software-house/nishiki/commit/d960c8e4b614538365c26390f81f568dbb8c2153)).
- Highlight XML with the scopes of Shiki's `xml` grammar, with the Java of `<% %>` lexed by the Java lexer.
  Markdown fences named `xml`, `xsd`, `tld`, `jsp`, `pt`, `cpt`, `dtml`, `rss`, `opml`, `xsl` or `xslt` are now highlighted as XML; they were plain text before ([`a41d9d3`](https://github.com/victor-software-house/nishiki/commit/a41d9d3440175145844186cecb089274dc12619a)).
- XML runs about 1% fewer instructions in a default release build, at sixteen codegen units ([`3f41446`](https://github.com/victor-software-house/nishiki/commit/3f41446bd5bea29c1cb500d82383595fb0bff916)).
- YAML directives read only spaces and tabs between their words, as Shiki does.
  A word after a no-break space is marked as a mistake, and so is a reserved directive's third word.
  The spaces before a directive's comment or line end do not take the directive's scope ([`957ca3c`](https://github.com/victor-software-house/nishiki/commit/957ca3cb0b7d72a9ff5bdf3f320945cb8cec8170)).
- Read a YAML directive's name without copying it.
  Lexing a document with directives again allocates nothing ([`f7848d5`](https://github.com/victor-software-house/nishiki/commit/f7848d5b086c7c5781f4d3340176c289368e41f3)).
- Tag a YAML key such as `a:` in `a:: b: c` when a later key follows on the line, as Shiki does.
  Its colons do not pair up with the `:` after it ([`2070c03`](https://github.com/victor-software-house/nishiki/commit/2070c0322fe82bfbf360cb4796e1186c6d12ccc6)).
- Highlight YAML with Shiki's scopes, in documents, in Markdown `yaml` and `yml` fences and in Markdown front matter ([`ea4508e`](https://github.com/victor-software-house/nishiki/commit/ea4508e1dbc4349a2a2516683b32eac437af880c)).
- Ship the licences of the YAML and TOML grammars whose scopes those lexers emit in the third-party notices ([`3631059`](https://github.com/victor-software-house/nishiki/commit/3631059f8dfb927bc889a606c6896120762f02fe)).
- Read the Unicode spaces as blanks in YAML where Shiki's grammar reads `\s`.
  That is after the colon of a key, after a dash or a question mark, between a directive's words, and around properties.
  Before a `#` and after a tag, only a space or a tab counts ([`db77711`](https://github.com/victor-software-house/nishiki/commit/db777112e3a4ba3d6a676ee77d88170655f7132d)).
- Highlight Zig with nishiki's TextMate engine on Shiki's `zig` grammar, behind the `lang-zig` feature in the opt-in `textmate` tier ([`f6bb8e5`](https://github.com/victor-software-house/nishiki/commit/f6bb8e55584c96e4c887ed4f0a936c5a127e07ac)).
- Highlight Zig with a lexer of its own, enabled by default and in `all`.
  The `lang-zig` feature leaves the opt-in `textmate` tier ([`5e0a963`](https://github.com/victor-software-house/nishiki/commit/5e0a96359631f6938ee3271851ab77b1c94f37fb)).

## nishiki 0.1.0

- One `Highlighter` for every language, chosen by `Language`, with configurable per-line `Limits` and a `LineReport` per line; `State` is now an interned handle, so nesting has no fixed ceiling, and `Highlighter::scopes` lists a stack innermost first ([#26](https://github.com/victor-software-house/nishiki-archive/pull/26)).
- The Rust lexer no longer stops advancing on an uppercase-led word followed by non-ASCII text, such as `R日`, which made it spin until the work limit ([#28](https://github.com/victor-software-house/nishiki-archive/pull/28)).

## nishiki 0.0.1

- Highlight Rust: `nishiki::rust::Highlighter` lexes one line at a time and gives each token the scope stack that Shiki's Rust grammar gives it, so a TextMate theme colours it as Shiki does ([#3](https://github.com/victor-software-house/nishiki-archive/pull/3)).
- The crate ships `THIRD-PARTY-NOTICES.md`, with the MIT notice of rust-syntax, whose scope names the Rust lexer emits ([#5](https://github.com/victor-software-house/nishiki-archive/pull/5)).

