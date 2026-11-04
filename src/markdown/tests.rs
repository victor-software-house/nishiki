use indoc::{formatdoc, indoc};

use crate::{Highlighter, Language, Limit, Limits, State};

/// Lexes `text` and returns, per line, each token's text with its scope stack.
fn lex(text: &str) -> Vec<Vec<(String, Vec<String>)>> {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut lines = Vec::new();
    for line in text.split('\n') {
        let mut tokens = Vec::new();
        let report = highlighter.line(line, &mut state, &mut tokens);
        assert!(report.is_complete(), "{line:?} reached a limit");
        let mut start = 0;
        let mut row = Vec::new();
        for token in &tokens {
            let mut scopes: Vec<String> = highlighter
                .scopes(token.stack())
                .map(ToString::to_string)
                .collect();
            scopes.reverse();
            row.push((line[start..token.end()].to_owned(), scopes));
            start = token.end();
        }
        assert_eq!(start, line.len(), "tokens must cover the line: {line:?}");
        lines.push(row);
    }
    lines
}

/// The innermost scope of the first token of `text` that reads `token`.
fn scope_of(text: &str, token: &str) -> String {
    let lines = lex(text);
    let found = lines.iter().flatten().find(|(t, _)| t == token);
    let (_, scopes) = found.unwrap_or_else(|| panic!("no token {token:?} in {lines:?}"));
    scopes.last().cloned().unwrap_or_default()
}

/// The full scope stack of the first token of `text` that reads `token`.
fn stack_of(text: &str, token: &str) -> Vec<String> {
    let lines = lex(text);
    let found = lines.iter().flatten().find(|(t, _)| t == token);
    let (_, scopes) = found.unwrap_or_else(|| panic!("no token {token:?} in {lines:?}"));
    scopes.clone()
}

#[test]
fn empty_line_passes_over_the_state() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("> quote", &mut state, &mut tokens);
    let before = state;
    tokens.clear();
    let _ = highlighter.line("", &mut state, &mut tokens);
    assert_eq!(tokens, []);
    assert_eq!(state, before);
}

#[test]
fn a_line_of_spaces_is_lexed() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("   ", &mut state, &mut tokens);
    assert_eq!(tokens.len(), 1);
}

#[test]
fn emphasis_and_code() {
    let text = "a *b* **c** `d` ~~e~~";
    assert_eq!(scope_of(text, "b"), "markup.italic.markdown");
    assert_eq!(scope_of(text, "c"), "markup.bold.markdown");
    assert_eq!(scope_of(text, "d"), "markup.inline.raw.string.markdown");
    assert_eq!(scope_of(text, "e"), "markup.strikethrough.markdown");
}

#[test]
fn a_delimiter_with_no_closer_is_text() {
    assert_eq!(scope_of("a *b", "a *b"), "meta.paragraph.markdown");
    assert_eq!(scope_of("**b **", "**b **"), "meta.paragraph.markdown");
}

#[test]
fn heading_with_closing_hashes() {
    let text = "## title ##";
    assert_eq!(scope_of(text, "title"), "entity.name.section.markdown");
    let tokens = lex(text);
    assert_eq!(tokens[0].last().map(|(t, _)| t.as_str()), Some("##"));
    assert_eq!(
        tokens[0]
            .last()
            .and_then(|(_, s)| s.last())
            .map(String::as_str),
        Some("punctuation.definition.heading.markdown")
    );
}

#[test]
fn a_heading_needs_a_blank_after_the_hashes() {
    assert_eq!(scope_of("#x", "#x"), "meta.paragraph.markdown");
    assert_eq!(
        scope_of("####### x", "####### x"),
        "meta.paragraph.markdown"
    );
}

#[test]
fn links_and_images() {
    let text = r#"[t](http://u "ti") ![a](b) <https://x.y> [r][s]"#;
    assert_eq!(scope_of(text, "t"), "string.other.link.title.markdown");
    assert_eq!(scope_of(text, "http://u"), "markup.underline.link.markdown");
    assert_eq!(
        scope_of(text, "ti"),
        "string.other.link.description.title.markdown"
    );
    assert_eq!(scope_of(text, "b"), "markup.underline.link.image.markdown");
    assert_eq!(
        scope_of(text, "https://x.y"),
        "markup.underline.link.markdown"
    );
    assert_eq!(
        scope_of(text, "s"),
        "constant.other.reference.link.markdown"
    );
}

#[test]
fn rust_fence_embeds_the_rust_lexer_without_its_root() {
    let text = indoc! {"
        ```rust
        let x = 1;
        ```
    "};
    assert_eq!(
        stack_of(text, "let"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.rust",
            "keyword.other.rust",
            "storage.type.rust",
        ]
    );
    assert_eq!(
        stack_of(text, "rust"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "fenced_code.block.language.markdown",
        ]
    );
}

#[test]
fn rust_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```rust
        /* a
        b */
        ```
    "};
    assert_eq!(scope_of(text, "*/"), "comment.block.rust",);
}

