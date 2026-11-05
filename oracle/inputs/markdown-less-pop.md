# Less fences that pop their body

A `)` or `}` at Less's root matches without advancing.
vscode-textmate then pops the fence's body.
The next line with text opens the body again and stays plain.
Less starts fresh on the line after.

```less
) a
b { c: d; }

e { f: g; }
h { i: j; } } k
l { m: n; }
o { p: q; }
```

```less
}

r { s: t; }
u { v: w; }
```

```less
a { b: (c) }
```
