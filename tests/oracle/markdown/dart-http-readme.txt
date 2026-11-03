L [![pub package](https://img.shields.io/pub/v/http.svg)](https://pub.dev/packages/http)
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
L [![package publisher](https://img.shields.io/pub/publisher/http.svg)](https://pub.dev/packages/http/publisher)
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
L A composable, Future-based library for making HTTP requests.
T 60 text.html.markdown meta.paragraph.markdown
L 
L This package contains a set of high-level functions and classes that make it
T 76 text.html.markdown meta.paragraph.markdown
L easy to consume HTTP resources. It's multi-platform (mobile, desktop, and
T 73 text.html.markdown meta.paragraph.markdown
L browser) and supports multiple implementations.
T 47 text.html.markdown meta.paragraph.markdown
L 
L ## Using
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 8 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L The easiest way to use this library is via the top-level functions. They allow
T 78 text.html.markdown meta.paragraph.markdown
L you to make individual HTTP requests with minimal hassle:
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import 'package:http/http.dart' as http;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L 
L var url = Uri.https('example.com', 'whatsit/create');
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L var response = await http.post(url, body: {'name': 'doodle', 'color': 'blue'});
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print('Response status: ${response.statusCode}');
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart variable.parameter.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart variable.parameter.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L print('Response body: ${response.body}');
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart variable.parameter.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart variable.parameter.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart meta.embedded.expression.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L print(await http.read(Uri.https('example.com', 'foobar.txt')));
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!NOTE]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 9 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > Flutter applications may require
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 34 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > [additional configuration](https://docs.flutter.dev/data-and-backend/networking#platform-notes)
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 96 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 97 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L > to make HTTP requests.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 24 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L If you're making multiple requests to the same server, you can keep open a
T 74 text.html.markdown meta.paragraph.markdown
L persistent connection by using a [Client][] rather than making one-off requests.
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L If you do this, make sure to close the client when you're done:
T 63 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L var client = http.Client();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L try {
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   var response = await client.post(
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       Uri.https('example.com', 'whatsit/create'),
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L       body: {'name': 'doodle', 'color': 'blue'});
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   var decodedResponse = jsonDecode(utf8.decode(response.bodyBytes)) as Map;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.cast.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   var uri = Uri.parse(decodedResponse['uri'] as String);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.cast.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   print(await client.get(uri));
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L } finally {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   client.close();
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!TIP]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > For detailed background information and practical usage examples, see:
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 72 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > - [Dart Development: Fetch data from the internet](https://dart.dev/tutorials/server/fetch-data)
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 5 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 51 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 52 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 53 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 97 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 98 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L > - [Flutter Cookbook: Fetch data from the internet](https://docs.flutter.dev/cookbook/networking/fetch-data)
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 5 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 51 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 52 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 53 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 108 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 109 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L > - [The Flutter HTTP example application][flutterhttpexample]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 4 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown
T 5 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 41 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 42 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 43 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 61 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 62 text.html.markdown markup.quote.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L 
L You can also exert more fine-grained control over your requests and responses by
T 80 text.html.markdown meta.paragraph.markdown
L creating [Request][] or [StreamedRequest][] objects yourself and passing them to
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 80 text.html.markdown meta.paragraph.markdown
L [Client.send][].
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 16 text.html.markdown meta.paragraph.markdown
L 
L [Request]: https://pub.dev/documentation/http/latest/http/Request-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 76 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [StreamedRequest]: https://pub.dev/documentation/http/latest/http/StreamedRequest-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 19 text.html.markdown meta.link.reference.def.markdown
T 92 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Client.send]: https://pub.dev/documentation/http/latest/http/Client/send.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 78 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L This package is designed to be composable. This makes it easy for external
T 74 text.html.markdown meta.paragraph.markdown
L libraries to work with one another to add behavior to it. Libraries wishing to
T 78 text.html.markdown meta.paragraph.markdown
L add behavior should create a subclass of [BaseClient][] that wraps another
T 41 text.html.markdown meta.paragraph.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 74 text.html.markdown meta.paragraph.markdown
L [Client][] and adds the desired behavior:
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 41 text.html.markdown meta.paragraph.markdown
L 
L [BaseClient]: https://pub.dev/documentation/http/latest/http/BaseClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 82 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Client]: https://pub.dev/documentation/http/latest/http/Client-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 74 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L class UserAgentClient extends http.BaseClient {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final String userAgent;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   final http.Client _inner;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   UserAgentClient(this.userAgent, this._inner);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart variable.language.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart variable.language.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   @override
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.annotation.dart
L   Future<http.StreamedResponse> send(http.BaseRequest request) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     request.headers['user-agent'] = userAgent;
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     return _inner.send(request);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L 
L   @override
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.annotation.dart
L   void close() => _inner.close();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.closure.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L ## Testing
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 10 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L For better testability, especially in Flutter applications, it's recommended
T 76 text.html.markdown meta.paragraph.markdown
L to use a [Client][] instance rather than the top-level functions like
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 69 text.html.markdown meta.paragraph.markdown
L `http.get()` or `http.post()`. This approach makes it easier to mock HTTP
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L requests in your tests.
T 23 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L // Instead of using static methods (harder to test):
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L // var response = await http.get(Uri.https('example.com', 'data.json'));
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L 
L // Use a Client instance (easier to test):
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L class DataService {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.declaration.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final http.Client client;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   DataService({http.Client? client}) : client = client ?? http.Client();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   Future<String> fetchData() async {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     var response = await client.get(Uri.https('example.com', 'data.json'));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     return response.body;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L In your tests, you can easily mock the HTTP client:
T 51 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import 'package:http/testing.dart';
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L import 'package:test/test.dart';
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L 
L void main() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   test('fetchData returns expected data', () async {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     final mockClient = MockClient((request) async {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       return http.Response('{"key": "value"}', 200);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.numeric.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     });
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L     final dataService = DataService(client: mockClient);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     final result = await dataService.fetchData();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L     expect(result, '{"key": "value"}');
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   });
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!TIP]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > You can also use `package:mockito` to mock `http.Client`.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 19 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 20 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 46 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > For more detailed guidance, see the
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 37 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > [Flutter testing cookbook](https://docs.flutter.dev/cookbook/testing/unit/mocking),
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 28 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 83 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 84 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 85 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > which demonstrates best practices for mocking HTTP requests.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 62 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L 
L ## Retrying requests
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L `package:http/retry.dart` provides a class [`RetryClient`][RetryClient] to wrap
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown meta.paragraph.markdown
T 44 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 45 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 79 text.html.markdown meta.paragraph.markdown
L an underlying [`http.Client`][Client] which transparently retries failing
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown meta.paragraph.markdown
L requests.
T 9 text.html.markdown meta.paragraph.markdown
L 
L [RetryClient]: https://pub.dev/documentation/http/latest/retry/RetryClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 85 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [Client]: https://pub.dev/documentation/http/latest/http/Client-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 74 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import 'package:http/http.dart' as http;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L import 'package:http/retry.dart';
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L 
L Future<void> main() async {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final client = RetryClient(http.Client());
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   try {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     print(await client.read(Uri.http('example.org', '')));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   } finally {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     client.close();
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L By default, this retries any request whose response has status code 503
T 71 text.html.markdown meta.paragraph.markdown
L Temporary Failure up to three retries. It waits 500ms before the first retry,
T 77 text.html.markdown meta.paragraph.markdown
L and increases the delay by 1.5x each time. All of this can be customized using
T 78 text.html.markdown meta.paragraph.markdown
L the [`RetryClient()`][new RetryClient] constructor.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 51 text.html.markdown meta.paragraph.markdown
L 
L [new RetryClient]: https://pub.dev/documentation/http/latest/retry/RetryClient/RetryClient.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 19 text.html.markdown meta.link.reference.def.markdown
T 95 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L ## Aborting requests
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 20 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L Some clients, such as [`BrowserClient`][browserclient], [`IOClient`][ioclient], and
T 22 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 56 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 78 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 83 text.html.markdown meta.paragraph.markdown
L [`RetryClient`][retryclient], support aborting requests before they complete.
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 16 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 77 text.html.markdown meta.paragraph.markdown
L 
L Aborting in this way can only be performed when using [`Client.send`][clientsend] or
T 54 text.html.markdown meta.paragraph.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 84 text.html.markdown meta.paragraph.markdown
L [`BaseRequest.send`][baserequestsend] with an [`Abortable`][abortable] request (such
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 84 text.html.markdown meta.paragraph.markdown
L as [`AbortableRequest`][abortablerequest]).
T 3 text.html.markdown meta.paragraph.markdown
T 4 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 24 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 43 text.html.markdown meta.paragraph.markdown
L 
L To abort a request, complete the [`Abortable.abortTrigger`][aborttrigger] `Future`.
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 72 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 73 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 74 text.html.markdown meta.paragraph.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 83 text.html.markdown meta.paragraph.markdown
L 
L If the request is aborted before the response `Future` completes, then the response
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 83 text.html.markdown meta.paragraph.markdown
L `Future` will complete with [`RequestAbortedException`][requestabortedexception]. If
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 84 text.html.markdown meta.paragraph.markdown
L the response is a `StreamedResponse` and the the request is cancelled while the
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown
L response stream is being consumed, then the response stream will contain a
T 74 text.html.markdown meta.paragraph.markdown
L [`RequestAbortedException`][requestabortedexception].
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 51 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 53 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L import 'dart:async';
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L 
L import 'package:http/http.dart' as http;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart punctuation.terminator.dart
L 
L Future<void> main() async {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final abortTrigger = Completer<void>();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   final client = Client();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   final request = AbortableRequest(
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     'GET',
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     Uri.https('example.com'),
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart string.interpolated.single.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     abortTrigger: abortTrigger.future,
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L   );
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   // Whenever abortion is required:
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   // > abortTrigger.complete();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L 
L   // Send request
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   final StreamedResponse response;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   try {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     response = await client.send(request);
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   } on RequestAbortedException {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // request aborted before it was fully sent
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     rethrow;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L 
L   // Using full response bytes listener
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   response.stream.listen(
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     (data) {
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       // consume response bytes
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     },
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     onError: (Object err) {
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       if (err is RequestAbortedException) {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L         // request aborted whilst response bytes are being streamed;
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L         // the stream will always be finished early
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L       }
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     },
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     onDone: () {
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       // response bytes consumed, or partially consumed if finished
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L       // early due to abortion
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     },
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L   );
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L 
L   // Alternatively, using `asFuture`
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L   try {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     await response.stream.listen(
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       (data) {
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L         // consume response bytes
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L       },
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L     ).asFuture<void>();
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   } on RequestAbortedException {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // request aborted whilst response bytes are being streamed
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L     rethrow;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.catch-exception.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     // response bytes fully consumed
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L [browserclient]: https://pub.dev/documentation/http/latest/browser_client/BrowserClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 98 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ioclient]: https://pub.dev/documentation/http/latest/io_client/IOClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 83 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [retryclient]: https://pub.dev/documentation/http/latest/retry/RetryClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 85 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [clientsend]: https://pub.dev/documentation/http/latest/http/Client/send.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 77 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [baserequestsend]: https://pub.dev/documentation/http/latest/http/BaseRequest/send.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 19 text.html.markdown meta.link.reference.def.markdown
T 87 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [abortable]: https://pub.dev/documentation/http/latest/http/Abortable-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 80 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [abortablerequest]: https://pub.dev/documentation/http/latest/http/AbortableRequest-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 17 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 19 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 20 text.html.markdown meta.link.reference.def.markdown
T 94 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [aborttrigger]: https://pub.dev/documentation/http/latest/http/Abortable/abortTrigger.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 16 text.html.markdown meta.link.reference.def.markdown
T 90 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [requestabortedexception]: https://pub.dev/documentation/http/latest/http/RequestAbortedException-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 24 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 25 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 26 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 27 text.html.markdown meta.link.reference.def.markdown
T 108 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
L 
L ## Choosing an implementation
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 29 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L There are multiple implementations of the `package:http` [`Client`][client] interface. By default, `package:http` uses [`BrowserClient`][browserclient] on the web and [`IOClient`][ioclient] on all other platforms. You can choose a different [`Client`][client] implementation based on the needs of your application.
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 75 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 99 text.html.markdown meta.paragraph.markdown
T 100 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 112 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 113 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 119 text.html.markdown meta.paragraph.markdown
T 120 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 121 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 134 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 135 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 136 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 137 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 150 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 151 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 167 text.html.markdown meta.paragraph.markdown
T 168 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 169 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 177 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 178 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 179 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 180 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 188 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 189 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 241 text.html.markdown meta.paragraph.markdown
T 242 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 243 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 249 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 250 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 251 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 252 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 258 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 259 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 314 text.html.markdown meta.paragraph.markdown
L 
L You can change implementations without changing your application code, except
T 77 text.html.markdown meta.paragraph.markdown
L for a few lines of [configuration](#2-configure-the-http-client).
T 19 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 65 text.html.markdown meta.paragraph.markdown
L 
L Some well-supported implementations are:
T 40 text.html.markdown meta.paragraph.markdown
L 
L | Implementation | Supported Platforms | SDK | Caching | HTTP3/QUIC | Platform Native | 
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 16 text.html.markdown markup.table.markdown
T 17 text.html.markdown markup.table.markdown
T 18 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 19 text.html.markdown markup.table.markdown
T 38 text.html.markdown markup.table.markdown
T 39 text.html.markdown markup.table.markdown
T 40 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 41 text.html.markdown markup.table.markdown
T 44 text.html.markdown markup.table.markdown
T 45 text.html.markdown markup.table.markdown
T 46 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 47 text.html.markdown markup.table.markdown
T 54 text.html.markdown markup.table.markdown
T 55 text.html.markdown markup.table.markdown
T 56 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 57 text.html.markdown markup.table.markdown
T 67 text.html.markdown markup.table.markdown
T 68 text.html.markdown markup.table.markdown
T 69 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 70 text.html.markdown markup.table.markdown
T 85 text.html.markdown markup.table.markdown
T 86 text.html.markdown markup.table.markdown
T 87 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 88 text.html.markdown markup.table.markdown
L | -------------- | ------------------- | ----| ------- | ---------- | --------------- |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 16 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 17 text.html.markdown markup.table.markdown
T 18 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 19 text.html.markdown markup.table.markdown
T 38 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 39 text.html.markdown markup.table.markdown
T 40 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 41 text.html.markdown markup.table.markdown
T 45 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 46 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 47 text.html.markdown markup.table.markdown
T 54 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 55 text.html.markdown markup.table.markdown
T 56 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 57 text.html.markdown markup.table.markdown
T 67 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 68 text.html.markdown markup.table.markdown
T 69 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 70 text.html.markdown markup.table.markdown
T 85 text.html.markdown markup.table.markdown punctuation.separator.table.markdown
T 86 text.html.markdown markup.table.markdown
T 87 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | `package:http` — [`IOClient`][ioclient] | Android, iOS, Linux, macOS, Windows | Dart, Flutter | ❌ | ❌ | ❌ |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown markup.table.markdown
T 22 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 34 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 42 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 43 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 44 text.html.markdown markup.table.markdown
T 45 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 46 text.html.markdown markup.table.markdown
T 81 text.html.markdown markup.table.markdown
T 82 text.html.markdown markup.table.markdown
T 83 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 84 text.html.markdown markup.table.markdown
T 97 text.html.markdown markup.table.markdown
T 98 text.html.markdown markup.table.markdown
T 99 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 100 text.html.markdown markup.table.markdown
T 103 text.html.markdown markup.table.markdown
T 104 text.html.markdown markup.table.markdown
T 105 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 106 text.html.markdown markup.table.markdown
T 109 text.html.markdown markup.table.markdown
T 110 text.html.markdown markup.table.markdown
T 111 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 112 text.html.markdown markup.table.markdown
T 115 text.html.markdown markup.table.markdown
T 116 text.html.markdown markup.table.markdown
T 117 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | `package:http` — [`BrowserClient`][browserclient] | Web | Dart, Flutter | ― | ✅︎ | ✅︎ | Dart, Flutter |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.table.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown markup.table.markdown
T 22 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 39 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 52 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 53 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 54 text.html.markdown markup.table.markdown
T 55 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 56 text.html.markdown markup.table.markdown
T 59 text.html.markdown markup.table.markdown
T 60 text.html.markdown markup.table.markdown
T 61 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 62 text.html.markdown markup.table.markdown
T 75 text.html.markdown markup.table.markdown
T 76 text.html.markdown markup.table.markdown
T 77 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 78 text.html.markdown markup.table.markdown
T 81 text.html.markdown markup.table.markdown
T 82 text.html.markdown markup.table.markdown
T 83 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 84 text.html.markdown markup.table.markdown
T 90 text.html.markdown markup.table.markdown
T 91 text.html.markdown markup.table.markdown
T 92 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 93 text.html.markdown markup.table.markdown
T 99 text.html.markdown markup.table.markdown
T 100 text.html.markdown markup.table.markdown
T 101 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 102 text.html.markdown markup.table.markdown
T 115 text.html.markdown markup.table.markdown
T 116 text.html.markdown markup.table.markdown
T 117 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | [`package:cupertino_http`][cupertinohttp] — [`CupertinoClient`][cupertinoclient] | iOS, macOS | Flutter | ✅︎ | ✅︎ | ✅︎ |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 29 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 42 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 43 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 48 text.html.markdown markup.table.markdown
T 49 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 50 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 68 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 83 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 84 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 85 text.html.markdown markup.table.markdown
T 86 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 87 text.html.markdown markup.table.markdown
T 97 text.html.markdown markup.table.markdown
T 98 text.html.markdown markup.table.markdown
T 99 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 100 text.html.markdown markup.table.markdown
T 107 text.html.markdown markup.table.markdown
T 108 text.html.markdown markup.table.markdown
T 109 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 110 text.html.markdown markup.table.markdown
T 116 text.html.markdown markup.table.markdown
T 117 text.html.markdown markup.table.markdown
T 118 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 119 text.html.markdown markup.table.markdown
T 125 text.html.markdown markup.table.markdown
T 126 text.html.markdown markup.table.markdown
T 127 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 128 text.html.markdown markup.table.markdown
T 134 text.html.markdown markup.table.markdown
T 135 text.html.markdown markup.table.markdown
T 136 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | [`package:cronet_http`][cronethttp] — [`CronetClient`][cronetclient] | Android | Flutter | ✅︎ | ✅︎ | ― |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 26 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 36 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 37 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 42 text.html.markdown markup.table.markdown
T 43 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 44 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 58 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 59 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 71 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 72 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown markup.table.markdown
T 74 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 75 text.html.markdown markup.table.markdown
T 82 text.html.markdown markup.table.markdown
T 83 text.html.markdown markup.table.markdown
T 84 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 85 text.html.markdown markup.table.markdown
T 92 text.html.markdown markup.table.markdown
T 93 text.html.markdown markup.table.markdown
T 94 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 95 text.html.markdown markup.table.markdown
T 101 text.html.markdown markup.table.markdown
T 102 text.html.markdown markup.table.markdown
T 103 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 104 text.html.markdown markup.table.markdown
T 110 text.html.markdown markup.table.markdown
T 111 text.html.markdown markup.table.markdown
T 112 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 113 text.html.markdown markup.table.markdown
T 116 text.html.markdown markup.table.markdown
T 117 text.html.markdown markup.table.markdown
T 118 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L | [`package:fetch_client`][fetch] — [`FetchClient`][fetchclient] | Web | Dart, Flutter | ✅︎ | ✅︎ | ✅︎ |
T 1 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 2 text.html.markdown markup.table.markdown
T 3 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 32 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 33 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 38 text.html.markdown markup.table.markdown
T 39 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 52 text.html.markdown markup.table.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 54 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 65 text.html.markdown markup.table.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 66 text.html.markdown markup.table.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 67 text.html.markdown markup.table.markdown
T 68 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 69 text.html.markdown markup.table.markdown
T 72 text.html.markdown markup.table.markdown
T 73 text.html.markdown markup.table.markdown
T 74 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 75 text.html.markdown markup.table.markdown
T 88 text.html.markdown markup.table.markdown
T 89 text.html.markdown markup.table.markdown
T 90 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 91 text.html.markdown markup.table.markdown
T 97 text.html.markdown markup.table.markdown
T 98 text.html.markdown markup.table.markdown
T 99 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 100 text.html.markdown markup.table.markdown
T 106 text.html.markdown markup.table.markdown
T 107 text.html.markdown markup.table.markdown
T 108 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
T 109 text.html.markdown markup.table.markdown
T 115 text.html.markdown markup.table.markdown
T 116 text.html.markdown markup.table.markdown
T 117 text.html.markdown markup.table.markdown punctuation.definition.table.markdown
L 
L > [!TIP]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > If you are writing a Dart package or Flutter plugin that uses
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 63 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > `package:http`, you should not depend on a particular [`Client`][client]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 15 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 16 text.html.markdown markup.quote.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 57 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 58 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 65 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 67 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 73 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 74 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
L > implementation. Let the application author decide what implementation is
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 74 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > best for their project. You can make that easier by accepting an explicit
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 75 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > [`Client`][client] argument. For example:
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 13 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 19 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 20 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 43 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L >
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
L > ```dart
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 5 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 9 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L > Future<Album> fetchAlbum({Client? client}) async {
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 8 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 9 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 14 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart other.source.dart
T 16 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 27 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 35 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 45 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 50 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 52 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L >   client ??= Client();
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 11 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 13 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 14 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 15 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 23 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L >   ...
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 4 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 5 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 6 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 7 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
L > }
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L > ```
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 5 text.html.markdown markup.quote.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## Configuration
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 16 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 
L To use an HTTP client implementation other than the default, you must:
T 70 text.html.markdown meta.paragraph.markdown
L 1. Add the HTTP client as a dependency.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 39 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 2. Configure the HTTP client.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 29 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 3. Connect the HTTP client to the code that uses it.
T 2 text.html.markdown markup.list.numbered.markdown punctuation.definition.list.begin.markdown
T 3 text.html.markdown markup.list.numbered.markdown
T 52 text.html.markdown markup.list.numbered.markdown meta.paragraph.markdown
L 
L ### 1. Add the HTTP client as a dependency.
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 43 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L To add a package compatible with the Dart SDK to your project, use `dart pub add`.
T 67 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L For example:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```terminal
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L # Replace  "fetch_client" with the package that you want to use.
T 64 text.html.markdown markup.fenced_code.block.markdown
L dart pub add fetch_client
T 25 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To add a package that requires the Flutter SDK, use `flutter pub add`.
T 52 text.html.markdown meta.paragraph.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 70 text.html.markdown meta.paragraph.markdown
L 
L For example:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```terminal
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L # Replace  "cupertino_http" with the package that you want to use.
T 66 text.html.markdown markup.fenced_code.block.markdown
L flutter pub add cupertino_http
T 30 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### 2. Configure the HTTP client.
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 33 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Different `package:http` [`Client`][client] implementations may require
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 43 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 71 text.html.markdown meta.paragraph.markdown
L different configuration options.
T 32 text.html.markdown meta.paragraph.markdown
L 
L Add a function that returns a correctly configured [`Client`][client]. You can
T 51 text.html.markdown meta.paragraph.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 78 text.html.markdown meta.paragraph.markdown
L return a different [`Client`][client] on different platforms.
T 19 text.html.markdown meta.paragraph.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 61 text.html.markdown meta.paragraph.markdown
L 
L For example:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Client httpClient() {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (Platform.isAndroid) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     final engine = CronetEngine.build(
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L         cacheMode: CacheMode.memory,
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
L         cacheMaxSize: 1000000);
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.numeric.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     return CronetClient.fromCronetEngine(engine);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (Platform.isIOS || Platform.isMacOS) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.bitwise.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.bitwise.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     final config = URLSessionConfiguration.ephemeralSessionConfiguration()
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L       ..cache = URLCache.withCapacity(memoryCapacity: 1000000);
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.ternary.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart constant.numeric.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L     return CupertinoClient.fromSessionConfiguration(config);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   return IOClient();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L > [!TIP]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > [The Flutter HTTP example application][flutterhttpexample] demonstrates
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 40 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 59 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 60 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > configuration best practices.
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 31 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L #### Supporting browser and native
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 34 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L If your application can be run in the browser and natively, you must put your
T 77 text.html.markdown meta.paragraph.markdown
L browser and native configurations in separate files and import the correct file
T 79 text.html.markdown meta.paragraph.markdown
L based on the platform.
T 22 text.html.markdown meta.paragraph.markdown
L 
L For example:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L // -- http_client_factory.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L Client httpClient() {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (Platform.isAndroid) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     return CronetClient.defaultCronetEngine();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   if (Platform.isIOS || Platform.isMacOS) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.bitwise.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.bitwise.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L     return CupertinoClient.defaultSessionConfiguration();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   }
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   return IOClient();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.control.return.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L // -- http_client_factory_web.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L Client httpClient() => FetchClient();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.closure.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart support.class.dart
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L // -- main.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart comment.line.double-slash.dart
L import 'http_client_factory.dart'
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.other.import.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
L     if (dart.library.js_interop) 'http_client_factory_web.dart'
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart keyword.control.dart
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart string.interpolated.single.dart
L 
L // The correct `httpClient` will be available.
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart meta.declaration.dart comment.line.double-slash.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### 3. Connect the HTTP client to the code that uses it.
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 56 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The best way to pass [`Client`][client] to the places that use it is
T 21 text.html.markdown meta.paragraph.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 23 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 68 text.html.markdown meta.paragraph.markdown
L explicitly through arguments.
T 29 text.html.markdown meta.paragraph.markdown
L 
L For example:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```dart
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void main() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.type.primitive.dart
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L   final client = httpClient();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart storage.modifier.dart
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart keyword.operator.assignment.dart
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L   fetchAlbum(client, ...);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart entity.name.function.dart
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.comma.dart
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.dot.dart
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart punctuation.terminator.dart
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.dart
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L When using the Flutter SDK, you can use a one of many
T 53 text.html.markdown meta.paragraph.markdown
L [state management approaches][flutterstatemanagement].
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 28 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 54 text.html.markdown meta.paragraph.markdown
L 
L > [!TIP]
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 7 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 8 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L > [The Flutter HTTP example application][flutterhttpexample] demonstrates
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 40 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 41 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 59 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 60 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > how to make the configured [`Client`][client] available using
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 29 text.html.markdown markup.quote.markdown meta.paragraph.markdown
T 30 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 31 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 40 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 46 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 47 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 63 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > [`package:provider`][provider] and
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 22 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 23 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 31 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 32 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 36 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L > [`package:http_image_provider`][http_image_provider].
T 1 text.html.markdown markup.quote.markdown punctuation.definition.quote.begin.markdown
T 2 text.html.markdown markup.quote.markdown
T 3 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 4 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 34 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 53 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 54 text.html.markdown markup.quote.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 55 text.html.markdown markup.quote.markdown meta.paragraph.markdown
L 
L When using the Dart SDK, you can use [`runWithClient`][runwithclient] to
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 39 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 72 text.html.markdown meta.paragraph.markdown
L ensure that the correct [`Client`][client] is used when explicit argument
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 73 text.html.markdown meta.paragraph.markdown
L passing is not an option. For example, if you depend on code that uses
T 70 text.html.markdown meta.paragraph.markdown
L top-level functions (e.g. `http.post`) or calls the
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown
L [`Client()`][clientconstructor] constructor. When an [Isolate][isolate] is
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 53 text.html.markdown meta.paragraph.markdown
T 54 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 61 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 70 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 74 text.html.markdown meta.paragraph.markdown
L spawned, it does not inherit any variables from the calling Zone, so
T 68 text.html.markdown meta.paragraph.markdown
L `runWithClient` needs to be used in each Isolate that uses `package:http`.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L 
L You can ensure that only the `Client` that you have explicitly configured is
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L used by defining `no_default_http_client=true` in the environment. This will
T 17 text.html.markdown meta.paragraph.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
L also allow the default `Client` implementation to be removed, resulting in
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 74 text.html.markdown meta.paragraph.markdown
L a reduced application size.
T 27 text.html.markdown meta.paragraph.markdown
L 
L ```terminal
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L $ flutter build appbundle --dart-define=no_default_http_client=true ...
T 71 text.html.markdown markup.fenced_code.block.markdown
L $ dart compile exe --define=no_default_http_client=true ...
T 59 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L [browserclient]: https://pub.dev/documentation/http/latest/browser_client/BrowserClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 98 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [client]: https://pub.dev/documentation/http/latest/http/Client-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 7 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 10 text.html.markdown meta.link.reference.def.markdown
T 74 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [clientconstructor]: https://pub.dev/documentation/http/latest/http/Client/Client.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 19 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 21 text.html.markdown meta.link.reference.def.markdown
T 86 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [cupertinohttp]: https://pub.dev/packages/cupertino_http
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 56 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [cupertinoclient]: https://pub.dev/documentation/cupertino_http/latest/cupertino_http/CupertinoClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 17 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 19 text.html.markdown meta.link.reference.def.markdown
T 112 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [cronethttp]: https://pub.dev/packages/cronet_http
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 50 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [cronetclient]: https://pub.dev/documentation/cronet_http/latest/cronet_http/CronetClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 16 text.html.markdown meta.link.reference.def.markdown
T 100 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [fetch]: https://pub.dev/packages/fetch_client
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 46 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [fetchclient]: https://pub.dev/documentation/fetch_client/latest/fetch_client/FetchClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 100 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [flutterhttpexample]: https://github.com/dart-lang/http/tree/master/pkgs/flutter_http_example
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 19 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 21 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 22 text.html.markdown meta.link.reference.def.markdown
T 93 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [http_image_provider]: https://pub.dev/documentation/http_image_provider
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 20 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 21 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 22 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 23 text.html.markdown meta.link.reference.def.markdown
T 72 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [ioclient]: https://pub.dev/documentation/http/latest/io_client/IOClient-class.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 83 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [isolate]: https://dart.dev/language/concurrency#how-isolates-work
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 9 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 11 text.html.markdown meta.link.reference.def.markdown
T 66 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [flutterstatemanagement]: https://docs.flutter.dev/data-and-backend/state-mgmt/options
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 23 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 24 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 25 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 26 text.html.markdown meta.link.reference.def.markdown
T 86 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [provider]: https://pub.dev/packages/provider
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 45 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L [runwithclient]: https://pub.dev/documentation/http/latest/http/runWithClient.html
T 1 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 15 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 16 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 17 text.html.markdown meta.link.reference.def.markdown
T 82 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
