#!/usr/bin/env swift
// Written for nishiki's oracle coverage: the rules of Shiki's Swift grammar
// that the other inputs do not reach.

let block = #/
  (?<year>\d{4})   # a line comment
  \g{year} \g{-1+2} \g{year+1} \g{2-1}
  \g1 \g-2+3 \g+1
  \k<year> \k'year' \k{year} \k<-1+0> \k'1-1'
  (?P=year) (?P>year) (?&year) (?&year+1) (?R) (?1) (?+1) (?-1) (?1+2)
  \g<year> \g'year' \g<1+0> \g<year-1>
  (*ACCEPT) (*FAIL) (*COMMIT) (*MARK:name) (*PRUNE:name) (*SKIP) (*THEN:name)
  (*LIMIT_DEPTH=10) (*LIMIT_HEAP=5) (*NOTEMPTY) (*NOTEMPTY_ATSTART) (*UTF) (*CR) (*LF)
  (?C) (?C12) (?C`na``me`) (?C'na''me') (?C"x""y") (?C^x^^y^) (?C%x%%y%) (?C#x##y#) (?C$x$$y$) (?C{x}}y})
  (*name) (*name[tag]) (*name[tag]{a,b})
  (?{code}) (?{code}[tag]>) (?{{code}}X) (?{{{code}}}<)
  \p{Letter} \P{L} \p{^Greek} \p{Script=Greek} [[:alpha:]] [[:^digit:]]
  [abc] [^a-z&&[aeiou]] [a--b] [a~~b] [\d\-\]]
  (?i) (?i-m) (?^) (?^i) (?i:x) (?<name>x) (?'name'x) (?P<name>x) (?:x) (?|x) (?>x) (?=x) (?!x) (?<=x) (?<!x) (?*x) (?<*x) (?<a-b>x) (?'a-b'x)
  (?(1)a|b) (?(<name>)a) (?('name')a) (?(R)a) (?(R1)a) (?(R&name)a) (?(DEFINE)a) (?(VERSION>=10.0)a) (?(+1)a) (?(-1)a)
  \Q quoted text \E
  (?# a comment)
  <{ embedded }>
  \u{1F600} \x{41} \N{U+41} \o{17} A \U00000041 \x41 \0 \01
  \A \z \Z \b \B \G \K \y \Y $ ^
  \cA \e \t \. \d \D \w \W \s \S \h \H \v \V \R \X \N \O
  a* a+ a? a{2} a{2,} a{,3} a{ 1 , 3 } a*? a++
  \k'year+1' \g'year-1' \k{year+1} \12 [:alpha:] [:^digit=x:] [\b] (?~absent)
  (?(1+2)a) (?(R&name+1)a) (?(VERSION=10.0)a) (?(<name-1>)a) (?((?=a)b|c))
/#

let line = /(?<word>\w+) \k<word> \Q/\E (?C1) (*COMMIT)/
let extended = #/a/b (?<n>x) \g{n}/#
let extendedReturn = #/a/ b
let doubleHash = ##/a#/b/##

/*: Playground markup, /* nested */ still a comment */
//: A playground line
let stray = 1 */ 2

func ordered() throws async {}
@available(*, deprecated: 13.0)
@available(iOS, unavailable, message)
@objc(first:second) func selectorPiece() {}
@`backticked` struct Wrapped {}

let lengthOfString = text.startsWith("a")
autoreleasepool { }
sort(values)
let args = CommandLine.arguments
let count = Process.argc
let buffer: ManagedBuffer<Int, Int>
let look: QuickLookObject
let actorType: Actor
let generator: GeneratorType
let raw: RawByte
let word: UWord

#if arch(arm64) || arch(sparc) || targetEnvironment(simulator)
#endif
#sourceLocation(file: "a.swift", line: 1, junk) trailing
#sourceLocation()

let f: nonisolated(nonsending) () async -> Void
let p: protocol<A, B>
let inline: [5 of Int]
let wildcard: [_]
let keyed: [String: [Int: Bool]]
struct Fixed<let N: Int> {}
func result() -> sending Value {}
init?!() {}

import `Module`;
import Module.$1
import operator Module.+.
class Empty: {}
macro `stringify`(_ value: Int) = #externalMacro(module: "M", type: "S")
infix operator .+. : `Group`
infix operator ~~ { associativity left precedence 140 assignment }
func labels(outer `inner`: Int, sent: sending Value, nested: x: Int) {}
precedencegroup `Group` { higherThan: `Other` }
protocol `Proto` {
  associatedtype `Element`
  init?!()
  func `method`()
}
enum Labels { case pair(first second: Int), single(_ name: Int, x: y: Int) }
let typed: `Typed` = value

call `label`: { }
if #available(* junk) {}
let compound = `method`(first:second:)
let viaOf = [3 of Int]
repeat {} while false
do {}
var property: Int { get throws async { 0 } }
var other: Int { get async throws { 0 } }
let imported = import
let path = #keyPath(Foo.bar)
let getter = #selector(getter: Foo.bar)
let badHex = 0x_1g
let member = value.`member`
@available(*, unavailable junk)
infix operator +.+
let sliced: [String: Int: Bool]
enum Typed { case value([Int]: Bool) }
call(import)
struct Generic<`T`>: `Base` {}

let extra = "closed"#
let block = """
    text"""#
let raw = #"""
    continued \#
    text"""##
let raw2 = ##"""
    body
    """###
let open = #"unterminated
let rawEscape = #"a \#u{41} \#(f(x)) b"#
let nested = "a \(f(x)) b"
