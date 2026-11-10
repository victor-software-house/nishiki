# Vue in a fence

A Markdown `vue` fence can carry attributes after the language, as VitePress's line highlights do.

```vue {2,4}
<template>
  <p :class="{ active }">{{ message }}</p>
</template>
<script setup lang="ts">
const message = 'Hello';
</script>
```

```vue:no-line-numbers
<button @click="count++">{{ count }}</button>
```
