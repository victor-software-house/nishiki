L # httparser
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 11 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L [![Go](https://github.com/antlabs/httparser/workflows/Go/badge.svg)](https://github.com/antlabs/httparser/actions)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 5 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 6 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 7 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 66 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 67 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 68 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 69 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 113 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 114 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L [![codecov](https://codecov.io/gh/antlabs/httparser/branch/main/graph/badge.svg)](https://codecov.io/gh/antlabs/httparser)  
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 3 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.begin.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown string.other.link.description.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.link.description.end.markdown
T 12 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 79 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown markup.underline.link.image.markdown
T 80 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown meta.image.inline.markdown punctuation.definition.metadata.markdown
T 81 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 82 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 121 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 122 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 124 text.html.markdown meta.paragraph.markdown
L 
L 高性能http 1.1解析器，为你的异步io库插上解析的翅膀[从零实现]
T 70 text.html.markdown meta.paragraph.markdown
T 71 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.begin.markdown
T 83 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown string.other.link.title.markdown
T 84 text.html.markdown meta.paragraph.markdown meta.link.reference.markdown punctuation.definition.link.title.end.markdown
L 
L ## 出发点
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 12 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 本来想基于异步io库写些好玩的代码，发现没有适用于这些库的http解析库，索性就自己写个，弥补golang生态一小片空白领域。
T 165 text.html.markdown meta.paragraph.markdown
L 
L ## 特性
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 9 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L * url解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 11 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * request or response header field解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 40 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * request or response  header value解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Content-Length数据包解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 31 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * chunked数据包解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ## parser request
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 17 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L ```go
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L 	var data = []byte(
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.var.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.assignment.go
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.assignment.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
L 		"POST /joyent/http-parser HTTP/1.1\r\n" +
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Host: github.com\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"DNT: 1\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Accept-Encoding: gzip, deflate, sdch\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Accept-Language: ru-RU,ru;q=0.8,en-US;q=0.6,en;q=0.4\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 62 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) " +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 64 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"AppleWebKit/537.36 (KHTML, like Gecko) " +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Chrome/39.0.2171.65 Safari/537.36\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Accept: text/html,application/xhtml+xml,application/xml;q=0.9," +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 68 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 69 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"image/webp,*/*;q=0.8\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Referer: https://github.com/joyent/http-parser\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Connection: keep-alive\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Transfer-Encoding: chunked\r\n" +
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.arithmetic.go
L 			"Cache-Control: max-age=0\r\n\r\nb\r\nhello world\r\n0\r\n\r\n")
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 56 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 57 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 63 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 65 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 66 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 67 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 
L 	var setting = httparser.Setting{
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.var.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.assignment.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.assignment.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 		MessageBegin: func(*httparser.Parser, int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			//解析器开始工作
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("begin\n")
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		URL: func(_ *httparser.Parser, buf []byte, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			//url数据
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("url->%s\n", buf)
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		Status: func(*httparser.Parser, []byte, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			// 响应包才需要用到
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		HeaderField: func(_ *httparser.Parser, buf []byte, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			// http header field
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("header field:%s\n", buf)
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		HeaderValue: func(_ *httparser.Parser, buf []byte, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 22 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 55 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 59 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 60 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 61 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			// http header value
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("header value:%s\n", buf)
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		HeadersComplete: func(_ *httparser.Parser, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			// http header解析结束
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("header complete\n")
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		Body: func(_ *httparser.Parser, buf []byte, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 32 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 39 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.square.go
T 40 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.square.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.byte.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 54 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			fmt.Printf("%s", buf)
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 			// Content-Length 或者chunked数据包
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 		MessageComplete: func(_ *httparser.Parser, _ int) {
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.property.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.colon.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.function.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 25 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.type.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 46 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.parameter.go
T 47 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 50 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go storage.type.numeric.go
T 51 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
T 52 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 53 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.curly.go
L 			// 消息解析结束
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go punctuation.definition.comment.go
T 24 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go comment.line.double-slash.go
L 			fmt.Printf("\n")
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 		},
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
L 	}
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.curly.go
L 
L 	p := httparser.New(httparser.REQUEST)
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 2 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.assignment.go
T 3 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.assignment.go
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 15 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 30 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 38 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 	success, err := p.Execute(&setting, data)
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.assignment.go
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.assignment.go
T 14 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.assignment.go
T 17 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 26 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 27 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 28 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go keyword.operator.address.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 37 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 41 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 42 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L 
L 	fmt.Printf("success:%d, err:%v\n", success, err)
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 4 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.period.go
T 11 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go entity.name.function.support.go
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.begin.bracket.round.go
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.begin.go
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 23 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 29 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go
T 31 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.other.placeholder.go
T 33 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go constant.character.escape.go
T 34 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go string.quoted.double.go punctuation.definition.string.end.go
T 35 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 36 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 43 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 44 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.other.comma.go
T 45 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go
T 48 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go variable.other.go
T 49 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.go punctuation.definition.end.bracket.round.go
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ## response
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 11 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L [response](./_example/response.go)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 9 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 10 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 11 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 33 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 34 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L ## request or response
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 22 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L 如果你不确定数据包是请求还是响应，可看下面的例子  
T 74 text.html.markdown meta.paragraph.markdown
L [request or response](./_example/request_or_response.go)
T 1 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.begin.markdown
T 20 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown string.other.link.title.markdown
T 21 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.link.title.end.markdown
T 22 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
T 55 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown markup.underline.link.markdown
T 56 text.html.markdown meta.paragraph.markdown meta.link.inline.markdown punctuation.definition.metadata.markdown
L 
L 
L ## 编译
T 2 text.html.markdown markup.heading.markdown heading.2.markdown punctuation.definition.heading.markdown
T 3 text.html.markdown markup.heading.markdown heading.2.markdown
T 9 text.html.markdown markup.heading.markdown heading.2.markdown entity.name.section.markdown
L ### 生成 unhex表和tokens表
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 31 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L 如果需要修改这两个表，可以到_cmd目录下面修改生成代码的代码
T 85 text.html.markdown meta.paragraph.markdown
L ```Makefile
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make gen
T 8 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L ### 编译example
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 17 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L ```Makefile
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make example
T 12 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L ### 运行示例
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L ```Makefile
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L make example.run
T 16 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L ### return value
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 16 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L * err != nil 错误
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 19 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * sucess == len(data) 所有数据成功解析
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 46 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * sucess < len(data) 只解析部分数据，未解析的数据需再送一次
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 10 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown meta.other.valid-bracket.markdown
T 78 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L ### 吞吐量
T 3 text.html.markdown markup.heading.markdown heading.3.markdown punctuation.definition.heading.markdown
T 4 text.html.markdown markup.heading.markdown heading.3.markdown
T 13 text.html.markdown markup.heading.markdown heading.3.markdown entity.name.section.markdown
L * 测试仓库 https://github.com/junelabs/httparser-benchmark
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L * Benchmark result: 8192.00 mb | 630.15 mb/s | 1275606.54 req/sec | 13.00 s
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 75 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
