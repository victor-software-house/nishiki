<!--
  Svelte's `$` prefixes in a CoffeeScript body. Svelte's grammar injects them
  into every rule under `source.coffee`: code, comments, interpolations and
  the JavaScript between backticks. Not after a quote, a dot, a `$`, `_` or a
  name character.
-->
<script lang="coffee">
count = $count + a.$b + _$c + x$d
# $e in a comment
###
  $f in a block comment
###
y = "$g #{$h}"
z = '$i'
w = `$j + $k`
$$props
$$restProps.a
$$slots
$$other
$ alone
($l) -> $m
heredoc = """
  $n #{$o}
"""
</script>
<p>{$count}</p>
