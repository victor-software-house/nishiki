// Perl for the differential fuzzer (fuzz.ts) and the Oniguruma harness
// (onig.ts): pieces of Perl to build documents from, texts to insert, the
// info strings of Markdown fences, and the texts the harness probes
// expressions with.
//
// A bare `/regex/` opens only after `&`, `(`, `{`, `|`, `~`, `if`, `unless`
// or at a line's start, so the pieces put a `/` after each of those, after a
// name and alone.

/** Where a bare `/` can follow: each opener of the lookbehind, a name, and nothing. */
const slashLeads = ["&", "(", "{", "|", "~", "if", "unless", "elsif", "x", "$a", ")", " "];

/** Pieces that random documents are made of. */
export const pieces: string[] = [
  "use strict;", "use warnings;", "package Foo::Bar;", "package Foo 1.0;", "require Exporter;", "our @ISA = qw(Exporter);",
  "sub foo {", "sub foo($$;@) {", "sub foo-bar", "sub {", "my $x = shift;", "my ($a, $b) = @_;", "our %h;", "local $_;",
  "return;", "}", "{", "if ($x) {", "} elsif ($y) {", "} else {", "unless (-e $f) {", "while (<STDIN>) {", "for my $i (0..$#a) {",
  "foreach (@list) {", "last;", "next if /^#/;", "die \"no\" unless $ok;", "print STDERR \"x\\n\";", "printf(\"%s\\n\", $x);",
  "open(my $fh, '<', $file) or die;", "close $fh;", "chomp(my $line = <$fh>);", "split /,/, $s;", "join(',', @a)",
  "$h{key} = 1;", "$h->{key}", "$a->[0]", "@{$r}", "%$r", "$#a", "$#{$r}", "${x}", "@x[1..2]", "$x =~ s/a/b/g;",
  "$x =~ m{a+}i", "$x =~ tr/a-z/A-Z/;", "y/a/b/", "s{a}{b}gx", "s(a)(b)", "s<a><b>", "s'a'b'", "s#a#b#g", "s/\\//x/;",
  "qr/a\\d+/i", "qr{a}", "qr'a'", "q{a {b} c}", "qq{$x}", "qw(a b c)", "qw/a b/", "qx{ls}", "q(a)", "qq[$a]", "qq<x>",
  "'single \\' quote'", "\"double $x @y ${z}\"", "`cmd`", "\"\\n\\t\\x41\\x{263A}\\N{U+41}\\cA\\o{17}\\101\"",
  "$x = <<EOT;", "$x = <<~EOT;", "$x = <<'EOT';", "$x = <<\"EOT\";", "$x = <<`EOT`;", "$x = <<\\EOT;", "EOT", "  EOT",
  "print <<HTML;", "print <<~'SQL';", "my $q = <<\"SQL\";", "<<XML", "<<CSS", "<<JAVASCRIPT", "<<POSTSCRIPT", "HTML", "SQL",
  "XML", "CSS", "JAVASCRIPT", "POSTSCRIPT", "<div class=\"a\">$x</div>", "SELECT a FROM t WHERE b = 1;", "body { color: red; }",
  "var x = 1;", "<a><b/></a>", "=pod", "=head1 NAME", "=head2 B<Bold> I<it> C<code>", "=item * L<Foo::Bar>", "=over 4",
  "=back", "=begin html", "=end html", "=for comment", "=encoding utf8", "=cut", "E<gt> F<file> S<x> X<i> Z<>",
  "C<< a >>", "I<< b >>", "C<\\s c \\s>", "__END__", "__DATA__", "__FILE__", "__LINE__", "__PACKAGE__", "__SUB__",
  "BEGIN {", "END {", "format STDOUT =", "@<<<< @>>>>", ".", "$x x 3", "-e $f", "-d _", "a => 1,", "{ key }", "<=>",
  "->", "=>", "and", "or", "not", "xor", "wantarray", "bless {}, $class;", "ref($x) eq 'HASH'", "defined $x",
  "$1", "$&", "$`", "$'", "$+", "$0", "$!", "$@", "$/", "$\\", "$;", "$_", "@ARGV", "%ENV", "$ENV{HOME}", "STDIN",
  "# comment", "  # indented comment", "\t$x;", "    $y;", "1;", "0x1F", "1_000", "3.14", "é", "(", ")", "[", "]",
  "{", "}", "<", ">", ";", ",", "=", "==", "=~", "!~", "&&", "||", "//", "?", ":", " ", "  ", "\t", "\n", "\n", "\n",
  ...slashLeads.map((lead) => `${lead}/a+/`),
  ...slashLeads.map((lead) => `${lead} /b/i;`),
];

/** Texts that mutation inserts. */
export const mutations: string[] = [
  "/", "s/", "m/", "tr/", "y/", "qr/", "q{", "qq(", "qw[", "{", "}", "(", ")", "[", "]", "<", ">", "'", "\"", "`", "\\",
  "\\\\", "#", "$", "@", "%", "<<EOT\n", "<<~EOT\n", "EOT\n", "=pod\n", "=cut\n", "__END__\n", "é",
  ...slashLeads.map((lead) => `${lead}/`),
];

