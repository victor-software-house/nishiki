# Written for nishiki: Nix that reaches the grammar's rarer rules.
{
  plain = { a
  }: a;
  dotted = { a
    .b = 1; };
  comma = { a
    , b }: a;
  equals = { a
    = 1; };
  default = { a
    ? 1, b }: a;
  reserved = { in };
  illegal = { a, % }: a;
  named = args@{ a }: a;
  terminal = { a, ... } @ args: a;
  bare = { a, b } @args: a;
  oldLet = let { body = 1; }.body;
  search = { a = <nixpkgs>; b = <nixpkgs/lib>; };
  branch = if a then b else c;
}
