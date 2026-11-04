#!/usr/bin/perl
# Heredocs that Perl's grammar hands to other languages.
use strict;
use warnings;

my ($id, $name) = (7, 'Ada');

my $page = <<HTML;
<div class="card">
  <a href="/users/$id">$name</a>
</div>
HTML

my $static = <<'HTML';
<p>No $interpolation here.</p>
HTML

my $feed = <<~XML;
    <?xml version="1.0"?>
    <feed><entry id="1">$name</entry></feed>
    XML

my $query = <<SQL;
SELECT u.id, COUNT(o.id) AS orders
FROM users u
LEFT JOIN orders o ON o.user_id = u.id
GROUP BY u.id
SQL

my $script = <<'JAVASCRIPT';
const total = items.reduce((sum, item) => sum + item.price, 0);
console.log(`total: ${total}`);
JAVASCRIPT

my $style = <<CSS;
.card { color: #333; margin: 0 auto; }
CSS

my $doc = <<POSTSCRIPT;
%!PS
newpath 100 100 moveto showpage
POSTSCRIPT

print length($page) + length($query), "\n";
