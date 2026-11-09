# Per-function splits

Self cost per function, head minus base, from callgrind_annotate at one codegen unit, against b7405d7e.

## S1, css

```text
change base head function
net 175960
   1777016           0     1777016 <nishiki::rules::lexer::Lexer<nishiki::css::engine::Css>>::emit_captures
  -1605012     2194590      589578 <nishiki::rules::lexer::Lexer<nishiki::css::engine::Css>>::emit_match
      3956      570975      574931 0x00000000001884a0
      -272         272           0 0x00000000052206b0 [???]
```

## S1, scss

```text
change base head function
net 253512
   2188575           0     2188575 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_captures
  -1951577     2617119      665542 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match
     61316           0       61316 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_captures'2
    -54836       65471       10635 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match'2
```

## S1, csharp

```text
change base head function
net 348763
   4529017           0     4529017 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_captures
  -4228085     5277019     1048934 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match
    705139           0      705139 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_captures'2
   -660753      827508      166755 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match'2
```

## S2, css

```text
change base head function
net -53664
    -88055     2194590     2106535 <nishiki::rules::lexer::Lexer<nishiki::css::engine::Css>>::emit_match
     34391           0       34391 <nishiki::rules::lexer::Lexer<nishiki::css::engine::Css>>::emit_spans
      -272         272           0 0x00000000052206b0 [???]
       272           0         272 0x00000000052216b0 [???]
```

## S2, scss

```text
change base head function
net -105478
   -119271     2617119     2497848 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match
     16120           0       16120 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_spans
     -2327       65471       63144 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match'2
      -366         366           0 0x00000000052206b0 [???]
```

## S2, csharp

```text
change base head function
net -290878
   -260136     5277019     5016883 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match
    -30742      827508      796766 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match'2
      -586         586           0 0x00000000052206b0 [???]
       586           0         586 0x00000000052216b0 [???]
```

## S2, graphql

```text
change base head function
net 58879
    428002           0      428002 <nishiki::rules::lexer::Lexer<nishiki::graphql::engine::GraphQl>>::emit_spans
   -369123      529353      160230 <nishiki::rules::lexer::Lexer<nishiki::graphql::engine::GraphQl>>::emit_match
      -148         148           0 0x00000000052206b0 [???]
       148           0         148 0x00000000052216b0 [???]
```

## S2, asm

```text
change base head function
net 206225
    829062           0      829062 <nishiki::rules::lexer::Lexer<nishiki::asm::engine::Asm>>::emit_spans
   -622837     1046932      424095 <nishiki::rules::lexer::Lexer<nishiki::asm::engine::Asm>>::emit_match
      -116         116           0 0x00000000052206b0 [???]
       116           0         116 0x00000000052216b0 [???]
```

## S2, make

```text
change base head function
net 217503
   1064146           0     1064146 <nishiki::rules::lexer::Lexer<nishiki::make::engine::Make>>::emit_spans
   -854698     1646404      791706 <nishiki::rules::lexer::Lexer<nishiki::make::engine::Make>>::emit_match
     39104           0       39104 <nishiki::rules::lexer::Lexer<nishiki::make::engine::Make>>::emit_spans'2
    -31049       83589       52540 <nishiki::rules::lexer::Lexer<nishiki::make::engine::Make>>::emit_match'2
```

## S2, c

```text
change base head function
net 470696
   4341412           0     4341412 <nishiki::rules::lexer::Lexer<nishiki::c::engine::C>>::emit_spans
  -3921384     5887410     1966026 <nishiki::rules::lexer::Lexer<nishiki::c::engine::C>>::emit_match
    487051           0      487051 <nishiki::rules::lexer::Lexer<nishiki::c::engine::C>>::emit_spans'2
   -438202      523334       85132 <nishiki::rules::lexer::Lexer<nishiki::c::engine::C>>::emit_match'2
```

## S3, css

```text
change base head function
net -2065
    -14433     2194590     2180157 <nishiki::rules::lexer::Lexer<nishiki::css::engine::Css>>::emit_match
     12368      570975      583343 0x00000000001884a0
      -272         272           0 0x00000000052206b0 [???]
       272           0         272 0x000000000521b6b0 [???]
```

## S3, scss

```text
change base head function
net 90816
     81823     2617119     2698942 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match
      6314      518879      525193 0x00000000001884a0
      2679       65471       68150 <nishiki::rules::lexer::Lexer<nishiki::scss::engine::Scss>>::emit_match'2
      -366         366           0 0x00000000052206b0 [???]
```

