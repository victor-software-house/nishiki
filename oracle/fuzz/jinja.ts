// Jinja for the differential fuzzer (fuzz.ts): pieces of Jinja to build
// documents from, texts to insert, and the info strings of Markdown fences.

/** Pieces that random documents are made of. */
export const pieces: string[] = [
  "{% extends \"base.html\" %}", "{% block content %}", "{% endblock %}", "{% block title scoped %}", "{%- if user -%}", "{% elif x %}",
  "{% else %}", "{% endif %}", "{% for item in items %}", "{% for k, v in d.items() recursive %}", "{% endfor %}", "{%- endfor -%}",
  "{% set x = 1 %}", "{% set a, b = 1, 2 %}", "{% macro m(a, b=2) %}", "{% endmacro %}", "{% call m(1) %}", "{% endcall %}",
  "{% filter upper %}", "{% endfilter %}", "{% include \"nav.html\" with context %}", "{% import \"m.html\" as m without context %}",
  "{% from \"m.html\" import a, b %}", "{% raw %}{{ x }}{% endraw %}", "{% raw %}", "{% endraw %}", "{%raw%}", "{%- raw -%}",
  "{{ x }}", "{{- x -}}", "{{ x|e }}", "{{ x | default(\"a\") | upper }}", "{{ a.b.c }}", "{{ a[0][\"k\"] }}", "{{ f(a, b=1) }}",
  "{{ {\"k\": 1} }}", "{{ {\"k\": {\"j\": [1, 2]}} }}", "{{ [1, 2, 3] }}", "{{ (1 + 2) * 3 // 4 ** 5 % 6 }}", "{{ a == b != c }}",
  "{{ a < b > c <= d >= e }}", "{{ x is defined }}", "{{ x is not none }}", "{{ true and false or not none }}", "{{ loop.index }}",
  "{{ super() }}", "{{ self.title() }}", "{{ varargs }}", "{{ kwargs }}", "{{ \"a \\\" b\" }}", "{{ 'a \\' b' }}",
  "{{ \"\\x41 \\u00e9 \\U0001F600 \\N{LATIN SMALL LETTER A}\" }}", "{{ \"\\n \\t \\r \\a \\b \\f \\v \\\\\" }}", "{{ \"a \\",
  "b\" }}", "{{ @/a+\\/b/ }}", "{# comment #}", "{#- trimmed -#}", "{# {# nested #} #}", "{# {# {# {# deep #} #} #} #}", "{# open",
  "close #}", "<div class=\"{{ cls }}\">", "</div>", "<a href=\"{{ url_for('x') }}\">{{ _('Link') }}</a>", "plain text",
  "Hello {{ name }}!", "{{ 1abc }}", "{{ x_1 }}", "{{ \u00e9 }}", "{{ \u00e9t\u00e9 }}", "{% for i in range(3) %}{{ i }}{% endfor %}",
  "{{", "}}", "{%", "%}", "{#", "#}", "", "    {{ indented }}", "\t{% if x %}",
];

/** Texts that mutation inserts. */
export const mutations: string[] = [
  "{{", "}}", "{%", "%}", "{#", "#}", "-", "{", "}", "[", "]", "(", ")", "\"", "'", "\\", "\\n", "\\\n", "@/", "/", "|", ".",
  ",", "=", "==", "with context", " is ", " in ", "raw", "endraw", "\u00e9", " ", "\t", "\n",
];

/** Info strings of Markdown fences, which Shiki's Markdown grammar does not
 * hand to Jinja. */
export const fences: string[] = ["jinja", "j2", "jinja2"];
