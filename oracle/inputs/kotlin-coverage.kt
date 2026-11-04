@file:[Suppress JvmName]

package coverage

/**
 * @param[value] the value to box
 * See {@link Box#map(Int) the mapper}.
 */
class Box<out T>(val value: T)

typealias Pairs<K, V> = Map<K, List<V>>

val <T> List<T>.second: T get() = this[1]

fun count(n: Int): Int {
    var i = n
    i++
    i--
    val text = """first \$ line
        second"""
    call(a= = 1)
    return i
}
