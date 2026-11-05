<!--
@component
An HTML block's `<script>` in an `@component` comment's Markdown.
Svelte's grammar injects its `$` prefixes into that JavaScript, as into its own.
A fence names its code without `source.js`, so it takes none.

<script>
  console.log($store, $$props, $$restProps, $$slots);
  const a = "$b" + c.$d + '$e' + f$g + _$h;
  let i = $j.k + $$propsx;
</script>

```js
const l = $m;
```

<div>
<button onclick="$n()">o</button>
<script>let p = $q;</script>
</div>
-->
<p>{name}</p>