/** Info strings of Markdown fences, some of which do not name Perl. */
export const fences: string[] = ["perl", "pl", "pm", "perl6", "raku"];

/** For the Oniguruma harness. */
export const probe = {
  /** Texts inserted into fixture lines. */
  snippets: [
    "qr{a}", "qr[a]", "qr<a>", "qr(a)", "qr'a'", "qr/a/", "qr#a#", "qr !a!", "m{a}", "m[a]", "m<a>", "m(a)", "m'a'", "m/a/",
    "m!a!", "m _a_", "m#a#", "s{a}{b}", "s[a][b]", "s<a><b>", "s(a)(b)", "s'a'b'", "s/a/b/", "s|a|b|g", "s #a#b#x", "s/a/b/x",
    "s'a'b'x", "s:a:b:", "tr/a/b/", "tr{a}{b}", "y/a/b/", "tr/\\//x/", "y/\\\\/a/", "tr|a|b|cd", "q{a}", "q(a)", "q[a]",
    "q<a>", "q/a/", "qw/a b/", "qw(a b)", "qq{a}", "qq(a)", "qq/a/", "qq!a!", "qx{a}", "qx'a'", "qx/a/", "->q(", "&s/a/b/",
    "$x", "@x", "%x", "$#x", "$#{", "${x}", "${::x}", "$x::y", "$::x", "$$", "$7a", "$_", "$0", "$1", "$12", "$&", "$`",
    "$'", "$+", "$\"", "$!", "$/", "$\\", "$;", "$a", "$b", "$|", "$~", "$^", "@_", "%_", "$#_", "\\1", "\\c[", "\\cA",
    "\\g{1}", "\\g{-1}", "\\g{name}", "\\g2", "\\k<n>", "\\k{n}", "\\k'n'", "\\N{U+41}", "\\o{17}", "\\p{L}", "\\PL",
    "\\P{Lu}", "\\x41", "\\x{263A}", "\\x", "\\n", "\\'", "\\\\", "<<EOT", "<<~EOT", "<<'EOT'", "<<~'EOT'", "<<\"EOT\"",
    "<<`EOT`", "<<\\EOT", "<< EOT", "<<~ 'A B'", "<<HTML", "<<~HTML", "<<'SQL'", "<<\"XML\"", "<<CSS", "<<JAVASCRIPT",
    "<<POSTSCRIPT", "<<;", "<<$x", "<<1", "I<a>", "I<< a >>", "I<<< a b >>>", "B<a>", "B<< a >>", "C<a>", "C<\\s a \\s>",
    "C<< a >>", "L<a>", "L<a|b>", "E<gt>", "F<f>", "S<s>", "X<x>", "Z<>", "a => 1", "a=>", "{ a }", "{a}", "-e", "-d",
    "-X", "-q", "a-e", "<=>", "->", "=>", "x 3", "x3", "x  10", "sub a {", "sub a($) {", "sub a-b", "sub ($$) {", "sub",
    "format A =", "format STDOUT=", "__FILE__", "__LINE__", "__SUB__", "__PACKAGE__", "__DATA__", "__END__", "my", "our",
    "local", "and", "or", "xor", "as", "not", "if", "unless", "elsif", "wantarray", "ARGV", "STDIN", "y", "print", "{}", "()",
    ...slashLeads.flatMap((lead) => [`${lead}/a/`, `${lead} /a/g`, `${lead}/`]),
  ] as string[],
  /** Texts tried at 0 for an expression that starts with `\G(`. */
  anchored: ["m/a/", "m{a}", "m!a!", "m _a_", "m_a_", "ma", "m", "m  /a/", "m'a'", "m(a)", "m#", "-m/a/", "{m/a/"] as string[],
  /** Whole texts tried at 0. */
  whole: [
    "=pod", "=back", "=cut", "=cut x", "=cutx", "=head1 Name", "=head5", "=item", "=item *", "=over 4", "=encoding utf8",
    "=begin html", "=begin  html  ", "=begin text", "=end html", "=end  html", "=end text", "=for x", "=a", "=",
    "package Foo;", "  package Foo::Bar 1.0;", "package", "BEGIN {", "  END", "INITX", "CHECK", "DESTROY", "UNITCHECK",
    "\tx", "    x", "   x", "\t\tx", "__END__", "__A__", "__a_b__", "____", "__é__", ".", ". ", ".x", "# c", "  # c", "\t#",
    "EOT", "  EOT", "EOT ", "HTML", "  SQL", "XML", "CSS", "JAVASCRIPT", "POSTSCRIPT", "A B", " A B", "",
    ...slashLeads.map((lead) => `/a/ ${lead}`),
  ] as string[],
};
