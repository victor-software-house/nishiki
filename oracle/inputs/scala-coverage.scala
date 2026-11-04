#!/usr/bin/env -S scala-cli shebang
//> using scala 3.3.1
//> using dep "com.lihaoyi::os-lib:0.9.1", true, os.Path
//> using test.dep org::lib:1.0
/**/
  // Written for nishiki: what the scala grammar's other inputs leave out.
package object tools
export scala.collection.{Map as M, given, Seq => S, given Ordering}
export Predef.{println => p}
export Foo.Bar.baz; export given
import java.util.{List => JList, given}
import a.b.{given Foo, c => d}
import x.y as z
import cats.given
import m.{given Ord => O, given bar}
export m.{given Ord => O, given bar}
var count, total = 0
val (a, b) = (1, 2)
var (c, d) = (3, 4)
type T = Int
case object Empty
object Point extends Base with Mixin derives CanEqual
given intOrd: Ordering[Int] = ???
extension (s: String) def shout = s.toUpperCase
inline def twice(inline x: Int): Int = inline if x > 0 then x else 0
val bin = 0b1010_1011L
val dec = .5e3f
val ok = x.isInstanceOf[Int] && y.asInstanceOf[Long] == classOf[String]
val bounds = (A <: B, C >: D, E <% F, G =:= H, I <:< J)
val m = Map(1 -> 2) ++ Map(3 -> 4) :: Nil
val q = '{ x }
val r = '[ T ]
val splice = ${ q }
val sym = 'alpha
val tick = `my var` + 1
val esc2 = '\na'
val cons = foo_:bar
def max[T: Ord as ord](a: T) = a
val ch = '\n' + 'a' + '\777' + '\q' + 'ab' + '''
val st = s"a\tb\q $$ $x ${y}"
val bad = s"x $1 + 1
val cost = s"total $
val raw1 = raw"a$$b $x ${y}"
val badRaw = raw"x $1 + 2
val raw3 = raw"""line $$ $x"""
val badRaw3 = raw"""x $1 + 3
val inter3 = f"""value \\ ${x} $y $$"""
val bad3 = f"""x $1 + 4
val esc = "tab\t\q"
val uni = "\u0041" + """\u0042 \\""" + '\u0043'
val kind = obj.type
val xml = <a href="x" ns:id='y'>{name}&amp;&#65;&#x41;</a>
val empty = <br></br>
val ns = <p:item></p:item>
val link = <p:item href={url} a="1">x</p:item>
}
private[this] def f(using ctx: Ctx): Unit = ()
protected[pkg] val g = 1
@volatile var h = 1
transparent inline def i = 1
end if
end while
end val
end var
end try
end Shape
end new
end foo // done
