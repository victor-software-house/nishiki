extends Base.Inner
class_name Outer.Named

#region Setup
func _init(a, b = 2, c: int = 3):
	var f = func(x): return x
	var t = Vector2.ZERO
	var e = Color.RED
	var o = item.new()
	var k = E_num.VALUE
	var p = Bar.new(1)
	if a is not  Node:
		pass
	if a & b | c ^ d << 1 >> 2 and ~b:
		pass
	if a && b || c:
		pass
	call(name = 1, x = 2)
	call(a == 1, b = 2)(3)
	call(1 = 2)
	get_node("a")(1)
	thing.method (1)
	return .e5 + .5 + .5e3 + 1.e5 + .1e-3 + 1e10 + 1_000e+2
#endregion

class Inner:
	var x = 1

match $Node:
	_: pass

match value:
	1: pass
	"a": match other:
		_: pass

func g():
	var s = "a\tb\n%s %5.2f %-03d %(name)s %%" % [1]
	var u = "{} {{ }} {0} {name!r:>10} {a.b[0]:{width}.{prec}} {a!s:{w}}"
	var v = 'it\'s'
	for i in range(3):
		print(i)
	var w = \
	"continued"
	var y = x \
		+ 1
	var z = 1 \ trailing
	lambda_call(func(a): pass)
	func_ref(func (b):
		pass)
	return func
