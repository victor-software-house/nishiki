// Go that nishiki's other oracle inputs leave out, written to exercise the
// rest of Shiki's `go` grammar. Much of it is not valid Go: the grammar has
// rules for names that start with a digit and for stray brackets.
package 1main

// Declarations whose types hold a stray bracket or a parenthesized channel
// element type.
const a ]b = 1
const n [b = 1
const g chan (int) = nil
const (
	c ]d = 2
	k chan (int) = nil
	m [b = 1
)
var h chan (int)
var (
	i chan (int)
)

// Functions with a name that starts with a digit, a result type alone on the
// line, and type parameters one per line.
func 1bad() {}

func h() foo

func g[
	A,
	B any,
](x A) {}

type I interface {
	x.Y[int]
}

// Expressions with braces inside type arguments, invalid literals, numbers
// with separators in each part, and a function literal whose signature holds
// more than its two parenthesized lists.
func f() {
	_ = (T[{}])(x)
	_ = x.(T[{}])
	_ = T[{}]{}
	_ = f[{}](x)
	_ = map[)]int{}
	_ = map
	a, 1b = 1, 2
	c, 1d := 3, 4
	_ = 089
	_ = 1abc
	_ = 'ab'
	_ = "\q"
	_ = []  int{}
	_ = 0x.1_0p0
	_ = 0x1p-1_0
	_ = 0x1.8_8p0i
	_ = 0x.8p1_0i
	_ = func() chan int { return nil }
	f := func(a) (b) [ ] { } x (c) {}
	switch v := x.(type) {
	case int: // comment
	}
}

// Lines of a language server's hover text, which the grammar scopes too.
field name invalid type
field name foo
returns foo

import"fmt"
