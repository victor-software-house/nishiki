<!-- A component written for nishiki to reach the vue grammar's block, directive and attribute rules. -->
<!-- @layout { "name": "wide", "order": [1, 2], "draft": true } -->
<script setup lang="ts" generic="T extends string, U = number, in out V">
defineProps<{ item: T }>();
</script>
<script
  lang="ts">export const six: number = 6;</script>
<script
  lang="tsx">export const seven = <u>7</u>;</script>
<script setup lang="tsx">
export const eight = <s>8</s>;
</script>
<script lang="tsx">export const nine = 9;</script>

<template>
  <div>
    <template><span>bare</span></template>
  </div>
</template>

<panel style="color: red; --gap: 2px; /* note */ margin: 0 auto;;" v-on:[eventName].once="go" :[attrName].prop="value" #[slotName]="slotProps">panel</panel>
<panel style='padding: 1px'>quoted</panel>

<docs lang="md">
# Title

Some *Markdown* text.
</docs>
<config lang="json">
{ "a": [1, true, null] }
</config>
<config lang="json">{ "b": 2 }</config>
<config lang="jsonc">
// a comment
{ "c": 3 }
</config>
<config lang="jsonc">{ "d": 4 }</config>
<settings lang="toml">
[server]
port = 8080
</settings>
<settings lang="toml">a = 1</settings>
<preview lang="vue">
<template><b>{{ msg }}</b></template>
</preview>
<preview lang="vue"><i>x</i></preview>
<style lang="postcss">
.g { color: green; }
</style>
<style
  lang="scss">
$w: 1px;
.h { width: $w; }
</style>
<style
  lang="scss">.i { color: red; }</style>
<style
  scoped>
.j { color: blue; }
</style>
<style
  lang="less">
@k: 1px;
</style>
<style
  lang="postcss">.l { color: red; }</style>
