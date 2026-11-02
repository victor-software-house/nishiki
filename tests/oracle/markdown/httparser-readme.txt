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
T 5 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L 	var data = []byte(
T 19 text.html.markdown markup.fenced_code.block.markdown
L 		"POST /joyent/http-parser HTTP/1.1\r\n" +
T 43 text.html.markdown markup.fenced_code.block.markdown
L 			"Host: github.com\r\n" +
T 27 text.html.markdown markup.fenced_code.block.markdown
L 			"DNT: 1\r\n" +
T 17 text.html.markdown markup.fenced_code.block.markdown
L 			"Accept-Encoding: gzip, deflate, sdch\r\n" +
T 47 text.html.markdown markup.fenced_code.block.markdown
L 			"Accept-Language: ru-RU,ru;q=0.8,en-US;q=0.6,en;q=0.4\r\n" +
T 63 text.html.markdown markup.fenced_code.block.markdown
L 			"User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) " +
T 67 text.html.markdown markup.fenced_code.block.markdown
L 			"AppleWebKit/537.36 (KHTML, like Gecko) " +
T 46 text.html.markdown markup.fenced_code.block.markdown
L 			"Chrome/39.0.2171.65 Safari/537.36\r\n" +
T 44 text.html.markdown markup.fenced_code.block.markdown
L 			"Accept: text/html,application/xhtml+xml,application/xml;q=0.9," +
T 69 text.html.markdown markup.fenced_code.block.markdown
L 			"image/webp,*/*;q=0.8\r\n" +
T 31 text.html.markdown markup.fenced_code.block.markdown
L 			"Referer: https://github.com/joyent/http-parser\r\n" +
T 57 text.html.markdown markup.fenced_code.block.markdown
L 			"Connection: keep-alive\r\n" +
T 33 text.html.markdown markup.fenced_code.block.markdown
L 			"Transfer-Encoding: chunked\r\n" +
T 37 text.html.markdown markup.fenced_code.block.markdown
L 			"Cache-Control: max-age=0\r\n\r\nb\r\nhello world\r\n0\r\n\r\n")
T 67 text.html.markdown markup.fenced_code.block.markdown
L 
L 	var setting = httparser.Setting{
T 33 text.html.markdown markup.fenced_code.block.markdown
L 		MessageBegin: func(*httparser.Parser, int) {
T 46 text.html.markdown markup.fenced_code.block.markdown
L 			//解析器开始工作
T 26 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("begin\n")
T 24 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		URL: func(_ *httparser.Parser, buf []byte, _ int) {
T 53 text.html.markdown markup.fenced_code.block.markdown
L 			//url数据
T 14 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("url->%s\n", buf)
T 31 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		Status: func(*httparser.Parser, []byte, _ int) {
T 50 text.html.markdown markup.fenced_code.block.markdown
L 			// 响应包才需要用到
T 30 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		HeaderField: func(_ *httparser.Parser, buf []byte, _ int) {
T 61 text.html.markdown markup.fenced_code.block.markdown
L 			// http header field
T 23 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("header field:%s\n", buf)
T 39 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		HeaderValue: func(_ *httparser.Parser, buf []byte, _ int) {
T 61 text.html.markdown markup.fenced_code.block.markdown
L 			// http header value
T 23 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("header value:%s\n", buf)
T 39 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		HeadersComplete: func(_ *httparser.Parser, _ int) {
T 53 text.html.markdown markup.fenced_code.block.markdown
L 			// http header解析结束
T 29 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("header complete\n")
T 34 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		Body: func(_ *httparser.Parser, buf []byte, _ int) {
T 54 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("%s", buf)
T 24 text.html.markdown markup.fenced_code.block.markdown
L 			// Content-Length 或者chunked数据包
T 43 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 		MessageComplete: func(_ *httparser.Parser, _ int) {
T 53 text.html.markdown markup.fenced_code.block.markdown
L 			// 消息解析结束
T 24 text.html.markdown markup.fenced_code.block.markdown
L 			fmt.Printf("\n")
T 19 text.html.markdown markup.fenced_code.block.markdown
L 		},
T 4 text.html.markdown markup.fenced_code.block.markdown
L 	}
T 2 text.html.markdown markup.fenced_code.block.markdown
L 
L 	p := httparser.New(httparser.REQUEST)
T 38 text.html.markdown markup.fenced_code.block.markdown
L 	success, err := p.Execute(&setting, data)
T 42 text.html.markdown markup.fenced_code.block.markdown
L 
L 	fmt.Printf("success:%d, err:%v\n", success, err)
T 49 text.html.markdown markup.fenced_code.block.markdown
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
