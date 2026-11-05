<!-- A component written for nishiki to reach the svelte grammar's rules. -->
<!-- @component Docs on one line -->
<!---> <!-- x --!> -->
<script lang="ts" generics="T extends string = 'a', in out U, const V">
  let { items }: { items: T[] } = $props();
  const all = $$props;
  const rest = $$restProps;
  const named = $$slots;
</script>

<script module generics='K extends keyof object'>
  export const k = 1;
</script>

<script lang="js">
let js = 1;
</script>

<script lang="ts">let one: number = 1;</script>

<script lang="coffee">
square = (x) -> x * x
</script>

<script src="./extra.js" />

<style lang="css">
a { color: red; }
</style>

<style lang="less">
@c: red;
</style>

<style lang="postcss">
a { color: blue; }
</style>

<style lang="sass">
a
  color: red
</style>

<style lang="scss">
$c: red;
</style>

<style lang="stylus">
a
  color red
</style>

<template>
  <p>plain template</p>
  <p a="1"> b</p>
  {#if a} x {/if}
</template>

<template lang="pug">
p hello
</template>

<div
  // a line comment
  /* a block comment */
  --custom-color="red"
  width="100%"
  height=10px
  class=plain
  {@attach tooltip}
></div>

<slot name="footer" />
<my-element data-a="1"></my-element>
<Foo.Bar></Foo.Bar>
<Widget />

{#await promise}
  <p>waiting</p>
{:then value}
  <p>{value}</p>
{:catch error}
  <p>{error}</p>
{/await}

{#await promise then value}
  <p>{value}</p>
{/await}

{#each items as [first, second], index (first)}
  <p>{first}</p>
{/each}

{#snippet row<T>(item: T)}
  <p>{item}</p>
{/snippet}

{@const total: number = 1 + 2}
{@debug first, second}
