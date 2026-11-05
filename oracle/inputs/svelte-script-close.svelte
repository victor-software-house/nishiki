<!--
  A `/>` right after a language tag's `>` starts the body.
  The body begins with an `L:` injection, which vscode-textmate takes over the
  tag's end pattern, `/>`, where both start at one place.
  A tag that closes itself with `/>` has no body.
-->
<script>/> a
</script>
<script context="module">/> b
</script>
<script lang="ts">/> c
</script>
<style>/> d
</style>
<script context="module" />/> e
<template>/> f
</template>
<p>end</p>
