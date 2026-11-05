<!--
  Less that stops without advancing at its root, where vscode-textmate pops
  the body that embeds it and ends the line: a `)` or `}` before anything
  else in the body. Less pops one of its own rules instead when one is open, as in the
  last body, which stays open to the end.
-->
<style lang="less">)
*/ x
</style>
<p>after</p>
<style lang="less">
)
a { b: c }
</style>
<style lang="less">
}
a { b: c }
</style>
<style lang="less">a { b: (c) }
</style>
<p>end</p>
<style lang="less">a { b: c }
)
a { b: c }
</style>
