# Ruby that nishiki's other oracle inputs leave out, written to exercise the
# rest of Shiki's `ruby` grammar.

# Percent literals with each delimiter, and the escapes each one allows.
regex_angle = %r<a<b>c\>d>imx
regex_comment = %r{
  a # a comment
  b
}x
symbols_angle = %I<a#{b} c\> d>
symbols_nested = %I{a{b}c}
symbols_pipe = %I|a#{b} c\| d|
symbols_square = %I[a [b] c\] d]
plain_symbols_angle = %i<a <b> c\> d>
plain_symbols_curly = %i{a {b} c\} d}
plain_symbols_pipe = %i|a b\| c|
plain_symbols_square = %i[a [b] c\] d]
plain_symbols_parens = %i(a (b) c\) d)
words_curly = %W{a#{b} c\} d}
words_pipe = %W|a#{b} c\| d|
plain_words_angle = %w<a <b> c\> d>
plain_words_curly = %w{a {b} c\} d}
plain_words_pipe = %w|a b\| c|
plain_words_square = %w[a [b] c\] d]
plain_words_parens = %w(a (b) c\) d)
string_angle = %<a <b> c>
quoted_pipe = %Q|a #{b} c|
single_angle = %q<a <b> c\> d>
single_square = %q[a [b] c\] d]
single_curly = %q{a {b} c\} d}
single_pipe = %q|a b\| c|
symbol_angle = %s<a <b> c\> d>
symbol_square = %s[a [b] c\] d]
symbol_curly = %s{a {b} c\} d}
symbol_pipe = %s|a b\| c|
symbol_parens = %s(a (b) c\) d)

# Control flow the grammar scopes on its own.
if a
  b
else if c
  d
end
end
if (x ||= compute)
  x
end
half = total /= 2

# A regular expression with a comment group.
pattern = /a(?# a comment )b/

# YARD tags with types and default values.
# @option opts [Integer] :size (10) the size
# @!parse [ruby] module Parsed; end

=begin
A block comment.
=end

plain = <<TEXT
body
TEXT

page = <<~HAML
  %p text
<<~INNER
HAML
