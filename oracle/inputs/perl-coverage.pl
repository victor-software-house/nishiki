#!/usr/bin/perl
# Perl written for nishiki's oracle, to reach every expression of the grammar.
  # an indented comment
	# a tab-indented comment
use strict;
use warnings;
package Foo::Bar;
  package Baz 1.0;

BEGIN { print "start\n"; }
  END { print "end\n" }
INIT { 1 }
CHECK { 1 }
UNITCHECK { 1 }
DESTROY { 1 }

my ($a, $b) = @_;
our %h = (key => 1, other => 2);
local $_ = shift;
my @list = (1, 2, 3);
print $#list, $#{$r}, $#_, @list[1..2], %h, "\n";
print ${x}, ${::y}, $Foo::bar, $::z, $$, $7a, @_, %_;
print $&, $`, $', $+, $", $0, $!, $@, $/, $\, $;, $|, $~, $^, $a, $b, $_;
print $1, $12, $0;
print @0, %_, $#9, $_1;
print $h{key}, $h->{key}, $r->[0], @{$r}, %$r;
print { key }, {other};
print {} ;
print ();

sub plain {
    my $self = shift;
    return;
}
sub with_proto($$;@) { 1 }
sub dashed-name
sub {
sub named;
sub spaced ($) { 1 }

if ($a and $b or not $c xor $d) {
    print "yes" if $a;
} elsif ($b) {
    die "no" unless $ok;
} else {
    last;
}
while (my $line = <STDIN>) { next if /^#/; redo; }
until ($done) { do { 1 } }
for my $i (0 .. $#list) { goto LABEL; }
foreach (@list) { continue }
given ($x) { when (1) { default { 1 } } }
switch ($x) { case 1 { 1 } }
my $v = eval { require Exporter; wait; select(undef, undef, undef, 0.1); exit 0 };
use constant PI => 3.14;
$obj->die; $obj->if;

print -e $file, -d _, -f $f, -A $f, -X $f, -z $f, -s $f;
my $cmp = $a <=> $b;
my $ref = \@list;
my $arrow = $r->method;
my %pairs = (a => 1);
my $rep = "-" x 10;
my $rep2 = "=" x3;

print __FILE__, __LINE__, __PACKAGE__, __SUB__, "\n";
binmode STDOUT, ':utf8';
open(my $fh, '<', $file) or die "cannot open: $!";
my $text = join(',', map { lc } sort grep { defined } keys %h);
printf STDERR "%s\n", ucfirst(lc($text));
my $time = localtime; chomp(my $in = <STDIN>); bless {}, __PACKAGE__;
wantarray ? 1 : 0;

my $s1 = 'single \' quote \\ done';
my $s2 = "double $a @list ${b} \n\t\x41\x{263A}\N{U+41}\cA\o{17}\101\e\0";
my $s3 = `ls -l $dir`;
my $re1 = qr{a\d+(?:b|c)}i;
my $re2 = qr[a\w+]x;
my $re3 = qr<a\s+>;
my $re4 = qr(a(b)c$);
my $re5 = qr'a$b';
my $re6 = qr/a\/b/msix;
my $re7 = qr!a\!b!;
my $re8 = qr #a#;
my $re9 = qr/a$/;
my $re10 = qr(a(b$)c);
my $re11 = qr!a$!;
if ($x =~ m{a{2}b}g) { 1 }
if ($x =~ m[a[b]c]) { 1 }
if ($x =~ m<a<b>c>) { 1 }
if ($x =~ m(a(b)c)) { 1 }
if ($x =~ m'a$b') { 1 }
if ($x =~ m/a\/b$/i) { 1 }
if ($x =~ m!a$!) { 1 }
if ($x =~ m _a_) { 1 }
if ($x =~ m#a[b]c#) { 1 }
if ($x =~ m/a[b$]c/) { 1 }
if ($x =~ m!a[$]!) { 1 }
$x =~ s{a}{b}g;
$x =~ s[a{1}][b]e;
$x =~ s<a><b>;
$x =~ s(a(b))(c);
$x =~ s{a}
  {b}x;
$x =~ s{a} [b];
$x =~ s{a} <b>;
$x =~ s{a} (b);
$x =~ s{a} 'b\'c';
$x =~ s{a} /b/;
$x =~ s'a'b\'c';
$x =~ s/a\/b/c/g;
$x =~ s|a|b|;
$x =~ s#a#b#;
$x =~ s/a/b$/x;
$x =~ s'a'b'x;
$x =~ s!a!b!xg;
$x =~ s %a%b%;
$x =~ s/a
  b/c/xg;
$x =~ s'a
  b'c'x;
$x =~ s!a
  !b!x;
$x =~ s/a/b/x.$y;
&s/a/b/;
$x =~ tr/a-z/A-Z/;
$x =~ tr{a}{b};
$x =~ y/\//|/cd;
$x =~ y/\\/x/;
$y = tr/a\\\/b/c/r;
my @w1 = qw(a b c);
my @w2 = qw/a b/;
my @w3 = qw[x y];
my @w4 = qw{p q};
my @w5 = qw<m n>;
my @w6 = q(a (b) c);
my @w7 = q{a {b} c};
my @w8 = q[a [b] c];
my @w9 = q<a <b> c>;
my @w10 = q/a b/;
my $q1 = qq{a $x {b}};
my $q2 = qq(a $y);
my $q3 = qq[a @z];
my $q4 = qq<a $w>;
my $q5 = qq/a $v/;
my $q6 = qq!a!;
my $x1 = qx{ls};
my $x2 = qx(ls);
my $x3 = qx[ls];
my $x4 = qx<ls>;
my $x5 = qx'ls $x';
my $x6 = qx/ls/;
my $x7 = qx!ls!;
$obj->q(1); $obj->qq(1); $obj->qx(1);
my $r1 = split /,/, $str;
my $r2 = grep { /a/ } @list;
my $r3 = ($str =~ /b+/);
if /c/;
unless /d/i;
my $r4 = $str ~ /e/;
my $r5 = $a | /f/;
my $r6 = $a & /g$/;
/h/ and print;
my $r7 = (/i\/j/g);
my $esc = "\1 \g{1} \g{-1} \g{name} \g2 \k<n> \k{n} \k'n' \N{U+41} \o{7} \p{L} \PL \P{Lu} \x41 \x{263A} \x \q";

$x = <<EOT;
plain $a
EOT
$x = <<~EOT;
  indented $a
  EOT
$x = <<'EOT';
raw $a
EOT
$x = <<~'EOT';
    raw indented
    EOT
$x = <<"EOT";
quoted $a
EOT
$x = <<~"EOT";
  quoted indented
  EOT
$x = <<\EOT;
backslash $a
EOT
$x = <<~\EOT;
  backslash indented
  EOT
$x = << "SPACED";
spaced label
SPACED
$x = <<'A B';
label with a space
A B
print <<HTML;
<div class="a">$a</div>
HTML
print <<~HTML;
  <p>indented</p>
  HTML
print <<'HTML';
<b>raw</b>
HTML
print <<~'HTML';
    <i>raw</i>
    HTML
print <<"HTML";
<u>quoted</u>
HTML
print <<~"HTML";
  <s>quoted</s>
  HTML
print <<XML;
<a><b/></a>
XML
print <<~XML;
  <a/>
  XML
print <<'XML';
<c/>
XML
print <<~'XML';
  <d/>
  XML
print <<"XML";
<e/>
XML
print <<~"XML";
  <f/>
  XML
print <<CSS;
body { color: red; }
CSS
print <<~CSS;
  a { margin: 0; }
  CSS
print <<'CSS';
p { top: 1px; }
CSS
print <<~'CSS';
  q { left: 0; }
  CSS
print <<"CSS";
r { right: 0; }
CSS
print <<~"CSS";
  s { width: 1em; }
  CSS
print <<JAVASCRIPT;
var a = 1;
JAVASCRIPT
print <<~JAVASCRIPT;
  let b = 2;
  JAVASCRIPT
print <<'JAVASCRIPT';
const c = 3;
JAVASCRIPT
print <<~'JAVASCRIPT';
  f(4);
  JAVASCRIPT
print <<"JAVASCRIPT";
g(5);
JAVASCRIPT
print <<~"JAVASCRIPT";
  h(6);
  JAVASCRIPT
my $sql = <<SQL;
SELECT a FROM t WHERE b = 1;
SQL
$sql = <<~SQL;
  UPDATE t SET a = 2;
  SQL
$sql = <<'SQL';
DELETE FROM t;
SQL
$sql = <<~'SQL';
  INSERT INTO t VALUES (1);
  SQL
$sql = <<"SQL";
SELECT * FROM u;
SQL
$sql = <<~"SQL";
  SELECT 1;
  SQL
print <<POSTSCRIPT;
newpath 0 0 moveto
POSTSCRIPT
print <<~POSTSCRIPT;
  stroke
  POSTSCRIPT
print <<'POSTSCRIPT';
showpage
POSTSCRIPT
print <<~'POSTSCRIPT';
  closepath
  POSTSCRIPT
print <<"POSTSCRIPT";
fill
POSTSCRIPT
print <<~"POSTSCRIPT";
  gsave
  POSTSCRIPT
print <<A, <<B;
first
A
second
B

format STDOUT =
@<<<< @>>>> @||||
$name, $value, $center
.
format OTHER=
text
 .
.

=pod

Text with B<bold>, I<italic>, C<code>, L<Foo::Bar>, E<gt>, F<file.pl>,
S<no break>, X<index>, Z<>, B<< spaced bold >>, I<<< spaced italic >>>,
C<\s code \s> and C<< not code >>, and C<<\s set off \s>>.

=head1 NAME

=head2 B<Bold> heading

=head3 Third

=head4 Fourth

=item * L<Link|Foo>

=over 4

=back

=encoding utf8

=for comment

=begin text

=end text

=begin html

<p>POD <b>HTML</b></p>

=end html

=begin html

<i>to the cut</i>

=cut

    print "indented code";
	print "tab code";
		print "two tabs";
        print "eight spaces";

my $data = 1;
__FOO__ a line that starts with a dunder name
__END__
This is data, which the grammar marks as a comment.
my $not_code = 1;
