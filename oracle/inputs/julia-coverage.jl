# TODO FIXME CHANGED XXX
#= block #= nested =# TODO =#
module M
baremodule B end
using Base: show
import LinearAlgebra as LA
export f, g
public h
const C = 1
global G
local L = 2
abstract type A <: B end
primitive type P 8 end
mutable struct S{T} <: A
    x::Vector{T}
    y::Union{Int,Nothing}
    z::Ref{"s"}
end
x!:_struct Foo <: Bar{Baz}
y!:_mutable struct Qux
typegroup
quote x end
function Base.show(io::IO, x::S{T}) where {T}
    println(io, x)
end
function (f::F)(x)
end
function g end
function f{T}(x) end
macro m(ex) ex end
f(x)::Int where T = x
h{T}(x) = x
k(x) = x == 1 ? :a : :b
for i in 1:10, j in a
    for outer k in b
    end
end
while true; break; continue end
if a elseif b else end
let x = 1 end
try catch e finally end
map(x) do y; return y end
begin end
x = [1, 2]'
y = (a, b).'
z = {c}'
w = ∇f'.''
v = a'
f(x)'
x]' y)'.'
arr = [begin end end]
a += 1; a -= 1; a *= 2; a //= 2; a /= 2; a .//= 2; a ./= 2; a .*= 2; a \= 2; a .\= 2
a ^= 2; a .^= 2; a %= 2; a .%= 2; a ÷= 2; a .÷= 2; a |= 1; a &= 1; a .&= 1
a ⊻= 1; a .⊻= 1; a $= 1; a <<= 1; a >>= 1; a >>>= 1; a := 1
b = a << 1 >> 2 >>> 3 .>> 4 .>>> 5 .<< 6
c = a <= b >= c < d > e ≤ f ≥ g == h === i ≡ j != k ≠ l !== m ∈ n .<= o
d = a ? b : c
e = a || b && !c
r = 1:2; s = a[1:end]
t = x |> f
u = a | b .| c & d .& e ~f ¬g .~h ⊻ i .⊻ j
v = a + b - c * d / e // f % g ^ h \ i ++ j -- k .+ l ± m × n ÷ o · p
f ∘ g
x isa Int
x in y
a.b; a..b; a...; x…
$x
x = a -> y ← z → w <--> v --> u <-- t => s ⟶ r
@doc """
    f(x)

Doc $x \t
""" ->
f
doc"""text"""
@doc doc"""text""" f
"""
    docstring $x \t
"""
 doc"""
"""
s = "a $b $(c) \n \\ \0 \12 \4 \x41 é \U0001F600 \q $(d+e) $α"
c = 'a'; d = '\''
t = """triple $x"""
r = r"a+\d"ims
rt = r"""multi
line"""xs
raw"a\"b\\" raw"x\\\"y"
raw"""x \" y"""
var"x y"
var"""z"""
cxx"""int x = $y;"""
icxx"""return 1;"""
py"""
def f(x): return x
x = 1"""
js"""let a = $(b);"""
R"""x <- $y"""
sql"""SELECT * FROM t WHERE a = $(b)"""
b"bytes"suffix
v"1.0"
html"""<b>$x</b>"""m
`echo $x \``
run`ls $d`x
cmd```ls```x
x = :sym
@time @. @! @async f()
@show(x)
0x1F 0o17 0b101 1_000 1.5e-3 1e10 2.5f0 3im 4Inf 5NaN32 6π 7pi 8ℯ Inf NaN64 π pi ℯ 9'
ARGS C_NULL DEPOT_PATH ENDIAN_BOM ENV LOAD_PATH PROGRAM_FILE stdin stdout stderr VERSION devnull
true false nothing missing
x::Int...
y::Union{}
z <: (A, B)'
w :: Vector{Vector{Vector{Int}}}
q :: "str"
