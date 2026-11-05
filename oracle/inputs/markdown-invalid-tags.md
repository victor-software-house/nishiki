# Attributes of invalid tags

Written for nishiki.
An invalid tag reads HTML's attribute list in a Markdown document, and MathML's in an HTML document.

```html
<foo close for="a" columnalign class>
<foo
  close
  for="b">
<bar mathsize href=c>
```

```php
<foo close for class>
<?php
$page = <<<HTML
<foo close for>
HTML;
?>
<baz columnalign for>
```

```perl
print <<HTML;
<foo close for="$x" class>
<foo
  close
  for>
HTML
print <<~'HTML';
  <bar columnalign for>
  HTML
```

Raw HTML in the document: <foo close for>.
