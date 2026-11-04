<!-- A component written for nishiki to reach the vue grammar's rules. -->
<script
  setup
  lang="ts"
>
import { ref, computed } from 'vue'
const count = ref(0)
const items = computed(() => [1, 2, 3].map((n) => n * count.value))
</script>

<script lang="jsx">
export const view = <span class="a">{'x'}</span>;
</script>

<template>
  <div :class="{ active: count > 0 }" @click.stop.prevent="count++">
    <template v-if="count > 2">
      <MyButton v-for="(item, index) in items" :key="index" #default="{ value }" />
    </template>
    <template v-else-if="count === 1"><span>one</span></template>
    <template v-else>
      <OtherWidget v-model:value.trim="name" v-on:custom-event='handle($event)' :title=`t${count}` />
    </template>
    <Teleport to="body" :disabled=flag v-bind="attrs"></Teleport>
    <slot name="footer" />
    <x-unknown data-a="1">{{ count }}</x-unknown>
  </div>
</template>

<style scoped lang="scss">
$size: 4px;
.a { padding: $size * 2; &:hover { color: red; } }
</style>

<style
  module
  lang="less"
>
@base: #f04615;
.b { color: @base; }
</style>

<style>
.c { margin: 0 auto; }
</style>

<i18n lang="yaml">
en:
  hello: Hello
</i18n>

<docs lang="graphql">
query Docs { component(name: "Coverage") { id } }
</docs>

<i18n lang="json5">{ en: { a: 1 } }</i18n>
<tpl lang="pug">div.a hello</tpl>
<script lang="coffee">square = (x) -> x * x</script>
<style lang="stylus">a
  color red
</style>
<style lang="sass">.a
  color: red
</style>
<style lang="postcss">.a { color: red; }</style>
<style lang="css">.d { color: blue; }</style>
<style lang="less">.e { color: @c; }</style>
<script lang="js">export const one = 1;</script>
<script lang="ts">export const two: number = 2;</script>
<script
  lang="tsx"
>
export const three = <b>3</b>;
</script>
<script
  lang="jsx">
export const four = <i>4</i>;
</script>
<script
  lang="coffee">
five = 5
</script>
<style
  lang="stylus">
a
  color blue
</style>
<style
  lang="postcss">
.f { color: green; }
</style>
<frag lang="html"><b>x</b></frag>
<custom-block>
plain text of a block no rule names
</custom-block>

<Icon name="star" />

<script lang="tsx">export default { render: () => <div>{1}</div> }</script>
