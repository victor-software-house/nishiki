L # Gin Web Framework
T 1 text.html.markdown markup.heading.markdown heading.1.markdown punctuation.definition.heading.markdown
T 2 text.html.markdown markup.heading.markdown heading.1.markdown
T 19 text.html.markdown markup.heading.markdown heading.1.markdown entity.name.section.markdown
L 
L <img align="right" width="159px" src="https://raw.githubusercontent.com/gin-gonic/logo/master/color.png">
T 1 text.html.markdown meta.tag.object.img.void.html punctuation.definition.tag.begin.html
T 4 text.html.markdown meta.tag.object.img.void.html entity.name.tag.html
T 5 text.html.markdown meta.tag.object.img.void.html
T 10 text.html.markdown meta.tag.object.img.void.html meta.attribute.align.html invalid.deprecated.entity.other.attribute-name.html
T 11 text.html.markdown meta.tag.object.img.void.html meta.attribute.align.html punctuation.separator.key-value.html
T 12 text.html.markdown meta.tag.object.img.void.html meta.attribute.align.html string.quoted.double.html punctuation.definition.string.begin.html
T 17 text.html.markdown meta.tag.object.img.void.html meta.attribute.align.html string.quoted.double.html
T 18 text.html.markdown meta.tag.object.img.void.html meta.attribute.align.html string.quoted.double.html punctuation.definition.string.end.html
T 19 text.html.markdown meta.tag.object.img.void.html
T 24 text.html.markdown meta.tag.object.img.void.html meta.attribute.width.html entity.other.attribute-name.html
T 25 text.html.markdown meta.tag.object.img.void.html meta.attribute.width.html punctuation.separator.key-value.html
T 26 text.html.markdown meta.tag.object.img.void.html meta.attribute.width.html string.quoted.double.html punctuation.definition.string.begin.html
T 31 text.html.markdown meta.tag.object.img.void.html meta.attribute.width.html string.quoted.double.html
T 32 text.html.markdown meta.tag.object.img.void.html meta.attribute.width.html string.quoted.double.html punctuation.definition.string.end.html
T 33 text.html.markdown meta.tag.object.img.void.html
T 36 text.html.markdown meta.tag.object.img.void.html meta.attribute.src.html entity.other.attribute-name.html
T 37 text.html.markdown meta.tag.object.img.void.html meta.attribute.src.html punctuation.separator.key-value.html
T 38 text.html.markdown meta.tag.object.img.void.html meta.attribute.src.html string.quoted.double.html punctuation.definition.string.begin.html
T 103 text.html.markdown meta.tag.object.img.void.html meta.attribute.src.html string.quoted.double.html
T 104 text.html.markdown meta.tag.object.img.void.html meta.attribute.src.html string.quoted.double.html punctuation.definition.string.end.html
T 105 text.html.markdown meta.tag.object.img.void.html punctuation.definition.tag.end.html
L 
L [![Build Status](https://github.com/gin-gonic/gin/actions/workflows/gin.yml/badge.svg?branch=master)](https://github.com/gin-gonic/gin/actions/workflows/gin.yml)
T 161 text.html.markdown
L [![Trivy Security Scan](https://github.com/gin-gonic/gin/actions/workflows/trivy-scan.yml/badge.svg)](https://github.com/gin-gonic/gin/actions/workflows/trivy-scan.yml)
T 168 text.html.markdown
L [![codecov](https://codecov.io/gh/gin-gonic/gin/branch/master/graph/badge.svg)](https://codecov.io/gh/gin-gonic/gin)
T 116 text.html.markdown
L [![Go Report Card](https://goreportcard.com/badge/github.com/gin-gonic/gin)](https://goreportcard.com/report/github.com/gin-gonic/gin)
T 134 text.html.markdown
L [![Go Reference](https://pkg.go.dev/badge/github.com/gin-gonic/gin?status.svg)](https://pkg.go.dev/github.com/gin-gonic/gin?tab=doc)
T 132 text.html.markdown
L [![Sourcegraph](https://sourcegraph.com/github.com/gin-gonic/gin/-/badge.svg)](https://sourcegraph.com/github.com/gin-gonic/gin?badge)
T 134 text.html.markdown
L [![Open Source Helpers](https://www.codetriage.com/gin-gonic/gin/badges/users.svg)](https://www.codetriage.com/gin-gonic/gin)
T 125 text.html.markdown
L [![Release](https://img.shields.io/github/release/gin-gonic/gin.svg?style=flat-square)](https://github.com/gin-gonic/gin/releases)
T 130 text.html.markdown
L 
L ## 📰 Gin 1.12.0 is now available!
T 36 text.html.markdown
L 
L We're excited to announce the release of **[Gin 1.12.0](https://gin-gonic.com/en/blog/news/gin-1-12-0-release-announcement/)**! This release brings new features, performance improvements, and important bug fixes. Check out the [release announcement](https://gin-gonic.com/en/blog/news/gin-1-12-0-release-announcement/) on our official blog for the full details.
T 361 text.html.markdown
L 
L ---
T 3 text.html.markdown
L 
L Gin is a high-performance HTTP web framework written in [Go](https://go.dev/). It provides a Martini-like API but with significantly better performance—up to 40 times faster—thanks to [httprouter](https://github.com/julienschmidt/httprouter). Gin is designed for building REST APIs, web applications, and microservices where speed and developer productivity are essential.
T 376 text.html.markdown
L 
L **Why choose Gin?**
T 19 text.html.markdown
L 
L Gin combines the simplicity of Express.js-style routing with Go's performance characteristics, making it ideal for:
T 115 text.html.markdown
L 
L - Building high-throughput REST APIs
T 36 text.html.markdown
L - Developing microservices that need to handle many concurrent requests
T 71 text.html.markdown
L - Creating web applications that require fast response times
T 60 text.html.markdown
L - Prototyping web services quickly with minimal boilerplate
T 59 text.html.markdown
L 
L **Gin's key features:**
T 23 text.html.markdown
L 
L - **Zero allocation router** - Extremely memory-efficient routing with no heap allocations
T 90 text.html.markdown
L - **High performance** - Benchmarks show superior speed compared to other Go web frameworks
T 91 text.html.markdown
L - **Middleware support** - Extensible middleware system for authentication, logging, CORS, etc.
T 95 text.html.markdown
L - **Crash-free** - Built-in recovery middleware prevents panics from crashing your server
T 89 text.html.markdown
L - **JSON validation** - Automatic request/response JSON binding and validation
T 78 text.html.markdown
L - **Route grouping** - Organize related routes and apply common middleware
T 74 text.html.markdown
L - **Error management** - Centralized error handling and logging
T 63 text.html.markdown
L - **Built-in rendering** - Support for JSON, XML, HTML templates, and more
T 74 text.html.markdown
L - **Extensible** - Large ecosystem of community middleware and plugins
T 70 text.html.markdown
L 
L ## Getting Started
T 18 text.html.markdown
L 
L ### Prerequisites
T 17 text.html.markdown
L 
L - **Go version**: Gin requires [Go](https://go.dev/) version [1.26](https://go.dev/doc/devel/release#go1.26.0) or above
T 119 text.html.markdown
L - **Basic Go knowledge**: Familiarity with Go syntax and package management is helpful
T 86 text.html.markdown
L 
L ### Installation
T 16 text.html.markdown
L 
L With [Go's module support](https://go.dev/wiki/Modules#how-to-use-modules), simply import Gin in your code and Go will automatically fetch it during build:
T 155 text.html.markdown
L 
L ```go
T 5 text.html.markdown
L import "github.com/gin-gonic/gin"
T 33 text.html.markdown
L ```
T 3 text.html.markdown
L 
L ### Your First Gin Application
T 30 text.html.markdown
L 
L Here's a complete example that demonstrates Gin's simplicity:
T 61 text.html.markdown
L 
L ```go
T 5 text.html.markdown
L package main
T 12 text.html.markdown
L 
L import (
T 8 text.html.markdown
L   "log"
T 7 text.html.markdown
L   "net/http"
T 12 text.html.markdown
L 
L   "github.com/gin-gonic/gin"
T 28 text.html.markdown
L )
T 1 text.html.markdown
L 
L func main() {
T 13 text.html.markdown
L   // Create a Gin router with default middleware (logger and recovery)
T 70 text.html.markdown
L   r := gin.Default()
T 20 text.html.markdown
L 
L   // Define a simple GET endpoint
T 33 text.html.markdown
L   r.GET("/ping", func(c *gin.Context) {
T 39 text.html.markdown
L     // Return JSON response
T 27 text.html.markdown
L     c.JSON(http.StatusOK, gin.H{
T 32 text.html.markdown
L       "message": "pong",
T 24 text.html.markdown
L     })
T 6 text.html.markdown
L   })
T 4 text.html.markdown
L 
L   // Start server on port 8080 (default)
T 40 text.html.markdown
L   // Server will listen on 0.0.0.0:8080 (localhost:8080 on Windows)
T 67 text.html.markdown
L   if err := r.Run(); err != nil {
T 33 text.html.markdown
L     log.Fatalf("failed to run server: %v", err)
T 47 text.html.markdown
L   }
T 3 text.html.markdown
L }
T 1 text.html.markdown
L ```
T 3 text.html.markdown
L 
L **Running the application:**
T 28 text.html.markdown
L 
L 1. Save the code above as `main.go`
T 35 text.html.markdown
L 2. Run the application:
T 23 text.html.markdown
L 
L    ```sh
T 8 text.html.markdown
L    go run main.go
T 17 text.html.markdown
L    ```
T 6 text.html.markdown
L 
L 3. Open your browser and visit [`http://localhost:8080/ping`](http://localhost:8080/ping)
T 89 text.html.markdown
L 4. You should see: `{"message":"pong"}`
T 39 text.html.markdown
L 
L **What this example demonstrates:**
T 35 text.html.markdown
L 
L - Creating a Gin router with default middleware
T 47 text.html.markdown
L - Defining HTTP endpoints with simple handler functions
T 55 text.html.markdown
L - Returning JSON responses
T 26 text.html.markdown
L - Starting an HTTP server
T 25 text.html.markdown
L 
L ### Next Steps
T 14 text.html.markdown
L 
L After running your first Gin application, explore these resources to learn more:
T 80 text.html.markdown
L 
L #### 📚 Learning Resources
T 28 text.html.markdown
L 
L - **[Gin Quick Start Guide](docs/doc.md)** - Comprehensive tutorial with API examples and build configurations
T 110 text.html.markdown
L - **[Example Repository](https://github.com/gin-gonic/examples)** - Ready-to-run examples demonstrating various Gin use cases:
T 126 text.html.markdown
L   - REST API development
T 24 text.html.markdown
L   - Authentication & middleware
T 31 text.html.markdown
L   - File uploads and downloads
T 30 text.html.markdown
L   - WebSocket connections
T 25 text.html.markdown
L   - Template rendering
T 22 text.html.markdown
L 
L ## 📖 Documentation
T 21 text.html.markdown
L 
L ### API Reference
T 17 text.html.markdown
L 
L - **[Go.dev API Documentation](https://pkg.go.dev/github.com/gin-gonic/gin)** - Complete API reference with examples
T 116 text.html.markdown
L 
L ### User Guides
T 15 text.html.markdown
L 
L The comprehensive documentation is available on [gin-gonic.com](https://gin-gonic.com) in multiple languages:
T 109 text.html.markdown
L 
L - [English](https://gin-gonic.com/en/docs/) | [简体中文](https://gin-gonic.com/zh-cn/docs/) | [繁體中文](https://gin-gonic.com/zh-tw/docs/)
T 147 text.html.markdown
L - [日本語](https://gin-gonic.com/ja/docs/) | [한국어](https://gin-gonic.com/ko-kr/docs/) | [Español](https://gin-gonic.com/es/docs/)
T 139 text.html.markdown
L - [Turkish](https://gin-gonic.com/tr/docs/) | [Persian](https://gin-gonic.com/fa/docs/) | [Português](https://gin-gonic.com/pt/docs/)
T 134 text.html.markdown
L - [Russian](https://gin-gonic.com/ru/docs/) | [Indonesian](https://gin-gonic.com/id/docs/)
T 90 text.html.markdown
L 
L ### Official Tutorials
T 22 text.html.markdown
L 
L - [Go.dev Tutorial: Developing a RESTful API with Go and Gin](https://go.dev/doc/tutorial/web-service-gin)
T 106 text.html.markdown
L 
L ## ⚡ Performance Benchmarks
T 29 text.html.markdown
L 
L Gin demonstrates exceptional performance compared to other Go web frameworks. It uses a custom version of [HttpRouter](https://github.com/julienschmidt/httprouter) for maximum efficiency. [View detailed benchmarks →](/BENCHMARKS.md)
T 234 text.html.markdown
L 
L **Gin vs. Other Go Frameworks** (GitHub API routing benchmark):
T 63 text.html.markdown
L 
L | Benchmark name                 |       (1) |             (2) |          (3) |             (4) |
T 97 text.html.markdown
L | ------------------------------ | --------: | --------------: | -----------: | --------------: |
T 97 text.html.markdown
L | BenchmarkGin_GithubAll         | **43550** | **27364 ns/op** |   **0 B/op** | **0 allocs/op** |
T 97 text.html.markdown
L | BenchmarkAce_GithubAll         |     40543 |     29670 ns/op |       0 B/op |     0 allocs/op |
T 97 text.html.markdown
L | BenchmarkAero_GithubAll        |     57632 |     20648 ns/op |       0 B/op |     0 allocs/op |
T 97 text.html.markdown
L | BenchmarkBear_GithubAll        |      9234 |    216179 ns/op |   86448 B/op |   943 allocs/op |
T 97 text.html.markdown
L | BenchmarkBeego_GithubAll       |      7407 |    243496 ns/op |   71456 B/op |   609 allocs/op |
T 97 text.html.markdown
L | BenchmarkBone_GithubAll        |       420 |   2922835 ns/op |  720160 B/op |  8620 allocs/op |
T 97 text.html.markdown
L | BenchmarkChi_GithubAll         |      7620 |    238331 ns/op |   87696 B/op |   609 allocs/op |
T 97 text.html.markdown
L | BenchmarkDenco_GithubAll       |     18355 |     64494 ns/op |   20224 B/op |   167 allocs/op |
T 97 text.html.markdown
L | BenchmarkEcho_GithubAll        |     31251 |     38479 ns/op |       0 B/op |     0 allocs/op |
T 97 text.html.markdown
L | BenchmarkGocraftWeb_GithubAll  |      4117 |    300062 ns/op |  131656 B/op |  1686 allocs/op |
T 97 text.html.markdown
L | BenchmarkGoji_GithubAll        |      3274 |    416158 ns/op |   56112 B/op |   334 allocs/op |
T 97 text.html.markdown
L | BenchmarkGojiv2_GithubAll      |      1402 |    870518 ns/op |  352720 B/op |  4321 allocs/op |
T 97 text.html.markdown
L | BenchmarkGoJsonRest_GithubAll  |      2976 |    401507 ns/op |  134371 B/op |  2737 allocs/op |
T 97 text.html.markdown
L | BenchmarkGoRestful_GithubAll   |       410 |   2913158 ns/op |  910144 B/op |  2938 allocs/op |
T 97 text.html.markdown
L | BenchmarkGorillaMux_GithubAll  |       346 |   3384987 ns/op |  251650 B/op |  1994 allocs/op |
T 97 text.html.markdown
L | BenchmarkGowwwRouter_GithubAll |     10000 |    143025 ns/op |   72144 B/op |   501 allocs/op |
T 97 text.html.markdown
L | BenchmarkHttpRouter_GithubAll  |     55938 |     21360 ns/op |       0 B/op |     0 allocs/op |
T 97 text.html.markdown
L | BenchmarkHttpTreeMux_GithubAll |     10000 |    153944 ns/op |   65856 B/op |   671 allocs/op |
T 97 text.html.markdown
L | BenchmarkKocha_GithubAll       |     10000 |    106315 ns/op |   23304 B/op |   843 allocs/op |
T 97 text.html.markdown
L | BenchmarkLARS_GithubAll        |     47779 |     25084 ns/op |       0 B/op |     0 allocs/op |
T 97 text.html.markdown
L | BenchmarkMacaron_GithubAll     |      3266 |    371907 ns/op |  149409 B/op |  1624 allocs/op |
T 97 text.html.markdown
L | BenchmarkMartini_GithubAll     |       331 |   3444706 ns/op |  226551 B/op |  2325 allocs/op |
T 97 text.html.markdown
L | BenchmarkPat_GithubAll         |       273 |   4381818 ns/op | 1483152 B/op | 26963 allocs/op |
T 97 text.html.markdown
L | BenchmarkPossum_GithubAll      |     10000 |    164367 ns/op |   84448 B/op |   609 allocs/op |
T 97 text.html.markdown
L | BenchmarkR2router_GithubAll    |     10000 |    160220 ns/op |   77328 B/op |   979 allocs/op |
T 97 text.html.markdown
L | BenchmarkRivet_GithubAll       |     14625 |     82453 ns/op |   16272 B/op |   167 allocs/op |
T 97 text.html.markdown
L | BenchmarkTango_GithubAll       |      6255 |    279611 ns/op |   63826 B/op |  1618 allocs/op |
T 97 text.html.markdown
L | BenchmarkTigerTonic_GithubAll  |      2008 |    687874 ns/op |  193856 B/op |  4474 allocs/op |
T 97 text.html.markdown
L | BenchmarkTraffic_GithubAll     |       355 |   3478508 ns/op |  820744 B/op | 14114 allocs/op |
T 97 text.html.markdown
L | BenchmarkVulcan_GithubAll      |      6885 |    193333 ns/op |   19894 B/op |   609 allocs/op |
T 97 text.html.markdown
L 
L - (1): Total Repetitions achieved in constant time, higher means more confident result
T 86 text.html.markdown
L - (2): Single Repetition Duration (ns/op), lower is better
T 58 text.html.markdown
L - (3): Heap Memory (B/op), lower is better
T 42 text.html.markdown
L - (4): Average Allocations per Repetition (allocs/op), lower is better
T 70 text.html.markdown
L 
L ## 🔌 Middleware Ecosystem
T 28 text.html.markdown
L 
L Gin has a rich ecosystem of middleware for common web development needs. Explore community-contributed middleware:
T 114 text.html.markdown
L 
L - **[gin-contrib](https://github.com/gin-contrib)** - Official middleware collection including:
T 95 text.html.markdown
L   - Authentication (JWT, Basic Auth, Sessions)
T 46 text.html.markdown
L   - CORS, Rate limiting, Compression
T 36 text.html.markdown
L   - Logging, Metrics, Tracing
T 29 text.html.markdown
L   - Static file serving, Template engines
T 41 text.html.markdown
L - **[gin-gonic/contrib](https://github.com/gin-gonic/contrib)** - Additional community middleware
T 97 text.html.markdown
L 
L ## 🏢 Production Usage
T 24 text.html.markdown
L 
L Gin powers many high-traffic applications and services in production:
T 69 text.html.markdown
L 
L - **[gorush](https://github.com/appleboy/gorush)** - High-performance push notification server
T 94 text.html.markdown
L - **[fnproject](https://github.com/fnproject/fn)** - Container-native, serverless platform
T 90 text.html.markdown
L - **[photoprism](https://github.com/photoprism/photoprism)** - AI-powered personal photo management
T 99 text.html.markdown
L - **[lura](https://github.com/luraproject/lura)** - Ultra-performant API Gateway framework
T 90 text.html.markdown
L - **[picfit](https://github.com/thoas/picfit)** - Real-time image processing server
T 83 text.html.markdown
L - **[dkron](https://github.com/distribworks/dkron)** - Distributed job scheduling system
T 88 text.html.markdown
L 
L ## 🤝 Contributing
T 20 text.html.markdown
L 
L Gin is the work of hundreds of contributors from around the world. We welcome and appreciate your contributions! See the full list of [contributors](https://github.com/gin-gonic/gin/graphs/contributors).
T 203 text.html.markdown
L 
L ### How to Contribute
T 21 text.html.markdown
L 
L - 🐛 **Report bugs** - Help us identify and fix issues
T 56 text.html.markdown
L - 💡 **Suggest features** - Share your ideas for improvements
T 63 text.html.markdown
L - 📝 **Improve documentation** - Help make our docs clearer
T 61 text.html.markdown
L - 🔧 **Submit code** - Fix bugs or implement new features
T 59 text.html.markdown
L - 🧪 **Write tests** - Improve our test coverage
T 50 text.html.markdown
L 
L ### Getting Started with Contributing
T 37 text.html.markdown
L 
L 1. Check out our [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines
T 75 text.html.markdown
L 2. Join our community discussions and ask questions
T 51 text.html.markdown
L 
L **All contributions are valued and help make Gin better for everyone!**
T 71 text.html.markdown
L 
