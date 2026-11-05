// Astro for the differential fuzzer (fuzz.ts): pieces of Astro to build
// documents from, texts to insert, and the info strings of Markdown fences.

/** Pieces that random documents are made of. */
export const pieces: string[] = [
  "---", "const { title } = Astro.props;", "import Card from \"./Card.astro\";", "const xs = [...ys];", "let a = b...",
  "<!-- comment -->", "<!-- open", "close -->", "<!--> x -->", "<!---> x -->", "a --!> b", "<!DOCTYPE html>",
  "<html lang=\"en\">", "</html>", "<head>", "</head>", "<body>", "</body>", "<meta charset=\"utf-8\" />", "<br>", "<img src={src} />",
  "<input type=\"checkbox\" checked>", "<div class=\"a\">", "</div>", "<p>text</p>", "<Card title={t} />", "<my-el a-b=\"x\"></my-el>",
  "<Fragment set:html={html} />", "<a href={url} class:list={[\"a\", { b: true }]}>", "</a>", "<span {...props}>", "</span>",
  "<script>", "</script>", "<script lang=\"ts\">", "<script type=\"module\">", "<script type=\"application/ld+json\">",
  "<script type=\"text/partytown\">", "<script lang=\"tsx\">", "<script src=\"/x.js\" />", "<script is:inline>", "<script lang=ts>",
  "<script\n  lang=\"ts\">", "<script define:vars={{ a }}>", "<script lang=\"ts\" data-x=\"a>b\">", "const n: number = 1;",
  "document.querySelector(\"a\")?.click();", "console.log(`x ${1}`);", "if (a < b) { c(); }", "x = \"</div>\";",
  "<style>", "</style>", "<style lang=\"scss\">", "<style lang=\"less\">", "<style lang=\"sass\">", "<style lang=\"postcss\">",
  "<style lang=\"css\">", "<style lang=\"stylus\">", "<style is:global>", "h1 { color: red; }", ".a { .b { color: $c; } }",
  "@c: green;", ".a\n  color: red", "<pre is:raw>", "</pre>", "<pre is:raw>{x} <b>raw</b></pre>", "<code is:raw />",
  "<button onclick=\"go(); // done\">", "<button onmouseover='a /* b */ c'>", "<a onclickx=\"x()\">", "<a onkeydown=handle()>",
  "<a onplayingw=\"x\">", "<a onclick-x=\"y\">", "<a onload=`x ${y}`>", "<a onclick={f}>", "<div title=`a ${b} \\n \\x41`>",
  "<div @click=\"x\" $y=z>", "<div data-n=1>", "<div a = \"b\">", "{x}", "{ {a: 1} }", "{ items.map((i) => <li>{i}</li>) }",
  "{cond && <p>yes</p>}", "{/* comment */}", "{", "}", "&amp;", "&#38;", "&#x26;", "&bogus;", "&nbsp;", "a & b", "plain text",
  "Hello {name}!", "été", "", "  ", "\t<li>",
];

/** Texts that mutation inserts. */
export const mutations: string[] = [
  "<", ">", "</", "/>", "<!--", "-->", "---", "...", "{", "}", "=", "\"", "'", "`", "${", "\\", "&", ";", "/", "//", "/*", "*/",
  "<script>", "</script>", "<style>", "</style>", " lang=\"ts\"", " is:raw", "on", "é", " ", "\t", "\n",
];

/** Info strings of Markdown fences, which Shiki's Markdown grammar does not
 * hand to Astro. */
export const fences: string[] = ["astro"];
