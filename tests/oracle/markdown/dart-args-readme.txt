L [![Dart CI](https://github.com/dart-lang/core/actions/workflows/args.yaml/badge.svg)](https://github.com/dart-lang/core/actions/workflows/args.yaml)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 86 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 147 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 148 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L [![pub package](https://img.shields.io/pub/v/args.svg)](https://pub.dev/packages/args)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 85 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 86 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L [![package publisher](https://img.shields.io/pub/publisher/args.svg)](https://pub.dev/packages/args/publisher)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 109 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 110 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L Parses raw command-line arguments into a set of options and values.
T 67 text.html.markdown meta.paragraph.markdown
L 
L This library supports [GNU][] and [POSIX][] style options, and it works in both
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 79 text.html.markdown meta.paragraph.markdown
L server-side and client-side apps.
T 33 text.html.markdown meta.paragraph.markdown
L 
L ## Defining options
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 19 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L First create an [ArgParser][]:
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 30 text.html.markdown meta.paragraph.markdown
L 
L     var parser = ArgParser();
T 4 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown
L 
L Then define a set of options on that parser using [addOption()][addOption] and
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 78 text.html.markdown meta.paragraph.markdown
L [addFlag()][addFlag]. Here's the minimal way to create an option named "name":
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 78 text.html.markdown meta.paragraph.markdown
L 
L     parser.addOption('name');
T 4 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown
L 
L When an option can only be set or unset (as opposed to taking a string value),
T 78 text.html.markdown meta.paragraph.markdown
L use a flag:
T 11 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addFlag('name');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Flag options, by default, accept a 'no-' prefix to negate the option. You can
T 77 text.html.markdown meta.paragraph.markdown
L disable the 'no-' prefix using the `negatable` parameter:
T 35 text.html.markdown meta.paragraph.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addFlag('name', negatable: false);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.language.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L *Note:* From here on out, "option" refers to both regular options and flags. In
T 1 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 79 text.html.markdown meta.paragraph.markdown
L cases where the distinction matters, we'll use "non-flag option."
T 65 text.html.markdown meta.paragraph.markdown
L 
L Options can have an optional single-character abbreviation, specified with the
T 78 text.html.markdown meta.paragraph.markdown
L `abbr` parameter:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', abbr: 'm');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('verbose', abbr: 'v');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Options can also have a default value, specified with the `defaultsTo`
T 58 text.html.markdown meta.paragraph.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 70 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L parameter. The default value is used when arguments don't specify the option.
T 77 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', defaultsTo: 'debug');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('verbose', defaultsTo: false);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.language.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The default value for non-flag options can be any string. For flags, it must
T 76 text.html.markdown meta.paragraph.markdown
L be a `bool`.
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown meta.paragraph.markdown
L 
L To validate a non-flag option, you can use the `allowed` parameter to provide an
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown
L allowed set of values. When you do, the parser throws an
T 56 text.html.markdown meta.paragraph.markdown
L [`ArgParserException`][ArgParserException] if the value for an option is not in
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 79 text.html.markdown meta.paragraph.markdown
L the allowed set. Here's an example of specifying allowed values:
T 64 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', allowed: ['debug', 'release']);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can use the `callback` parameter to associate a function with an option.
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L Later, when parsing occurs, the callback function is invoked with the value of
T 78 text.html.markdown meta.paragraph.markdown
L the option:
T 11 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', callback: (mode) => print('Got mode $mode'));
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.closure.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart variable.parameter.dart
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('verbose', callback: (verbose) {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (verbose) print('Verbose');
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L });
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The callbacks for all options are called whenever a set of arguments is parsed.
T 79 text.html.markdown meta.paragraph.markdown
L If an option isn't provided in the args, its callback is passed the default
T 75 text.html.markdown meta.paragraph.markdown
L value, or `null` if no default value is set.
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown
L 
L If an option is `mandatory` but not provided, the results object throws an
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L [`ArgumentError`][ArgumentError] on retrieval.
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 46 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', mandatory: true);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.language.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Parsing arguments
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Once you have an [ArgParser][] set up with some options and flags, you use it by
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L calling [ArgParser.parse()][parse] with a set of arguments:
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 59 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var results = parser.parse(['some', 'command', 'line', 'args']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L These arguments usually come from the arguments to `main()`. For example:
T 51 text.html.markdown meta.paragraph.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L 
L     main(List<String> args) {
T 4 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.tag.other.unrecognized.html.derivative punctuation.definition.tag.begin.html
T 20 text.html.markdown meta.paragraph.markdown meta.tag.other.unrecognized.html.derivative entity.name.tag.html
T 21 text.html.markdown meta.paragraph.markdown meta.tag.other.unrecognized.html.derivative punctuation.definition.tag.end.html
T 29 text.html.markdown meta.paragraph.markdown
L       // ...
T 6 text.html.markdown meta.paragraph.markdown
T 12 text.html.markdown meta.paragraph.markdown
L       var results = parser.parse(args);
T 6 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown
L     }
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown
L 
L However, you can pass in any list of strings. The `parse()` method returns an
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown
L instance of [ArgResults][], a map-like object that contains the values of the
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 77 text.html.markdown meta.paragraph.markdown
L parsed options.
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addOption('mode');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('verbose', defaultsTo: true);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.language.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var results = parser.parse(['--mode', 'debug', 'something', 'else']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L print(results.option('mode')); // debug
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L print(results.flag('verbose')); // true
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L By default, the `parse()` method allows additional flags and options to be
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L passed after positional parameters unless `--` is used to indicate that all
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L further parameters will be positional. The positional arguments go into
T 71 text.html.markdown meta.paragraph.markdown
L [ArgResults.rest][rest].
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 24 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L print(results.rest); // ['something', 'else']
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To stop parsing options as soon as a positional argument is found,
T 66 text.html.markdown meta.paragraph.markdown
L `allowTrailingOptions: false` when creating the [ArgParser][].
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 62 text.html.markdown meta.paragraph.markdown
L 
L ## Specifying options
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 21 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L To actually pass in options and flags on the command line, use GNU or POSIX
T 75 text.html.markdown meta.paragraph.markdown
L style. Consider this option:
T 28 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('name', abbr: 'n');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can specify its value on the command line using any of the following:
T 73 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L --name=somevalue
T 16 text.html.markdown markup.fenced_code.block.markdown
L --name somevalue
T 16 text.html.markdown markup.fenced_code.block.markdown
L -nsomevalue
T 11 text.html.markdown markup.fenced_code.block.markdown
L -n somevalue
T 12 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Consider this flag:
T 19 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addFlag('name', abbr: 'n');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can set it to true using one of the following:
T 50 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L --name
T 6 text.html.markdown markup.fenced_code.block.markdown
L -n
T 2 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can set it to false using the following:
T 44 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L --no-name
T 9 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Multiple flag abbreviations can be collapsed into a single argument. Say you
T 76 text.html.markdown meta.paragraph.markdown
L define these flags:
T 19 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   ..addFlag('verbose', abbr: 'v')
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   ..addFlag('french', abbr: 'f')
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   ..addFlag('iambic-pentameter', abbr: 'i');
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can set all three flags at once:
T 36 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L -vfi
T 4 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L By default, an option has only a single value, with later option values
T 71 text.html.markdown meta.paragraph.markdown
L overriding earlier ones; for example:
T 37 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addOption('mode');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var results = parser.parse(['--mode', 'on', '--mode', 'off']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print(results.option('mode')); // prints 'off'
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Multiple values can be parsed with `addMultiOption()`. With this method, an
T 35 text.html.markdown meta.paragraph.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L option can occur multiple times, and the `parse()` method returns a list of
T 41 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L values:
T 7 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addMultiOption('mode');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var results = parser.parse(['--mode', 'on', '--mode', 'off']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print(results.multiOption('mode')); // prints '[on, off]'
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L By default, values for a multi-valued option may also be separated with commas:
T 79 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addMultiOption('mode');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var results = parser.parse(['--mode', 'on,off']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print(results.multiOption('mode')); // prints '[on, off]'
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This can be disabled by passing `splitCommas: false`.
T 32 text.html.markdown meta.paragraph.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown
L 
L ## Defining commands
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L In addition to *options*, you can also define *commands*. A command is a named
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 78 text.html.markdown meta.paragraph.markdown
L argument that has its own set of options. For example, consider this shell
T 74 text.html.markdown meta.paragraph.markdown
L command:
T 8 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L $ git commit -a
T 15 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The executable is `git`, the command is `commit`, and the `-a` option is an
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown
L option passed to the command. You can add a command using the [addCommand][]
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 75 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L method:
T 7 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var command = parser.addCommand('commit');
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L It returns another [ArgParser][], which you can then use to define options
T 19 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 74 text.html.markdown meta.paragraph.markdown
L specific to that command. If you already have an [ArgParser][] for the command's
T 49 text.html.markdown meta.paragraph.markdown
T 50 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L options, you can pass it in:
T 28 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var command = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addCommand('commit', command);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The [ArgParser][] for a command can then define options or flags:
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 65 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L command.addFlag('all', abbr: 'a');
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L You can add multiple commands to the same parser so that a user can select one
T 78 text.html.markdown meta.paragraph.markdown
L from a range of possible commands. When parsing an argument list, you can then
T 78 text.html.markdown meta.paragraph.markdown
L determine which command was entered and what options were provided for it.
T 74 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var results = parser.parse(['commit', '-a']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print(results.command.name);   // "commit"
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L print(results.command['all']); // true
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Options for a command must appear after the command in the argument list. For
T 77 text.html.markdown meta.paragraph.markdown
L example, given the above parser, `"git -a commit"` is *not* valid. The parser
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 59 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 77 text.html.markdown meta.paragraph.markdown
L tries to find the right-most command that accepts an option. For example:
T 73 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var parser = ArgParser();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('all', abbr: 'a');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var command = parser.addCommand('commit');
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L command.addFlag('all', abbr: 'a');
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L var results = parser.parse(['commit', '-a']);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print(results.command['all']); // true
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Here, both the top-level parser and the `"commit"` command can accept a `"-a"`
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L (which is probably a bad command line interface, admittedly). In that case, when
T 80 text.html.markdown meta.paragraph.markdown
L `"-a"` appears after `"commit"`, it is applied to that command. If it appears to
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown
L the left of `"commit"`, it is given to the top-level parser.
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown meta.paragraph.markdown
L 
L ## Dispatching Commands
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 23 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L If you're writing a command-based application, you can use the [CommandRunner][]
T 63 text.html.markdown meta.paragraph.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L and [Command][] classes to help structure it. [CommandRunner][] has built-in
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 76 text.html.markdown meta.paragraph.markdown
L support for dispatching to [Command][]s based on command-line arguments, as well
T 27 text.html.markdown meta.paragraph.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L as handling `--help` flags and invalid arguments.
T 12 text.html.markdown meta.paragraph.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 49 text.html.markdown meta.paragraph.markdown
L 
L When using the [CommandRunner][] it replaces the [ArgParser][].
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 49 text.html.markdown meta.paragraph.markdown
T 50 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 63 text.html.markdown meta.paragraph.markdown
L 
L In the following example we build a dart application called `dgit` that takes commands `commit` and `stash`.
T 60 text.html.markdown meta.paragraph.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 95 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 100 text.html.markdown meta.paragraph.markdown
T 101 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 107 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 108 text.html.markdown meta.paragraph.markdown
L 
L The [CommandRunner][] takes an `executableName` which is used to generate the help message.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown meta.paragraph.markdown
L 
L e.g.
T 4 text.html.markdown meta.paragraph.markdown
L `dgit commit -a`
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L File `dgit.dart`
T 5 text.html.markdown meta.paragraph.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void main(List<String> args) {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   var runner = CommandRunner("dgit", "A dart implementation of distributed version control.")
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 91 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 92 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     ..addCommand(CommitCommand())
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     ..addCommand(StashCommand())
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     ..run(args);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L When the above `run(args)` line executes it parses the command line args looking for one of the commands (`commit` or `stash`).
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 106 text.html.markdown meta.paragraph.markdown
T 107 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 113 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 114 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 118 text.html.markdown meta.paragraph.markdown
T 119 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 124 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 125 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 127 text.html.markdown meta.paragraph.markdown
L 
L If the [CommandRunner][] finds a matching command then the [CommandRunner][] calls the overridden `run()` method on the matching command (e.g. CommitCommand().run).
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 75 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 98 text.html.markdown meta.paragraph.markdown
T 99 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 104 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 105 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 164 text.html.markdown meta.paragraph.markdown
L 
L Commands are defined by extending the [Command][] class. For example:
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 69 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class CommitCommand extends Command {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   // The [name] and [description] properties must be defined by every
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   // subclass.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   final name = "commit";
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   final description = "Record changes to the repository.";
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   CommitCommand() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // we can add command specific arguments here.
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     // [argParser] is automatically created by the parent class.
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     argParser.addFlag('all', abbr: 'a');
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L 
L   // [run] may also return a Future.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   void run() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // [argResults] is set before [run()] is called and contains the flags/options
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     // passed to this command.
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     print(argResults.flag('all'));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### CommandRunner Arguments
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L The [CommandRunner][] allows you to specify both global args as well as command specific arguments (and even sub-command specific arguments).
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 141 text.html.markdown meta.paragraph.markdown
L 
L #### Global Arguments
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 21 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L Add argments directly to the [CommandRunner] to specify global arguments:
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 73 text.html.markdown meta.paragraph.markdown
L 
L Adding global arguments
T 23 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var runner = CommandRunner('dgit',  "A dart implementation of distributed version control.");
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 90 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 91 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 92 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 93 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L // add global flag
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L runner.argParser.addFlag('verbose', abbr: 'v', help: 'increase logging');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Command specific Arguments
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 31 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L Add arguments to each [Command][] to specify [Command][] specific arguments.
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 76 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L   CommitCommand() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // we can add command specific arguments here.
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     // [argParser] is automatically created by the parent class.
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     argParser.addFlag('all', abbr: 'a');
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### SubCommands
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Commands can also have subcommands, which are added with [addSubcommand][]. A
T 57 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 72 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 77 text.html.markdown meta.paragraph.markdown
L command with subcommands can't run its own code, so [run][] doesn't need to be
T 52 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 78 text.html.markdown meta.paragraph.markdown
L implemented. For example:
T 25 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class StashCommand extends Command {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final String name = "stash";
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   final String description = "Stash changes in the working directory.";
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.double.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   StashCommand() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     addSubcommand(StashSaveCommand());
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     addSubcommand(StashListCommand());
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Default Help Command
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 24 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L [CommandRunner][] automatically adds a `help` command that displays usage
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L information for commands, as well as support for the `--help` flag for all
T 53 text.html.markdown meta.paragraph.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L commands. If it encounters an error parsing the arguments or processing a
T 73 text.html.markdown meta.paragraph.markdown
L command, it throws a [UsageException][]; your `main()` method should catch these and
T 21 text.html.markdown meta.paragraph.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown meta.paragraph.markdown
L print them appropriately. For example:
T 38 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L runner.run(arguments).catchError((error) {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (error is! UsageException) throw error;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.logical.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   print(error);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   exit(64); // Exit code 64 indicates a usage error.
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.numeric.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L });
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Displaying usage
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 19 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L You can automatically generate nice help text, suitable for use as the output of
T 80 text.html.markdown meta.paragraph.markdown
L `--help`. To display good usage information, you should provide some help text
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown
L when you create your options.
T 29 text.html.markdown meta.paragraph.markdown
L 
L To define help text for an entire option, use the `help:` parameter:
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('mode', help: 'The compiler configuration',
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     allowed: ['debug', 'release']);
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L parser.addFlag('verbose', help: 'Show additional diagnostic info');
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For non-flag options, you can also provide a help string for the parameter:
T 75 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('out', help: 'The output path', valueHelp: 'path',
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     allowed: ['debug', 'release']);
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For non-flag options, you can also provide detailed help for each expected value
T 80 text.html.markdown meta.paragraph.markdown
L by using the `allowedHelp:` parameter:
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L parser.addOption('arch', help: 'The architecture to compile for',
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     allowedHelp: {
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       'ia32': 'Intel x86',
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L       'arm': 'ARM Holding 32-bit chip'
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
L     });
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To display the help, use the [usage][usage] getter:
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 51 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L print(parser.usage);
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The resulting string looks something like this:
T 47 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L --mode            The compiler configuration
T 44 text.html.markdown markup.fenced_code.block.markdown
L                   [debug, release]
T 34 text.html.markdown markup.fenced_code.block.markdown
L 
L --out=<path>      The output path
T 33 text.html.markdown markup.fenced_code.block.markdown
L --[no-]verbose    Show additional diagnostic info
T 49 text.html.markdown markup.fenced_code.block.markdown
L --arch            The architecture to compile for
T 49 text.html.markdown markup.fenced_code.block.markdown
L       [arm]       ARM Holding 32-bit chip
T 41 text.html.markdown markup.fenced_code.block.markdown
L       [ia32]      Intel x86
T 27 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L [posix]: https://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap12.html#tag_12_02
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 91 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [gnu]: https://www.gnu.org/prep/standards/standards.html#Command_002dLine-Interfaces
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 4 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 5 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 7 text.html.markdown meta.link.reference.def.markdown
T 84 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ArgParser]: https://pub.dev/documentation/args/latest/args/ArgParser/ArgParser.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 84 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ArgParserException]: https://pub.dev/documentation/args/latest/args/ArgParserException-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 19 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 21 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 22 text.html.markdown meta.link.reference.def.markdown
T 98 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ArgResults]: https://pub.dev/documentation/args/latest/args/ArgResults-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 82 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ArgumentError]: https://api.dart.dev/dart-core/ArgumentError-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 72 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [CommandRunner]: https://pub.dev/documentation/args/latest/command_runner/CommandRunner-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 98 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Command]: https://pub.dev/documentation/args/latest/command_runner/Command-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 86 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [UsageException]: https://pub.dev/documentation/args/latest/command_runner/UsageException-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 15 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 18 text.html.markdown meta.link.reference.def.markdown
T 100 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [addOption]: https://pub.dev/documentation/args/latest/args/ArgParser/addOption.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 84 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [addFlag]: https://pub.dev/documentation/args/latest/args/ArgParser/addFlag.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 80 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [parse]: https://pub.dev/documentation/args/latest/args/ArgParser/parse.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 76 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [rest]: https://pub.dev/documentation/args/latest/args/ArgResults/rest.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 5 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 8 text.html.markdown meta.link.reference.def.markdown
T 75 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [addCommand]: https://pub.dev/documentation/args/latest/args/ArgParser/addCommand.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 86 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [usage]: https://pub.dev/documentation/args/latest/args/ArgParser/usage.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 76 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [addSubcommand]: https://pub.dev/documentation/args/latest/command_runner/Command/addSubcommand.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 100 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [run]: https://pub.dev/documentation/args/latest/command_runner/CommandRunner/run.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 4 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 5 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 7 text.html.markdown meta.link.reference.def.markdown
T 86 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
