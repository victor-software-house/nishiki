// GDScript for the differential fuzzer (fuzz.ts): pieces of GDScript to build
// documents from, texts to insert, and the info strings of Markdown fences.

/** Pieces that random documents are made of. */
export const pieces: string[] = [
  "extends Node2D", "extends Base.Inner", "class_name Player", "class_name Outer.Named", "class Inner:", "class Inner extends Node:",
  "func _ready() -> void:", "func f(a, b = 2, c: int = 3):", "static func g(x: Vector2) -> int:", "func h(", "  a: int,", "  b = 1)",
  "signal hit", "signal moved(from, to)", "enum State { IDLE, RUN }", "const SPEED := 10", "var speed: float = 1.5",
  "var x setget set_x, get_x", "var y: int:", "\tget:", "\t\treturn y", "\tset(value):", "\t\ty = value", "var z = 1 setget , get_z",
  "@export var n := 1", "@onready var sprite = $Sprite2D", "@export_range(0, 10) var r = 5", "@tool", "@warning_ignore(\"unused\")",
  "var f = func(x): return x", "var g = func (y):", "call(func(a): pass)", "await get_tree().process_frame", "yield(self, \"done\")",
  "if a is not  Node:", "if a is Node2D:", "elif b and not c:", "else:", "for i in range(3):", "while true:", "match value:",
  "match $Node:", "\t1, 2: pass", "\t\"a\": pass", "\t[var x, ..]: pass", "\t_: pass", "\tvar d when d > 0:", "break", "continue",
  "return .e5 + .5 + 1.e5 + 1e10 + 0x1F + 0b101 + 1_000", "x += 1", "y **= 2", "a & b | c ^ ~d << 1 >> 2", "a && b || !c",
  "$Sprite/Body", "$\"Path/To Node\"", "%Unique", "$%Unique/Child", "get_node(\"a/b\")", "get_node_or_null('%c')", "NodePath(\"x/y\")",
  "has_node_and_resource(\"r\")", "find_node(\"n\")", "&\"string_name\"", "^\"node/path\"", "x = a / $Node", "x = a /$Node",
  "$A / B / %C", "$ A/ B", "item.new()", "Foo.new(1)", "Foonew(1)", "E_num.VALUE", "Color.RED", "Vector2.ZERO", "MY_CONST.x",
  "\"a\\tb\\n\"", "'it\\'s'", "r\"raw\\\"", "\"\"\"doc\"\"\"", "'''doc'''", "\"%s %5.2f %-03d %(name)s %%\" % [1]",
  "\"{} {{ }} {0} {name!r:>10} {a.b[0]:{width}.{prec}}\"", "\"{a!s:{w}}\"", "\"{x.y.z}\"", "\"{a[b][c]}\"", "\"{:{}}\"",
  "# comment", "## doc comment", "#region Name", "#endregion", "var w = \\", "\"continued\"", "var v = x \\", "\t+ 1",
  "var u = 1 \\ trailing", "\\", "x \\", "", "print(\"a\", b = 2)", "call(1 = 2)", "call(a == 1, b = 2)(3)", "thing.method (1)",
  "self.x = 1", "super._ready()", "preload(\"res://a.gd\")", "assert(x)", "breakpoint", "é = 1", "café(1)", "Ünïcode.f(x)",
];

/** Texts that mutation inserts. */
export const mutations: string[] = [
  "\"", "'", "\"\"\"", "'''", "r\"", "{", "}", "{{", "(", ")", "[", "]", ":", "=", "==", "\\", "\\\n", "\\" + "\n\n", "\\\n\"",
  "$", "%", "&", "^", "@", "/", "/ ", ".", ".new(", "!r", ":{", "#", "##", "func ", "var ", "is not  ", " ", "\t", "é",
];

/** Info strings of Markdown fences, which Shiki's Markdown grammar does not
 * hand to GDScript. */
export const fences: string[] = ["gdscript", "gd"];
