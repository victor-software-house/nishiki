<!--
  A `/>` right after a language tag's `>` is part of the body.
  A tag that closes itself with `/>` has no body.
-->
<script>/> a
</script>
<script setup lang="ts">/> b
</script>
<style>/> c
</style>
<template>/> d
</template>
<script setup />/> e
