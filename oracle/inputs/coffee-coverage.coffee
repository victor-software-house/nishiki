# Reaches the units of the coffee grammar that the other inputs do not.
x = new class Foo.Bar
y = new class
s = """
  a\tb #{x}
"""
###
  @param x
###
'f': -> 1
"g" = -> 2
a extends b
class extends Base
class C extends @D
x...map
f = (@x, @y...) ->
q = 9lives
h = 3abc()
r = ///
  \1 (?=a) (?!b) [^a-z] [\x41-B] [\cA-\101] [\.-\-] [a-\cB]
  \x41 \012 \cB
///
<div class="a" b={x}>{x}</div>
<span/>
foo.bar baz
Foo::bar()
Foo::bar qux
el.onBlur()
o.4bad()
@go a
n = 0b101 + 0o17 + 017
d = [1.5e3, 1.e5, .5e2, 1e3, 1.5, 1..3, 3., .5, 7]
CONST.x
a.FOO.b
a::BAR.c
a::BAR
a.5
a::5
a ?= b
b and= c
a <<= 1
b = 1 << 2 >>> 3
