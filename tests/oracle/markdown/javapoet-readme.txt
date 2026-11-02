L JavaPoet
T 8 text.html.markdown meta.paragraph.markdown
L ========
T 8 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L `JavaPoet` is a Java API for generating `.java` source files.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown
L 
L Source file generation can be useful when doing things such as annotation processing or interacting
T 99 text.html.markdown meta.paragraph.markdown
L with metadata files (e.g., database schemas, protocol formats). By generating code, you eliminate
T 97 text.html.markdown meta.paragraph.markdown
L the need to write boilerplate while also keeping a single source of truth for the metadata.
T 91 text.html.markdown meta.paragraph.markdown
L 
L 
L Deprecated
T 10 text.html.markdown meta.paragraph.markdown
L ----------
T 10 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L As of 2020-10-10, Square's JavaPoet project is deprecated. We're proud of our work but we haven't
T 97 text.html.markdown meta.paragraph.markdown
L kept up on maintaining it.
T 26 text.html.markdown meta.paragraph.markdown
L 
L If you'd like an alternative that supports the latest Java language features, one option is
T 91 text.html.markdown meta.paragraph.markdown
L [Palantir's JavaPoet](https://github.com/palantir/javapoet).
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 58 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 60 text.html.markdown meta.paragraph.markdown
L 
L To switch to that project, you'll need new Maven coordinates:
T 61 text.html.markdown meta.paragraph.markdown
L 
L ```diff
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L - javapoet = { module = "com.squareup:javapoet", version = "1.13.0" }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.deleted.diff punctuation.definition.deleted.diff
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.deleted.diff
L + javapoet = { module = "com.palantir.javapoet:javapoet", version = "0.5.0" }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.inserted.diff punctuation.definition.inserted.diff
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.inserted.diff
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And new imports:
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L sed -i "" \
T 11 text.html.markdown markup.fenced_code.block.markdown
L   's/com.squareup.javapoet.\([A-Za-z]*\)/com.palantir.javapoet.\1/g' \
T 70 text.html.markdown markup.fenced_code.block.markdown
L   `find . -name "*.kt" -or -name "*.java"`
T 42 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And you might need to track some API changes where fields became functions:
T 75 text.html.markdown meta.paragraph.markdown
L 
L ```diff
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L - javaFile.packageName
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.deleted.diff punctuation.definition.deleted.diff
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.deleted.diff
L + javaFile.packageName()
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.inserted.diff punctuation.definition.inserted.diff
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.diff markup.inserted.diff
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Example
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Here's a (boring) `HelloWorld` class:
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L package com.example.helloworld;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java keyword.other.package.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java punctuation.terminator.java
L 
L public final class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   public static void main(String[] args) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java storage.modifier.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.primitive.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java storage.type.object.array.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.bracket.square.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.bracket.square.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java variable.parameter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     System.out.println("Hello, JavaPoet!");
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.property.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java string.quoted.double.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And this is the (exciting) code to generate it with JavaPoet:
T 61 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec main = MethodSpec.methodBuilder("main")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.STATIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(void.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(String[].class, "args")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java storage.type.object.array.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.square.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.square.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T.out.println($S)", System.class, "Hello, JavaPoet!")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(main)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L JavaFile javaFile = JavaFile.builder("com.example.helloworld", helloWorld)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L javaFile.writeTo(System.out);
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To declare the main method, we've created a `MethodSpec` "main" configured with modifiers, return
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown meta.paragraph.markdown
L type, parameters and code statements. We add the main method to a `HelloWorld` class, and then add
T 66 text.html.markdown meta.paragraph.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L that to a `HelloWorld.java` file.
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown
L 
L In this case we write the file to `System.out`, but we could also get it as a string
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown meta.paragraph.markdown
L (`JavaFile.toString()`) or write it to the file system (`JavaFile.writeTo()`).
T 1 text.html.markdown meta.paragraph.markdown
T 2 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown
L 
L The [Javadoc][javadoc] catalogs the complete JavaPoet API, which we explore below.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 13 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 14 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L ### Code & Control Flow
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 9 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown meta.other.valid-ampersand.markdown
T 23 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Most of JavaPoet's API uses plain old immutable Java objects. There's also builders, method chaining
T 100 text.html.markdown meta.paragraph.markdown
L and varargs to make the API friendly. JavaPoet offers models for classes & interfaces (`TypeSpec`),
T 73 text.html.markdown meta.paragraph.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 87 text.html.markdown meta.paragraph.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 97 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 99 text.html.markdown meta.paragraph.markdown
L fields (`FieldSpec`), methods & constructors (`MethodSpec`), parameters (`ParameterSpec`) and
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown meta.other.valid-ampersand.markdown
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 93 text.html.markdown meta.paragraph.markdown
L annotations (`AnnotationSpec`).
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown
L 
L But the _body_ of methods and constructors is not modeled. There's no expression class, no
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 13 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 90 text.html.markdown meta.paragraph.markdown
L statement class or syntax tree nodes. Instead, JavaPoet uses strings for code blocks:
T 85 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec main = MethodSpec.methodBuilder("main")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addCode(""
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "int total = 0;\n"
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "for (int i = 0; i < 10; i++) {\n"
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "  total += i;\n"
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "}\n")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void main() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   int total = 0;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   for (int i = 0; i < 10; i++) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.comparison.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.increment-decrement.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     total += i;
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.arithmetic.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The manual semicolons, line wrapping, and indentation are tedious and so JavaPoet offers APIs to
T 96 text.html.markdown meta.paragraph.markdown
L make it easier. There's `addStatement()` which takes care of semicolons and newline, and
T 24 text.html.markdown meta.paragraph.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown meta.paragraph.markdown
L `beginControlFlow()` + `endControlFlow()` which are used together for braces, newlines, and
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown meta.paragraph.markdown
L indentation:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec main = MethodSpec.methodBuilder("main")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("int total = 0")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .beginControlFlow("for (int i = 0; i < 10; i++)")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("total += i")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .endControlFlow()
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This example is lame because the generated code is constant! Suppose instead of just adding 0 to 10,
T 100 text.html.markdown meta.paragraph.markdown
L we want to make the operation and range configurable. Here's a method that generates a method:
T 94 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L private MethodSpec computeRange(String name, int from, int to, String op) {
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   return MethodSpec.methodBuilder(name)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .returns(int.class)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("int result = 1")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .beginControlFlow("for (int i = " + from + "; i < " + to + "; i++)")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("result = result " + op + " i")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .endControlFlow()
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("return result")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .build();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And here's what we get when we call `computeRange("multiply10to20", 10, 20, "*")`:
T 36 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L int multiply10to20() {
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   int result = 1;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   for (int i = 10; i < 20; i++) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.comparison.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.increment-decrement.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     result = result * i;
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.arithmetic.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L   return result;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Methods generating methods! And since JavaPoet generates source instead of bytecode, you can
T 92 text.html.markdown meta.paragraph.markdown
L read through it to make sure it's right.
T 40 text.html.markdown meta.paragraph.markdown
L 
L Some control flow statements, such as `if/else`, can have unlimited control flow possibilities.
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown
L You can handle those options using `nextControlFlow()`:
T 35 text.html.markdown meta.paragraph.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec main = MethodSpec.methodBuilder("main")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("long now = $T.currentTimeMillis()", System.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .beginControlFlow("if ($T.currentTimeMillis() < now)", System.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T.out.println($S)", System.class, "Time travelling, woo hoo!")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .nextControlFlow("else if ($T.currentTimeMillis() == now)", System.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T.out.println($S)", System.class, "Time stood still!")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .nextControlFlow("else")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T.out.println($S)", System.class, "Ok, time still moving forward")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 84 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 86 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .endControlFlow()
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates:
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void main() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   long now = System.currentTimeMillis();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   if (System.currentTimeMillis() < now)  {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.comparison.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     System.out.println("Time travelling, woo hoo!");
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.property.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   } else if (System.currentTimeMillis() == now) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.comparison.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     System.out.println("Time stood still!");
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.property.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   } else {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     System.out.println("Ok, time still moving forward");
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.property.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ``` 
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 4 text.html.markdown markup.fenced_code.block.markdown
L 
L Catching exceptions using `try/catch` is also a use case for `nextControlFlow()`:
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 61 text.html.markdown meta.paragraph.markdown
T 62 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 80 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec main = MethodSpec.methodBuilder("main")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .beginControlFlow("try")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("throw new Exception($S)", "Failed")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .nextControlFlow("catch ($T e)", Exception.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("throw new $T(e)", RuntimeException.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .endControlFlow()
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which produces:
T 15 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void main() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   try {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java keyword.control.try.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java punctuation.section.try.begin.bracket.curly.java
L     throw new Exception("Failed");
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java keyword.control.throw.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java keyword.control.new.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java punctuation.terminator.java
L   } catch (Exception e) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java meta.try.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.try.java punctuation.section.try.end.bracket.curly.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java keyword.control.catch.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.parameters.java storage.type.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.parameters.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.parameters.java variable.parameter.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java punctuation.definition.parameters.end.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java punctuation.section.catch.begin.bracket.curly.java
L     throw new RuntimeException(e);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java keyword.control.throw.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java keyword.control.new.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java meta.function-call.java entity.name.function.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java meta.function-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java meta.catch.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.catch.java punctuation.section.catch.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### $L for Literals
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 19 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L The string-concatenation in calls to `beginControlFlow()` and `addStatement` is distracting. Too
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 75 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown meta.paragraph.markdown
L many operators. To address this, JavaPoet offers a syntax inspired-by but incompatible-with
T 91 text.html.markdown meta.paragraph.markdown
L [`String.format()`][formatter]. It accepts **`$L`** to emit a **literal** value in the output. This
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 17 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 18 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 19 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 43 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 62 text.html.markdown meta.paragraph.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 71 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 73 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 99 text.html.markdown meta.paragraph.markdown
L works just like `Formatter`'s `%s`:
T 16 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L private MethodSpec computeRange(String name, int from, int to, String op) {
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   return MethodSpec.methodBuilder(name)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .returns(int.class)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("int result = 0")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .beginControlFlow("for (int i = $L; i < $L; i++)", from, to)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("result = result $L i", op)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .endControlFlow()
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("return result")
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .build();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Literals are emitted directly to the output code with no escaping. Arguments for literals may be
T 96 text.html.markdown meta.paragraph.markdown
L strings, primitives, and a few JavaPoet types described below.
T 62 text.html.markdown meta.paragraph.markdown
L 
L ### $S for Strings
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 18 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L When emitting code that includes string literals, we can use **`$S`** to emit a **string**, complete
T 61 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 80 text.html.markdown meta.paragraph.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 90 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 100 text.html.markdown meta.paragraph.markdown
L with wrapping quotation marks and escaping. Here's a program that emits 3 methods, each of which
T 96 text.html.markdown meta.paragraph.markdown
L returns its own name:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public static void main(String[] args) throws Exception {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.object.array.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.bracket.square.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.bracket.square.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addModifiers(Modifier.PUBLIC, Modifier.FINAL)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addMethod(whatsMyName("slimShady"))
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java entity.name.function.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.end.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addMethod(whatsMyName("eminem"))
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java entity.name.function.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.end.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addMethod(whatsMyName("marshallMathers"))
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java entity.name.function.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .build();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L   JavaFile javaFile = JavaFile.builder("com.example.helloworld", helloWorld)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .build();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L   javaFile.writeTo(System.out);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L 
L private static MethodSpec whatsMyName(String name) {
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   return MethodSpec.methodBuilder(name)
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .returns(String.class)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .addStatement("return $S", name)
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L       .build();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L In this case, using `$S` gives us quotation marks:
T 20 text.html.markdown meta.paragraph.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public final class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   String slimShady() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     return "slimShady";
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.begin.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.end.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L 
L   String eminem() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     return "eminem";
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.begin.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.end.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L 
L   String marshallMathers() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     return "marshallMathers";
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.begin.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.end.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### $T for Types
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L We Java programmers love our types: they make our code easier to understand. And JavaPoet is on
T 95 text.html.markdown meta.paragraph.markdown
L board. It has rich built-in support for types, including automatic generation of `import`
T 81 text.html.markdown meta.paragraph.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 89 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L statements. Just use **`$T`** to reference **types**:
T 21 text.html.markdown meta.paragraph.markdown
T 23 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 43 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 50 text.html.markdown meta.paragraph.markdown markup.bold.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 53 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec today = MethodSpec.methodBuilder("today")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(Date.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return new $T()", Date.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(today)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L JavaFile javaFile = JavaFile.builder("com.example.helloworld", helloWorld)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L javaFile.writeTo(System.out);
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L That generates the following `.java` file, complete with the necessary `import`:
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 71 text.html.markdown meta.paragraph.markdown
T 72 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 78 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 79 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 80 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L package com.example.helloworld;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java keyword.other.package.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java punctuation.terminator.java
L 
L import java.util.Date;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L 
L public final class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   Date today() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     return new Date();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.new.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L We passed `Date.class` to reference a class that just-so-happens to be available when we're
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 91 text.html.markdown meta.paragraph.markdown
L generating code. This doesn't need to be the case. Here's a similar example, but this one
T 89 text.html.markdown meta.paragraph.markdown
L references a class that doesn't exist (yet):
T 44 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ClassName hoverboard = ClassName.get("com.mattel", "Hoverboard");
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L MethodSpec today = MethodSpec.methodBuilder("tomorrow")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return new $T()", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L And that not-yet-existent class is imported as well:
T 52 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L package com.example.helloworld;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java keyword.other.package.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java punctuation.terminator.java
L 
L import com.mattel.Hoverboard;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L 
L public final class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   Hoverboard tomorrow() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     return new Hoverboard();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.new.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The `ClassName` type is very important, and you'll need it frequently when you're using JavaPoet.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown meta.paragraph.markdown
L It can identify any _declared_ class. Declared types are just the beginning of Java's rich type
T 20 text.html.markdown meta.paragraph.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 95 text.html.markdown meta.paragraph.markdown
L system: we also have arrays, parameterized types, wildcard types, and type variables. JavaPoet has
T 98 text.html.markdown meta.paragraph.markdown
L classes for building each of these:
T 35 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ClassName hoverboard = ClassName.get("com.mattel", "Hoverboard");
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ClassName list = ClassName.get("java.util", "List");
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ClassName arrayList = ClassName.get("java.util", "ArrayList");
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L TypeName listOfHoverboards = ParameterizedTypeName.get(list, hoverboard);
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L MethodSpec beyond = MethodSpec.methodBuilder("beyond")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(listOfHoverboards)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T result = new $T<>()", listOfHoverboards, arrayList)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add(new $T())", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add(new $T())", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add(new $T())", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return result")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L JavaPoet will decompose each type and import its components where possible.
T 75 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L package com.example.helloworld;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java keyword.other.package.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java punctuation.terminator.java
L 
L import com.mattel.Hoverboard;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import java.util.ArrayList;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import java.util.List;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L 
L public final class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   List<Hoverboard> beyond() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java punctuation.bracket.angle.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.generic.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java punctuation.bracket.angle.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     List<Hoverboard> result = new ArrayList<>();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java punctuation.bracket.angle.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java storage.type.generic.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java punctuation.bracket.angle.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java variable.other.definition.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.operator.assignment.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.new.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java storage.type.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.angle.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.angle.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.round.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(new Hoverboard());
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java keyword.control.new.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(new Hoverboard());
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java keyword.control.new.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(new Hoverboard());
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java keyword.control.new.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     return result;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Import static
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 18 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L JavaPoet supports `import static`. It does it via explicitly collecting type member names. Let's
T 18 text.html.markdown meta.paragraph.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 96 text.html.markdown meta.paragraph.markdown
L enhance the previous example with some static sugar:
T 52 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ...
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.separator.period.java
L ClassName namedBoards = ClassName.get("com.mattel", "Hoverboard", "Boards");
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L MethodSpec beyond = MethodSpec.methodBuilder("beyond")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(listOfHoverboards)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T result = new $T<>()", listOfHoverboards, arrayList)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add($T.createNimbus(2000))", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add($T.createNimbus(\"2001\"))", hoverboard)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result.add($T.createNimbus($T.THUNDERBOLT))", hoverboard, namedBoards)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("$T.sort(result)", Collections.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return result.isEmpty() ? $T.emptyList() : result", Collections.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 88 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 89 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec hello = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(beyond)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L JavaFile.builder("com.example.helloworld", hello)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStaticImport(hoverboard, "createNimbus")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStaticImport(namedBoards, "*")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStaticImport(Collections.class, "*")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L JavaPoet will first add your `import static` block to the file as configured, match and mangle
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown
L all calls accordingly and also import all other types as needed.
T 64 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L package com.example.helloworld;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java keyword.other.package.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java punctuation.separator.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java storage.modifier.package.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.package.java punctuation.terminator.java
L 
L import static com.mattel.Hoverboard.Boards.*;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java variable.language.wildcard.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import static com.mattel.Hoverboard.createNimbus;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import static java.util.Collections.*;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java variable.language.wildcard.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L 
L import com.mattel.Hoverboard;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import java.util.ArrayList;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L import java.util.List;
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java keyword.other.import.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java punctuation.separator.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java storage.modifier.import.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.import.java punctuation.terminator.java
L 
L class HelloWorld {
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   List<Hoverboard> beyond() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java punctuation.bracket.angle.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.generic.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java punctuation.bracket.angle.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     List<Hoverboard> result = new ArrayList<>();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java punctuation.bracket.angle.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java storage.type.generic.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java punctuation.bracket.angle.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java variable.other.definition.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.definition.variable.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.operator.assignment.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.new.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java storage.type.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.angle.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.angle.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.round.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(createNimbus(2000));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java constant.numeric.decimal.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(createNimbus("2001"));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java string.quoted.double.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java string.quoted.double.java punctuation.definition.string.end.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     result.add(createNimbus(THUNDERBOLT));
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java entity.name.function.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     sort(result);
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java entity.name.function.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L     return result.isEmpty() ? emptyList() : result;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.ternary.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java entity.name.function.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.control.ternary.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### $N for Names
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Generated code is often self-referential. Use **`$N`** to refer to another generated declaration by
T 46 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown
T 52 text.html.markdown meta.paragraph.markdown markup.bold.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.bold.markdown punctuation.definition.bold.markdown
T 99 text.html.markdown meta.paragraph.markdown
L its name. Here's a method that calls another:
T 45 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public String byteToHex(int b) {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   char[] result = new char[2];
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.primitive.array.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java punctuation.bracket.square.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java punctuation.bracket.square.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.new.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.array.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   result[0] = hexDigit((b >>> 4) & 0xf);
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java keyword.operator.bitwise.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java constant.numeric.decimal.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java keyword.operator.bitwise.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java constant.numeric.hex.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   result[1] = hexDigit(b & 0xf);
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.square.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java keyword.operator.bitwise.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java constant.numeric.hex.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   return new String(result);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.new.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L 
L public char hexDigit(int i) {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.primitive.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   return (char) (i < 10 ? i + '0' : i - 10 + 'a');
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.comparison.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.ternary.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.arithmetic.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java punctuation.definition.string.begin.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java punctuation.definition.string.end.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.ternary.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.arithmetic.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.arithmetic.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java punctuation.definition.string.begin.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.single.java punctuation.definition.string.end.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L When generating the code above, we pass the `hexDigit()` method as an argument to the `byteToHex()`
T 44 text.html.markdown meta.paragraph.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 56 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 86 text.html.markdown meta.paragraph.markdown
T 87 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 99 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L method using `$N`:
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec hexDigit = MethodSpec.methodBuilder("hexDigit")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(int.class, "i")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(char.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return (char) (i < 10 ? i + '0' : i - 10 + 'a')")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L MethodSpec byteToHex = MethodSpec.methodBuilder("byteToHex")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(int.class, "b")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(String.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("char[] result = new char[2]")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result[0] = $N((b >>> 4) & 0xf)", hexDigit)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("result[1] = $N(b & 0xf)", hexDigit)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return new String(result)")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Code block format strings
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 29 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Code blocks may specify the values for their placeholders in a few ways. Only one style may be used
T 99 text.html.markdown meta.paragraph.markdown
L for each operation on a code block.
T 35 text.html.markdown meta.paragraph.markdown
L 
L #### Relative Arguments
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 23 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Pass an argument value for each placeholder in the format string to `CodeBlock.add()`. In each
T 68 text.html.markdown meta.paragraph.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 84 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 85 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 94 text.html.markdown meta.paragraph.markdown
L example, we generate code to say "I ate 3 tacos"
T 48 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CodeBlock.builder().add("I ate $L $L", 3, "tacos")
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java constant.numeric.decimal.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Positional Arguments
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 25 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Place an integer index (1-based) before the placeholder in the format string to specify which
T 93 text.html.markdown meta.paragraph.markdown
L  argument to use.
T 1 text.html.markdown meta.paragraph.markdown
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L CodeBlock.builder().add("I ate $2L $1L", "tacos", 3)
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java constant.numeric.decimal.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L #### Named Arguments
T 4 text.html.markdown markup.heading.markdown heading.4.markdown punctuation.definition.heading.markdown
T 5 text.html.markdown markup.heading.markdown heading.4.markdown
T 20 text.html.markdown markup.heading.markdown heading.4.markdown entity.name.section.markdown
L 
L Use the syntax `$argumentName:X` where `X` is the format character and call `CodeBlock.addNamed()`
T 15 text.html.markdown meta.paragraph.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 97 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 98 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L with a map containing all argument keys in the format string. Argument names use characters in
T 94 text.html.markdown meta.paragraph.markdown
L `a-z`, `A-Z`, `0-9`, and `_`, and must start with a lowercase character.
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 4 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 7 text.html.markdown meta.paragraph.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 18 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 19 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 25 text.html.markdown meta.paragraph.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 28 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L Map<String, Object> map = new LinkedHashMap<>();
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java punctuation.bracket.angle.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.generic.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java punctuation.separator.delimiter.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.generic.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java punctuation.bracket.angle.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.new.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.angle.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.angle.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L map.put("food", "tacos");
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L map.put("count", 3);
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java constant.numeric.decimal.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L CodeBlock.builder().addNamed("I ate $count:L $food:L", map)
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Methods
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L All of the above methods have a code body. Use `Modifiers.ABSTRACT` to get a method without any
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown
L body. This is only legal if the enclosing class is either abstract or an interface.
T 83 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec flux = MethodSpec.methodBuilder("flux")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.ABSTRACT, Modifier.PROTECTED)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.ABSTRACT)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(flux)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public abstract class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   protected abstract void flux();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java storage.modifier.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java storage.modifier.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.primitive.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The other modifiers work where permitted. Note that when specifying modifiers, JavaPoet uses
T 92 text.html.markdown meta.paragraph.markdown
L [`javax.lang.model.element.Modifier`][modifier], a class that is not available on Android. This
T 1 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 2 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 38 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 95 text.html.markdown meta.paragraph.markdown
L limitation applies to code-generating-code only; the output code runs everywhere: JVMs, Android,
T 96 text.html.markdown meta.paragraph.markdown
L and GWT.
T 8 text.html.markdown meta.paragraph.markdown
L 
L Methods also have parameters, exceptions, varargs, Javadoc, annotations, type variables, and a
T 94 text.html.markdown meta.paragraph.markdown
L return type. All of these are configured with `MethodSpec.Builder`.
T 46 text.html.markdown meta.paragraph.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown
L 
L ### Constructors
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L `MethodSpec` is a slight misnomer; it can also be used for constructors:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 72 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec flux = MethodSpec.constructorBuilder()
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(String.class, "greeting")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("this.$N = $N", "greeting", "greeting")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addField(String.class, "greeting", Modifier.PRIVATE, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(flux)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   private final String greeting;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java storage.type.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java variable.other.definition.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L 
L   public HelloWorld(String greeting) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java storage.modifier.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java storage.type.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java variable.parameter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     this.greeting = greeting;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.language.this.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java variable.other.object.property.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java keyword.operator.assignment.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L For the most part, constructors work just like methods. When emitting code, JavaPoet will place
T 95 text.html.markdown meta.paragraph.markdown
L constructors before methods in the output file.
T 47 text.html.markdown meta.paragraph.markdown
L 
L ### Parameters
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 14 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Declare parameters on methods and constructors with either `ParameterSpec.builder()` or
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 83 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 84 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
L `MethodSpec`'s convenient `addParameter()` API:
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 12 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 41 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L ParameterSpec android = ParameterSpec.builder(String.class, "android")
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L MethodSpec welcomeOverlords = MethodSpec.methodBuilder("welcomeOverlords")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(android)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(String.class, "robot", Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Though the code above to generate `android` and `robot` parameters is different, the output is the
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 42 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 55 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L same:
T 5 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void welcomeOverlords(final String android, final String robot) {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.modifier.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.separator.delimiter.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.modifier.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The extended `Builder` form is necessary when the parameter has annotations (such as `@Nullable`).
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 85 text.html.markdown meta.paragraph.markdown
T 86 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 95 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 96 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L 
L ### Fields
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 10 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Like parameters, fields can be created either with builders or by using convenient helper methods:
T 98 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L FieldSpec android = FieldSpec.builder(String.class, "android")
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PRIVATE, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addField(android)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addField(String.class, "robot", Modifier.PRIVATE, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates:
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public class HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   private final String android;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java storage.type.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java variable.other.definition.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L 
L   private final String robot;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java storage.type.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java variable.other.definition.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L The extended `Builder` form is necessary when a field has Javadoc, annotations, or a field
T 13 text.html.markdown meta.paragraph.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 22 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 90 text.html.markdown meta.paragraph.markdown
L initializer. Field initializers use the same [`String.format()`][formatter]-like syntax as the code
T 45 text.html.markdown meta.paragraph.markdown
T 46 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 63 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 65 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 74 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 75 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 99 text.html.markdown meta.paragraph.markdown
L blocks above:
T 13 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L FieldSpec android = FieldSpec.builder(String.class, "android")
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PRIVATE, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .initializer("$S + $L", "Lollipop v.", 5.0d)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java constant.numeric.decimal.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates:
T 16 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L private final String android = "Lollipop v." + 5.0;
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java punctuation.definition.string.begin.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java punctuation.definition.string.end.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.arithmetic.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java constant.numeric.decimal.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Interfaces
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 14 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L JavaPoet has no trouble with interfaces. Note that interface methods must always be `PUBLIC
T 91 text.html.markdown meta.paragraph.markdown
L ABSTRACT` and interface fields must always be `PUBLIC STATIC FINAL`. These modifiers are necessary
T 8 text.html.markdown meta.paragraph.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L when defining the interface:
T 28 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L TypeSpec helloWorld = TypeSpec.interfaceBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addField(FieldSpec.builder(String.class, "ONLY_THING_THAT_IS_CONSTANT")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addModifiers(Modifier.PUBLIC, Modifier.STATIC, Modifier.FINAL)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .initializer("$S", "change")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(MethodSpec.methodBuilder("beep")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addModifiers(Modifier.PUBLIC, Modifier.ABSTRACT)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L But these modifiers are omitted when the code is generated. These are the defaults so we don't need
T 99 text.html.markdown meta.paragraph.markdown
L to include them for `javac`'s benefit!
T 20 text.html.markdown meta.paragraph.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public interface HelloWorld {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java storage.modifier.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.identifier.java entity.name.type.class.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.begin.bracket.curly.java
L   String ONLY_THING_THAT_IS_CONSTANT = "change";
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java variable.other.definition.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.definition.variable.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java keyword.operator.assignment.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java string.quoted.double.java punctuation.definition.string.begin.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java string.quoted.double.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java string.quoted.double.java punctuation.definition.string.end.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L 
L   void beep();
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.return-type.java storage.type.primitive.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java entity.name.function.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java meta.class.body.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.class.java punctuation.section.class.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Enums
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 9 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Use `enumBuilder` to create the enum type, and `addEnumConstant()` for each value:
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 16 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 17 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 66 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 82 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L TypeSpec helloWorld = TypeSpec.enumBuilder("Roshambo")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("ROCK")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("SCISSORS")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("PAPER")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L To generate this:
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public enum Roshambo {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java entity.name.type.enum.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.section.enum.begin.bracket.curly.java
L   ROCK,
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.separator.delimiter.java
L 
L   SCISSORS,
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.separator.delimiter.java
L 
L   PAPER
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.section.enum.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Fancy enums are supported, where the enum values override methods or call a superclass constructor.
T 99 text.html.markdown meta.paragraph.markdown
L Here's a comprehensive example:
T 31 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L TypeSpec helloWorld = TypeSpec.enumBuilder("Roshambo")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("ROCK", TypeSpec.anonymousClassBuilder("$S", "fist")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addMethod(MethodSpec.methodBuilder("toString")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addAnnotation(Override.class)
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addModifiers(Modifier.PUBLIC)
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addStatement("return $S", "avalanche!")
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .returns(String.class)
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .build())
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("SCISSORS", TypeSpec.anonymousClassBuilder("$S", "peace")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 77 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 78 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addEnumConstant("PAPER", TypeSpec.anonymousClassBuilder("$S", "flat")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addField(String.class, "handsign", Modifier.PRIVATE, Modifier.FINAL)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(MethodSpec.constructorBuilder()
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addParameter(String.class, "handsign")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addStatement("this.$N = $N", "handsign", "handsign")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L public enum Roshambo {
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java entity.name.type.enum.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.section.enum.begin.bracket.curly.java
L   ROCK("fist") {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.begin.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.end.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.curly.java
L     @Override
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.declaration.annotation.java storage.type.annotation.java
L     public String toString() {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java storage.modifier.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.return-type.java storage.type.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java entity.name.function.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L       return "avalanche!";
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java keyword.control.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.begin.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java string.quoted.double.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java string.quoted.double.java punctuation.definition.string.end.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java punctuation.terminator.java
L     }
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java punctuation.section.method.end.bracket.curly.java
L   },
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.curly.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.separator.delimiter.java
L 
L   SCISSORS("peace"),
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.begin.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.end.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.separator.delimiter.java
L 
L   PAPER("flat");
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java constant.other.enum.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.begin.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java string.quoted.double.java punctuation.definition.string.end.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.terminator.java
L 
L   private final String handsign;
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java storage.modifier.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.definition.variable.java storage.type.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.definition.variable.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.definition.variable.java variable.other.definition.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.terminator.java
L 
L   Roshambo(String handsign) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java storage.type.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java variable.parameter.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L     this.handsign = handsign;
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java variable.language.this.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java variable.other.object.property.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java keyword.operator.assignment.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java meta.method.body.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java meta.method.java punctuation.section.method.end.bracket.curly.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.enum.java punctuation.section.enum.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### Anonymous Inner Classes
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 27 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L In the enum code, we used `TypeSpec.anonymousInnerClass()`. Anonymous inner classes can also be used in
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 103 text.html.markdown meta.paragraph.markdown
L code blocks. They are values that can be referenced with `$L`:
T 57 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L TypeSpec comparator = TypeSpec.anonymousClassBuilder("")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addSuperinterface(ParameterizedTypeName.get(Comparator.class, String.class))
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 79 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 80 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(MethodSpec.methodBuilder("compare")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addAnnotation(Override.class)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addModifiers(Modifier.PUBLIC)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addParameter(String.class, "a")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addParameter(String.class, "b")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .returns(int.class)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addStatement("return $N.length() - $N.length()", "a", "b")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L 
L TypeSpec helloWorld = TypeSpec.classBuilder("HelloWorld")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addMethod(MethodSpec.methodBuilder("sortByLength")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addParameter(ParameterizedTypeName.get(List.class, String.class), "strings")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 75 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 76 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 83 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 84 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 85 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addStatement("$T.sort($N, $L)", Collections.class, "strings", comparator)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 81 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 82 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L This generates a method that contains a class that contains a method:
T 69 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L void sortByLength(List<String> strings) {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java keyword.operator.comparison.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java keyword.operator.comparison.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L   Collections.sort(strings, new Comparator<String>() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.control.new.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java storage.type.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.angle.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java storage.type.generic.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.angle.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.bracket.round.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java punctuation.section.inner-class.begin.bracket.curly.java
L     @Override
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.declaration.annotation.java storage.type.annotation.java
L     public int compare(String a, String b) {
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java storage.modifier.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.return-type.java storage.type.primitive.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.begin.bracket.round.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java storage.type.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java variable.parameter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java punctuation.separator.delimiter.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java storage.type.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java variable.parameter.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.identifier.java punctuation.definition.parameters.end.bracket.round.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java punctuation.section.method.begin.bracket.curly.java
L       return a.length() - b.length();
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java keyword.control.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java variable.other.object.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java keyword.operator.arithmetic.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java variable.other.object.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.separator.period.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java entity.name.function.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java punctuation.terminator.java
L     }
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java meta.method.body.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java meta.method.java punctuation.section.method.end.bracket.curly.java
L   });
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.inner-class.java punctuation.section.inner-class.end.bracket.curly.java
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L }
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L One particularly tricky part of defining anonymous inner classes is the arguments to the superclass
T 99 text.html.markdown meta.paragraph.markdown
L constructor. In the above code we're passing the empty string for no arguments:
T 79 text.html.markdown meta.paragraph.markdown
L `TypeSpec.anonymousClassBuilder("")`. To pass different parameters use JavaPoet's code block
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 92 text.html.markdown meta.paragraph.markdown
L syntax with commas to separate arguments.
T 41 text.html.markdown meta.paragraph.markdown
L 
L 
L ### Annotations
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 15 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Simple annotations are easy:
T 28 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec toString = MethodSpec.methodBuilder("toString")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addAnnotation(Override.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(String.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addStatement("return $S", "Hoverboard")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this method with an `@Override` annotation:
T 36 text.html.markdown meta.paragraph.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L   @Override
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java storage.type.annotation.java
L   public String toString() {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.modifier.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.begin.bracket.curly.java
L     return "Hoverboard";
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.control.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java punctuation.definition.string.begin.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java string.quoted.double.java punctuation.definition.string.end.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L   }
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.section.block.end.bracket.curly.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Use `AnnotationSpec.builder()` to set properties on annotations:
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 30 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec logRecord = MethodSpec.methodBuilder("recordEvent")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.ABSTRACT)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addAnnotation(AnnotationSpec.builder(Headers.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addMember("accept", "$S", "application/json; charset=utf-8")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addMember("userAgent", "$S", "Square Cash")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(LogRecord.class, "logRecord")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(LogReceipt.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this annotation with `accept` and `userAgent` properties:
T 37 text.html.markdown meta.paragraph.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 50 text.html.markdown meta.paragraph.markdown
T 51 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 61 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 73 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L @Headers(
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java storage.type.annotation.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.begin.bracket.round.java
L     accept = "application/json; charset=utf-8",
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java constant.other.key.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java keyword.operator.assignment.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.separator.delimiter.java
L     userAgent = "Square Cash"
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java constant.other.key.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java keyword.operator.assignment.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
L )
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.end.bracket.round.java
L LogReceipt recordEvent(LogRecord logRecord);
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L When you get fancy, annotation values can be annotations themselves. Use `$L` for embedded
T 73 text.html.markdown meta.paragraph.markdown
T 74 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 77 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 90 text.html.markdown meta.paragraph.markdown
L annotations:
T 12 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec logRecord = MethodSpec.methodBuilder("recordEvent")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.ABSTRACT)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addAnnotation(AnnotationSpec.builder(HeaderList.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addMember("value", "$L", AnnotationSpec.builder(Header.class)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addMember("name", "$S", "Accept")
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addMember("value", "$S", "application/json; charset=utf-8")
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .build())
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .addMember("value", "$L", AnnotationSpec.builder(Header.class)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java variable.other.object.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java variable.other.object.property.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addMember("name", "$S", "User-Agent")
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .addMember("value", "$S", "Square Cash")
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.delimiter.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L             .build())
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java entity.name.function.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L         .build())
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.separator.period.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(LogRecord.class, "logRecord")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .returns(LogReceipt.class)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L @HeaderList({
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java storage.type.annotation.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.begin.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.section.block.begin.bracket.curly.java
L     @Header(name = "Accept", value = "application/json; charset=utf-8"),
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java storage.type.annotation.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java constant.other.key.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java keyword.operator.assignment.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.separator.delimiter.java
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java constant.other.key.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java keyword.operator.assignment.java
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.end.bracket.round.java
T 72 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.separator.delimiter.java
L     @Header(name = "User-Agent", value = "Square Cash")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java storage.type.annotation.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.begin.bracket.round.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java constant.other.key.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java keyword.operator.assignment.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.separator.delimiter.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java constant.other.key.java
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java keyword.operator.assignment.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.begin.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java string.quoted.double.java punctuation.definition.string.end.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.end.bracket.round.java
L })
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.section.block.end.bracket.curly.java
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.declaration.annotation.java punctuation.definition.annotation-arguments.end.bracket.round.java
L LogReceipt recordEvent(LogRecord logRecord);
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note that you can call `addMember()` multiple times with the same property name to populate a list
T 23 text.html.markdown meta.paragraph.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 98 text.html.markdown meta.paragraph.markdown
L of values for that property.
T 28 text.html.markdown meta.paragraph.markdown
L 
L ### Javadoc
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 11 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 
L Fields, methods and types can be documented with Javadoc:
T 57 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L MethodSpec dismiss = MethodSpec.methodBuilder("dismiss")
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java storage.type.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java variable.other.definition.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.definition.variable.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java keyword.operator.assignment.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java variable.other.object.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addJavadoc("Hides {@code message} from the caller's history. Other\n"
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 73 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 74 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "participants in the conversation will continue to see the\n"
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "message in their own history unless they also delete it.\n")
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 70 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 71 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addJavadoc("\n")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addJavadoc("<p>Use {@link #delete($T)} to delete the entire\n"
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
L         + "conversation for all participants.\n", Conversation.class)
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java keyword.operator.arithmetic.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java constant.character.escape.java
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addModifiers(Modifier.PUBLIC, Modifier.ABSTRACT)
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .addParameter(Message.class, "message")
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.java
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java variable.other.object.property.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.delimiter.java
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.begin.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java string.quoted.double.java punctuation.definition.string.end.java
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
L     .build();
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.separator.period.java
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java entity.name.function.java
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.begin.bracket.round.java
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.method-call.java punctuation.definition.parameters.end.bracket.round.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Which generates this:
T 21 text.html.markdown meta.paragraph.markdown
L 
L ```java
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 7 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L   /**
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java punctuation.definition.comment.java
L    * Hides {@code message} from the caller's history. Other
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    * participants in the conversation will continue to see the
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    * message in their own history unless they also delete it.
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    *
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    * <p>Use {@link #delete(Conversation)} to delete the entire
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java keyword.other.documentation.javadoc.java
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java variable.parameter.java
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    * conversation for all participants.
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
L    */
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java comment.block.javadoc.java punctuation.definition.comment.java
L   void dismiss(Message message);
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java storage.type.primitive.java
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java entity.name.function.java
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.begin.bracket.round.java
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java storage.type.java
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java meta.function-call.java punctuation.definition.parameters.end.bracket.round.java
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.java punctuation.terminator.java
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Use `$T` when referencing types in Javadoc to get automatic imports.
T 4 text.html.markdown meta.paragraph.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 7 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 8 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown
L 
L Download
T 8 text.html.markdown meta.paragraph.markdown
L --------
T 8 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L Download [the latest .jar][dl] or depend via Maven:
T 9 text.html.markdown meta.paragraph.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 25 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 26 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 27 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 29 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 51 text.html.markdown meta.paragraph.markdown
L ```xml
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 6 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L <dependency>
T 12 text.html.markdown markup.fenced_code.block.markdown
L   <groupId>com.squareup</groupId>
T 33 text.html.markdown markup.fenced_code.block.markdown
L   <artifactId>javapoet</artifactId>
T 35 text.html.markdown markup.fenced_code.block.markdown
L   <version>1.13.0</version>
T 27 text.html.markdown markup.fenced_code.block.markdown
L </dependency>
T 13 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L or Gradle:
T 10 text.html.markdown meta.paragraph.markdown
L ```groovy
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 9 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L compile 'com.squareup:javapoet:1.13.0'
T 38 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Snapshots of the development version are available in [Sonatype's `snapshots` repository][snap].
T 54 text.html.markdown meta.paragraph.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 76 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown
T 77 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 88 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 89 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 90 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 94 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 95 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 96 text.html.markdown meta.paragraph.markdown
L 
L 
L 
L License
T 7 text.html.markdown meta.paragraph.markdown
L -------
T 7 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L     Copyright 2015 Square, Inc.
T 4 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown
L 
L     Licensed under the Apache License, Version 2.0 (the "License");
T 4 text.html.markdown meta.paragraph.markdown
T 67 text.html.markdown meta.paragraph.markdown
L     you may not use this file except in compliance with the License.
T 4 text.html.markdown meta.paragraph.markdown
T 68 text.html.markdown meta.paragraph.markdown
L     You may obtain a copy of the License at
T 4 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown
L 
L        http://www.apache.org/licenses/LICENSE-2.0
T 7 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown
L 
L     Unless required by applicable law or agreed to in writing, software
T 4 text.html.markdown meta.paragraph.markdown
T 71 text.html.markdown meta.paragraph.markdown
L     distributed under the License is distributed on an "AS IS" BASIS,
T 4 text.html.markdown meta.paragraph.markdown
T 69 text.html.markdown meta.paragraph.markdown
L     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
T 4 text.html.markdown meta.paragraph.markdown
T 76 text.html.markdown meta.paragraph.markdown
L     See the License for the specific language governing permissions and
T 4 text.html.markdown meta.paragraph.markdown
T 71 text.html.markdown meta.paragraph.markdown
L     limitations under the License.
T 4 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown
L 
L 
L 
L JavaWriter
T 10 text.html.markdown meta.paragraph.markdown
L ==========
T 10 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L JavaPoet is the successor to [JavaWriter][javawriter]. New projects should prefer JavaPoet because
T 29 text.html.markdown meta.paragraph.markdown
T 30 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 40 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 42 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 52 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 53 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 98 text.html.markdown meta.paragraph.markdown
L it has a stronger code model: it understands types and can manage imports automatically. JavaPoet is
T 100 text.html.markdown meta.paragraph.markdown
L also better suited to composition: rather than streaming the contents of a `.java` file
T 75 text.html.markdown meta.paragraph.markdown
T 76 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 81 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 82 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 87 text.html.markdown meta.paragraph.markdown
L top-to-bottom in a single pass, a file can be assembled as a tree of declarations.
T 82 text.html.markdown meta.paragraph.markdown
L 
L JavaWriter continues to be available in [GitHub][javawriter] and [Maven Central][javawriter_maven].
T 40 text.html.markdown meta.paragraph.markdown
T 41 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 47 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 48 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 49 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 59 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 60 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 65 text.html.markdown meta.paragraph.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.begin.markdown
T 97 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown constant.other.reference.link.markdown
T 98 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.constant.end.markdown
T 99 text.html.markdown meta.paragraph.markdown
L 
L 
L  [dl]: https://search.maven.org/remote_content?g=com.squareup&a=javapoet&v=LATEST
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 4 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 5 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 7 text.html.markdown meta.link.reference.def.markdown
T 81 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [snap]: https://oss.sonatype.org/content/repositories/snapshots/com/squareup/javapoet/
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 6 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 7 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 8 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 9 text.html.markdown meta.link.reference.def.markdown
T 87 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [javadoc]: https://square.github.io/javapoet/1.x/javapoet/
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 9 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 10 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 12 text.html.markdown meta.link.reference.def.markdown
T 59 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [javawriter]: https://github.com/square/javapoet/tree/javawriter_2
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 14 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 15 text.html.markdown meta.link.reference.def.markdown
T 67 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [javawriter_maven]: https://search.maven.org/#artifactdetails%7Ccom.squareup%7Cjavawriter%7C2.5.1%7Cjar
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 18 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 19 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 20 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 21 text.html.markdown meta.link.reference.def.markdown
T 104 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [formatter]: https://developer.android.com/reference/java/util/Formatter.html
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 11 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 13 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 14 text.html.markdown meta.link.reference.def.markdown
T 78 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L  [modifier]: https://docs.oracle.com/javase/8/docs/api/javax/lang/model/element/Modifier.html
T 1 text.html.markdown meta.link.reference.def.markdown
T 2 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 10 text.html.markdown meta.link.reference.def.markdown constant.other.reference.link.markdown
T 11 text.html.markdown meta.link.reference.def.markdown punctuation.definition.constant.markdown
T 12 text.html.markdown meta.link.reference.def.markdown punctuation.separator.key-value.markdown
T 13 text.html.markdown meta.link.reference.def.markdown
T 93 text.html.markdown meta.link.reference.def.markdown markup.underline.link.markdown
L 
