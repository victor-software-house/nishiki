#!/usr/bin/env dart

/**/
/* A block comment. */

/**
 * A documentation comment.
 *     indented code
 * ```
 * final fenced = 1;
 * ```
 */
class Cache<T extends Object> {
  final Map<String, int> sizes = {};

  Cache();
  Cache factory() => Cache.new();
  static final make = Cache.new;
  int factory;
  void factory;

  void run(int a) {
    var b = new Cache<int>();
    a &= 3;
    a <<= 1;
    a ~/= 2;
    a -= 1;
    final tab = 'a\tb';
    final block = "${ {a} }";
    final triple = """one
      two""";
    final single = '''one
      two''';
    final raw = r"""raw
      $a""";
    final rawSingle = r'''raw
      $a''';
    final quoted = r'raw $a';
  }
}

// Strings left open at the end of a line, as while text is being edited.
final a = "open
";
final b = r"open
";
final c = 'open
';
final d = r'open
';