## S3, csharp

```text
change base head function
net 80017
     65625     5277019     5342644 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match
      8609      827508      836117 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match'2
      5783      447097      452880 0x00000000001884a0
      -586         586           0 0x00000000052206b0 [???]
```

## S3, graphql

```text
change base head function
net -52320
    -52320      529353      477033 <nishiki::rules::lexer::Lexer<nishiki::graphql::engine::GraphQl>>::emit_match
      -148         148           0 0x00000000052206b0 [???]
       148           0         148 0x000000000521b6b0 [???]
```

## S4, css

```text
change base head function
net 4973
      4973      570975      575948 0x00000000001884a0
      -272         272           0 0x00000000052206b0 [???]
       272           0         272 0x00000000052136b0 [???]
```

## S4, scss

```text
change base head function
net 4184
      4184      518879      523063 0x00000000001884a0
      -366         366           0 0x00000000052206b0 [???]
       366           0         366 0x00000000052136b0 [???]
```

## S4, c

```text
change base head function
net 3019
      2455      248412      250867 0x00000000001884a0
       872       29995       30867 0x0000000000188c00
      -306        7812        7506 0x00000000000a9bb0
      -298       57166       56868 0x00000000000ab650
```

## S4, make

```text
change base head function
net 604
       705       88289       88994 0x00000000001884a0
      -258         258           0 0x00000000052206b0 [???]
       258           0         258 0x00000000052136b0 [???]
      -101       32217       32116 <nishiki::arena::Arena>::intern
```

## S4, csharp

```text
change base head function
net -166788
   -152757     5277019     5124262 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match
    -15131      827508      812377 <nishiki::rules::lexer::Lexer<nishiki::csharp::engine::CSharp>>::emit_match'2
      1100      447097      448197 0x00000000001884a0
      -586         586           0 0x00000000052206b0 [???]
```

## S4, graphql

```text
change base head function
net -57254
    -57254      529353      472099 <nishiki::rules::lexer::Lexer<nishiki::graphql::engine::GraphQl>>::emit_match
      -148         148           0 0x00000000052206b0 [???]
       148           0         148 0x00000000052136b0 [???]
```

## S4, tex

```text
change base head function
net 14240
     14202      275756      289958 <nishiki::rules::lexer::Lexer<nishiki::tex::engine::Tex>>::emit_match
       -84          84           0 0x00000000052206b0 [???]
        84           0          84 0x00000000052136b0 [???]
        49        5056        5105 0x00000000001884a0
```

## S4, php

```text
change base head function
net 40323
    -72700       72700           0 <nishiki::rules::lexer::Lexer<nishiki::php::engine::Php>>::dynamic_path
     71000           0       71000 <nishiki::rules::lexer::Lexer<nishiki::php::engine::Php>>::path_one
     49642     1866452     1916094 <nishiki::rules::lexer::Lexer<nishiki::php::engine::Php>>::emit_match
    -11549      138616      127067 <nishiki::rules::lexer::Lexer<nishiki::php::engine::Php>>::path_miss
```

## S4, haskell

```text
change base head function
net 15650
    -42595       42595           0 <nishiki::rules::lexer::Lexer<nishiki::haskell::engine::Haskell>>::dynamic_path
     40350           0       40350 <nishiki::rules::lexer::Lexer<nishiki::haskell::engine::Haskell>>::path_one
     22922     1099893     1122815 <nishiki::rules::lexer::Lexer<nishiki::haskell::engine::Haskell>>::emit_match
     -4461       48174       43713 <nishiki::rules::lexer::Lexer<nishiki::haskell::engine::Haskell>>::path_miss
```

## S4, matlab

```text
change base head function
net 22974
     18844      569376      588220 <nishiki::rules::lexer::Lexer<nishiki::matlab::engine::Matlab>>::emit_match
      2632       73083       75715 0x00000000001884a0
      1498       47766       49264 <nishiki::rules::lexer::Lexer<nishiki::matlab::engine::Matlab>>::emit_match'2
      -206         206           0 0x00000000052206b0 [???]
```

## S4, java

```text
change base head function
net 31928
     30209     3472860     3503069 <nishiki::rules::lexer::Lexer<nishiki::java::engine::Java>>::emit_match
      1719      341651      343370 0x00000000001884a0
      -280         280           0 0x00000000052206b0 [???]
       280           0         280 0x00000000052136b0 [???]
```
