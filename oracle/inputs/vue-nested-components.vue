<!-- Each block in lang="vue" holds a component on the stack of the one around it. -->
<outer lang="vue">
<inner lang="vue">
<deepest lang="vue">
<template>
  <p>{{ depth }}</p>
</template>
</deepest>
</inner>
<script setup lang="ts">
const level = 1;
</script>
</outer>
<a lang="vue"><b lang="vue"><c lang="vue"><template><i>{{ x }}</i></template></c></b></a>
<template>
  <p>{{ after }}</p>
</template>
