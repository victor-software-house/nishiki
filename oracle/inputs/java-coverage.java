// Not valid Java throughout: the commented sections reach grammar units that
// only text being edited holds.
package com.Example..app._.9.class-;
import com.example..util._.7.goto-;
import com.example.*;
import com.example*;
import static com.example.Util .*;

/**/
open module com.example {
  requires transitive base;
}

sealed class Shape permits Circle, Square {}

record Pair<T>(T left, T right) implements Comparable {}

class Coverage {
  privateCount = 1;
  threadsafeFlag = 2;
  java.lang.String[] names;
  int[] values = {1, 2};

  <T extends Number & Comparable<T>> void sort(T value) {}

  void take(int a, { b }) {}

  void run(boolean ok) {
    int bits = 0b1010;
    int mode = 0755;
    try {
      go();
    } catch (IOException | RuntimeException e) {
      stop();
    }
    assert ok : "not ok";
    const int a = 0;
    goto end;
    var count = 1;
    var list = new java.util.ArrayList<String>();
    int n = items.0x;
    call({ go(); });
    String block = """
        a \""" b \n c
        """;
    Object o = new int[] {1, 2}.length;
  }

  enum Kind extends Base implements Tag {
    ONE, TWO
  }
}