#[test]
fn typescript_fence_embeds_the_typescript_lexer_without_its_root() {
    for info in ["ts", "typescript", "TS"] {
        let text = formatdoc! {"
            ```{info}
            const x = 1;
            ```
        "};
        assert_eq!(
            stack_of(&text, "const"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.typescript",
                "meta.var.expr.ts",
                "storage.type.ts",
            ],
            "{info}"
        );
    }
}

#[test]
fn javascript_fences_embed_the_javascript_grammar() {
    // The Markdown grammar sends `jsx` to the JavaScript grammar too.
    for info in ["js", "javascript", "jsx", "mjs", "cjs", "es6", "JS"] {
        let text = formatdoc! {"
            ```{info}
            const x = 1;
            ```
        "};
        assert_eq!(
            stack_of(&text, "const"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.javascript",
                "meta.var.expr.js",
                "storage.type.js",
            ],
            "{info}"
        );
    }
}

#[test]
fn a_tsx_fence_embeds_the_tsx_grammar() {
    let text = indoc! {"
        ```tsx
        const x = <a b=\"1\" />;
        ```
    "};
    let stack = stack_of(text, "a");
    assert_eq!(
        stack[..3],
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.typescriptreact",
        ]
    );
    assert_eq!(
        stack.last().map(String::as_str),
        Some("entity.name.tag.tsx")
    );
}

#[test]
fn typescript_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```ts
        /* a
        b */
        ```
    "};
    assert_eq!(stack_of(text, "b ")[3], "comment.block.ts");
}

#[test]
fn a_shell_fence_embeds_the_shell_lexer_without_its_root() {
    for info in [
        "sh",
        "bash",
        "shell",
        "zsh",
        "BASH",
        "bash_profile",
        "{.bash .x}",
    ] {
        let text = formatdoc! {"
            ```{info}
            ls -l
            ```
        "};
        assert_eq!(
            stack_of(&text, "ls"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.shellscript",
                "meta.statement.shell",
                "meta.statement.command.shell",
                "meta.statement.command.name.shell",
                "entity.name.function.call.shell",
                "entity.name.command.shell",
            ],
            "{info}"
        );
    }
}

#[test]
fn shell_text_after_a_containers_indent_is_not_at_the_start_of_the_line() {
    let text = indoc! {"
        - a

          ```sh
          brew update
          ```
    "};
    let plain = lex(text)
        .iter()
        .flatten()
        .filter(|(text, _)| text.contains("brew"))
        .all(|(_, scopes)| {
            scopes
                .last()
                .is_some_and(|s| s == "meta.embedded.block.shellscript")
        });
    assert!(plain, "Shiki leaves the command unscoped here");
}

#[test]
fn a_shell_function_body_survives_a_line_of_only_container_indent() {
    // The line holding only the list item's indent is read whole by the
    // containers; Shiki's shell rules still see its line feed there, and
    // the function's `{ ... }` stays open across it.
    let text = indoc! {"
        - a

          ```sh
          f() {
        \x20\x20
          echo hi
          }
          ```
    "};
    // Shiki leaves the body's text unsplit, as one token under the body.
    let stack = stack_of(text, "echo hi");
    assert_eq!(
        stack.last().map(String::as_str),
        Some("meta.function.body.shell")
    );
}

#[test]
fn shell_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```sh
        echo 'a
        b'
        ```
    "};
    assert_eq!(stack_of(text, "b")[3], "meta.statement.shell");
}

#[test]
fn the_line_after_a_continuation_in_a_fence_is_anchored_at_its_start() {
    // The fence's `while` rule matches the line's start, so `\G` is there and
    // the shell grammar reads the whole line as a command name. A shell
    // document does not do this.
    let text = indoc! {"
        ```sh
        a \\
          b
        ```
    "};
    let lines = lex(text);
    assert_eq!(lines[2].len(), 1, "one token: {:?}", lines[2]);
    assert_eq!(lines[2][0].0, "  b");
    assert_eq!(
        lines[2][0].1.last().map(String::as_str),
        Some("meta.statement.command.name.shell")
    );
}

#[test]
fn json_fences_embed_the_json_lexer_in_their_own_spelling() {
    for (info, embedded, suffix) in [
        ("json", "meta.embedded.block.json", ".json"),
        ("JSON5", "meta.embedded.block.json", ".json"),
        ("sublime-settings", "meta.embedded.block.json", ".json"),
        ("jsonc", "meta.embedded.block.jsonc", ".json.comments"),
        ("jsonl", "meta.embedded.block.jsonl", ".json.lines"),
        ("jsonlines", "meta.embedded.block.jsonl", ".json.lines"),
    ] {
        let text = formatdoc! {"
            ```{info}
            [null]
            ```
        "};
        assert_eq!(
            stack_of(&text, "null"),
            [
                "text.html.markdown".to_owned(),
                "markup.fenced_code.block.markdown".to_owned(),
                embedded.to_owned(),
                format!("meta.structure.array{suffix}"),
                format!("constant.language{suffix}"),
            ],
            "{info}"
        );
    }
}

#[test]
fn python_fences_embed_the_python_lexer_without_its_root() {
    for info in ["python", "py", "PY3", "SConstruct", "gypi", "{.python .x}"] {
        let text = formatdoc! {"
            ```{info}
            def f(): pass
            ```
        "};
        assert_eq!(
            stack_of(&text, "def"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.python",
                "meta.function.python",
                "storage.type.function.python",
            ],
            "{info}"
        );
    }
}

#[test]
fn a_json_value_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```json
        {\"a\": [1,
        2]}
        ```
    "};
    assert_eq!(stack_of(text, "2")[3], "meta.structure.dictionary.json");
}

#[test]
fn a_python_decorator_after_a_containers_indent_is_not_at_the_start_of_the_line() {
    let text = indoc! {"
        - a

          ```python
          @property
          ```
    "};
    let decorated = lex(text)
        .iter()
        .flatten()
        .any(|(_, scopes)| scopes.iter().any(|s| s == "meta.function.decorator.python"));
    assert!(
        !decorated,
        "Shiki matches `^` only at the start of the line"
    );
}

#[test]
fn python_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```py
        f(1,
        2)
        ```
    "};
    assert_eq!(stack_of(text, "2")[3], "meta.function-call.python");
}

#[test]
fn java_fences_embed_the_java_lexer_without_its_root() {
    for info in ["java", "JAVA", "java title=x"] {
        let text = formatdoc! {"
            ```{info}
            int x;
            ```
        "};
        assert_eq!(
            stack_of(&text, "int"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.java",
                "meta.definition.variable.java",
                "storage.type.primitive.java",
            ],
            "{info}"
        );
    }
}

#[test]
fn csharp_fences_embed_the_csharp_lexer_without_its_root() {
    for info in ["cs", "csharp", "c#", "C#", "csharp title=x"] {
        let text = formatdoc! {"
            ```{info}
            int x;
            ```
        "};
        assert_eq!(
            stack_of(&text, "int"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.csharp",
                "keyword.type.int.cs",
            ],
            "{info}"
        );
    }
    for info in ["csx", "c-sharp"] {
        let text = formatdoc! {"
            ```{info}
            int x;
            ```
        "};
        assert_eq!(
            stack_of(&text, "int x;"),
            ["text.html.markdown", "markup.fenced_code.block.markdown"],
            "{info}"
        );
    }
}

#[test]
fn css_fences_embed_the_css_lexer_without_its_root() {
    for info in ["css", "CSS", "css.erb", "css title=x"] {
        let text = formatdoc! {"
            ```{info}
            a {{ color: red }}
            ```
        "};
        assert_eq!(
            stack_of(&text, "color"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.css",
                "meta.property-list.css",
                "meta.property-name.css",
                "support.type.property-name.css",
            ],
            "{info}"
        );
    }
}

#[test]
fn xml_fences_embed_the_xml_lexer_without_its_root() {
    for info in ["xml", "XSD", "jsp", "rss"] {
        let text = formatdoc! {r#"
            ```{info}
            <a b="c"/>
            ```
        "#};
        assert_eq!(
            stack_of(&text, "a"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.xml",
                "meta.tag.xml",
                "entity.name.tag.localname.xml",
            ],
            "{info}"
        );
    }
}

/// The `xsl` rule embeds the XSL grammar, which nishiki does not lex, so
/// its fences are plain, as in Shiki without that grammar.
#[test]
fn xsl_fences_are_plain() {
    for info in ["xsl", "XSLT title=x"] {
        let text = formatdoc! {r#"
            ```{info}
            <a b="c"/>
            ```
        "#};
        assert_eq!(
            stack_of(&text, r#"<a b="c"/>"#),
            ["text.html.markdown", "markup.fenced_code.block.markdown"],
            "{info}"
        );
    }
}

/// The `php` rule includes `text.html.basic` and then `source.php`, so
/// without a PHP grammar HTML takes the body, under the rule's own scope.
#[test]
fn php_fences_embed_html_under_the_php_scope() {
    for info in ["php", "php3", "php4", "aw", "ctp", "PHTML title=x"] {
        let text = formatdoc! {"
            ```{info}
            <b>x</b>
            ```
        "};
        assert_eq!(
            stack_of(&text, "b"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.php",
                "meta.tag.inline.b.start.html",
                "entity.name.tag.html",
            ],
            "{info}"
        );
    }
}

/// Batch runs on the engine, whose `lex` takes the fence's body line by
/// line while Markdown finds the closing fence.
#[cfg(feature = "lang-bat")]
#[test]
fn batch_fences_embed_the_batch_lexer_without_its_root() {
    for info in ["batch", "Bat x"] {
        let text = formatdoc! {"
            ```{info}
            rem hi
            ```
        "};
        assert_eq!(
            stack_of(&text, "rem"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.dosbatch",
                "comment.line.rem.batchfile",
                "keyword.command.rem.batchfile",
            ],
            "{info}"
        );
    }
}

#[test]
fn css_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```css
        /* a
        b */ a {}
        ```
    "};
    assert_eq!(stack_of(text, "b ")[3], "comment.block.css");
}

#[test]
fn a_css_fence_line_is_anchored_at_its_start() {
    // The fence's `while` rule matches the line's start, so `\G` is there and
    // the header of a generic at-rule can begin on a line of its own.
    let text = indoc! {"
        ```css
        @x a;
        @b
        ```
    "};
    assert_eq!(stack_of(text, "b")[3], "meta.at-rule.header.css");
}

#[test]
fn go_fences_embed_the_go_lexer_without_its_root() {
    for info in ["go", "golang", "GO", "go title=x"] {
        let text = formatdoc! {"
            ```{info}
            return x
            ```
        "};
        assert_eq!(
            stack_of(&text, "return"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.go",
                "keyword.control.go",
            ],
            "{info}"
        );
    }
}

#[test]
fn lua_fences_embed_the_lua_lexer_without_its_root() {
    for info in ["lua", "LUA", "lua title=x"] {
        let text = formatdoc! {"
            ```{info}
            return x
            ```
        "};
        assert_eq!(
            stack_of(&text, "return"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.lua",
                "keyword.control.lua",
            ],
            "{info}"
        );
    }
}

#[cfg(feature = "lang-scss")]
#[test]
fn scss_fences_embed_the_scss_lexer_without_its_root() {
    for info in ["scss", "SCSS", "scss title=x"] {
        let text = formatdoc! {"
            ```{info}
            $a: 1;
            ```
        "};
        assert_eq!(
            stack_of(&text, "$a"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.scss",
                "meta.definition.variable.scss",
                "variable.scss",
            ],
            "{info}"
        );
    }
}

#[cfg(feature = "lang-less")]
#[test]
fn less_fences_embed_the_less_lexer_without_its_root() {
    for info in ["less", "LESS", "less title=x"] {
        let text = formatdoc! {"
            ```{info}
            @a: 1;
            ```
        "};
        assert_eq!(
            stack_of(&text, "a"),
            [
                "text.html.markdown",
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.less",
                "meta.property-value.less",
                "variable.other.readwrite.less",
                "support.other.variable.less",
            ],
            "{info}"
        );
    }
}

#[test]
fn go_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```go
        /* a
        b */ x := 1
        ```
    "};
    assert_eq!(stack_of(text, "b ")[3], "comment.block.go");
}

/// `\G` holds where the list's indent ends, so `\G()$` keeps the continuation
/// open there on a line of nothing but the indent; it closes at the end of that
/// line, and the quote after it opens a string, as in Shiki.
#[test]
fn a_python_continuation_in_a_list_fence_closes_after_a_line_of_indent() {
    let text = formatdoc! {"
        - a

          ```python
          x = \\
          {nothing}
          'a'
          ```
    ", nothing = ""};
    let lines = lex(&text);
    assert_eq!(lines[4].len(), 1, "{:?}", lines[4]);
    let (quote, scopes) = &lines[5][1];
    assert_eq!(quote, "'");
    assert_eq!(scopes[4], "string.quoted.single.python");
}

/// The comment's outer rule ends at `(?!\G)` at the end of the comment's line,
/// before the anchor of the next line could keep it open, as in Shiki.
#[test]
fn a_java_fence_line_after_a_line_comment_starts_a_definition() {
    let text = indoc! {"
        ```java
        // a
        int x;
        ```
    "};
    assert_eq!(scope_of(text, "int"), "storage.type.primitive.java");
}

#[test]
fn swift_fences_embed_the_swift_lexer_without_its_root() {
    let text = indoc! {"
        ```swift
        let x = 1
        ```
    "};
    assert_eq!(
        stack_of(text, "let"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.swift",
            "keyword.other.declaration-specifier.swift",
        ]
    );
}

/// The fence's `while` rule matches the line's start, so `\G` is there: the
/// line after an opening `"""` is content after the delimiter, and a `}` that
/// starts a line leaves an enum's case list open until a name.
#[test]
fn a_swift_fence_line_is_anchored_at_its_start() {
    let rows = lex(indoc! {r#"
        ```swift
        print("""
            a
            """)
        enum A {
            case b, c
        }
        struct Example {
        ```
    "#});
    let leaf = |row: usize, text: &str| {
        rows[row]
            .iter()
            .find(|(t, _)| t.trim() == text)
            .and_then(|(_, scopes)| scopes.last().cloned())
            .unwrap_or_default()
    };
    assert_eq!(
        leaf(2, "a"),
        "invalid.illegal.content-after-opening-delimiter.swift"
    );
    assert_eq!(leaf(6, "}"), "meta.enum-case.more-cases");
    assert_eq!(leaf(7, "struct"), "variable.other.enummember.swift");
    let example = rows[7]
        .iter()
        .find(|(t, _)| t == "Example")
        .map(|(_, scopes)| scopes.as_slice())
        .unwrap_or_default();
    assert!(
        example
            .iter()
            .any(|s| s == "meta.definition.type.enum.swift")
    );
    assert!(!example.iter().any(|s| s == "meta.enum-case.more-cases"));
}

#[test]
fn java_state_continues_across_the_lines_of_a_fence() {
    let text = indoc! {"
        ```java
        /* a
        b */ int x;
        ```
    "};
    assert_eq!(stack_of(text, "b ")[3], "comment.block.java");
}

#[test]
fn a_javadoc_after_a_containers_indent_is_not_at_the_start_of_the_line() {
    let text = indoc! {"
        - a

          ```java
          /** @param x */
          ```
    "};
    let javadoc = lex(text)
        .iter()
        .flatten()
        .any(|(_, scopes)| scopes.iter().any(|s| s == "comment.block.javadoc.java"));
    assert!(!javadoc, "Shiki matches `^` only at the start of the line");
}

#[test]
fn a_shebang_is_not_special_inside_a_fence() {
    let text = indoc! {"
        ```ts
        #!/usr/bin/env node
        ```
    "};
    let shebang = lex(text)
        .iter()
        .flatten()
        .any(|(_, scopes)| scopes.iter().any(|s| s == "comment.line.shebang.ts"));
    assert!(!shebang, "only the first line of the document can be one");
}

#[test]
fn another_language_is_plain_text_with_its_info_string() {
    let text = indoc! {"
        ```haskell
        # not a heading
        ```
    "};
    assert_eq!(scope_of(text, "haskell"), "fenced_code.block.language");
    assert_eq!(
        stack_of(text, "# not a heading"),
        ["text.html.markdown", "markup.fenced_code.block.markdown"]
    );
}

#[test]
fn a_longer_fence_is_not_closed_by_a_shorter_one() {
    let text = indoc! {"
        ````
        ```
        still inside
        ````
        outside
    "};
    assert_eq!(
        stack_of(text, "still inside"),
        ["text.html.markdown", "markup.fenced_code.block.markdown"]
    );
    assert_eq!(scope_of(text, "outside"), "meta.paragraph.markdown");
}

#[test]
fn a_markdown_fence_embeds_markdown() {
    let text = indoc! {"
        ````md
        # inner
        ```rust
        fn f() {}
        ```
        ````
    "};
    assert_eq!(
        stack_of(text, "inner"),
        [
            "text.html.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.markdown",
            "markup.heading.markdown",
            "heading.1.markdown",
            "entity.name.section.markdown",
        ]
    );
}

#[test]
fn a_list_continues_over_blank_lines_and_indented_lines() {
    let text = indoc! {"
        - a

          b
        c
    "};
    assert_eq!(
        stack_of(text, "b"),
        [
            "text.html.markdown",
            "markup.list.unnumbered.markdown",
            "meta.paragraph.markdown",
        ]
    );
    assert_eq!(
        stack_of(text, "c"),
        ["text.html.markdown", "meta.paragraph.markdown"]
    );
}

#[test]
fn a_quote_ends_at_a_line_without_the_marker() {
    let text = indoc! {"
        > a
        b
    "};
    assert_eq!(
        stack_of(text, "a"),
        [
            "text.html.markdown",
            "markup.quote.markdown",
            "meta.paragraph.markdown"
        ]
    );
    assert_eq!(
        stack_of(text, "b"),
        ["text.html.markdown", "meta.paragraph.markdown"]
    );
}

#[test]
fn setext_underline_follows_its_paragraph() {
    let text = indoc! {"
        Title
        =====
    "};
    assert_eq!(scope_of(text, "====="), "markup.heading.setext.1.markdown");
}

#[test]
fn table_cells_hold_inline_markup() {
    let text = "| a | *b* |";
    assert_eq!(scope_of(text, "b"), "markup.italic.markdown");
    assert_eq!(scope_of(text, "|"), "punctuation.definition.table.markdown");
}

#[test]
fn html_comment_spans_lines() {
    let text = indoc! {"
        <!-- a
        *b*
        -->
        *c*
    "};
    assert_eq!(scope_of(text, "*b*"), "comment.block.html");
    assert_eq!(scope_of(text, "c"), "markup.italic.markdown");
}

#[test]
fn unicode_whitespace_is_whitespace() {
    // `**x<NBSP>**` cannot close: `\S` must precede the closing `**`.
    assert_eq!(
        scope_of("**x\u{a0}**", "**x\u{a0}**"),
        "meta.paragraph.markdown"
    );
}

#[test]
fn nesting_stops_at_the_depth_limit() {
    let limits = Limits {
        depth: Some(8),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let text = format!("{}deep", "> ".repeat(100));
    let report = highlighter.line(&text, &mut state, &mut tokens);
    assert!(report.reached(Limit::Depth));
    assert_eq!(tokens.last().map(|token| token.end()), Some(text.len()));
    let quotes = highlighter
        .scopes(tokens[tokens.len() - 1].stack())
        .filter(|name| *name == "markup.quote.markdown")
        .count();
    assert_eq!(quotes, 8, "the document's own context is not counted");
}

#[test]
fn hostile_lines_are_covered() {
    for unit in [
        "**", "[", "[](", "`", "``` a ` ", "~~ a ", "<a ", "_ a ", "![a](", "*a ",
    ] {
        let line = unit.repeat(20_000);
        let mut highlighter = Highlighter::new(Language::Markdown);
        let mut state = State::default();
        let mut tokens = Vec::new();
        let report = highlighter.line(&line, &mut state, &mut tokens);
        assert!(
            report.limits().all(|limit| limit == Limit::Work),
            "{unit:?} reached {:?}",
            report.limits().collect::<Vec<_>>()
        );
        assert_eq!(tokens.last().map(|t| t.end()), Some(line.len()), "{unit:?}");
    }
}

#[test]
fn the_state_resumes_from_any_line() {
    let text = indoc! {"
        - item
          ```rust
          fn a() {
              /* c
          "};
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut states = Vec::new();
    let mut tokens = Vec::new();
    let lines: Vec<&str> = text.split('\n').collect();
    for line in &lines {
        let _ = highlighter.line(line, &mut state, &mut tokens);
        states.push(state);
    }
    // Resume from the state after line 1: the same highlighter, which owns
    // the state, reaches the same state again.
    let mut resumed = states[1];
    let mut second = Vec::new();
    for line in &lines[2..] {
        let _ = highlighter.line(line, &mut resumed, &mut second);
    }
    assert_eq!(resumed, state);
}

#[test]
fn a_yaml_fence_embeds_the_yaml_lexer_without_its_root() {
    for info in ["yaml", "yml", "YAML"] {
        let text = formatdoc! {"
            ```{info}
            key: value
            ```"};
        assert_eq!(
            stack_of(&text, "key")[1..],
            [
                "markup.fenced_code.block.markdown",
                "meta.embedded.block.yaml",
                "string.unquoted.plain.out.yaml",
                "entity.name.tag.yaml"
            ],
            "{info}"
        );
    }
}

#[test]
fn front_matter_is_yaml_between_dashes_on_the_first_line() {
    let text = indoc! {"
        ---
        title: x
        ---
        # H"};
    let lines = lex(text);
    assert_eq!(
        lines[0][0].1[1..],
        ["punctuation.definition.begin.frontmatter"]
    );
    assert_eq!(
        stack_of(text, "title")[1..],
        [
            "meta.embedded.block.frontmatter",
            "string.unquoted.plain.out.yaml",
            "entity.name.tag.yaml"
        ]
    );
    assert_eq!(
        lines[2][0].1[1..],
        ["punctuation.definition.end.frontmatter"]
    );
    assert_eq!(scope_of(text, "H"), "entity.name.section.markdown");
}

#[test]
fn front_matter_ends_at_three_dots_or_more_dashes() {
    for closer in ["...", "----", "  ---", "--- "] {
        let lines = lex(&formatdoc! {"
            ---
            a: 1
            {closer}
            plain"});
        assert_eq!(
            lines[2][0].1.last().map(String::as_str),
            Some("punctuation.definition.end.frontmatter"),
            "{closer:?}"
        );
        assert_eq!(
            lines[3][0].1.last().map(String::as_str),
            Some("meta.paragraph.markdown"),
            "{closer:?}"
        );
    }
}

#[test]
fn front_matter_opens_on_the_first_line_only() {
    let lines = lex(indoc! {"
        # Title
        ---
        text"});
    assert_eq!(
        lines[1][0].1.last().map(String::as_str),
        Some("meta.separator.markdown")
    );
    let lines = lex(" ---\ntext");
    assert!(
        lines[0]
            .iter()
            .all(|(_, scopes)| scopes.iter().all(|s| !s.contains("frontmatter")))
    );
}

#[test]
fn an_empty_first_line_leaves_the_first_line_for_the_next() {
    let lines = lex(indoc! {"

        ---
        a: 1
        ---"});
    assert_eq!(
        lines[1][0].1.last().map(String::as_str),
        Some("punctuation.definition.begin.frontmatter")
    );
}

#[test]
fn the_start_state_is_not_the_state_after_a_first_line_that_opens_nothing() {
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut tokens = Vec::new();
    let _ = highlighter.line("# Title", &mut state, &mut tokens);
    assert_ne!(state, State::default());
}

#[test]
fn a_yaml_fence_in_a_list_has_no_line_starts_for_yaml() {
    // The fence's text begins after the list's indent, where `^` does not
    // match, so a block scalar has no lines and the first character of the
    // line after its header has no scope, and a comment needs a blank before
    // it.
    let text = indoc! {"
        - ```yaml
          run: |
            echo
          # c
          ```
    "};
    assert_eq!(
        stack_of(text, "cho")[1..],
        [
            "markup.list.unnumbered.markdown",
            "markup.fenced_code.block.markdown",
            "meta.embedded.block.yaml",
            "string.unquoted.plain.out.yaml"
        ]
    );
    assert_eq!(scope_of(text, "#"), "meta.embedded.block.yaml");
    assert_eq!(scope_of(text, "e"), "meta.embedded.block.yaml");
}

/// Whether the second line of `text` is embedded code.
fn second_line_is_embedded(text: &str) -> bool {
    lex(text)[1].iter().any(|(_, scopes)| {
        scopes
            .iter()
            .any(|scope| scope.starts_with("meta.embedded.block."))
    })
}

/// Shiki matches the names without regard to case under Oniguruma's folding:
/// the Kelvin sign is `k`, the long s is `s`, `ß` is `ss`, and a ligature is
/// the letters it stands for. A fold stands for all its letters, so `rußt`,
/// which reads as `russt`, is no name.
#[test]
fn a_fence_name_matches_with_oracles_case_folding() {
    for name in [
        "ma\u{212a}efile",
        "GNUmake\u{fb01}le",
        "\u{17f}h",
        "ru\u{fb05}",
        "di\u{fb00}",
        "{.ru\u{17f}t .x}",
        "le\u{df}",
        "\u{17f}c\u{df}",
    ] {
        let text = formatdoc! {"
            ```{name}
            let x = 1; + 1
            ```"};
        assert!(second_line_is_embedded(&text), "{name:?}");
    }
    assert!(!second_line_is_embedded(indoc! {"
        ```ru\u{df}t
        let x = 1;
        ```"}));
}

/// A dot in a fence name matches any one character but a line feed, as the
/// grammar's unescaped dot does in Shiki.
/// A word that starts with a dot matches after any first character.
/// Two dots in a row stand for two characters, so `css..erb` is no name.
#[test]
fn a_dot_in_a_fence_name_matches_any_one_character() {
    for (name, embedded) in [
        ("css.erb", "meta.embedded.block.css"),
        ("cssXerb", "meta.embedded.block.css"),
        ("css\u{e9}erb", "meta.embedded.block.css"),
        ("css\u{4e00}erb", "meta.embedded.block.css"),
        (".textmate_init", "meta.embedded.block.shellscript"),
        ("xtextmate_init", "meta.embedded.block.shellscript"),
        ("\u{e9}textmate_init", "meta.embedded.block.shellscript"),
        ("{textmate_init", "meta.embedded.block.shellscript"),
        ("Berksfile.lock", "meta.embedded.block.ruby"),
        ("BerksfileXlock", "meta.embedded.block.ruby"),
        ("Berksfile\u{e9}lock", "meta.embedded.block.ruby"),
        ("berksfile_LOCK", "meta.embedded.block.ruby"),
    ] {
        let text = formatdoc! {"
            ```{name}
            a
            ```"};
        let found = lex(&text)[1]
            .iter()
            .any(|(_, scopes)| scopes.iter().any(|scope| scope == embedded));
        assert!(found, "{name:?} embeds {embedded}");
    }
    for name in ["css..erb", "cssx", "textmate_init"] {
        let text = formatdoc! {"
            ```{name}
            a
            ```"};
        assert!(!second_line_is_embedded(&text), "{name:?}");
    }
}

#[test]
fn a_caret_does_not_match_where_a_list_took_the_start_of_the_line() {
    // The list consumed the indentation, so `^` matches nowhere in the fence:
    // a type annotation, which ends at `^` or `$`, goes on to the next line,
    // and a triple-slash directive, which starts with `^`, is a plain comment.
    let text = indoc! {"
        - a:
          ```ts
          const c: A
          export default c
          ```"};
    assert!(
        stack_of(text, "export")
            .iter()
            .any(|scope| scope == "meta.type.annotation.ts")
    );
    let text = indoc! {"
        - a:
          ```ts
          /// <reference types=\"x\" />
          ```"};
    assert!(
        stack_of(text, "//")
            .iter()
            .any(|scope| scope == "comment.line.double-slash.ts")
    );
}

#[test]
fn a_fence_line_that_runs_out_of_nodes_resumes_the_same_on_a_cache_hit_and_miss() {
    let mut hits = 0;
    for nodes in 1..256 {
        let limits = Limits {
            nodes: Some(nodes),
            ..Limits::default()
        };
        let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
        let mut tokens = Vec::new();
        let mut fence = State::default();
        let _ = highlighter.line("```rust", &mut fence, &mut tokens);
        let mut state = fence;
        let report = highlighter.line("/* open", &mut state, &mut tokens);
        if !report.reached(Limit::Nodes) || state != fence {
            continue;
        }
        hits += 1;
        let mut hit = Vec::new();
        let _ = highlighter.line("x", &mut state, &mut hit);
        let _ = highlighter.line("other", &mut State::default(), &mut tokens);
        let mut miss = Vec::new();
        let mut again = fence;
        let _ = highlighter.line("x", &mut again, &mut miss);
        assert_eq!(hit, miss, "node limit {nodes}");
    }
    assert!(hits > 0, "no node limit stops the fence's push alone");
}

#[test]
fn word_characters_are_the_ones_shiki_measures() {
    let italic = |text: &str| {
        lex(text)
            .iter()
            .flatten()
            .any(|(_, scopes)| scopes.iter().any(|scope| scope == "markup.italic.markdown"))
    };
    assert!(italic("_a_ b"));
    assert!(italic("_a_. b"));
    for mark in ['\u{93c}', '\u{951}', '\u{300}', 'x'] {
        assert!(!italic(&format!("_a_{mark} b")), "{mark:?} is \\w");
    }
}

/// Each line gets the same tokens from the frames kept after the line before
/// it and from frames rebuilt from the arena, after another line moved the
/// highlighter on.
#[test]
fn a_line_lexes_the_same_from_kept_and_rebuilt_frames() {
    let lines = [
        "---",
        "a: 1",
        "b: [",
        "---",
        "> ```rust",
        "> /* c",
        "> */",
        "> ```",
    ];
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut kept = Vec::new();
    let mut states = Vec::new();
    for line in lines {
        states.push(state);
        let mut tokens = Vec::new();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        kept.push(tokens);
    }
    for (index, line) in lines.iter().enumerate().skip(1) {
        let _ = highlighter.line("other", &mut State::default(), &mut Vec::new());
        let mut from = states[index];
        let mut rebuilt = Vec::new();
        let _ = highlighter.line(line, &mut from, &mut rebuilt);
        assert_eq!(rebuilt, kept[index], "{line:?}");
    }
}

/// The lines of `a_cpp_fence_keeps_its_scopes_from_kept_and_rebuilt_frames`
/// and, per line, each token's text with the scopes Shiki 4.6.0 gives it
/// below the fence's `text.html.markdown markup.fenced_code.block.markdown`.
#[cfg(feature = "lang-cpp")]
const CPP_FENCE: [(&str, &[(&str, &str)]); 5] = [
    (
        "```cpp",
        &[
            ("```", "punctuation.definition.markdown"),
            ("cpp", "fenced_code.block.language.markdown"),
        ],
    ),
    (
        "int a = 1; /* one",
        &[
            (
                "int",
                "meta.embedded.block.cpp source.cpp storage.type.primitive.cpp storage.type.built-in.primitive.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "a",
                "meta.embedded.block.cpp source.cpp variable.lower-case.cpp variable.other.unknown.a.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "=",
                "meta.embedded.block.cpp source.cpp keyword.operator.assignment.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "1",
                "meta.embedded.block.cpp source.cpp constant.numeric.decimal.cpp",
            ),
            (
                ";",
                "meta.embedded.block.cpp source.cpp punctuation.terminator.statement.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp comment.block.cpp"),
            (
                "/*",
                "meta.embedded.block.cpp source.cpp comment.block.cpp punctuation.definition.comment.begin.cpp",
            ),
            (
                " one",
                "meta.embedded.block.cpp source.cpp comment.block.cpp",
            ),
        ],
    ),
    (
        "two */ int b;",
        &[
            (
                "two ",
                "meta.embedded.block.cpp source.cpp comment.block.cpp",
            ),
            (
                "*/",
                "meta.embedded.block.cpp source.cpp comment.block.cpp punctuation.definition.comment.end.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "int",
                "meta.embedded.block.cpp source.cpp storage.type.primitive.cpp storage.type.built-in.primitive.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "b",
                "meta.embedded.block.cpp source.cpp variable.lower-case.cpp variable.other.unknown.b.cpp",
            ),
            (
                ";",
                "meta.embedded.block.cpp source.cpp punctuation.terminator.statement.cpp",
            ),
        ],
    ),
    (
        "return a;",
        &[
            (
                "return",
                "meta.embedded.block.cpp source.cpp keyword.control.return.cpp",
            ),
            (" ", "meta.embedded.block.cpp source.cpp"),
            (
                "a",
                "meta.embedded.block.cpp source.cpp variable.lower-case.cpp variable.other.unknown.a.cpp",
            ),
            (
                ";",
                "meta.embedded.block.cpp source.cpp punctuation.terminator.statement.cpp",
            ),
        ],
    ),
    ("```", &[("```", "punctuation.definition.markdown")]),
];

/// A C++ fence's lines, lexed in order and again from each saved state after
/// another line moved the highlighter on, so that the frames are rebuilt from
/// the arena: the scopes Shiki gives, with `meta.embedded.block.cpp
/// source.cpp`, both from the fence's `contentName`, under one frame.
#[cfg(feature = "lang-cpp")]
#[test]
fn a_cpp_fence_keeps_its_scopes_from_kept_and_rebuilt_frames() {
    const FENCE: &str = "text.html.markdown markup.fenced_code.block.markdown";
    let row = |highlighter: &Highlighter, line: &str, tokens: &[crate::Token]| {
        let mut start = 0;
        tokens
            .iter()
            .map(|token| {
                let mut scopes: Vec<String> = highlighter
                    .scopes(token.stack())
                    .map(ToString::to_string)
                    .collect();
                scopes.reverse();
                let text = line[start..token.end()].to_owned();
                start = token.end();
                (text, scopes.join(" "))
            })
            .collect::<Vec<_>>()
    };
    let want = |index: usize| -> Vec<(String, String)> {
        CPP_FENCE[index]
            .1
            .iter()
            .map(|&(text, rest)| (text.to_owned(), format!("{FENCE} {rest}")))
            .collect()
    };
    let mut highlighter = Highlighter::new(Language::Markdown);
    let mut state = State::default();
    let mut states = Vec::new();
    for (index, &(line, _)) in CPP_FENCE.iter().enumerate() {
        states.push(state);
        let mut tokens = Vec::new();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        assert_eq!(
            row(&highlighter, line, &tokens),
            want(index),
            "kept: {line:?}"
        );
    }
    states.push(state);
    for (index, &(line, _)) in CPP_FENCE.iter().enumerate().skip(1) {
        let _ = highlighter.line("other", &mut State::default(), &mut Vec::new());
        let mut from = states[index];
        let mut tokens = Vec::new();
        let _ = highlighter.line(line, &mut from, &mut tokens);
        assert_eq!(
            row(&highlighter, line, &tokens),
            want(index),
            "rebuilt: {line:?}"
        );
        assert_eq!(from, states[index + 1], "rebuilt: {line:?}");
    }
}

/// The scope stacks of the tokens of line `index` of `text`.
#[cfg(feature = "lang-html-derivative")]
fn row(text: &str, index: usize) -> Vec<(String, Vec<String>)> {
    lex(text).swap_remove(index)
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn an_html_block_takes_its_lines_up_to_a_blank_one() {
    let text = [
        "<div>",
        "# not a heading",
        "",
        "> still html",
        " ",
        "# heading",
    ]
    .join("\n");
    let text = text.as_str();
    assert_eq!(
        stack_of(text, "div"),
        [
            "text.html.markdown",
            "meta.tag.structure.div.start.html",
            "entity.name.tag.html"
        ]
    );
    assert_eq!(stack_of(text, "# not a heading"), ["text.html.markdown"]);
    assert_eq!(stack_of(text, "> still html"), ["text.html.markdown"]);
    assert_eq!(scope_of(text, "heading"), "entity.name.section.markdown");
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn a_raw_html_block_names_its_closing_tag_as_spelled() {
    let text = indoc! {"
        <SCRIPT>
        x
        </SCRIPT> tail"};
    let last = row(text, 2);
    assert_eq!(
        last[1].1,
        [
            "text.html.markdown",
            "meta.tag.structure.SCRIPT.end.html",
            "entity.name.tag.html"
        ]
    );
    assert_eq!(
        last[3],
        (" tail".to_owned(), vec!["text.html.markdown".to_owned()])
    );
    assert_eq!(
        stack_of(text, "x"),
        [
            "text.html.markdown",
            "meta.embedded.block.html",
            "source.js",
            "variable.other.readwrite.js"
        ]
    );
}

/// A raw block's tag names read a character that folds to letters as those
/// letters, as `ﬆ` for `st`, at its start and at its end. Shiki's answers.
#[cfg(feature = "lang-html-derivative")]
#[test]
fn a_raw_html_block_reads_case_folds_in_its_tag_names() {
    let close_folded = indoc! {"
        <style>
        a{}
        </\u{fb06}yle>
        x"};
    for text in [
        indoc! {"
            <\u{fb06}yle>
            a{}
            </style>
            x"},
        close_folded,
    ] {
        assert_eq!(
            stack_of(text, "a"),
            [
                "text.html.markdown",
                "meta.embedded.block.html",
                "source.css",
                "meta.selector.css",
                "entity.name.tag.css"
            ],
            "{text:?}"
        );
        assert_eq!(
            row(text, 3),
            [(
                "x".to_owned(),
                vec![
                    "text.html.markdown".to_owned(),
                    "meta.paragraph.markdown".to_owned()
                ]
            )],
            "{text:?}"
        );
    }
    assert_eq!(
        row(close_folded, 2)[1].1,
        [
            "text.html.markdown",
            "meta.tag.structure.\u{fb06}yle.end.html",
            "entity.name.tag.html"
        ]
    );
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn a_raw_html_block_in_a_quote_starts_again_on_each_line() {
    let text = indoc! {"
        > <script>
        > <b>x</b>
        > </script>"};
    assert_eq!(
        stack_of(text, "b"),
        [
            "text.html.markdown",
            "markup.quote.markdown",
            "meta.tag.inline.b.start.html",
            "entity.name.tag.html"
        ]
    );
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn a_paragraph_reads_tags_and_entities() {
    assert_eq!(
        stack_of("See <kbd>Ctrl</kbd> now", "kbd"),
        [
            "text.html.markdown",
            "meta.paragraph.markdown",
            "meta.tag.inline.kbd.start.html",
            "entity.name.tag.html"
        ]
    );
    assert_eq!(
        stack_of("See <kbd>Ctrl</kbd> now", " now"),
        ["text.html.markdown", "meta.paragraph.markdown"]
    );
    assert_eq!(
        scope_of("x &amp; y", "amp"),
        "constant.character.entity.named.amp.html"
    );
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn a_heading_reads_an_html_comment() {
    assert_eq!(
        stack_of("# Title <!-- note -->", " note "),
        [
            "text.html.markdown",
            "markup.heading.markdown",
            "heading.1.markdown",
            "entity.name.section.markdown",
            "comment.block.html"
        ]
    );
}

#[cfg(feature = "lang-html-derivative")]
#[test]
fn emphasis_hands_a_tag_to_html() {
    assert_eq!(
        stack_of("**a <b>x</b> c**", "b"),
        [
            "text.html.markdown",
            "meta.paragraph.markdown",
            "markup.bold.markdown",
            "meta.tag.inline.b.start.html",
            "entity.name.tag.html"
        ]
    );
    assert_eq!(
        stack_of("**a> <b** z", "a> <b"),
        [
            "text.html.markdown",
            "meta.paragraph.markdown",
            "markup.bold.markdown"
        ]
    );
}

/// Whether `lines`, lexed in order under a work budget of `base` steps per
/// line, reach the work limit on the last line.
fn last_line_runs_out(language: Language, lines: &[&str], base: u32) -> bool {
    let limits = Limits {
        work: Some(crate::Work { per_byte: 0, base }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(language, limits);
    let mut state = State::default();
    let mut report = None;
    for line in lines {
        report = Some(highlighter.line(line, &mut state, &mut Vec::new()));
    }
    report.is_some_and(|report| report.reached(Limit::Work))
}

#[test]
fn a_fence_line_shares_one_work_budget_with_its_language() {
    let code = "let a = /* b */ c(d, e) + 1;";
    let mut base = 0;
    while last_line_runs_out(Language::Rust, &[code], base) {
        base += 1;
    }
    assert!(
        last_line_runs_out(Language::Markdown, &["```rust", code], base),
        "the budget Rust needs alone, {base} steps, must not also cover the fence"
    );
    assert!(!last_line_runs_out(
        Language::Markdown,
        &["```rust", code],
        2 * base
    ));
}

/// Whether `line`, lexed alone under a work budget of `base` steps, reaches
/// the work limit, and whether its last byte but two sits in bold.
fn reached_and_bold_before_end(line: &str, base: u32) -> (bool, bool) {
    let limits = Limits {
        work: Some(crate::Work { per_byte: 0, base }),
        ..Limits::default()
    };
    let mut highlighter = Highlighter::new_with_limits(Language::Markdown, limits);
    let mut tokens = Vec::new();
    let report = highlighter.line(line, &mut State::default(), &mut tokens);
    let at = line.len() - 3;
    let bold = tokens
        .iter()
        .find(|token| token.end() > at)
        .is_some_and(|token| {
            highlighter
                .scopes(token.stack())
                .any(|scope| scope == "markup.bold.markdown")
        });
    (report.reached(Limit::Work), bold)
}

/// Markdown lends its count to HTML, and HTML to an event handler's
/// JavaScript. A handler long enough to spend the line's budget, which HTML
/// and its JavaScript count together, stops Markdown too, before the bold
/// text after the tag.
#[test]
fn an_event_handler_that_spends_the_budget_stops_the_markdown_line() {
    let short = r#"x <a onclick="f(1);">y</a> **b**"#;
    assert_eq!(scope_of(short, "f"), "entity.name.function.js");
    let long = format!(r#"x <a onclick="{}">y</a> **b**"#, "f(1);".repeat(100));
    let mut base = 0;
    while last_line_runs_out(Language::Markdown, &[short], base) {
        base += 1;
    }
    assert_eq!(reached_and_bold_before_end(short, base), (false, true));
    assert_eq!(
        reached_and_bold_before_end(&long, base),
        (true, false),
        "the handler spent the {base} steps the short line needs"
    );
}
