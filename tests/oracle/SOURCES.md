# Oracle inputs

`mise run oracle` fetches each input below at its pinned commit and writes Shiki's scopes for it to `<language>/<name>.txt`.
Each fixture repeats its input's text line by line.
The fixtures are test data: `Cargo.toml` leaves `tests/` out of the published crate.
[`COVERAGE.md`](COVERAGE.md) shows how much of each hand-lexed grammar these inputs exercise; an input added to close a gap is listed here like any other.

| Fixture | Source | Licence |
|:--|:--|:--|
| `asm/blake3-sse41-msvc` | [`c/blake3_sse41_x86-64_windows_msvc.asm`][blake3-sse41-msvc] in BLAKE3-team/BLAKE3 | [Apache-2.0][blake3-licence] |
| `asm/sample` | [`samples/asm.sample`][asm-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `asm/xv6-bootasm` | [`bootasm.S`][xv6-bootasm] in mit-pdos/xv6-public | [MIT][xv6-licence] |
| `asm/xv6-swtch` | [`swtch.S`][xv6-swtch] in mit-pdos/xv6-public | [MIT][xv6-licence] |
| `bat/coverage` | `oracle/inputs/bat-coverage.bat`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `bat/gradlew` | [`gradlew.bat`][gradlew-bat] in gradle/gradle | [Apache-2.0][gradle-licence] |
| `bat/sample` | [`samples/bat.sample`][bat-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `bat/tomcat-catalina` | [`bin/catalina.bat`][tomcat-catalina-bat] in apache/tomcat | [Apache-2.0][tomcat-licence] |
| `bat/tomcat-setclasspath` | [`bin/setclasspath.bat`][tomcat-setclasspath] in apache/tomcat | [Apache-2.0][tomcat-licence] |
| `c/cjson` | [`cJSON.c`][cjson] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `c/coverage` | `oracle/inputs/c-coverage.c`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `c/jq-print` | [`src/jv_print.c`][jq-print] in jqlang/jq | [MIT][jq-licence] |
| `c/libuv-core` | [`src/unix/core.c`][libuv-core] in libuv/libuv | [MIT][libuv-licence] |
| `c/libuv-freertos-startup` | [`test/freertos/mps2/startup.c`][libuv-freertos-startup] in libuv/libuv | [MIT][libuv-licence] |
| `c/linenoise` | [`linenoise.c`][linenoise] in antirez/linenoise | [BSD-2-Clause][linenoise-licence] |
| `c/lua-vm` | [`lvm.c`][lua-vm] in lua/lua | [MIT][lua-licence] |
| `c/sample` | [`samples/c.sample`][c-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `c/zlib-inflate` | [`inflate.c`][zlib-inflate] in madler/zlib | [zlib][zlib-licence] |
| `cmake/catch2-lists` | [`CMakeLists.txt`][catch2-cmake-lists] in catchorg/Catch2 | [BSL-1.0][catch2-licence] |
| `cmake/coverage` | `oracle/inputs/cmake-coverage.cmake`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `cmake/googletest-utils` | [`googletest/cmake/internal_utils.cmake`][googletest-utils] in google/googletest | [BSD-3-Clause][googletest-licence] |
| `cmake/json-lists` | [`CMakeLists.txt`][json-cmake-lists] in nlohmann/json | [MIT][nlohmann-json-licence] |
| `cmake/sample` | [`samples/cmake.sample`][cmake-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `coffee/coffeescript-browser` | [`src/browser.coffee`][coffeescript-browser] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coffeescript-cake` | [`src/cake.coffee`][coffeescript-cake] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coffeescript-helpers` | [`src/helpers.coffee`][coffeescript-helpers] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coffeescript-javascript-literals` | [`test/javascript_literals.coffee`][coffeescript-javascript-literals] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coffeescript-optparse` | [`src/optparse.coffee`][coffeescript-optparse] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coffeescript-repl` | [`src/repl.coffee`][coffeescript-repl] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `coffee/coverage` | `oracle/inputs/coffee-coverage.coffee`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `coffee/sample` | [`samples/coffee.sample`][coffee-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `common-lisp/cl-ppcre-optimize` | [`optimize.lisp`][cl-ppcre-optimize] in edicl/cl-ppcre | [BSD-2-Clause][cl-ppcre-licence] |
| `common-lisp/cl-ppcre-scanner` | [`scanner.lisp`][cl-ppcre-scanner] in edicl/cl-ppcre | [BSD-2-Clause][cl-ppcre-licence] |
| `common-lisp/grammar-demo` | [`syntaxes/fixtures/cases/demo.lsp`][common-lisp-grammar-demo] in qingpeng9802/vscode-common-lisp | [MIT][common-lisp-grammar-licence] |
| `common-lisp/grammar-format-strings` | [`syntaxes/fixtures/cases/fstr.lsp`][common-lisp-grammar-fstr] in qingpeng9802/vscode-common-lisp | [MIT][common-lisp-grammar-licence] |
| `common-lisp/quicklisp-http` | [`quicklisp/http.lisp`][quicklisp-http] in quicklisp/quicklisp-client | [MIT][quicklisp-licence] |
| `common-lisp/sample` | [`samples/common-lisp.sample`][common-lisp-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `common-lisp/split-sequence-list` | [`list.lisp`][split-sequence-list] in sharplispers/split-sequence | [MIT][split-sequence-licence] |
| `common-lisp/woo-response` | [`src/response.lisp`][woo-response] in fukamachi/woo | [MIT][woo-licence] |
| `cpp/absl-any-invocable` | [`absl/functional/internal/any_invocable.h`][absl-any-invocable] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-any-invocable-test` | [`absl/functional/any_invocable_test.h`][absl-any-invocable-test] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-check-op` | [`absl/log/internal/check_op.h`][absl-check-op] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-cpu-detect` | [`absl/base/internal/cpu_detect.cc`][absl-cpu-detect] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-crc-x86-arm-combined` | [`absl/crc/internal/crc_x86_arm_combined.cc`][absl-crc-x86-arm-combined] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-crc32c` | [`absl/crc/crc32c.h`][absl-crc32c] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-dynamic-annotations` | [`absl/base/internal/dynamic_annotations.h`][absl-dynamic-annotations] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-exception-safety-testing` | [`absl/base/internal/exception_safety_testing.h`][absl-exception-safety-testing] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-flag` | [`absl/flags/flag.h`][absl-flag] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-flag-internal` | [`absl/flags/internal/flag.h`][absl-flag-internal] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-flag-test` | [`absl/flags/flag_test.cc`][absl-flag-test] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-hash-benchmark` | [`absl/hash/hash_benchmark.cc`][absl-hash-benchmark] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-mutex` | [`absl/synchronization/mutex.h`][absl-mutex] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-profile-builder` | [`absl/profiling/internal/profile_builder.cc`][absl-profile-builder] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-status-matchers` | [`absl/status/internal/status_matchers.h`][absl-status-matchers] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-str-split` | [`absl/strings/str_split.h`][absl-str-split] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/absl-symbolize-test` | [`absl/debugging/symbolize_test.cc`][absl-symbolize-test] in abseil/abseil-cpp | [Apache-2.0][abseil-licence] |
| `cpp/benchmark-core` | [`src/benchmark.cc`][benchmark-core] in google/benchmark | [Apache-2.0][benchmark-licence] |
| `cpp/benchmark-sysinfo` | [`src/sysinfo.cc`][benchmark-sysinfo] in google/benchmark | [Apache-2.0][benchmark-licence] |
| `cpp/coverage` | `oracle/inputs/cpp-coverage.cpp`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `cpp/fmt-c` | [`include/fmt/fmt-c.h`][fmt-c] in fmtlib/fmt | [MIT][fmt-licence] |
| `cpp/fmt-core-test` | [`test/core-test.cc`][fmt-core-test] in fmtlib/fmt | [MIT][fmt-licence] |
| `cpp/fmt-gtest-extra` | [`test/gtest-extra.h`][fmt-gtest-extra] in fmtlib/fmt | [MIT][fmt-licence] |
| `cpp/fmt-ranges` | [`include/fmt/ranges.h`][fmt-ranges] in fmtlib/fmt | [MIT][fmt-licence] |
| `cpp/fmt-xchar-test` | [`test/xchar-test.cc`][fmt-xchar-test] in fmtlib/fmt | [MIT][fmt-licence] |
| `cpp/leveldb-version-set` | [`db/version_set.cc`][leveldb-version-set] in google/leveldb | [BSD-3-Clause][leveldb-licence] |
| `cpp/rapidjson-reader` | [`include/rapidjson/reader.h`][rapidjson-reader] in Tencent/rapidjson | [MIT][rapidjson-licence] |
| `cpp/sample` | [`samples/cpp.sample`][cpp-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `cpp/spdlog-logger` | [`include/spdlog/logger.h`][spdlog-logger] in gabime/spdlog | [MIT][spdlog-licence] |
| `csharp/coverage` | `oracle/inputs/csharp-coverage.cs`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `csharp/docs-async-wpf` | [`MainWindow.xaml.cs`][csharp-docs-async-wpf] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-bitwise-shift` | [`BitwiseAndShiftOperators.cs`][csharp-docs-bitwise-shift] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-closed-hierarchy-patterns` | [`ClosedHierarchyPatterns.cs`][csharp-docs-closed-hierarchy-patterns] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-delegate-operator` | [`DelegateOperator.cs`][csharp-docs-delegate-operator] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-doc-comments` | [`DocComments.cs`][csharp-docs-doc-comments] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-events` | [`events.cs`][csharp-docs-events] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-exception-example` | [`ExampleClass.cs`][csharp-docs-exception-example] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-fixed` | [`FixedKeywordExamples.cs`][csharp-docs-fixed] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-generic-where` | [`GenericWhereConstraints.cs`][csharp-docs-generic-where] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-goto` | [`GotoStatement.cs`][csharp-docs-goto] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-group` | [`group.cs`][csharp-docs-group] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-keywords-contextual` | [`csrefKeywordsContextual.cs`][csharp-docs-keywords-contextual] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-keywords-modifiers` | [`csrefKeywordsModifiers.cs`][csharp-docs-keywords-modifiers] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-lock` | [`Program.cs`][csharp-docs-lock] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-raw-string-literal` | [`raw-string-literal.cs`][csharp-docs-raw-string-literal] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-record-type` | [`RecordType.cs`][csharp-docs-record-type] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-string-interpolation` | [`string-interpolation.cs`][csharp-docs-string-interpolation] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-union-body-members` | [`BodyMembers.cs`][csharp-docs-union-body-members] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-using-alias` | [`UsingAliasQualifier.cs`][csharp-docs-using-alias] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-value-tuples` | [`ValueTuples.cs`][csharp-docs-value-tuples] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/docs-var-pattern` | [`VarPattern.cs`][csharp-docs-var-pattern] in dotnet/docs | [MIT][dotnet-docs-licence] |
| `csharp/newtonsoft-json-convert` | [`JsonConvert.cs`][newtonsoft-json-convert] in JamesNK/Newtonsoft.Json | [MIT][newtonsoft-licence] |
| `csharp/roslyn-text-span` | [`TextSpan.cs`][roslyn-text-span] in dotnet/roslyn | [MIT][roslyn-licence] |
| `csharp/runtime-linq-where` | [`Where.cs`][runtime-linq-where] in dotnet/runtime | [MIT][dotnet-runtime-licence] |
| `csharp/runtime-stack` | [`Stack.cs`][runtime-stack] in dotnet/runtime | [MIT][dotnet-runtime-licence] |
| `csharp/sample` | [`samples/csharp.sample`][csharp-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `css/charset-no-space` | `oracle/inputs/css-charset-no-space.css`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `css/charset-unclosed` | `oracle/inputs/css-charset-unclosed.css`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `css/charset-unquoted` | `oracle/inputs/css-charset-unquoted.css`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `css/coverage` | `oracle/inputs/css-coverage.css`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `css/github-markdown` | [`github-markdown.css`][github-markdown-css] in sindresorhus/github-markdown-css | [MIT][github-markdown-css-licence] |
| `css/highlightjs-atom-one-dark` | [`src/styles/atom-one-dark.css`][highlightjs-atom-one-dark] in highlightjs/highlight.js | [BSD-3-Clause][highlightjs-licence] |
| `css/highlightjs-github` | [`src/styles/github.css`][highlightjs-github] in highlightjs/highlight.js | [BSD-3-Clause][highlightjs-licence] |
| `css/lessjs-at-rules-compressed-evaluation` | [`at-rules-compressed-evaluation.css`][css-lessjs-at-rules-compressed-evaluation] in less/less.js | [Apache-2.0][lessjs-licence] |
| `css/lessjs-css-3` | [`css-3.css`][css-lessjs-css-3] in less/less.js | [Apache-2.0][lessjs-licence] |
| `css/lessjs-math-strict` | [`css.css`][css-lessjs-math-strict] in less/less.js | [Apache-2.0][lessjs-licence] |
| `css/lessjs-media` | [`media.css`][css-lessjs-media] in less/less.js | [Apache-2.0][lessjs-licence] |
| `css/lessjs-selectors` | [`selectors.css`][css-lessjs-selectors] in less/less.js | [Apache-2.0][lessjs-licence] |
| `css/milligram` | [`dist/milligram.css`][milligram-css] in milligram/milligram | [MIT][milligram-licence] |
| `css/normalize` | [`normalize.css`][normalize-css] in necolas/normalize.css | [MIT][normalize-licence] |
| `css/sample` | [`samples/css.sample`][css-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `css/tailwind-preflight` | [`packages/tailwindcss/preflight.css`][tailwind-preflight] in tailwindlabs/tailwindcss | [MIT][tailwind-licence] |
| `dart/collection-algorithms` | [`pkgs/collection/lib/src/algorithms.dart`][dart-collection-algorithms] in dart-lang/core | [BSD-3-Clause][dart-core-licence] |
| `dart/coverage` | `oracle/inputs/dart-coverage.dart`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `dart/http-base-request` | [`pkgs/http/lib/src/base_request.dart`][dart-http-base-request] in dart-lang/http | [BSD-3-Clause][dart-http-licence] |
| `dart/http-client` | [`pkgs/http/lib/src/client.dart`][dart-http-client] in dart-lang/http | [BSD-3-Clause][dart-http-licence] |
| `dart/sample` | [`samples/dart.sample`][dart-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `diff/coverage` | `oracle/inputs/diff-coverage.diff`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `diff/cranelift-stdarch-sve` | [`compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch`][cranelift-sve] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/cranelift-stdlib-atomics` | [`compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch`][cranelift-atomics] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/cranelift-sysroot-tests` | [`compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch`][cranelift-tests] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/mir-opt-gvn-cast` | [`tests/mir-opt/const_prop/cast.main.GVN.diff`][mir-cast] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/mir-opt-gvn-slices` | [`tests/mir-opt/gvn.slices.GVN.panic-abort.diff`][mir-slices] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `diff/sample` | [`samples/diff.sample`][diff-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `diff/vscode-foundry-local-sdk` | [`build/npm/patches/foundry-local-sdk.patch`][vscode-foundry] in microsoft/vscode | [MIT][vscode-licence] |
| `docker/buildx` | [`Dockerfile`][buildx-docker] in docker/buildx | [Apache-2.0][buildx-licence] |
| `docker/coverage` | `oracle/inputs/docker-coverage.dockerfile`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `docker/moby` | [`Dockerfile`][moby-docker] in moby/moby | [Apache-2.0][moby-licence] |
| `docker/next-with-docker` | [`examples/with-docker/Dockerfile`][next-docker] in vercel/next.js | [MIT][next-licence] |
| `docker/postgres-bookworm` | [`18/bookworm/Dockerfile`][postgres-docker] in docker-library/postgres | [MIT][postgres-docker-licence] |
| `docker/prometheus` | [`Dockerfile`][prometheus-docker] in prometheus/prometheus | [Apache-2.0][prometheus-make-licence] |
| `docker/python-slim-bookworm` | [`3.13/slim-bookworm/Dockerfile`][python-docker] in docker-library/python | [MIT][python-docker-licence] |
| `docker/sample` | [`samples/docker.sample`][docker-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `elixir/coverage` | `oracle/inputs/elixir-coverage.ex`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `elixir/elixir-keyword` | [`lib/elixir/lib/keyword.ex`][elixir-keyword] in elixir-lang/elixir | [Apache-2.0][elixir-lang-licence] |
| `elixir/elixir-map-set` | [`lib/elixir/lib/map_set.ex`][elixir-map-set] in elixir-lang/elixir | [Apache-2.0][elixir-lang-licence] |
| `elixir/elixir-range` | [`lib/elixir/lib/range.ex`][elixir-range] in elixir-lang/elixir | [Apache-2.0][elixir-lang-licence] |
| `elixir/embedded` | `oracle/inputs/elixir-embedded.ex`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `elixir/live-dashboard-layered-graph` | [`lib/phoenix/live_dashboard/components/layered_graph_component.ex`][live-dashboard-layered-graph] in phoenixframework/phoenix_live_dashboard | [MIT][live-dashboard-licence] |
| `elixir/live-dashboard-nav-bar` | [`lib/phoenix/live_dashboard/components/nav_bar_component.ex`][live-dashboard-nav-bar] in phoenixframework/phoenix_live_dashboard | [MIT][live-dashboard-licence] |
| `elixir/live-dashboard-table` | [`lib/phoenix/live_dashboard/components/table_component.ex`][live-dashboard-table] in phoenixframework/phoenix_live_dashboard | [MIT][live-dashboard-licence] |
| `elixir/sample` | [`samples/elixir.sample`][elixir-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `erb/coverage` | `oracle/inputs/erb-coverage.erb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `erb/devise-confirmation-instructions` | [`app/views/devise/mailer/confirmation_instructions.html.erb`][devise-confirmation-instructions] in heartcombo/devise | [MIT][devise-licence] |
| `erb/devise-error-messages` | [`app/views/devise/shared/_error_messages.html.erb`][devise-error-messages] in heartcombo/devise | [MIT][devise-licence] |
| `erb/devise-registrations-edit` | [`app/views/devise/registrations/edit.html.erb`][devise-registrations-edit] in heartcombo/devise | [MIT][devise-licence] |
| `erb/devise-sessions-new` | [`app/views/devise/sessions/new.html.erb`][devise-sessions-new] in heartcombo/devise | [MIT][devise-licence] |
| `erb/devise-shared-links` | [`app/views/devise/shared/_links.html.erb`][devise-shared-links] in heartcombo/devise | [MIT][devise-licence] |
| `erb/rails-app-layout` | [`railties/lib/rails/generators/rails/app/templates/app/views/layouts/application.html.erb.tt`][rails-app-layout] in rails/rails | [MIT][railties-licence] |
| `erb/rails-info-notes` | [`railties/lib/rails/templates/rails/info/notes.html.erb`][rails-info-notes] in rails/rails | [MIT][railties-licence] |
| `erb/rails-mailer-email` | [`railties/lib/rails/templates/rails/mailers/email.html.erb`][rails-mailer-email] in rails/rails | [MIT][railties-licence] |
| `erb/rails-pwa-offline` | [`railties/lib/rails/generators/rails/app/templates/app/views/pwa/offline.html.erb`][rails-pwa-offline] in rails/rails | [MIT][railties-licence] |
| `erb/rails-rescues-diagnostics` | [`actionpack/lib/action_dispatch/middleware/templates/rescues/diagnostics.html.erb`][rails-rescues-diagnostics] in rails/rails | [MIT][actionpack-licence] |
| `erb/rails-rescues-source` | [`actionpack/lib/action_dispatch/middleware/templates/rescues/_source.html.erb`][rails-rescues-source] in rails/rails | [MIT][actionpack-licence] |
| `erb/rails-routes-table` | [`actionpack/lib/action_dispatch/middleware/templates/routes/_table.html.erb`][rails-routes-table] in rails/rails | [MIT][actionpack-licence] |
| `erb/rails-scaffold-form` | [`railties/lib/rails/generators/erb/scaffold/templates/_form.html.erb.tt`][rails-scaffold-form] in rails/rails | [MIT][railties-licence] |
| `erb/rails-welcome` | [`railties/lib/rails/templates/rails/welcome/index.html.erb`][rails-welcome] in rails/rails | [MIT][railties-licence] |
| `erb/sample` | [`samples/erb.sample`][erb-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `gdscript/godot-paint-control` | [`gui/gd_paint/paint_control.gd`][godot-paint-control] in godotengine/godot-demo-projects | [MIT][godot-demos-licence] |
| `gdscript/godot-voxel-chunk` | [`3d/voxel/world/chunk.gd`][godot-voxel-chunk] in godotengine/godot-demo-projects | [MIT][godot-demos-licence] |
| `gdscript/grammar-example-1` | [`syntaxes/examples/gdscript1.gd`][godot-vscode-example-1] in godotengine/godot-vscode-plugin | [MIT][godot-vscode-licence] |
| `gdscript/grammar-example-2` | [`syntaxes/examples/gdscript2.gd`][godot-vscode-example-2] in godotengine/godot-vscode-plugin | [MIT][godot-vscode-licence] |
| `gdscript/sample` | [`samples/gdscript.sample`][gdscript-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `go/cobra-command` | [`command.go`][cobra-command] in spf13/cobra | [Apache-2.0][cobra-licence] |
| `go/coverage` | `oracle/inputs/go-coverage.go`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `go/gin-context` | [`context.go`][gin-context] in gin-gonic/gin | [MIT][gin-licence] |
| `go/go-json-decode` | [`src/encoding/json/decode.go`][go-json-decode] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-printer-declarations` | [`src/go/printer/testdata/declarations.golden`][go-printer-declarations] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-printer-go2numbers` | [`src/go/printer/testdata/go2numbers.golden`][go-printer-go2numbers] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-slices` | [`src/slices/slices.go`][go-slices] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-sort` | [`src/sort/sort.go`][go-sort] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-test-bug141` | [`test/fixedbugs/bug141.go`][go-test-bug141] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-test-issue15528` | [`test/fixedbugs/issue15528.go`][go-test-issue15528] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-types-typeinst1` | [`src/internal/types/testdata/check/typeinst1.go`][go-types-typeinst1] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/go-types-typeparams` | [`src/internal/types/testdata/check/typeparams.go`][go-types-typeparams] in golang/go | [BSD-3-Clause][golang-licence] |
| `go/kubernetes-sets` | [`staging/src/k8s.io/apimachinery/pkg/util/sets/set.go`][kubernetes-sets] in kubernetes/kubernetes | [Apache-2.0][kubernetes-licence] |
| `go/prometheus-storage` | [`storage/interface.go`][prometheus-storage] in prometheus/prometheus | [Apache-2.0][prometheus-licence] |
| `go/sample` | [`samples/go.sample`][go-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `go/testify-assertions` | [`assert/assertions.go`][testify-assertions] in stretchr/testify | [MIT][testify-licence] |
| `go/viper-viper` | [`viper.go`][viper-viper] in spf13/viper | [MIT][viper-licence] |
| `go/zap-logger` | [`logger.go`][zap-logger] in uber-go/zap | [MIT][zap-licence] |
| `graphql/codegen-apollo-tooling` | [`dev-test/apollo-tooling/schema.graphql`][codegen-apollo] in dotansimha/graphql-code-generator | [MIT][codegen-licence] |
| `graphql/coverage` | `oracle/inputs/graphql-coverage.graphql`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `graphql/graphiql-kitchen-sink` | [`packages/codemirror-graphql/src/__tests__/kitchen-sink.graphql`][graphiql-kitchen-sink] in graphql/graphiql | [MIT][graphiql-licence] |
| `graphql/graphiql-merged-query` | [`packages/graphiql-toolkit/src/graphql-helpers/__tests__/__queries__/mergedQueryWithSchema.graphql`][graphiql-merged-query] in graphql/graphiql | [MIT][graphiql-licence] |
| `graphql/graphiql-schema-kitchen-sink` | [`packages/codemirror-graphql/src/__tests__/schema-kitchen-sink.graphql`][graphiql-schema-kitchen-sink] in graphql/graphiql | [MIT][graphiql-licence] |
| `graphql/graphiql-star-wars` | [`packages/graphql-language-service-server/src/__tests__/__schema__/StarWarsSchema.graphql`][graphiql-star-wars] in graphql/graphiql | [MIT][graphiql-licence] |
| `graphql/graphql-js-kitchen-sink` | [`benchmark/kitchen-sink.graphql`][graphql-js-kitchen-sink] in graphql/graphql-js | [MIT][graphql-js-licence] |
| `graphql/sample` | [`samples/graphql.sample`][graphql-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `haml/filters` | `oracle/inputs/haml-filters.haml`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `haml/haml-helpers` | [`test/haml/templates/helpers.haml`][haml-helpers] in haml/haml | [MIT][haml-licence] |
| `haml/haml-just-stuff` | [`test/haml/templates/just_stuff.haml`][haml-just-stuff] in haml/haml | [MIT][haml-licence] |
| `haml/haml-real-sample` | [`benchmark/etc/real_sample.haml`][haml-real-sample] in haml/haml | [MIT][haml-licence] |
| `haml/haml-silent-script` | [`test/haml/templates/silent_script.haml`][haml-silent-script] in haml/haml | [MIT][haml-licence] |
| `haml/haml-standard` | [`test/haml/templates/standard.haml`][haml-standard] in haml/haml | [MIT][haml-licence] |
| `haml/haml-whitespace` | [`test/haml/templates/whitespace_handling.haml`][haml-whitespace] in haml/haml | [MIT][haml-licence] |
| `haml/sample` | [`samples/haml.sample`][haml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `haskell/aeson-builder` | [`src/Data/Aeson/Encoding/Builder.hs`][aeson-builder] in haskell/aeson | [BSD-3-Clause][aeson-licence] |
| `haskell/aeson-decoding-text` | [`src/Data/Aeson/Decoding/Text.hs`][aeson-decoding-text] in haskell/aeson | [BSD-3-Clause][aeson-licence] |
| `haskell/aeson-github` | [`examples/src/GitHub.hs`][aeson-github] in haskell/aeson | [BSD-3-Clause][aeson-licence] |
| `haskell/coverage` | `oracle/inputs/haskell-coverage.hs`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `haskell/sample` | [`samples/haskell.sample`][haskell-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `html/bootstrap3-visual-modal` | [`js/tests/visual/modal.html`][bootstrap3-visual-modal] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `html/h5bp-404` | [`src/404.html`][h5bp-404] in h5bp/html5-boilerplate | [MIT][h5bp-licence] |
| `html/h5bp-index` | [`src/index.html`][h5bp-index] in h5bp/html5-boilerplate | [MIT][h5bp-licence] |
| `html/jquery-delegatetest` | [`test/delegatetest.html`][jquery-delegatetest] in jquery/jquery | [MIT][jquery-licence] |
| `html/jquery-qunit-fixture` | [`test/data/qunit-fixture.html`][jquery-qunit-fixture] in jquery/jquery | [MIT][jquery-licence] |
| `html/mathjax-mml-chtml` | [`page/mml-chtml.html`][mathjax-mml-chtml] in mathjax/MathJax-demos-web | [Apache-2.0][mathjax-demos-licence] |
| `html/mathjax-tex-mml` | [`page/tex-mml.html`][mathjax-tex-mml] in mathjax/MathJax-demos-web | [Apache-2.0][mathjax-demos-licence] |
| `html/sample` | [`samples/html.sample`][html-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `html/three-animation-keyframes` | [`examples/webgl_animation_keyframes.html`][three-animation-keyframes] in mrdoob/three.js | [MIT][three-licence] |
| `html/three-css3d-periodictable` | [`examples/css3d_periodictable.html`][three-css3d-periodictable] in mrdoob/three.js | [MIT][three-licence] |
| `html/three-geometry-colors` | [`examples/webgl_geometry_colors.html`][three-geometry-colors] in mrdoob/three.js | [MIT][three-licence] |
| `html/three-loader-gltf` | [`examples/webgl_loader_gltf.html`][three-loader-gltf] in mrdoob/three.js | [MIT][three-licence] |
| `html/todomvc-backbone` | [`examples/backbone/index.html`][todomvc-backbone] in tastejs/todomvc | [MIT][todomvc-licence] |
| `html/todomvc-jquery` | [`examples/jquery/index.html`][todomvc-jquery] in tastejs/todomvc | [MIT][todomvc-licence] |
| `html/todomvc-knockoutjs` | [`examples/knockoutjs/index.html`][todomvc-knockoutjs] in tastejs/todomvc | [MIT][todomvc-licence] |
| `html/todomvc-lit` | [`examples/lit/index.html`][todomvc-lit] in tastejs/todomvc | [MIT][todomvc-licence] |
| `html/todomvc-polymer` | [`examples/polymer/index.html`][todomvc-polymer] in tastejs/todomvc | [BSD-3-Clause][todomvc-polymer-licence] |
| `html-derivative/vue-classic-grid` | [`packages/vue/examples/classic/grid.html`][derivative-vue-classic-grid] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-classic-todomvc` | [`packages/vue/examples/classic/todomvc.html`][derivative-vue-classic-todomvc] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-commits` | [`packages/vue/examples/composition/commits.html`][derivative-vue-commits] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-grid` | [`packages/vue/examples/composition/grid.html`][derivative-vue-grid] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-hydration-visible` | [`packages/vue/__tests__/e2e/hydration-strat-visible.html`][derivative-vue-hydration-visible] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-markdown` | [`packages/vue/examples/composition/markdown.html`][derivative-vue-markdown] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-modal` | [`packages/vue/examples/transition/modal.html`][derivative-vue-modal] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-svg` | [`packages/vue/examples/composition/svg.html`][derivative-vue-svg] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-todomvc` | [`packages/vue/examples/composition/todomvc.html`][derivative-vue-todomvc] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-transition-list` | [`packages/vue/examples/transition/list.html`][derivative-vue-transition-list] in vuejs/core | [MIT][vue-licence] |
| `html-derivative/vue-tree` | [`packages/vue/examples/composition/tree.html`][derivative-vue-tree] in vuejs/core | [MIT][vue-licence] |
| `ini/black-tox` | [`tox.ini`][black-tox] in psf/black | [MIT][black-licence] |
| `ini/coverage` | `oracle/inputs/ini-coverage.ini`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ini/django-tox` | [`tox.ini`][django-tox] in django/django | [BSD-3-Clause][django-licence] |
| `ini/kafka-consumer` | [`config/consumer.properties`][kafka-consumer] in apache/kafka | [Apache-2.0][kafka-licence] |
| `ini/kafka-server` | [`config/server.properties`][kafka-server] in apache/kafka | [Apache-2.0][kafka-licence] |
| `ini/pytest-tox` | [`tox.ini`][pytest-tox] in pytest-dev/pytest | [MIT][pytest-licence] |
| `ini/sample` | [`samples/ini.sample`][ini-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `ini/sqlalchemy-tox` | [`tox.ini`][sqlalchemy-tox] in sqlalchemy/sqlalchemy | [MIT][sqlalchemy-licence] |
| `ini/tomcat-catalina` | [`conf/catalina.properties`][tomcat-catalina] in apache/tomcat | [Apache-2.0][tomcat-licence] |
| `java/commons-lang-range` | [`Range.java`][commons-lang-range] in apache/commons-lang | [Apache-2.0][commons-lang-licence] |
| `java/commons-lang-validate` | [`Validate.java`][commons-lang-validate] in apache/commons-lang | [Apache-2.0][commons-lang-licence] |
| `java/coverage` | `oracle/inputs/java-coverage.java`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `java/google-java-format-b380299722` | [`B380299722.input`][google-java-format-b380299722] in google/google-java-format | [Apache-2.0][google-java-format-licence] |
| `java/google-java-format-i574` | [`I574.input`][google-java-format-i574] in google/google-java-format | [Apache-2.0][google-java-format-licence] |
| `java/google-java-format-lambda-hug` | [`LambdaHug.input`][google-java-format-lambda-hug] in google/google-java-format | [Apache-2.0][google-java-format-licence] |
| `java/google-java-format-m1` | [`M1.input`][google-java-format-m1] in google/google-java-format | [Apache-2.0][google-java-format-licence] |
| `java/gson-gson` | [`Gson.java`][gson-gson] in google/gson | [Apache-2.0][gson-licence] |
| `java/gson-json-reader` | [`JsonReader.java`][gson-json-reader] in google/gson | [Apache-2.0][gson-licence] |
| `java/guava-immutable-list` | [`ImmutableList.java`][guava-immutable-list] in google/guava | [Apache-2.0][guava-licence] |
| `java/guava-preconditions` | [`Preconditions.java`][guava-preconditions] in google/guava | [Apache-2.0][guava-licence] |
| `java/rewrite-switch-patterns` | [`SwitchPatternMatchingTest.java`][rewrite-switch-patterns] in openrewrite/rewrite | [Apache-2.0][rewrite-licence] |
| `java/sample` | [`samples/java.sample`][java-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `javascript/axios-xhr` | [`lib/adapters/xhr.js`][axios-xhr] in axios/axios | [MIT][axios-licence] |
| `javascript/coverage` | `oracle/inputs/typescript-coverage.ts`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `javascript/express-application` | [`lib/application.js`][express-application] in expressjs/express | [MIT][express-licence] |
| `javascript/express-response` | [`lib/response.js`][express-response] in expressjs/express | [MIT][express-licence] |
| `javascript/sample` | [`samples/javascript.sample`][js-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `javascript/tsc-await-using-declarations-11` | [`awaitUsingDeclarations.11.ts`][tsc-await-using-declarations-11] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-check-merged-global-umdsymbol` | [`checkMergedGlobalUMDSymbol.ts`][tsc-check-merged-global-umdsymbol] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-contextual-type-caching` | [`contextualTypeCaching.ts`][tsc-contextual-type-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-contravariant-only-inference-from-annotated-function-js` | [`contravariantOnlyInferenceFromAnnotatedFunctionJs.ts`][tsc-contravariant-only-inference-from-annotated-function-js] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-declaration-emit-higher-order-retained-generics` | [`declarationEmitHigherOrderRetainedGenerics.ts`][tsc-declaration-emit-higher-order-retained-generics] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-divergent-accessors-types7` | [`divergentAccessorsTypes7.ts`][tsc-divergent-accessors-types7] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-fix-signature-caching` | [`fixSignatureCaching.ts`][tsc-fix-signature-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-jsdoc-disallowed-in-typescript` | [`jsdocDisallowedInTypescript.ts`][tsc-jsdoc-disallowed-in-typescript] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-jsx-namespace-prefix-in-name` | [`jsxNamespacePrefixInName.tsx`][tsc-jsx-namespace-prefix-in-name] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-parserharness` | [`parserharness.ts`][tsc-parserharness] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-plain-jsgrammar-errors` | [`plainJSGrammarErrors.ts`][tsc-plain-jsgrammar-errors] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-source-map-validation-statements` | [`sourceMapValidationStatements.ts`][tsc-source-map-validation-statements] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-tsx-react-emit-entities` | [`tsxReactEmitEntities.tsx`][tsc-tsx-react-emit-entities] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-variadic-tuples1` | [`variadicTuples1.ts`][tsc-variadic-tuples1] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `javascript/tsc-yield-expression-inner-comment-emit` | [`yieldExpressionInnerCommentEmit.ts`][tsc-yield-expression-inner-comment-emit] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `json/coverage` | `oracle/inputs/json-coverage.json`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `json/sample` | [`samples/json.sample`][json-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `json/vscode-typescript-grammar` | [`extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json`][vscode-ts-grammar] in microsoft/vscode | [MIT][vscode-licence] |
| `json/vue-package` | [`package.json`][vue-package] in vuejs/core | [MIT][vue-licence] |
| `json/zod-package` | [`packages/zod/package.json`][zod-package] in colinhacks/zod | [MIT][zod-licence] |
| `jsonc/coverage` | `oracle/inputs/jsonc-coverage.jsonc`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `jsonc/sample` | [`samples/jsonc.sample`][jsonc-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsonc/vscode-launch` | [`.vscode/launch.json`][vscode-launch] in microsoft/vscode | [MIT][vscode-licence] |
| `jsonl/coverage` | `oracle/inputs/jsonl-coverage.jsonl`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `jsonl/rag-output` | [`output.jsonl`][rag-output] in jxnl/n-levels-of-rag | [MIT][rag-licence] |
| `jsonl/rag-output-50` | [`output-50.jsonl`][rag-output-50] in jxnl/n-levels-of-rag | [MIT][rag-licence] |
| `jsonl/sample` | [`samples/jsonl.sample`][jsonl-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsx/coverage` | `oracle/inputs/typescript-coverage.ts`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `jsx/preact-keys` | [`test/browser/keys.test.jsx`][preact-keys] in preactjs/preact | [MIT][preact-licence] |
| `jsx/preact-render` | [`compat/test/browser/render.test.jsx`][preact-render] in preactjs/preact | [MIT][preact-licence] |
| `jsx/sample` | [`samples/jsx.sample`][jsx-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `jsx/tsc-arbitrary-module-namespace-identifiers-syntax` | [`arbitraryModuleNamespaceIdentifiers_syntax.ts`][tsc-arbitrary-module-namespace-identifiers-syntax] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-async-with-var-shadowing-es6` | [`asyncWithVarShadowing_es6.ts`][tsc-async-with-var-shadowing-es6] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-await-using-declarations-1` | [`awaitUsingDeclarations.1.ts`][tsc-await-using-declarations-1] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-checker-initialization-crash` | [`checkerInitializationCrash.ts`][tsc-checker-initialization-crash] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-computed-property-names-contextual-type6-es5` | [`computedPropertyNamesContextualType6_ES5.ts`][tsc-computed-property-names-contextual-type6-es5] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-contravariant-only-inference-from-annotated-function-js` | [`contravariantOnlyInferenceFromAnnotatedFunctionJs.ts`][tsc-contravariant-only-inference-from-annotated-function-js] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-control-flow-optional-chain` | [`controlFlowOptionalChain.ts`][tsc-control-flow-optional-chain] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-correlated-unions` | [`correlatedUnions.ts`][tsc-correlated-unions] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-declaration-emit-binding-patterns-unused` | [`declarationEmitBindingPatternsUnused.ts`][tsc-declaration-emit-binding-patterns-unused] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-declaration-emit-higher-order-retained-generics` | [`declarationEmitHigherOrderRetainedGenerics.ts`][tsc-declaration-emit-higher-order-retained-generics] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-divergent-accessors-types7` | [`divergentAccessorsTypes7.ts`][tsc-divergent-accessors-types7] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-fix-signature-caching` | [`fixSignatureCaching.ts`][tsc-fix-signature-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-jsdoc-disallowed-in-typescript` | [`jsdocDisallowedInTypescript.ts`][tsc-jsdoc-disallowed-in-typescript] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-jsx-complex-signature-has-applicability-error` | [`jsxComplexSignatureHasApplicabilityError.tsx`][tsc-jsx-complex-signature-has-applicability-error] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-jsx-namespace-prefix-in-name` | [`jsxNamespacePrefixInName.tsx`][tsc-jsx-namespace-prefix-in-name] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-parserharness` | [`parserharness.ts`][tsc-parserharness] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-source-map-validation-statements` | [`sourceMapValidationStatements.ts`][tsc-source-map-validation-statements] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-tsx-react-emit-entities` | [`tsxReactEmitEntities.tsx`][tsc-tsx-react-emit-entities] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `jsx/tsc-yield-expression-inner-comment-emit` | [`yieldExpressionInnerCommentEmit.ts`][tsc-yield-expression-inner-comment-emit] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `julia/base-abstractset` | [`base/abstractset.jl`][julia-base-abstractset] in JuliaLang/julia | [MIT][julia-licence] |
| `julia/base-sort` | [`base/sort.jl`][julia-base-sort] in JuliaLang/julia | [MIT][julia-licence] |
| `julia/coverage` | `oracle/inputs/julia-coverage.jl`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `julia/sample` | [`samples/julia.sample`][julia-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `julia/strings-util` | [`base/strings/util.jl`][julia-strings-util] in JuliaLang/julia | [MIT][julia-licence] |
| `kotlin/coroutines-produce` | [`Produce.kt`][coroutines-produce] in Kotlin/kotlinx.coroutines | [Apache-2.0][coroutines-licence] |
| `kotlin/coroutines-state-flow` | [`StateFlow.kt`][coroutines-state-flow] in Kotlin/kotlinx.coroutines | [Apache-2.0][coroutines-licence] |
| `kotlin/coverage` | `oracle/inputs/kotlin-coverage.kt`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `kotlin/exposed-op` | [`Op.kt`][exposed-op] in JetBrains/Exposed | [Apache-2.0][exposed-licence] |
| `kotlin/ktor-logging` | [`Logging.kt`][ktor-logging] in ktorio/ktor | [Apache-2.0][ktor-licence] |
| `kotlin/moshi-util` | [`Util.kt`][moshi-util] in square/moshi | [Apache-2.0][moshi-licence] |
| `kotlin/okhttp-build` | [`okhttp/build.gradle.kts`][okhttp-build] in square/okhttp | [Apache-2.0][okhttp-licence] |
| `kotlin/okhttp-cookie` | [`Cookie.kt`][okhttp-cookie] in square/okhttp | [Apache-2.0][okhttp-licence] |
| `kotlin/okhttp-request` | [`Request.kt`][okhttp-request] in square/okhttp | [Apache-2.0][okhttp-licence] |
| `kotlin/sample` | [`samples/kotlin.sample`][kotlin-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `kotlin/serialization-json` | [`Json.kt`][serialization-json] in Kotlin/kotlinx.serialization | [Apache-2.0][kotlinx-serialization-licence] |
| `kotlin/turbine-flow-test` | [`FlowTest.kt`][turbine-flow-test] in cashapp/turbine | [Apache-2.0][turbine-licence] |
| `less/antd-button-mixin` | [`components/button/style/mixin.less`][less-antd-button-mixin] in ant-design/ant-design | [MIT][antd-licence] |
| `less/antd-color-palette` | [`components/style/color/colorPalette.less`][less-antd-color-palette] in ant-design/ant-design | [MIT][antd-licence] |
| `less/antd-compact-item` | [`components/style/mixins/compact-item.less`][less-antd-compact-item] in ant-design/ant-design | [MIT][antd-licence] |
| `less/antd-customize` | [`components/style/mixins/customize.less`][less-antd-customize] in ant-design/ant-design | [MIT][antd-licence] |
| `less/bootstrap3-forms` | [`less/forms.less`][less-bootstrap3-forms] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/bootstrap3-mixins-buttons` | [`less/mixins/buttons.less`][less-bootstrap3-mixins-buttons] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/bootstrap3-mixins-gradients` | [`less/mixins/gradients.less`][less-bootstrap3-mixins-gradients] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/bootstrap3-mixins-grid-framework` | [`less/mixins/grid-framework.less`][less-bootstrap3-mixins-grid-framework] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/bootstrap3-navbar` | [`less/navbar.less`][less-bootstrap3-navbar] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/bootstrap3-variables` | [`less/variables.less`][less-bootstrap3-variables] in twbs/bootstrap | [MIT][bootstrap3-licence] |
| `less/coverage` | `oracle/inputs/less-coverage.less`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `less/lessjs-at-rules` | [`at-rules.less`][less-lessjs-at-rules] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-at-rules-compressed` | [`at-rules-compressed.less`][less-lessjs-at-rules-compressed] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-container` | [`container.less`][less-lessjs-container] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-css-3` | [`css-3.less`][less-lessjs-css-3] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-extract-and-length` | [`extract-and-length.less`][less-lessjs-extract-and-length] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-functions` | [`functions.less`][less-lessjs-functions] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-functions-each` | [`functions-each.less`][less-lessjs-functions-each] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-import-reference` | [`import-reference.less`][less-lessjs-import-reference] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-math-strict` | [`css.less`][less-lessjs-math-strict] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-merge` | [`merge.less`][less-lessjs-merge] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-selectors` | [`selectors.less`][less-lessjs-selectors] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-sourcemaps-comprehensive` | [`comprehensive.less`][less-lessjs-sourcemaps-comprehensive] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/lessjs-urls` | [`urls.less`][less-lessjs-urls] in less/less.js | [Apache-2.0][lessjs-licence] |
| `less/sample` | [`samples/less.sample`][less-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `lua/coverage` | `oracle/inputs/lua-coverage.lua`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `lua/inspect` | [`inspect.lua`][inspect-lua] in kikito/inspect.lua | [MIT][inspect-lua-licence] |
| `lua/json` | [`json.lua`][json-lua] in rxi/json.lua | [MIT][json-lua-licence] |
| `lua/lua-literals` | [`testes/literals.lua`][lua-literals] in lua/lua | [MIT][lua-licence] |
| `lua/lua-locals` | [`testes/locals.lua`][lua-locals] in lua/lua | [MIT][lua-licence] |
| `lua/luals-basic` | [`meta/template/basic.lua`][luals-basic] in LuaLS/lua-language-server | [MIT][luals-licence] |
| `lua/luals-string` | [`meta/template/string.lua`][luals-string] in LuaLS/lua-language-server | [MIT][luals-licence] |
| `lua/luals-table` | [`meta/template/table.lua`][luals-table] in LuaLS/lua-language-server | [MIT][luals-licence] |
| `lua/luarocks-util` | [`src/luarocks/core/util.lua`][luarocks-util] in luarocks/luarocks | [MIT][luarocks-licence] |
| `lua/lume` | [`lume.lua`][lume] in rxi/lume | [MIT][lume-licence] |
| `lua/penlight-stringx` | [`lua/pl/stringx.lua`][penlight-stringx] in lunarmodules/Penlight | [MIT][penlight-licence] |
| `lua/sample` | [`samples/lua.sample`][lua-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `make/cjson` | [`Makefile`][cjson-make] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `make/coverage` | `oracle/inputs/make-coverage.mk`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `make/helm` | [`Makefile`][helm-make] in helm/helm | [Apache-2.0][helm-licence] |
| `make/minikube` | [`Makefile`][minikube-make] in kubernetes/minikube | [Apache-2.0][minikube-licence] |
| `make/moby` | [`Makefile`][moby-make] in moby/moby | [Apache-2.0][moby-licence] |
| `make/node` | [`Makefile`][node-make] in nodejs/node | [MIT][node-licence] |
| `make/nvm` | [`Makefile`][nvm-make] in nvm-sh/nvm | [MIT][nvm-make-licence] |
| `make/prometheus-common` | [`Makefile.common`][prometheus-make] in prometheus/prometheus | [Apache-2.0][prometheus-make-licence] |
| `make/sample` | [`samples/make.sample`][make-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `make/zlib` | [`Makefile.in`][zlib-make] in madler/zlib | [zlib][zlib-licence] |
| `markdown/argument-parser-declaring-arguments` | [`DeclaringArguments.md`][argument-parser-declaring-arguments] in apple/swift-argument-parser | [Apache-2.0][argument-parser-licence] |
| `markdown/argument-parser-getting-started` | [`GettingStarted.md`][argument-parser-getting-started] in apple/swift-argument-parser | [Apache-2.0][argument-parser-licence] |
| `markdown/benchmark-user-guide` | [`docs/user_guide.md`][benchmark-user-guide] in google/benchmark | [Apache-2.0][benchmark-licence] |
| `markdown/black-other-tools` | [`docs/guides/using_black_with_other_tools.md`][black-other-tools] in psf/black | [MIT][black-licence] |
| `markdown/book-ownership` | [`src/ch04-01-what-is-ownership.md`][book-ownership] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/book-strings` | [`src/ch08-02-strings.md`][book-strings] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/cargo-release` | [`doc/contrib/src/process/release.md`][cargo-release] in rust-lang/cargo | [MIT or Apache-2.0][cargo-licence] |
| `markdown/catch2-tutorial` | [`docs/tutorial.md`][catch2-tutorial] in catchorg/Catch2 | [BSL-1.0][catch2-licence] |
| `markdown/cjson-readme` | [`README.md`][cjson-readme] in DaveGamble/cJSON | [MIT][cjson-licence] |
| `markdown/clap-readme` | [`README.md`][clap-readme] in clap-rs/clap | [MIT or Apache-2.0][clap-licence] |
| `markdown/click-man-readme` | [`README.md`][click-man-readme] in click-contrib/click-man | [MIT][click-man-licence] |
| `markdown/cobra-user-guide` | [`site/content/user_guide.md`][cobra-user-guide] in spf13/cobra | [Apache-2.0][cobra-licence] |
| `markdown/coffeescript-classes` | [`documentation/sections/breaking_changes_classes.md`][coffeescript-classes] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `markdown/coffeescript-super-this` | [`documentation/sections/breaking_changes_super_this.md`][coffeescript-super-this] in jashkenas/coffeescript | [MIT][coffeescript-licence] |
| `markdown/coverage` | `oracle/inputs/markdown-coverage.md`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `markdown/dart-args-readme` | [`pkgs/args/README.md`][dart-args-readme] in dart-lang/core | [BSD-3-Clause][dart-core-licence] |
| `markdown/dart-http-readme` | [`pkgs/http/README.md`][dart-http-readme] in dart-lang/http | [BSD-3-Clause][dart-http-licence] |
| `markdown/device-virtual-readme` | [`README.md`][device-virtual-readme] in edgexfoundry/device-virtual-go | [Apache-2.0][device-virtual-licence] |
| `markdown/devise-readme` | [`README.md`][devise-readme] in heartcombo/devise | [MIT][devise-licence] |
| `markdown/eslint-mcp` | [`docs/src/use/mcp.md`][eslint-mcp] in eslint/eslint | [MIT][eslint-licence] |
| `markdown/eslint-no-restricted-imports` | [`docs/src/rules/no-restricted-imports.md`][eslint-imports] in eslint/eslint | [MIT][eslint-licence] |
| `markdown/fastapi-docker` | [`docs/en/docs/deployment/docker.md`][fastapi-docker] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `markdown/fastapi-readme` | [`README.md`][fastapi-readme] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `markdown/gin-readme` | [`README.md`][gin-readme] in gin-gonic/gin | [MIT][gin-licence] |
| `markdown/gson-user-guide` | [`UserGuide.md`][gson-user-guide] in google/gson | [Apache-2.0][gson-licence] |
| `markdown/highlightjs-readme` | [`README.md`][highlightjs-readme] in highlightjs/highlight.js | [BSD-3-Clause][highlightjs-licence] |
| `markdown/httparser-readme` | [`README.md`][httparser-readme] in antlabs/httparser | [Apache-2.0][httparser-licence] |
| `markdown/javapoet-readme` | [`README.md`][javapoet-readme] in square/javapoet | [Apache-2.0][javapoet-licence] |
| `markdown/jekyll-front-matter` | [`docs/_docs/front-matter.md`][jekyll-front-matter] in jekyll/jekyll | [MIT][jekyll-licence] |
| `markdown/julia-functions` | [`doc/src/manual/functions.md`][julia-manual-functions] in JuliaLang/julia | [MIT][julia-licence] |
| `markdown/julia-strings` | [`doc/src/manual/strings.md`][julia-manual-strings] in JuliaLang/julia | [MIT][julia-licence] |
| `markdown/kingfisher-common-tasks-cache` | [`CommonTasks_Cache.md`][kingfisher-common-tasks-cache] in onevcat/Kingfisher | [MIT][kingfisher-licence] |
| `markdown/knex-schema-builder` | [`docs/src/guide/schema-builder.md`][knex-schema] in knex/knex | [MIT][knex-licence] |
| `markdown/ky-readme` | [`readme.md`][ky-readme] in sindresorhus/ky | [MIT][ky-licence] |
| `markdown/latexrun-readme` | [`README.md`][latexrun-readme] in aclements/latexrun | [MIT][latexrun-licence] |
| `markdown/markdownlint-blanks-around-headings-arrays` | [`blanks-around-headings-arrays.md`][markdown-markdownlint-blanks-around-headings-arrays] in DavidAnson/markdownlint | [MIT][markdownlint-licence] |
| `markdown/markdownlint-link-style-autolink-only` | [`link-style-autolink-only.md`][markdown-markdownlint-link-style-autolink-only] in DavidAnson/markdownlint | [MIT][markdownlint-licence] |
| `markdown/next-with-docker-export-readme` | [`examples/with-docker-export-output/README.md`][next-docker-export-readme] in vercel/next.js | [MIT][next-licence] |
| `markdown/next-with-docker-readme` | [`examples/with-docker/README.md`][next-docker-readme] in vercel/next.js | [MIT][next-licence] |
| `markdown/nvm-readme` | [`README.md`][nvm-readme] in nvm-sh/nvm | [MIT][nvm-licence] |
| `markdown/pgvector-readme` | [`README.md`][pgvector-readme] in pgvector/pgvector | [PostgreSQL][pgvector-licence] |
| `markdown/poetry-faq` | [`docs/faq.md`][poetry-faq] in python-poetry/poetry | [MIT][poetry-licence] |
| `markdown/powershell-releasing` | [`docs/maintainers/releasing.md`][powershell-releasing] in PowerShell/PowerShell | [MIT][powershell-licence] |
| `markdown/powershell-windows-build` | [`docs/building/windows-core.md`][powershell-windows-build] in PowerShell/PowerShell | [MIT][powershell-licence] |
| `markdown/preact-readme` | [`README.md`][preact-readme] in preactjs/preact | [MIT][preact-licence] |
| `markdown/prettier-plugins` | [`docs/plugins.md`][prettier-plugins] in prettier/prettier | [MIT][prettier-licence] |
| `markdown/pyenv-readme` | [`README.md`][pyenv-readme] in pyenv/pyenv | [MIT][pyenv-licence] |
| `markdown/rack-readme` | [`README.md`][rack-readme] in rack/rack | [MIT][rack-licence] |
| `markdown/rich-readme` | [`README.md`][rich-readme] in Textualize/rich | [MIT][rich-licence] |
| `markdown/sample` | [`samples/markdown.sample`][markdown-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `markdown/scala3-enums` | [`docs/_docs/reference/enums/enums.md`][scala3-enums] in scala/scala3 | [Apache-2.0][scala3-licence] |
| `markdown/scala3-givens` | [`docs/_docs/reference/contextual/givens.md`][scala3-givens] in scala/scala3 | [Apache-2.0][scala3-licence] |
| `markdown/serde-readme` | [`README.md`][serde-readme] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `markdown/shadcn-data-table` | [`apps/v4/content/docs/components/radix/data-table.mdx`][shadcn-data-table] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `markdown/shadcn-message-scroller` | [`apps/v4/content/docs/components/radix/message-scroller.mdx`][shadcn-message-scroller] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `markdown/tokio-readme` | [`README.md`][tokio-readme] in tokio-rs/tokio | [MIT][tokio-licence] |
| `markdown/traefik-docker` | [`docs/content/getting-started/docker.md`][traefik-docker] in traefik/traefik | [MIT][traefik-licence] |
| `markdown/typeorm-indexes` | [`docs/docs/indexes.md`][typeorm-indexes] in typeorm/typeorm | [MIT][typeorm-licence] |
| `markdown/unity-assertions` | [`docs/UnityAssertionsReference.md`][unity-assertions] in ThrowTheSwitch/Unity | [MIT][unity-licence] |
| `markdown/unity-getting-started` | [`docs/UnityGettingStartedGuide.md`][unity-getting-started] in ThrowTheSwitch/Unity | [MIT][unity-licence] |
| `markdown/vite-api-plugin` | [`docs/guide/api-plugin.md`][vite-api-plugin] in vitejs/vite | [MIT][vite-licence] |
| `markdown/vite-features` | [`docs/guide/features.md`][vite-features] in vitejs/vite | [MIT][vite-licence] |
| `markdown/vite-ssr` | [`docs/guide/ssr.md`][vite-ssr] in vitejs/vite | [MIT][vite-licence] |
| `markdown/vitepress-runtime-api` | [`docs/en/reference/runtime-api.md`][vitepress-runtime-api] in vuejs/vitepress | [MIT][vitepress-licence] |
| `markdown/vitest-migration-jest` | [`docs/guide/migration/jest.md`][vitest-jest] in vitest-dev/vitest | [MIT][vitest-licence] |
| `markdown/zod-api` | [`packages/docs/content/api.mdx`][zod-api] in colinhacks/zod | [MIT][zod-licence] |
| `markdown/zod-readme` | [`packages/zod/README.md`][zod-readme] in colinhacks/zod | [MIT][zod-licence] |
| `matlab/chebfun-feval` | [`@chebfun/feval.m`][chebfun-feval] in chebfun/chebfun | [BSD-3-Clause][chebfun-licence] |
| `matlab/chebfun-roots` | [`@chebfun/roots.m`][chebfun-roots] in chebfun/chebfun | [BSD-3-Clause][chebfun-licence] |
| `matlab/grammar-arguments-block` | [`test/t13ArgumentsBlock.m`][matlab-grammar-arguments-block] in mathworks/MATLAB-Language-grammar | [BSD-2-Clause][matlab-grammar-licence] |
| `matlab/grammar-property-validation` | [`test/t08PropertyValidation.m`][matlab-grammar-property-validation] in mathworks/MATLAB-Language-grammar | [BSD-2-Clause][matlab-grammar-licence] |
| `matlab/sample` | [`samples/matlab.sample`][matlab-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `mermaid/api-sequence` | [`docs/diagrams/mermaid-api-sequence.mmd`][mermaid-api-sequence] in mermaid-js/mermaid | [MIT][mermaid-licence] |
| `mermaid/coverage` | `oracle/inputs/mermaid-coverage.mmd`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `mermaid/flowchart-code-flow` | [`docs/diagrams/flowchart-code-flow.mmd`][mermaid-flowchart-code-flow] in mermaid-js/mermaid | [MIT][mermaid-licence] |
| `mermaid/mindmap-sequence` | [`docs/diagrams/mindmap-implementation-sequence.mmd`][mermaid-mindmap-sequence] in mermaid-js/mermaid | [MIT][mermaid-licence] |
| `mermaid/sample` | [`samples/mermaid.sample`][mermaid-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `nix/coverage` | `oracle/inputs/nix-coverage.nix`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `nix/home-manager-flake` | [`flake.nix`][home-manager-flake] in nix-community/home-manager | [MIT][home-manager-licence] |
| `nix/home-manager-tmux` | [`modules/programs/tmux.nix`][home-manager-tmux] in nix-community/home-manager | [MIT][home-manager-licence] |
| `nix/nixpkgs-lists` | [`lib/lists.nix`][nixpkgs-lists] in NixOS/nixpkgs | [MIT][nixpkgs-licence] |
| `nix/nixpkgs-ripgrep` | [`pkgs/by-name/ri/ripgrep/package.nix`][nixpkgs-ripgrep] in NixOS/nixpkgs | [MIT][nixpkgs-licence] |
| `nix/sample` | [`samples/nix.sample`][nix-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `objective-c/afnetworking-http-session-manager` | [`AFNetworking/AFHTTPSessionManager.h`][afnetworking-http-session-manager] in AFNetworking/AFNetworking | [MIT][afnetworking-licence] |
| `objective-c/afnetworking-url-session-manager` | [`AFNetworking/AFURLSessionManager.m`][afnetworking-url-session-manager] in AFNetworking/AFNetworking | [MIT][afnetworking-licence] |
| `objective-c/coverage` | `oracle/inputs/objective-c-coverage.m`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `objective-c/masonry-constraint-maker` | [`Masonry/MASConstraintMaker.m`][masonry-constraint-maker] in SnapKit/Masonry | [MIT][masonry-licence] |
| `objective-c/mbprogresshud` | [`MBProgressHUD.m`][mbprogresshud] in jdg/MBProgressHUD | [MIT][mbprogresshud-licence] |
| `objective-c/sample` | [`samples/objective-c.sample`][objective-c-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `objective-c/sdwebimage-graphics-image-renderer` | [`SDWebImage/Core/SDGraphicsImageRenderer.m`][sdwebimage-graphics-image-renderer] in SDWebImage/SDWebImage | [MIT][sdwebimage-licence] |
| `objective-c/sdwebimage-image-cache` | [`SDWebImage/Core/SDImageCache.h`][sdwebimage-image-cache] in SDWebImage/SDWebImage | [MIT][sdwebimage-licence] |
| `objective-c/sdwebimage-manager` | [`SDWebImage/Core/SDWebImageManager.m`][sdwebimage-manager] in SDWebImage/SDWebImage | [MIT][sdwebimage-licence] |
| `objective-c/yykit-macro` | [`YYKit/Base/YYKitMacro.h`][yykit-macro] in ibireme/YYKit | [MIT][yykit-licence] |
| `objective-cpp/coverage` | `oracle/inputs/objective-cpp-coverage.mm`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `objective-cpp/grammar-misc-002` | [`language_examples/misc_002.mm`][objcpp-grammar-misc-002] in jeff-hykin/better-objcpp-syntax | [MIT][objcpp-grammar-licence] |
| `objective-cpp/grammar-misc-003` | [`language_examples/misc_003.mm`][objcpp-grammar-misc-003] in jeff-hykin/better-objcpp-syntax | [MIT][objcpp-grammar-licence] |
| `objective-cpp/objective-c-coverage` | `oracle/inputs/objective-c-coverage.m`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `objective-cpp/react-native-component-view-registry` | [`RCTComponentViewRegistry.mm`][react-native-component-view-registry] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-dev-loading-view` | [`RCTDevLoadingView.mm`][react-native-dev-loading-view] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-device-info` | [`RCTDeviceInfo.mm`][react-native-device-info] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-legacy-interop-coordinator` | [`RCTLegacyViewManagerInteropCoordinator.mm`][react-native-legacy-interop-coordinator] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-mounting-manager` | [`RCTMountingManager.mm`][react-native-mounting-manager] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-surface-presenter` | [`RCTSurfacePresenter.mm`][react-native-surface-presenter] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/react-native-virtual-view` | [`RCTVirtualViewComponentView.mm`][react-native-virtual-view] in facebook/react-native | [MIT][react-native-licence] |
| `objective-cpp/sample` | [`samples/objective-cpp.sample`][objective-cpp-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `pascal/horse-commons` | [`src/Horse.Commons.pas`][horse-commons] in HashLoad/horse | [MIT][horse-licence] |
| `pascal/horse-http2-framing` | [`src/Horse.Core.Http2.Framing.pas`][horse-http2-framing] in HashLoad/horse | [MIT][horse-licence] |
| `pascal/horse-websocket-console` | [`samples/lazarus/console_websocket/console_websocket.lpr`][horse-websocket-console] in HashLoad/horse | [MIT][horse-licence] |
| `pascal/r8brain-dll` | [`DLL/r8bsrc.pas`][r8brain-dll] in avaneev/r8brain-free-src | [MIT][r8brain-licence] |
| `pascal/sample` | [`samples/pascal.sample`][pascal-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `perl/embedded` | `oracle/inputs/perl-embedded.pl`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `perl/sample` | [`samples/perl.sample`][perl-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `perl/sqitch-engine-pg` | [`lib/App/Sqitch/Engine/pg.pm`][sqitch-engine-pg] in sqitchers/sqitch | [MIT][sqitch-licence] |
| `perl/sqitch-engine-sqlite` | [`lib/App/Sqitch/Engine/sqlite.pm`][sqitch-engine-sqlite] in sqitchers/sqitch | [MIT][sqitch-licence] |
| `php/coverage` | `oracle/inputs/php-coverage.php`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `php/embedded` | `oracle/inputs/php-embedded.php`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `php/laravel-postgres-grammar` | [`src/Illuminate/Database/Query/Grammars/PostgresGrammar.php`][laravel-postgres-grammar] in laravel/framework | [MIT][laravel-licence] |
| `php/laravel-router` | [`src/Illuminate/Routing/Router.php`][laravel-router] in laravel/framework | [MIT][laravel-licence] |
| `php/sample` | [`samples/php.sample`][php-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `postcss/custom-media-basic` | [`plugins/postcss-custom-media/test/basic.css`][csstools-custom-media-basic] in csstools/postcss-plugins | [MIT][csstools-custom-media-licence] |
| `postcss/custom-selectors-basic` | [`plugins/postcss-custom-selectors/test/basic.css`][csstools-custom-selectors-basic] in csstools/postcss-plugins | [MIT][csstools-custom-selectors-licence] |
| `postcss/mixins-arguments` | [`plugins/postcss-mixins/test/arguments.css`][csstools-mixins-arguments] in csstools/postcss-plugins | [MIT-0][csstools-mixins-licence] |
| `postcss/mixins-spaces` | [`plugins/postcss-mixins/test/spaces.css`][csstools-mixins-spaces] in csstools/postcss-plugins | [MIT-0][csstools-mixins-licence] |
| `postcss/nesting-at-nest` | [`plugins/postcss-nesting/test/at-nest.css`][csstools-nesting-at-nest] in csstools/postcss-plugins | [MIT-0][csstools-nesting-licence] |
| `postcss/nesting-spec-examples` | [`plugins/postcss-nesting/test/spec-examples.css`][csstools-nesting-spec-examples] in csstools/postcss-plugins | [MIT-0][csstools-nesting-licence] |
| `postcss/sample` | [`samples/postcss.sample`][postcss-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `postcss/tailwind-preflight` | [`packages/tailwindcss/preflight.css`][tailwind-preflight] in tailwindlabs/tailwindcss | [MIT][tailwind-licence] |
| `powershell/coverage` | `oracle/inputs/powershell-coverage.ps1`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `powershell/dotnet-install` | [`src/dotnet-install.ps1`][dotnet-install] in dotnet/install-scripts | [MIT][dotnet-install-licence] |
| `powershell/install-powershell` | [`tools/install-powershell.ps1`][install-powershell] in PowerShell/PowerShell | [MIT][powershell-licence] |
| `powershell/sample` | [`samples/powershell.sample`][powershell-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `powershell/set-release-tag` | [`tools/releaseBuild/setReleaseTag.ps1`][set-release-tag] in PowerShell/PowerShell | [MIT][powershell-licence] |
| `proto/coverage` | `oracle/inputs/proto-coverage.proto`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `proto/googleapis-http` | [`google/api/http.proto`][googleapis-http] in googleapis/googleapis | [Apache-2.0][googleapis-licence] |
| `proto/googleapis-operations` | [`google/longrunning/operations.proto`][googleapis-operations] in googleapis/googleapis | [Apache-2.0][googleapis-licence] |
| `proto/grpc-route-guide` | [`examples/protos/route_guide.proto`][grpc-route-guide] in grpc/grpc | [Apache-2.0][grpc-licence] |
| `proto/sample` | [`samples/proto.sample`][proto-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `python/black-attribute-access-on-number-literals` | [`attribute_access_on_number_literals.py`][python-black-attribute-access-on-number-literals] in psf/black | [MIT][black-licence] |
| `python/black-bytes-docstring` | [`bytes_docstring.py`][python-black-bytes-docstring] in psf/black | [MIT][black-licence] |
| `python/black-fmtonoff` | [`fmtonoff.py`][python-black-fmtonoff] in psf/black | [MIT][black-licence] |
| `python/black-pep-701` | [`pep_701.py`][python-black-pep-701] in psf/black | [MIT][black-licence] |
| `python/black-python2-detection` | [`python2_detection.py`][python-black-python2-detection] in psf/black | [MIT][black-licence] |
| `python/black-string-prefixes` | [`string_prefixes.py`][python-black-string-prefixes] in psf/black | [MIT][black-licence] |
| `python/black-string-quotes` | [`string_quotes.py`][python-black-string-quotes] in psf/black | [MIT][black-licence] |
| `python/black-strings` | [`src/black/strings.py`][black-strings] in psf/black | [MIT][black-licence] |
| `python/coverage` | `oracle/inputs/python-coverage.py`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `python/fastapi-routing` | [`fastapi/routing.py`][fastapi-routing] in tiangolo/fastapi | [MIT][fastapi-licence] |
| `python/flask-helpers` | [`src/flask/helpers.py`][flask-helpers] in pallets/flask | [BSD-3-Clause][flask-licence] |
| `python/pytest-python` | [`src/_pytest/python.py`][pytest-python] in pytest-dev/pytest | [MIT][pytest-licence] |
| `python/requests-models` | [`src/requests/models.py`][requests-models] in psf/requests | [Apache-2.0][requests-licence] |
| `python/rich-console` | [`rich/console.py`][rich-console] in Textualize/rich | [MIT][rich-licence] |
| `python/ruff-class` | [`class.py`][python-ruff-class] in astral-sh/ruff | [MIT][ruff-licence] |
| `python/sample` | [`samples/python.sample`][python-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `ruby/coverage` | `oracle/inputs/ruby-coverage.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/cpp-lua-heredocs` | `oracle/inputs/ruby-cpp-lua-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/devise` | [`lib/devise.rb`][devise] in heartcombo/devise | [MIT][devise-licence] |
| `ruby/end-html` | `oracle/inputs/ruby-end-html.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/heredocs` | `oracle/inputs/ruby-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/html-heredocs` | `oracle/inputs/ruby-html-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/jekyll-site` | [`lib/jekyll/site.rb`][jekyll-site] in jekyll/jekyll | [MIT][jekyll-licence] |
| `ruby/js-heredocs` | `oracle/inputs/ruby-js-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/puma-server` | [`lib/puma/server.rb`][puma-server] in puma/puma | [BSD-3-Clause][puma-licence] |
| `ruby/rack-request` | [`lib/rack/request.rb`][rack-request] in rack/rack | [MIT][rack-licence] |
| `ruby/rack-response` | [`lib/rack/response.rb`][rack-response] in rack/rack | [MIT][rack-licence] |
| `ruby/rack-spec-headers` | [`test/spec_headers.rb`][rack-spec-headers] in rack/rack | [MIT][rack-licence] |
| `ruby/rack-spec-lint` | [`test/spec_lint.rb`][rack-spec-lint] in rack/rack | [MIT][rack-licence] |
| `ruby/rails-callbacks` | [`activesupport/lib/active_support/callbacks.rb`][rails-callbacks] in rails/rails | [MIT][rails-licence] |
| `ruby/rails-inflector` | [`activesupport/lib/active_support/inflector/methods.rb`][rails-inflector] in rails/rails | [MIT][rails-licence] |
| `ruby/rake-reduce-compat-test` | [`test/test_rake_reduce_compat.rb`][rake-reduce-compat-test] in ruby/rake | [MIT][rake-licence] |
| `ruby/rake-task` | [`lib/rake/task.rb`][rake-task] in ruby/rake | [MIT][rake-licence] |
| `ruby/rspec-example` | [`lib/rspec/core/example.rb`][rspec-example] in rspec/rspec-core | [MIT][rspec-core-licence] |
| `ruby/rubocop-cop-base` | [`lib/rubocop/cop/base.rb`][rubocop-cop-base] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-duplicated-gem` | [`lib/rubocop/cop/bundler/duplicated_gem.rb`][rubocop-duplicated-gem] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-line-length-spec` | [`spec/rubocop/cop/layout/line_length_spec.rb`][rubocop-line-length-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-percent-literal-delimiters-spec` | [`spec/rubocop/cop/style/percent_literal_delimiters_spec.rb`][rubocop-percent-literal-delimiters-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-quoted-symbols-spec` | [`spec/rubocop/cop/style/quoted_symbols_spec.rb`][rubocop-quoted-symbols-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-redundant-percent-q-spec` | [`spec/rubocop/cop/style/redundant_percent_q_spec.rb`][rubocop-redundant-percent-q-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-regexp-literal-spec` | [`spec/rubocop/cop/style/regexp_literal_spec.rb`][rubocop-regexp-literal-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-symbol-array-spec` | [`spec/rubocop/cop/style/symbol_array_spec.rb`][rubocop-symbol-array-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/rubocop-useless-assignment-spec` | [`spec/rubocop/cop/lint/useless_assignment_spec.rb`][rubocop-useless-assignment-spec] in rubocop/rubocop | [MIT][rubocop-licence] |
| `ruby/sample` | [`samples/ruby.sample`][ruby-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `ruby/thor` | [`lib/thor.rb`][thor] in rails/thor | [MIT][thor-licence] |
| `ruby/xml-heredocs` | `oracle/inputs/ruby-xml-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `ruby/yaml-heredocs` | `oracle/inputs/ruby-yaml-heredocs.rb`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `rust/core-macros` | [`library/core/src/macros/mod.rs`][rust-core-macros] in rust-lang/rust | [MIT or Apache-2.0][rust-licence] |
| `rust/coverage` | `oracle/inputs/rust-coverage.rs`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `rust/ripgrep-searcher` | [`crates/searcher/src/searcher/mod.rs`][ripgrep-searcher] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `rust/sample` | [`samples/rust.sample`][rust-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `rust/serde-de` | [`serde_core/src/de/mod.rs`][serde-de] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `sass/bulma-functions` | [`sass/utilities/functions.sass`][sass-bulma-functions] in jgthms/bulma | [MIT][bulma-sass-licence] |
| `sass/bulma-generic` | [`sass/base/generic.sass`][sass-bulma-generic] in jgthms/bulma | [MIT][bulma-sass-licence] |
| `sass/bulma-mixins` | [`sass/utilities/mixins.sass`][sass-bulma-mixins] in jgthms/bulma | [MIT][bulma-sass-licence] |
| `sass/bulma-navbar` | [`sass/components/navbar.sass`][sass-bulma-navbar] in jgthms/bulma | [MIT][bulma-sass-licence] |
| `sass/coverage` | `oracle/inputs/sass-coverage.sass`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `sass/sample` | [`samples/sass.sample`][sass-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `scala/circe-build` | [`build.sbt`][circe-build] in circe/circe | [Apache-2.0][circe-licence] |
| `scala/circe-json` | [`modules/core/shared/src/main/scala/io/circe/Json.scala`][circe-json] in circe/circe | [Apache-2.0][circe-licence] |
| `scala/coverage` | `oracle/inputs/scala-coverage.scala`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `scala/sample` | [`samples/scala.sample`][scala-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `scala/scala3-boundary` | [`library/src/scala/util/boundary.scala`][scala3-boundary] in scala/scala3 | [Apache-2.0][scala3-licence] |
| `scala/scala3-source-position` | [`compiler/src/dotty/tools/dotc/util/SourcePosition.scala`][scala3-source-position] in scala/scala3 | [Apache-2.0][scala3-licence] |
| `scss/angular-menu` | [`menu.scss`][scss-angular-menu] in angular/components | [MIT][angular-components-licence] |
| `scss/bootstrap-button` | [`scss/buttons/_button.scss`][scss-bootstrap-button] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-functions` | [`scss/_functions.scss`][scss-bootstrap-functions] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-mixins-form-validation` | [`_form-validation.scss`][scss-bootstrap-mixins-form-validation] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-mixins-grid` | [`scss/mixins/_grid.scss`][scss-bootstrap-mixins-grid] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-navbar` | [`scss/_navbar.scss`][scss-bootstrap-navbar] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-root` | [`scss/_root.scss`][scss-bootstrap-root] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bootstrap-utilities` | [`scss/_utilities.scss`][scss-bootstrap-utilities] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `scss/bulma-button` | [`sass/elements/button.scss`][scss-bulma-button] in jgthms/bulma | [MIT][bulma-licence] |
| `scss/bulma-functions` | [`sass/utilities/functions.scss`][scss-bulma-functions] in jgthms/bulma | [MIT][bulma-licence] |
| `scss/bulma-mixins` | [`sass/utilities/mixins.scss`][scss-bulma-mixins] in jgthms/bulma | [MIT][bulma-licence] |
| `scss/coverage` | `oracle/inputs/scss-coverage.scss`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `scss/sample` | [`samples/scss.sample`][scss-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `shellscript/bats-core-bats` | [`libexec/bats-core/bats`][bats-core-bats] in bats-core/bats-core | [MIT][bats-core-licence] |
| `shellscript/bats-core-gather-tests` | [`libexec/bats-core/bats-gather-tests`][bats-core-gather-tests] in bats-core/bats-core | [MIT][bats-core-licence] |
| `shellscript/coverage` | `oracle/inputs/shellscript-coverage.sh`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `shellscript/docker-install` | [`install.sh`][docker-install] in docker/docker-install | [Apache-2.0][docker-install-licence] |
| `shellscript/homebrew-uninstall` | [`uninstall.sh`][homebrew-uninstall] in Homebrew/install | [BSD-2-Clause][homebrew-install-licence] |
| `shellscript/nvm` | [`nvm.sh`][nvm] in nvm-sh/nvm | [MIT][nvm-licence] |
| `shellscript/ohmyzsh-alias-finder-test` | [`plugins/alias-finder/tests/test_run.sh`][ohmyzsh-alias-finder-test] in ohmyzsh/ohmyzsh | [MIT][ohmyzsh-licence] |
| `shellscript/pyenv-init` | [`libexec/pyenv-init`][pyenv-init] in pyenv/pyenv | [MIT][pyenv-licence] |
| `shellscript/ruby-build` | [`bin/ruby-build`][ruby-build] in rbenv/ruby-build | [MIT][ruby-build-licence] |
| `shellscript/sample` | [`samples/shellscript.sample`][shell-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `shellscript/unicode-words` | `oracle/inputs/shellscript-unicode-words.sh`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `sql/coverage` | `oracle/inputs/sql-coverage.sql`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `sql/northwind-pubs` | [`samples/databases/northwind-pubs/instpubs.sql`][pubs] in microsoft/sql-server-samples | [MIT][sqlserver-licence] |
| `sql/postgres-create-table` | [`src/test/regress/sql/create_table.sql`][pg-create] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/postgres-select` | [`src/test/regress/sql/select.sql`][pg-select] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/postgres-with` | [`src/test/regress/sql/with.sql`][pg-with] in postgres/postgres | [PostgreSQL][postgres-licence] |
| `sql/sample` | [`samples/sql.sample`][sql-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `sql/ssms-create-table` | [`samples/features/ssms-templates/Sql/Table/Create Table.sql`][ssms-create] in microsoft/sql-server-samples | [MIT][sqlserver-licence] |
| `svelte/coverage` | `oracle/inputs/svelte-coverage.svelte`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `svelte/kit-root` | [`packages/kit/src/runtime/components/root.svelte`][svelte-kit-root] in sveltejs/kit | [MIT][svelte-kit-licence] |
| `svelte/sample` | [`samples/svelte.sample`][svelte-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `svelte/site-kit-docs-contents` | [`packages/site-kit/src/lib/docs/DocsContents.svelte`][svelte-site-kit-docs-contents] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-mobile-menu` | [`packages/site-kit/src/lib/nav/MobileMenu.svelte`][svelte-site-kit-mobile-menu] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-nav` | [`packages/site-kit/src/lib/nav/Nav.svelte`][svelte-site-kit-nav] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-search-box` | [`packages/site-kit/src/lib/search/SearchBox.svelte`][svelte-site-kit-search-box] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-search-result-list` | [`packages/site-kit/src/lib/search/SearchResultList.svelte`][svelte-site-kit-search-result-list] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-text` | [`packages/site-kit/src/lib/components/Text.svelte`][svelte-site-kit-text] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/site-kit-tooltip` | [`packages/site-kit/src/lib/docs/Tooltip.svelte`][svelte-site-kit-tooltip] in sveltejs/svelte.dev | [MIT][svelte-site-kit-licence] |
| `svelte/tests-const-tag-component` | [`packages/svelte/tests/runtime-legacy/samples/const-tag-component/main.svelte`][svelte-tests-const-tag-component] in sveltejs/svelte | [MIT][svelte-licence] |
| `svelte/tests-destructure-async-assignments` | [`packages/svelte/tests/runtime-runes/samples/destructure-async-assignments/main.svelte`][svelte-tests-destructure-async-assignments] in sveltejs/svelte | [MIT][svelte-licence] |
| `svelte/tests-form-default-value` | [`packages/svelte/tests/runtime-runes/samples/form-default-value/main.svelte`][svelte-tests-form-default-value] in sveltejs/svelte | [MIT][svelte-licence] |
| `svelte/tests-props-default-value-behavior` | [`packages/svelte/tests/runtime-runes/samples/props-default-value-behavior/main.svelte`][svelte-tests-props-default-value-behavior] in sveltejs/svelte | [MIT][svelte-licence] |
| `svelte/tests-style-directive-mutations` | [`packages/svelte/tests/runtime-runes/samples/style-directive-mutations/main.svelte`][svelte-tests-style-directive-mutations] in sveltejs/svelte | [MIT][svelte-licence] |
| `swift/alamofire-af-error` | [`AFError.swift`][alamofire-af-error] in Alamofire/Alamofire | [MIT][alamofire-licence] |
| `swift/algorithms-chunked` | [`Chunked.swift`][algorithms-chunked] in apple/swift-algorithms | [Apache-2.0][algorithms-licence] |
| `swift/argument-parser-argument` | [`Argument.swift`][argument-parser-argument] in apple/swift-argument-parser | [Apache-2.0][argument-parser-licence] |
| `swift/argument-parser-command-parser` | [`CommandParser.swift`][argument-parser-command-parser] in apple/swift-argument-parser | [Apache-2.0][argument-parser-licence] |
| `swift/composable-store` | [`Store.swift`][composable-store] in pointfreeco/swift-composable-architecture | [MIT][composable-licence] |
| `swift/coverage` | `oracle/inputs/swift-coverage.swift`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `swift/kingfisher-image-processor` | [`ImageProcessor.swift`][kingfisher-image-processor] in onevcat/Kingfisher | [MIT][kingfisher-licence] |
| `swift/nio-codec` | [`Codec.swift`][nio-codec] in apple/swift-nio | [Apache-2.0][nio-licence] |
| `swift/sample` | [`samples/swift.sample`][swift-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `swift/snapshot-assert-snapshot` | [`AssertSnapshot.swift`][snapshot-assert-snapshot] in pointfreeco/swift-snapshot-testing | [MIT][snapshot-testing-licence] |
| `swift/string-processing-algorithms-tests` | [`AlgorithmsTests.swift`][string-processing-algorithms-tests] in swiftlang/swift-experimental-string-processing | [Apache-2.0][string-processing-licence] |
| `swift/string-processing-custom-tests` | [`CustomTests.swift`][string-processing-custom-tests] in swiftlang/swift-experimental-string-processing | [Apache-2.0][string-processing-licence] |
| `swift/swift-test-decl-func-keyword-argument-labels` | [`test/decl/func/keyword-argument-labels.swift`][swift-test-decl-func-keyword-argument-labels] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-decl-func-rethrows` | [`test/decl/func/rethrows.swift`][swift-test-decl-func-rethrows] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-decl-import-import` | [`test/decl/import/import.swift`][swift-test-decl-import-import] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-decl-protocol-protocol-enum-witness` | [`test/decl/protocol/protocol_enum_witness.swift`][swift-test-decl-protocol-protocol-enum-witness] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-decl-protocol-protocols` | [`test/decl/protocol/protocols.swift`][swift-test-decl-protocol-protocols] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-expr-expressions` | [`test/expr/expressions.swift`][swift-test-expr-expressions] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-expr-primary-selector-selector` | [`test/expr/primary/selector/selector.swift`][swift-test-expr-primary-selector-selector] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-expr-unary-async-await` | [`test/expr/unary/async_await.swift`][swift-test-expr-unary-async-await] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-conditionalcompilation-basicparseerrors` | [`test/Parse/ConditionalCompilation/basicParseErrors.swift`][swift-test-parse-conditionalcompilation-basicparseerrors] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-escaped-identifiers` | [`test/Parse/escaped_identifiers.swift`][swift-test-parse-escaped-identifiers] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-operator-decl` | [`test/Parse/operator_decl.swift`][swift-test-parse-operator-decl] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-raw-string` | [`test/Parse/raw_string.swift`][swift-test-parse-raw-string] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-raw-string-errors` | [`test/Parse/raw_string_errors.swift`][swift-test-parse-raw-string-errors] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-trailing-comma` | [`test/Parse/trailing-comma.swift`][swift-test-parse-trailing-comma] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/swift-test-parse-type-expr` | [`test/Parse/type_expr.swift`][swift-test-parse-type-expr] in swiftlang/swift | [Apache-2.0 with the Runtime Library Exception][swift-licence] |
| `swift/testing-runner-tests` | [`RunnerTests.swift`][testing-runner-tests] in swiftlang/swift-testing | [Apache-2.0][swift-testing-licence] |
| `toml/burntsushi-example` | [`_example/example.toml`][burntsushi-example] in BurntSushi/toml | [MIT][burntsushi-toml-licence] |
| `toml/cargo-manifest` | [`Cargo.toml`][cargo-manifest] in rust-lang/cargo | [MIT or Apache-2.0][cargo-licence] |
| `toml/clap-manifest` | [`Cargo.toml`][clap-manifest] in clap-rs/clap | [MIT or Apache-2.0][clap-licence] |
| `toml/coverage` | `oracle/inputs/toml-coverage.toml`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `toml/poetry-pyproject` | [`pyproject.toml`][poetry-pyproject] in python-poetry/poetry | [MIT][poetry-licence] |
| `toml/ripgrep-manifest` | [`Cargo.toml`][ripgrep-manifest] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `toml/ruff-pyproject` | [`pyproject.toml`][ruff-pyproject] in astral-sh/ruff | [MIT][ruff-licence] |
| `toml/sample` | [`samples/toml.sample`][toml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `toml/tokio-manifest` | [`tokio/Cargo.toml`][tokio-manifest] in tokio-rs/tokio | [MIT][tokio-licence] |
| `toml/toml-test-inline-table-key-dotted-01` | [`tests/valid/inline-table/key-dotted-01.toml`][toml-test-inline-table-key-dotted-01] in toml-lang/toml-test | [MIT][toml-test-licence] |
| `toml/toml-test-key-quoted-unicode` | [`tests/valid/key/quoted-unicode.toml`][toml-test-key-quoted-unicode] in toml-lang/toml-test | [MIT][toml-test-licence] |
| `tsx/coverage` | `oracle/inputs/typescript-coverage.ts`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `tsx/sample` | [`samples/tsx.sample`][tsx-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `tsx/shadcn-chart` | [`apps/v4/registry/new-york-v4/ui/chart.tsx`][shadcn-chart] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `tsx/shadcn-combobox` | [`apps/v4/registry/new-york-v4/ui/combobox.tsx`][shadcn-combobox] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `tsx/shadcn-sidebar` | [`apps/v4/registry/new-york-v4/ui/sidebar.tsx`][shadcn-sidebar] in shadcn-ui/ui | [MIT][shadcn-licence] |
| `tsx/tsc-async-with-var-shadowing-es6` | [`asyncWithVarShadowing_es6.ts`][tsc-async-with-var-shadowing-es6] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-await-using-declarations-11` | [`awaitUsingDeclarations.11.ts`][tsc-await-using-declarations-11] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-co-and-contra-variant-inferences3` | [`coAndContraVariantInferences3.ts`][tsc-co-and-contra-variant-inferences3] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-conflicting-declarations-import-from-namespace2` | [`conflictingDeclarationsImportFromNamespace2.ts`][tsc-conflicting-declarations-import-from-namespace2] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-contextual-type-caching` | [`contextualTypeCaching.ts`][tsc-contextual-type-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-contravariant-only-inference-from-annotated-function-js` | [`contravariantOnlyInferenceFromAnnotatedFunctionJs.ts`][tsc-contravariant-only-inference-from-annotated-function-js] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-declaration-emit-higher-order-retained-generics` | [`declarationEmitHigherOrderRetainedGenerics.ts`][tsc-declaration-emit-higher-order-retained-generics] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-dependent-destructured-variables-from-nested-patterns` | [`dependentDestructuredVariablesFromNestedPatterns.ts`][tsc-dependent-destructured-variables-from-nested-patterns] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-divergent-accessors-types7` | [`divergentAccessorsTypes7.ts`][tsc-divergent-accessors-types7] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-fix-signature-caching` | [`fixSignatureCaching.ts`][tsc-fix-signature-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-jsdoc-disallowed-in-typescript` | [`jsdocDisallowedInTypescript.ts`][tsc-jsdoc-disallowed-in-typescript] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-jsx-namespace-prefix-in-name` | [`jsxNamespacePrefixInName.tsx`][tsc-jsx-namespace-prefix-in-name] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-parserharness` | [`parserharness.ts`][tsc-parserharness] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-plain-jsgrammar-errors` | [`plainJSGrammarErrors.ts`][tsc-plain-jsgrammar-errors] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-source-map-validation-statements` | [`sourceMapValidationStatements.ts`][tsc-source-map-validation-statements] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-tsx-react-emit-entities` | [`tsxReactEmitEntities.tsx`][tsc-tsx-react-emit-entities] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `tsx/tsc-yield-expression-inner-comment-emit` | [`yieldExpressionInnerCommentEmit.ts`][tsc-yield-expression-inner-comment-emit] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/coverage` | `oracle/inputs/typescript-coverage.ts`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `typescript/ky-core` | [`source/core/Ky.ts`][ky-core] in sindresorhus/ky | [MIT][ky-licence] |
| `typescript/sample` | [`samples/typescript.sample`][ts-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `typescript/tsc-assertion-type-predicates1` | [`assertionTypePredicates1.ts`][tsc-assertion-type-predicates1] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-async-with-var-shadowing-es6` | [`asyncWithVarShadowing_es6.ts`][tsc-async-with-var-shadowing-es6] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-await-using-declarations-11` | [`awaitUsingDeclarations.11.ts`][tsc-await-using-declarations-11] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-check-merged-global-umdsymbol` | [`checkMergedGlobalUMDSymbol.ts`][tsc-check-merged-global-umdsymbol] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-const-assertions` | [`constAssertions.ts`][tsc-const-assertions] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-constructor-with-incomplete-type-annotation` | [`constructorWithIncompleteTypeAnnotation.ts`][tsc-constructor-with-incomplete-type-annotation] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-contravariant-only-inference-from-annotated-function-js` | [`contravariantOnlyInferenceFromAnnotatedFunctionJs.ts`][tsc-contravariant-only-inference-from-annotated-function-js] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-declaration-emit-higher-order-retained-generics` | [`declarationEmitHigherOrderRetainedGenerics.ts`][tsc-declaration-emit-higher-order-retained-generics] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-declaration-emit-mapped-type-preserves-type-parameter-constraint` | [`declarationEmitMappedTypePreservesTypeParameterConstraint.ts`][tsc-declaration-emit-mapped-type-preserves-type-parameter-constraint] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-duplicate-identifier-binding-element-in-parameter-declaration1` | [`duplicateIdentifierBindingElementInParameterDeclaration1.ts`][tsc-duplicate-identifier-binding-element-in-parameter-declaration1] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-erasable-syntax-only` | [`erasableSyntaxOnly.ts`][tsc-erasable-syntax-only] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-fix-signature-caching` | [`fixSignatureCaching.ts`][tsc-fix-signature-caching] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-intra-expression-inferences` | [`intraExpressionInferences.ts`][tsc-intra-expression-inferences] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-jsdoc-disallowed-in-typescript` | [`jsdocDisallowedInTypescript.ts`][tsc-jsdoc-disallowed-in-typescript] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-source-map-validation-statements` | [`sourceMapValidationStatements.ts`][tsc-source-map-validation-statements] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/tsc-yield-expression-inner-comment-emit` | [`yieldExpressionInnerCommentEmit.ts`][tsc-yield-expression-inner-comment-emit] in microsoft/TypeScript | [Apache-2.0][typescript-tests-licence] |
| `typescript/vue-api-create-app` | [`packages/runtime-core/src/apiCreateApp.ts`][vue-app] in vuejs/core | [MIT][vue-licence] |
| `typescript/vue-effect` | [`packages/reactivity/src/effect.ts`][vue-effect] in vuejs/core | [MIT][vue-licence] |
| `typescript/zod-classic-schemas` | [`packages/zod/src/v4/classic/schemas.ts`][zod-schemas] in colinhacks/zod | [MIT][zod-licence] |
| `vb/roslyn-error-method-symbol` | [`src/Compilers/VisualBasic/Portable/Symbols/ErrorMethodSymbol.vb`][roslyn-error-method-symbol] in dotnet/roslyn | [MIT][roslyn-licence] |
| `vb/roslyn-named-type-binder` | [`src/Compilers/VisualBasic/Portable/Binding/NamedTypeBinder.vb`][roslyn-named-type-binder] in dotnet/roslyn | [MIT][roslyn-licence] |
| `vb/sample` | [`samples/vb.sample`][vb-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `vue/coverage` | `oracle/inputs/vue-coverage.vue`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `vue/playground-app` | [`packages-private/sfc-playground/src/App.vue`][vue-playground-app] in vuejs/core | [MIT][vue-licence] |
| `vue/playground-header` | [`packages-private/sfc-playground/src/Header.vue`][vue-playground-header] in vuejs/core | [MIT][vue-licence] |
| `vue/playground-sun` | [`packages-private/sfc-playground/src/icons/Sun.vue`][vue-playground-sun] in vuejs/core | [MIT][vue-licence] |
| `vue/playground-version-select` | [`packages-private/sfc-playground/src/VersionSelect.vue`][vue-playground-version-select] in vuejs/core | [MIT][vue-licence] |
| `vue/sample` | [`samples/vue.sample`][vue-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `vue/vitepress-badge` | [`src/client/theme-default/components/VPBadge.vue`][vitepress-badge] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-carbon-ads` | [`src/client/theme-default/components/VPCarbonAds.vue`][vitepress-carbon-ads] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-image` | [`src/client/theme-default/components/VPImage.vue`][vitepress-image] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-layout` | [`src/client/theme-default/Layout.vue`][vitepress-layout] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-local-search-box` | [`src/client/theme-default/components/VPLocalSearchBox.vue`][vitepress-local-search-box] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-modal-demo` | [`docs/components/ModalDemo.vue`][vitepress-modal-demo] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-nav-bar-hamburger` | [`src/client/theme-default/components/VPNavBarHamburger.vue`][vitepress-nav-bar-hamburger] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-nav-menu-group` | [`src/client/theme-default/components/VPNavMenuGroup.vue`][vitepress-nav-menu-group] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-not-found` | [`src/client/theme-default/NotFound.vue`][vitepress-not-found] in vuejs/vitepress | [MIT][vitepress-licence] |
| `vue/vitepress-switch` | [`src/client/theme-default/components/VPSwitch.vue`][vitepress-switch] in vuejs/vitepress | [MIT][vitepress-licence] |
| `xml/ant-build` | [`build.xml`][ant-build] in apache/ant | [Apache-2.0][ant-licence] |
| `xml/ant-checkstyle-frames` | [`src/etc/checkstyle/checkstyle-frames.xsl`][ant-checkstyle-frames] in apache/ant | [Apache-2.0][ant-licence] |
| `xml/ant-junit-frames` | [`src/etc/junit-frames.xsl`][ant-junit-frames] in apache/ant | [Apache-2.0][ant-licence] |
| `xml/ant-junit-noframes` | [`src/etc/junit-noframes.xsl`][ant-junit-noframes] in apache/ant | [Apache-2.0][ant-licence] |
| `xml/bootstrap-logo` | [`bootstrap-logo.svg`][bootstrap-logo] in twbs/bootstrap | [MIT][bootstrap-licence] |
| `xml/commons-lang-pom` | [`pom.xml`][commons-lang-pom] in apache/commons-lang | [Apache-2.0][commons-lang-licence] |
| `xml/coverage` | `oracle/inputs/xml-coverage.xml`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `xml/coverage-jsp-comment` | `oracle/inputs/xml-coverage-jsp-comment.xml`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `xml/foundation-keyed-archive` | [`NSKeyedUnarchiver-ConcreteValueTest.plist`][foundation-keyed-archive] in apple/swift-corelibs-foundation | [Apache-2.0][foundation-licence] |
| `xml/rome-at` | [`xml/at.xml`][rome-at] in rometools/rome | [Apache-2.0][rome-licence] |
| `xml/rome-slashdot` | [`xml/slashdot.xml`][rome-slashdot] in rometools/rome | [Apache-2.0][rome-licence] |
| `xml/rome-yahoo-top-songs` | [`data/YahooTopSongs.xml`][rome-yahoo-top-songs] in rometools/rome | [Apache-2.0][rome-licence] |
| `xml/sample` | [`samples/xml.sample`][xml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `xml/views-activity-entrance` | [`activity_entrance.xml`][views-activity-entrance] in android/views-widgets-samples | [Apache-2.0][views-licence] |
| `xml/views-lots-of-cards` | [`lots_of_cards.xml`][views-lots-of-cards] in android/views-widgets-samples | [Apache-2.0][views-licence] |
| `xml/xerces-design` | [`design/xml/design.xml`][xerces-design] in apache/xerces2-j | [Apache-2.0][xerces-licence] |
| `xml/xerces-schedule` | [`design/xml/schedule.xml`][xerces-schedule] in apache/xerces2-j | [Apache-2.0][xerces-licence] |
| `xml/xerces-xni-config` | [`docs/xni-config.xml`][xerces-xni-config] in apache/xerces2-j | [Apache-2.0][xerces-licence] |
| `yaml/airflow-compose` | [`airflow-core/docs/howto/docker-compose/docker-compose.yaml`][airflow-compose] in apache/airflow | [Apache-2.0][airflow-licence] |
| `yaml/checkout-test` | [`.github/workflows/test.yml`][checkout-test] in actions/checkout | [MIT][checkout-licence] |
| `yaml/coverage` | `oracle/inputs/yaml-coverage.yaml`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `yaml/petstore-openapi` | [`src/main/resources/openapi.yaml`][petstore-openapi] in swagger-api/swagger-petstore | [Apache-2.0][petstore-licence] |
| `yaml/prometheus-config` | [`documentation/examples/prometheus.yml`][prometheus-config] in prometheus/prometheus | [Apache-2.0][prometheus-licence] |
| `yaml/ripgrep-ci` | [`.github/workflows/ci.yml`][ripgrep-ci] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `yaml/sample` | [`samples/yaml.sample`][yaml-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `yaml/yaml-test-suite-c4hz` | [`C4HZ/in.yaml`][yaml-test-suite-c4hz] in yaml/yaml-test-suite, data branch | [MIT][yaml-test-suite-licence] |
| `yaml/yaml-test-suite-kk5p` | [`KK5P/in.yaml`][yaml-test-suite-kk5p] in yaml/yaml-test-suite, data branch | [MIT][yaml-test-suite-licence] |
| `yaml/yaml-test-suite-w4tn` | [`W4TN/in.yaml`][yaml-test-suite-w4tn] in yaml/yaml-test-suite, data branch | [MIT][yaml-test-suite-licence] |
| `zig/coverage` | `oracle/inputs/zig-coverage.zig`, written for nishiki | MIT or Apache-2.0, as nishiki |
| `zig/sample` | [`samples/zig.sample`][zig-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `zig/std-allocator` | [`lib/std/mem/Allocator.zig`][zig-std-allocator] in ziglang/zig | [MIT][zig-licence] |
| `zig/std-fmt` | [`lib/std/fmt.zig`][zig-std-fmt] in ziglang/zig | [MIT][zig-licence] |
| `zig/zls-build` | [`build.zig`][zls-build] in zigtools/zls | [MIT][zls-licence] |
| `zig/zls-offsets` | [`src/offsets.zig`][zls-offsets] in zigtools/zls | [MIT][zls-licence] |

`ruby/sample`, `ruby/rack-request`, `ruby/rubocop-cop-base`, `ruby/rails-callbacks`, `ruby/rails-inflector`, `ruby/heredocs`, `ruby/js-heredocs`, `ruby/cpp-lua-heredocs`, `ruby/html-heredocs`, `ruby/xml-heredocs`, `ruby/yaml-heredocs` and `ruby/end-html` are the development inputs for Ruby, and `markdown/rack-readme` is the development input for Ruby fences.
`ruby/jekyll-site`, `ruby/devise`, `ruby/puma-server`, `ruby/rspec-example`, `ruby/thor`, `ruby/rake-task` and `markdown/devise-readme` are held out.

Each fixture also has one `.colors` file per theme below: Shiki's colour and font style for each token.

| Theme | Source | Licence |
|:--|:--|:--|
| `themes/github-dark` | [primer/github-vscode-theme][github-theme], through `@shikijs/themes` 4.6.0 | [MIT][github-theme-licence] |
| `themes/one-light` | [akamud/vscode-theme-onelight][one-light], through `@shikijs/themes` 4.6.0 | [MIT][one-light-licence] |
| `themes/catppuccin-mocha` | [catppuccin/vscode][catppuccin], through `@shikijs/themes` 4.6.0 | [MIT][catppuccin-licence] |

`ripgrep-searcher` and `serde-de` are held out: they were not among the inputs the Rust lexer was written against.
`core-macros` and `coverage` are held out too: they were added to exercise the rest of the `rust` grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/book-ownership`, `markdown/book-strings` and `markdown/clap-readme` are held out in the same way: they were added after the other Markdown inputs passed.
`markdown/ky-readme` and `markdown/zod-api` are held out for the TypeScript fences, and `markdown/zod-readme` is the development input for them.
`zod-classic-schemas` and `ky-core` are held out for the TypeScript lexer the same way.
The `tsc-*` and `coverage` inputs of `javascript`, `jsx`, `tsx` and `typescript` are held out: they were added to exercise the rest of those grammars, as [`COVERAGE.md`](COVERAGE.md) counts them.
Their test cases are pinned at TypeScript's `v5.9.3` tag, the release they were picked from, because microsoft/TypeScript's main branch now holds the Go port and no longer has `tests/cases`.
`ruby-build` and `pyenv-init` are held out for shell, for the same reason.
`shellscript/unicode-words` puts combining marks in names, where `\w` decides a token, because no other input has one.
The `bats-core-*`, `docker-install`, `homebrew-uninstall`, `ohmyzsh-alias-finder-test` and `coverage` inputs of `shellscript` are held out: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`python/sample`, `python/requests-models` and `python/black-strings` are the development inputs for Python.
`python/flask-helpers`, `python/rich-console`, `python/fastapi-routing` and `python/pytest-python` are held out.
The `black-*`, `ruff-class` and `coverage` inputs of `python` are held out too: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/fastapi-readme` is the development input for Python fences and `markdown/rich-readme` is held out.
The `markdownlint-*` and `coverage` inputs of `markdown` are held out: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/coverage` opens a fence in each language the grammar names, and ends with HTML blocks, because in the oracle a block-level HTML tag carries on to the end of the document.
`markdown/nvm-readme` is the development input for shell fences and `markdown/pyenv-readme` is held out.
`json/sample`, `json/vue-package`, `jsonc/sample`, `jsonl/sample` and `jsonl/rag-output` are the development inputs for JSON.
`json/zod-package`, `json/vscode-typescript-grammar`, `jsonc/vscode-launch` and `jsonl/rag-output-50` are held out.
`json/coverage`, `jsonc/coverage` and `jsonl/coverage` are held out too: they were added to exercise the rest of the grammars, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/eslint-no-restricted-imports` is the development input for JSON fences and `markdown/eslint-mcp` is held out.
No input has a `jsonc` or `jsonl` fence, so those two spellings are covered by unit tests only.
`sql/sample`, `sql/postgres-select`, `sql/postgres-create-table` and `sql/ssms-create-table` are the development inputs for SQL.
`sql/postgres-with` and `sql/northwind-pubs` are held out.
`sql/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/pgvector-readme` is the development input for `sql` fences, and `markdown/typeorm-indexes` and `markdown/knex-schema-builder` are held out.
The YAML lexer was written from the YAML 1.2 specification and from Shiki's output alone, as invariant 2 allows for a grammar with no licence, and it was checked against a local set of YAML files and fuzzed against Shiki before these inputs were added.
`yaml/sample`, `yaml/checkout-test` and `yaml/petstore-openapi` are its development inputs, and `markdown/jekyll-front-matter` is the development input for front matter and YAML fences.
`yaml/airflow-compose`, `yaml/ripgrep-ci`, `yaml/prometheus-config` and `markdown/traefik-docker` are held out.
The `yaml-test-suite-*` and `coverage` inputs of `yaml`, and the `toml-test-*` and `coverage` inputs of `toml`, are held out too: they were added to exercise the rest of each grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
The YAML test suite keeps its test files on its `data` branch, generated from its `main` branch, whose licence covers them.
Their first scores, before any fix, were 100% for the three YAML files and 64 of 3,517 bytes different for `traefik-docker`, all of them on the line after a `\` in a shell fence, which the fix for shell fences (nishiki-archive#54) makes match.
`make/sample`, `make/nvm`, `make/cjson` and `make/prometheus-common` are the development inputs for Makefiles.
`make/node`, `make/minikube`, `make/moby`, `make/zlib` and `make/helm` are held out.
`make/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/click-man-readme` and `markdown/httparser-readme` are the development inputs for `makefile` fences, and `markdown/latexrun-readme` and `markdown/device-virtual-readme` are held out.
`diff/sample`, `diff/cranelift-stdarch-sve`, `diff/cranelift-stdlib-atomics` and `diff/mir-opt-gvn-cast` are the development inputs for diff.
`diff/cranelift-sysroot-tests`, `diff/mir-opt-gvn-slices` and `diff/vscode-foundry-local-sdk` are held out.
`diff/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/vitest-migration-jest` is the development input for diff fences, and `markdown/vite-ssr` and `markdown/prettier-plugins` are held out.
`graphql/sample`, `graphql/graphql-js-kitchen-sink`, `graphql/graphiql-schema-kitchen-sink` and `graphql/codegen-apollo-tooling` are the development inputs for GraphQL.
`graphql/graphiql-kitchen-sink`, `graphql/graphiql-star-wars` and `graphql/graphiql-merged-query` are held out.
`graphql/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
Their first scores, before any fix, were 351 of 709, 6,316 of 6,338 and 12 of 708 bytes different, all from one cause, a `\s*\n` rule that matched nothing at the end of the text and so closed the rule it sat in.
`docker/sample`, `docker/python-slim-bookworm`, `docker/next-with-docker` and `docker/prometheus` are the development inputs for Dockerfiles.
`docker/moby`, `docker/postgres-bookworm` and `docker/buildx` are held out.
`docker/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/fastapi-docker` is the development input for `dockerfile` fences, and `markdown/next-with-docker-readme` and `markdown/next-with-docker-export-readme` are held out.
The lexer matched every input on its first run, held-out ones included.
`xml/sample`, `xml/ant-build`, `xml/commons-lang-pom`, `xml/views-lots-of-cards`, `xml/ant-junit-frames`, `xml/rome-slashdot`, `xml/xerces-design` and `xml/xerces-xni-config` are the development inputs for XML.
They cover an Ant build, a Maven POM, an Android layout, two XSLT stylesheets, an RSS 1.0 feed, and DOCTYPE declarations with and without an internal subset.
`xml/foundation-keyed-archive`, `xml/bootstrap-logo`, `xml/xerces-schedule`, `xml/ant-checkstyle-frames`, `xml/rome-at`, `xml/rome-yahoo-top-songs`, `xml/ant-junit-noframes` and `xml/views-activity-entrance` are held out.
`xml/coverage` and `xml/coverage-jsp-comment` are held out too: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
Each ends in a rule the grammar never closes, `--` in a comment and `<%--`, so they are two inputs.
They cover a property list, an SVG file, entity declarations, CDATA sections and namespaces.
The lexer matched every input on its first run, held-out ones included.
The Markdown fixtures `markdown/gson-user-guide` and `markdown/javapoet-readme` hold `xml` fences, and were regenerated once the `xml` grammar was loaded, because Shiki's Markdown grammar embeds it.

`c/sample`, `c/cjson` and `c/linenoise` are the development inputs for C, and `markdown/cjson-readme` is the development input for C fences.
`c/libuv-core`, `c/jq-print`, `c/lua-vm`, `c/zlib-inflate`, `markdown/unity-assertions` and `markdown/unity-getting-started` are held out.
Their first scores, before any fix, were 115 of 39,696 bytes different for `libuv-core` and 48 of 9,496 for `jq-print`, from one bug in where a macro's line ends and one in how often the lexer scanned a word, and none for the other two C files or the two Markdown files.
`cpp/sample`, `cpp/leveldb-version-set`, `cpp/rapidjson-reader` and `cpp/spdlog-logger` are the development inputs for C++, and `markdown/catch2-tutorial` is the development input for C++ fences.
`cpp/absl-str-split`, `cpp/benchmark-core`, `cpp/fmt-ranges` and `markdown/benchmark-user-guide` are held out.
Their first scores, before any fix, were 239 of 18,266 bytes different for `absl-str-split`, 327 of 28,465 for `benchmark-core`, 2,762 of 23,892 for `fmt-ranges` and 14,391 of 52,094 for `benchmark-user-guide`.
They came from six bugs: the generator cut a rule's or a capture's pattern list short at a list being expanded above it, `$0` in a scope name was empty, the first-byte table read the quantifier `{1,0}` as required, an alias whose line ends after `=` did not give back the line feed, captures nested at most 8 deep, and C++ in a Markdown fence lacked `source.cpp`.
`ini/sample`, `ini/pytest-tox`, `ini/kafka-server` and `ini/black-tox` are the development inputs for INI.
`ini/django-tox`, `ini/sqlalchemy-tox`, `ini/tomcat-catalina` and `ini/kafka-consumer` are held out.
`ini/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/black-other-tools` is the development input for `ini` fences, and `markdown/poetry-faq` is held out.
The lexer matched every input on its first run, held-out ones included.

`kotlin/sample`, `kotlin/coroutines-state-flow`, `kotlin/okhttp-cookie`, `kotlin/exposed-op` and `kotlin/serialization-json` are the development inputs for Kotlin.
`kotlin/coroutines-produce`, `kotlin/okhttp-request`, `kotlin/moshi-util`, `kotlin/turbine-flow-test`, `kotlin/ktor-logging` and `kotlin/okhttp-build` (a Gradle script, which Shiki lexes as Kotlin) are held out.
The `google-java-format-*` and `coverage` inputs of `java` are held out: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`kotlin/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
Shiki's Markdown grammar has no rule that embeds Kotlin, so no Markdown input has a Kotlin fence.
The lexer matched every input on its first run, held-out ones included.
`swift/sample`, `swift/argument-parser-argument`, `swift/algorithms-chunked`, `swift/alamofire-af-error`, `swift/composable-store` and `swift/string-processing-algorithms-tests` are the development inputs for Swift.
The last one is full of regex literals.
`swift/argument-parser-command-parser`, `swift/nio-codec`, `swift/string-processing-custom-tests`, `swift/testing-runner-tests`, `swift/kingfisher-image-processor` and `swift/snapshot-assert-snapshot` are held out.
Their first score, before any fix, was 2 of 27,734 non-whitespace bytes different for `kingfisher-image-processor` and none for the other five: a name that two capture groups of the operator declaration cover got its scope twice.
The `swift-test-*` and `coverage` inputs of `swift` are held out too: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
The swiftlang/swift files are parser tests, some of them invalid Swift on purpose.
`markdown/argument-parser-declaring-arguments` is the development input for `swift` fences, and `markdown/kingfisher-common-tasks-cache` and `markdown/argument-parser-getting-started` are held out.
Their first scores, before any fix, were 538 of 17,090 non-whitespace bytes different for `argument-parser-declaring-arguments` and 276 of 8,936 for `argument-parser-getting-started`; the score of `kingfisher-common-tasks-cache` was not recorded.
Two causes made them differ: `\G` holds at the start of every line in a fence, which the lexer did not apply to Swift, and the first-byte table of the enum case list's end left out letters, so a case list that a fence's `\G` kept open did not close.

`go/sample`, `go/go-sort`, `go/cobra-command`, `go/gin-context` and `go/testify-assertions` are the development inputs for Go.
`go/go-slices`, `go/go-json-decode`, `go/kubernetes-sets`, `go/zap-logger`, `go/viper-viper` and `go/prometheus-storage` are held out.
`markdown/gin-readme` is the development input for `go` fences and `markdown/cobra-user-guide` is held out; `markdown/httparser-readme` has a `go` fence too.
`powershell/sample`, `powershell/install-powershell` and `powershell/coverage` are PowerShell's development inputs, and `markdown/powershell-windows-build` is the development input for PowerShell fences.
`powershell/dotnet-install`, `powershell/set-release-tag` and `markdown/powershell-releasing` are held out.
`scala/sample` and `scala/scala3-boundary` are Scala's development inputs, and `markdown/scala3-givens` is the development input for Scala fences.
`scala/scala3-source-position`, `scala/circe-json`, `scala/circe-build` and `markdown/scala3-enums` are held out.
`scala/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`zig/sample`, `zig/std-allocator` and `zig/coverage` are Zig's development inputs, and `zig/std-fmt`, `zig/zls-offsets` and `zig/zls-build` are held out.
Shiki's Markdown grammar has no Zig fence, so no Markdown input embeds Zig.
`cmake/sample`, `cmake/json-lists` and `cmake/coverage` are CMake's development inputs, and `cmake/googletest-utils` and `cmake/catch2-lists` are held out.
Shiki's Markdown grammar has no CMake fence, so no Markdown input embeds CMake.
`proto/sample`, `proto/googleapis-http` and `proto/coverage` are Protobuf's development inputs, and `proto/googleapis-operations` and `proto/grpc-route-guide` are held out.
Shiki's Markdown grammar has no Protobuf fence, so no Markdown input embeds Protobuf.
`sass/sample`, `sass/bulma-functions` and `sass/coverage` are Sass's development inputs, and `sass/bulma-generic`, `sass/bulma-mixins` and `sass/bulma-navbar` are held out.
Shiki's Markdown grammar has no Sass fence, so no Markdown input embeds Sass.
`bat/sample` and `bat/gradlew` are Batch's development inputs, and `bat/tomcat-catalina` and `bat/tomcat-setclasspath` are held out.
`bat/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
Shiki's Markdown grammar embeds Batch in `bat` and `batch` fences, and no Markdown input holds one yet.
`nix/sample`, `nix/nixpkgs-ripgrep` and `nix/coverage` are Nix's development inputs, and `nix/nixpkgs-lists`, `nix/home-manager-tmux` and `nix/home-manager-flake` are held out.
Shiki's Markdown grammar has no Nix fence, so no Markdown input embeds Nix.
`dart/sample`, `dart/http-client` and `dart/coverage` are Dart's development inputs, and `markdown/dart-http-readme` is the development input for Dart fences.
`dart/http-base-request`, `dart/collection-algorithms` and `markdown/dart-args-readme` are held out.
`julia/sample`, `julia/base-abstractset` and `julia/coverage` are Julia's development inputs, and `markdown/julia-functions` is the development input for Julia fences.
`julia/base-sort`, `julia/strings-util` and `markdown/julia-strings` are held out.
None of them has an `R` string macro, whose contents Shiki lexes as R and nishiki leaves plain.
`coffee/sample`, `coffee/coffeescript-helpers`, `coffee/coffeescript-javascript-literals`, `coffee/coffeescript-browser` and `coffee/coverage` are CoffeeScript's development inputs, and `markdown/coffeescript-classes` is the development input for CoffeeScript fences.
`coffee/coffeescript-cake`, `coffee/coffeescript-optparse`, `coffee/coffeescript-repl` and `markdown/coffeescript-super-this` are held out.

`html/sample`, `html/h5bp-index`, `html/todomvc-backbone`, `html/three-animation-keyframes`, `html/mathjax-mml-chtml`, `html/three-css3d-periodictable` and `html/jquery-qunit-fixture` are the development inputs for HTML.
`html/todomvc-polymer`, `html/h5bp-404`, `html/todomvc-lit`, `html/three-geometry-colors`, `html/mathjax-tex-mml`, `html/three-loader-gltf`, `html/bootstrap3-visual-modal`, `html/jquery-delegatetest`, `html/todomvc-knockoutjs` and `html/todomvc-jquery` are held out.
The held-out ones cover `<style>` CSS, classic and module `<script>` JavaScript, a Polymer `<template>`, and character entities; the MathML of `html/mathjax-mml-chtml` is a development input.
TodoMVC's `license.md` puts the repository under MIT unless a file says otherwise, and its Polymer example carries its own BSD-3-Clause `LICENSE`.
`mermaid/sample`, `mermaid/flowchart-code-flow` and `mermaid/coverage` are its development inputs, and `mermaid/api-sequence` and `mermaid/mindmap-sequence` are held out.
Shiki's Markdown grammar has no Mermaid fence, so no Markdown input embeds Mermaid.
`haskell/sample` and `haskell/aeson-github` are Haskell's development inputs, and `haskell/aeson-builder` and `haskell/aeson-decoding-text` are held out.
`haskell/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
Shiki's Markdown grammar has no Haskell fence, so no Markdown input embeds Haskell.

`erb/sample`, `erb/rails-welcome`, `erb/rails-routes-table`, `erb/rails-rescues-source`, `erb/devise-registrations-edit` and `erb/rails-scaffold-form` are the development inputs for ERB.
`erb/rails-mailer-email`, `erb/rails-rescues-diagnostics`, `erb/rails-info-notes`, `erb/rails-app-layout`, `erb/rails-pwa-offline`, `erb/devise-shared-links`, `erb/devise-confirmation-instructions`, `erb/devise-sessions-new` and `erb/devise-error-messages` are held out.
`erb/coverage` is held out too: it was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
They cover code inside quoted values and `<style>` and `<script>` blocks, `-%>`, and the `<%%` of the generators' Thor templates; none has an ERB comment, which the unit tests and the mutation differential cover.
Rails licenses each of its gems by the `MIT-LICENSE` at the gem's root, `railties/` and `actionpack/` here.

`lua/sample`, `lua/inspect`, `lua/lume`, `lua/luals-string` and `lua/lua-literals` are the development inputs for Lua; `lua/lua-literals` holds long strings and a long comment at several levels, across lines.
`lua/json`, `lua/penlight-stringx`, `lua/luarocks-util` and `lua/luals-table` are held out.
The `coverage`, `lua-locals` and `luals-basic` inputs of `lua` are held out too: they were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it, `luals-basic` for its annotations.
`less/sample`, `less/bootstrap3-variables`, `less/bootstrap3-mixins-buttons`, `less/bootstrap3-mixins-grid-framework`, `less/antd-button-mixin` and `less/antd-customize` are the development inputs for Less.
`less/bootstrap3-forms`, `less/bootstrap3-navbar`, `less/bootstrap3-mixins-gradients`, `less/antd-compact-item` and `less/antd-color-palette` are held out; `less/antd-color-palette` holds JavaScript in backticks.

`scss/sample`, `scss/bootstrap-functions`, `scss/bootstrap-button`, `scss/bootstrap-mixins-grid` and `scss/bulma-mixins` are the development inputs for SCSS.
`scss/bootstrap-utilities`, `scss/bootstrap-navbar`, `scss/bootstrap-root`, `scss/bulma-functions` and `scss/bulma-button` are held out.
The `lessjs-*`, `angular-menu`, `bootstrap-mixins-form-validation`, `coverage` and `charset-*` inputs of `css`, `less` and `scss` are held out: they were added to exercise the rest of those grammars, as [`COVERAGE.md`](COVERAGE.md) counts them.
The three `css/charset-*` inputs are one line each, because the grammar's `@charset` rule begins only at the start of a document, and each of its error forms needs a document of its own.

`html-derivative/vue-todomvc`, `html-derivative/vue-grid`, `html-derivative/vue-svg`, `html-derivative/vue-modal` and `html-derivative/vue-tree` are the development inputs for html-derivative.
`html-derivative/vue-classic-todomvc`, `html-derivative/vue-classic-grid`, `html-derivative/vue-transition-list`, `html-derivative/vue-commits`, `html-derivative/vue-markdown` and `html-derivative/vue-hydration-visible` are held out.
They are Vue's examples: HTML with custom elements and components at the top level, and templates in `<script type="text/x-template">` blocks, which the HTML grammar reads as `text.html.basic`.

`vue/sample`, `vue/playground-app`, `vue/playground-header`, `vue/vitepress-badge`, `vue/vitepress-nav-menu-group` and `vue/vitepress-switch` are the development inputs for Vue.
The other `vue` inputs are held out, and `vue/coverage` was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.
`markdown/vitepress-runtime-api` is the development input for `vue` fences in Markdown.

`postcss/sample`, `postcss/custom-media-basic`, `postcss/mixins-spaces`, `postcss/nesting-spec-examples` and `postcss/tailwind-preflight` are the development inputs for PostCSS.
`postcss/custom-selectors-basic`, `postcss/mixins-arguments` and `postcss/nesting-at-nest` are held out: they were added after the development inputs passed.
`postcss/coverage` was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.

`common-lisp/sample`, `common-lisp/grammar-demo`, `common-lisp/grammar-format-strings`, `common-lisp/split-sequence-list` and `common-lisp/cl-ppcre-scanner` are the development inputs for Common Lisp.
`common-lisp/cl-ppcre-optimize`, `common-lisp/quicklisp-http` and `common-lisp/woo-response` are held out: they were added after the development inputs passed.
`svelte/sample`, `svelte/kit-root`, `svelte/site-kit-nav`, `svelte/site-kit-search-box`, `svelte/tests-const-tag-component` and `svelte/tests-form-default-value` are the development inputs for Svelte.
The other `svelte` inputs are held out.

`objective-c/sample`, `objective-c/afnetworking-http-session-manager`, `objective-c/afnetworking-url-session-manager`, `objective-c/sdwebimage-image-cache`, `objective-c/sdwebimage-manager` and `objective-c/yykit-macro` are the development inputs for Objective-C.
`objective-c/masonry-constraint-maker`, `objective-c/mbprogresshud` and `objective-c/sdwebimage-graphics-image-renderer` are held out: they were added after the development inputs passed.
`objective-c/coverage` was added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.

`objective-cpp/sample`, `objective-cpp/grammar-misc-002`, `objective-cpp/react-native-component-view-registry` and `objective-cpp/react-native-dev-loading-view` are the development inputs for Objective-C++.
`objective-cpp/grammar-misc-003`, `objective-cpp/react-native-legacy-interop-coordinator` and `objective-cpp/react-native-virtual-view` were held out from the `TextMate` engine's tables.
The hand lexer was written against all seven.
`objective-cpp/react-native-device-info`, `objective-cpp/react-native-mounting-manager` and `objective-cpp/react-native-surface-presenter` are held out from the hand lexer: they were added after the other inputs passed.
`objective-cpp/coverage` and `objective-cpp/objective-c-coverage` were added to exercise the rest of the grammar, as [`COVERAGE.md`](COVERAGE.md) counts it.

[abseil-licence]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/LICENSE
[absl-any-invocable]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/functional/internal/any_invocable.h
[absl-any-invocable-test]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/functional/any_invocable_test.h
[absl-check-op]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/log/internal/check_op.h
[absl-cpu-detect]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/base/internal/cpu_detect.cc
[absl-crc-x86-arm-combined]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/crc/internal/crc_x86_arm_combined.cc
[absl-crc32c]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/crc/crc32c.h
[absl-dynamic-annotations]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/base/internal/dynamic_annotations.h
[absl-exception-safety-testing]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/base/internal/exception_safety_testing.h
[absl-flag]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/flags/flag.h
[absl-flag-internal]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/flags/internal/flag.h
[absl-flag-test]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/flags/flag_test.cc
[absl-hash-benchmark]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/hash/hash_benchmark.cc
[absl-mutex]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/synchronization/mutex.h
[absl-profile-builder]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/profiling/internal/profile_builder.cc
[absl-status-matchers]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/status/internal/status_matchers.h
[absl-str-split]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/strings/str_split.h
[absl-symbolize-test]: https://github.com/abseil/abseil-cpp/blob/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8/absl/debugging/symbolize_test.cc
[actionpack-licence]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/actionpack/MIT-LICENSE
[aeson-builder]: https://github.com/haskell/aeson/blob/42190167dbe120578a06d67b13b4d96991b34e01/src/Data/Aeson/Encoding/Builder.hs
[aeson-decoding-text]: https://github.com/haskell/aeson/blob/42190167dbe120578a06d67b13b4d96991b34e01/src/Data/Aeson/Decoding/Text.hs
[aeson-github]: https://github.com/haskell/aeson/blob/42190167dbe120578a06d67b13b4d96991b34e01/examples/src/GitHub.hs
[aeson-licence]: https://github.com/haskell/aeson/blob/42190167dbe120578a06d67b13b4d96991b34e01/LICENSE
[afnetworking-http-session-manager]: https://github.com/AFNetworking/AFNetworking/blob/d9f589cc2c1fe9d55eb5eea00558010afea7a41e/AFNetworking/AFHTTPSessionManager.h
[afnetworking-licence]: https://github.com/AFNetworking/AFNetworking/blob/d9f589cc2c1fe9d55eb5eea00558010afea7a41e/LICENSE
[afnetworking-url-session-manager]: https://github.com/AFNetworking/AFNetworking/blob/d9f589cc2c1fe9d55eb5eea00558010afea7a41e/AFNetworking/AFURLSessionManager.m
[airflow-compose]: https://github.com/apache/airflow/blob/cfbd70b66d11f5d464eb9c0ba926dd34304e110d/airflow-core/docs/howto/docker-compose/docker-compose.yaml
[airflow-licence]: https://github.com/apache/airflow/blob/cfbd70b66d11f5d464eb9c0ba926dd34304e110d/LICENSE
[alamofire-af-error]: https://github.com/Alamofire/Alamofire/blob/6c2892ea5adb5a03fa81c510933dd782c5ddd9cb/Source/Core/AFError.swift
[alamofire-licence]: https://github.com/Alamofire/Alamofire/blob/6c2892ea5adb5a03fa81c510933dd782c5ddd9cb/LICENSE
[algorithms-chunked]: https://github.com/apple/swift-algorithms/blob/5b7143f8e291dee0e14c118fd0212487f0b37af5/Sources/Algorithms/Chunked.swift
[algorithms-licence]: https://github.com/apple/swift-algorithms/blob/5b7143f8e291dee0e14c118fd0212487f0b37af5/LICENSE.txt
[angular-components-licence]: https://github.com/angular/components/blob/6b8d6a47693cf9c862c46919e5de8982fa3fd417/LICENSE
[ant-build]: https://github.com/apache/ant/blob/dbb01a2d528568b1d7ba0c2d84e533577f7f0876/build.xml
[ant-checkstyle-frames]: https://github.com/apache/ant/blob/dbb01a2d528568b1d7ba0c2d84e533577f7f0876/src/etc/checkstyle/checkstyle-frames.xsl
[ant-junit-frames]: https://github.com/apache/ant/blob/dbb01a2d528568b1d7ba0c2d84e533577f7f0876/src/etc/junit-frames.xsl
[ant-junit-noframes]: https://github.com/apache/ant/blob/dbb01a2d528568b1d7ba0c2d84e533577f7f0876/src/etc/junit-noframes.xsl
[ant-licence]: https://github.com/apache/ant/blob/dbb01a2d528568b1d7ba0c2d84e533577f7f0876/LICENSE
[antd-licence]: https://github.com/ant-design/ant-design/blob/7c060247516925f48175d7f29cd43b626632665e/LICENSE
[argument-parser-argument]: https://github.com/apple/swift-argument-parser/blob/38ca4eceb349155836701b88da3e0986eb9d1e1c/Sources/ArgumentParser/Parsable%20Properties/Argument.swift
[argument-parser-command-parser]: https://github.com/apple/swift-argument-parser/blob/38ca4eceb349155836701b88da3e0986eb9d1e1c/Sources/ArgumentParser/Parsing/CommandParser.swift
[argument-parser-declaring-arguments]: https://github.com/apple/swift-argument-parser/blob/38ca4eceb349155836701b88da3e0986eb9d1e1c/Sources/ArgumentParser/Documentation.docc/Articles/DeclaringArguments.md
[argument-parser-getting-started]: https://github.com/apple/swift-argument-parser/blob/38ca4eceb349155836701b88da3e0986eb9d1e1c/Sources/ArgumentParser/Documentation.docc/Articles/GettingStarted.md
[argument-parser-licence]: https://github.com/apple/swift-argument-parser/blob/38ca4eceb349155836701b88da3e0986eb9d1e1c/LICENSE.txt
[asm-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/asm.sample
[axios-licence]: https://github.com/axios/axios/blob/7324e94d8a7b5156a4baf3c86c8b5daa242daee3/LICENSE
[axios-xhr]: https://github.com/axios/axios/blob/7324e94d8a7b5156a4baf3c86c8b5daa242daee3/lib/adapters/xhr.js
[bat-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/bat.sample
[bats-core-bats]: https://github.com/bats-core/bats-core/blob/48d8d75a836fb322cf98bd305e2db5a84c1be419/libexec/bats-core/bats
[bats-core-gather-tests]: https://github.com/bats-core/bats-core/blob/48d8d75a836fb322cf98bd305e2db5a84c1be419/libexec/bats-core/bats-gather-tests
[bats-core-licence]: https://github.com/bats-core/bats-core/blob/48d8d75a836fb322cf98bd305e2db5a84c1be419/LICENSE.md
[benchmark-core]: https://github.com/google/benchmark/blob/1813ad9c3a828cee126f399739443c329e75bc79/src/benchmark.cc
[benchmark-licence]: https://github.com/google/benchmark/blob/1813ad9c3a828cee126f399739443c329e75bc79/LICENSE
[benchmark-sysinfo]: https://github.com/google/benchmark/blob/1813ad9c3a828cee126f399739443c329e75bc79/src/sysinfo.cc
[benchmark-user-guide]: https://github.com/google/benchmark/blob/1813ad9c3a828cee126f399739443c329e75bc79/docs/user_guide.md
[black-licence]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/LICENSE
[black-other-tools]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/docs/guides/using_black_with_other_tools.md
[black-strings]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/src/black/strings.py
[black-tox]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tox.ini
[blake3-licence]: https://github.com/BLAKE3-team/BLAKE3/blob/f55849f89cd85083c1c9daa2c5de20766f309c00/LICENSE_A2
[blake3-sse41-msvc]: https://github.com/BLAKE3-team/BLAKE3/blob/f55849f89cd85083c1c9daa2c5de20766f309c00/c/blake3_sse41_x86-64_windows_msvc.asm
[book-licence]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/LICENSE-MIT
[book-ownership]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch04-01-what-is-ownership.md
[book-strings]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch08-02-strings.md
[bootstrap-licence]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/LICENSE
[bootstrap-logo]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/site/static/docs/%5Bversion%5D/assets/brand/bootstrap-logo.svg
[bootstrap3-licence]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/LICENSE
[bootstrap3-visual-modal]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/js/tests/visual/modal.html
[buildx-docker]: https://github.com/docker/buildx/blob/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d/Dockerfile
[buildx-licence]: https://github.com/docker/buildx/blob/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d/LICENSE
[bulma-licence]: https://github.com/jgthms/bulma/blob/c5cd04baf0258cfcc77b4e8610488a0d439effa2/LICENSE
[bulma-sass-licence]: https://github.com/jgthms/bulma/blob/3e00a8e6d0d0e566d507328f0185ef84854effba/LICENSE
[burntsushi-example]: https://github.com/BurntSushi/toml/blob/d733fc535e4a9f3c454e421a87b90ade5d49bf31/_example/example.toml
[burntsushi-toml-licence]: https://github.com/BurntSushi/toml/blob/d733fc535e4a9f3c454e421a87b90ade5d49bf31/COPYING
[c-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/c.sample
[cargo-licence]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/LICENSE-MIT
[cargo-manifest]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/Cargo.toml
[cargo-release]: https://github.com/rust-lang/cargo/blob/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc/doc/contrib/src/process/release.md
[catch2-cmake-lists]: https://github.com/catchorg/Catch2/blob/a1a991fee5f217b40035cb2f8ebc83864d66408d/CMakeLists.txt
[catch2-licence]: https://github.com/catchorg/Catch2/blob/a1a991fee5f217b40035cb2f8ebc83864d66408d/LICENSE.txt
[catch2-tutorial]: https://github.com/catchorg/Catch2/blob/a1a991fee5f217b40035cb2f8ebc83864d66408d/docs/tutorial.md
[catppuccin]: https://github.com/catppuccin/vscode/tree/befc9e6fc41980f4241408f7049755d47c06ff45
[catppuccin-licence]: https://github.com/catppuccin/vscode/blob/befc9e6fc41980f4241408f7049755d47c06ff45/LICENSE
[chebfun-feval]: https://github.com/chebfun/chebfun/blob/7574c77680d7e82b79626300bf255498271a72df/@chebfun/feval.m
[chebfun-licence]: https://github.com/chebfun/chebfun/blob/7574c77680d7e82b79626300bf255498271a72df/LICENSE.txt
[chebfun-roots]: https://github.com/chebfun/chebfun/blob/7574c77680d7e82b79626300bf255498271a72df/@chebfun/roots.m
[checkout-licence]: https://github.com/actions/checkout/blob/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a/LICENSE
[checkout-test]: https://github.com/actions/checkout/blob/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a/.github/workflows/test.yml
[circe-build]: https://github.com/circe/circe/blob/b38c17d9a9dd9b761a68319cff038d75bbf0cf8d/build.sbt
[circe-json]: https://github.com/circe/circe/blob/b38c17d9a9dd9b761a68319cff038d75bbf0cf8d/modules/core/shared/src/main/scala/io/circe/Json.scala
[circe-licence]: https://github.com/circe/circe/blob/b38c17d9a9dd9b761a68319cff038d75bbf0cf8d/LICENSE
[cjson]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/cJSON.c
[cjson-licence]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/LICENSE
[cjson-make]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/Makefile
[cjson-readme]: https://github.com/DaveGamble/cJSON/blob/6d9f2443ab071f86e5d9b43025a40929ec41c46c/README.md
[cl-ppcre-licence]: https://github.com/edicl/cl-ppcre/blob/a2ea581c23fdc184168423adbd4b4c1f48d42743/LICENSE
[cl-ppcre-optimize]: https://github.com/edicl/cl-ppcre/blob/a2ea581c23fdc184168423adbd4b4c1f48d42743/optimize.lisp
[cl-ppcre-scanner]: https://github.com/edicl/cl-ppcre/blob/a2ea581c23fdc184168423adbd4b4c1f48d42743/scanner.lisp
[clap-licence]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/LICENSE-MIT
[clap-manifest]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/Cargo.toml
[clap-readme]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/README.md
[click-man-licence]: https://github.com/click-contrib/click-man/blob/be9eac7642e32a43a93ec2c1f5deea7047bb7288/LICENSE
[click-man-readme]: https://github.com/click-contrib/click-man/blob/be9eac7642e32a43a93ec2c1f5deea7047bb7288/README.md
[cmake-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/cmake.sample
[cobra-command]: https://github.com/spf13/cobra/blob/adbc8813901bba65827259daa8e22ff94ec1f30e/command.go
[cobra-licence]: https://github.com/spf13/cobra/blob/adbc8813901bba65827259daa8e22ff94ec1f30e/LICENSE.txt
[cobra-user-guide]: https://github.com/spf13/cobra/blob/adbc8813901bba65827259daa8e22ff94ec1f30e/site/content/user_guide.md
[codegen-apollo]: https://github.com/dotansimha/graphql-code-generator/blob/11ac951a30b79369d3b0aa78f83a79d309ffcc77/dev-test/apollo-tooling/schema.graphql
[codegen-licence]: https://github.com/dotansimha/graphql-code-generator/blob/11ac951a30b79369d3b0aa78f83a79d309ffcc77/LICENSE
[coffee-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/coffee.sample
[coffeescript-browser]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/src/browser.coffee
[coffeescript-cake]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/src/cake.coffee
[coffeescript-classes]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/documentation/sections/breaking_changes_classes.md
[coffeescript-helpers]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/src/helpers.coffee
[coffeescript-javascript-literals]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/test/javascript_literals.coffee
[coffeescript-licence]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/LICENSE
[coffeescript-optparse]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/src/optparse.coffee
[coffeescript-repl]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/src/repl.coffee
[coffeescript-super-this]: https://github.com/jashkenas/coffeescript/blob/116890ee072305d3f4ad2686b70c9f28b967c11d/documentation/sections/breaking_changes_super_this.md
[common-lisp-grammar-demo]: https://github.com/qingpeng9802/vscode-common-lisp/blob/79d04dc9ca80d98e65c4f28b3d67a24a7829b18f/syntaxes/fixtures/cases/demo.lsp
[common-lisp-grammar-fstr]: https://github.com/qingpeng9802/vscode-common-lisp/blob/79d04dc9ca80d98e65c4f28b3d67a24a7829b18f/syntaxes/fixtures/cases/fstr.lsp
[common-lisp-grammar-licence]: https://github.com/qingpeng9802/vscode-common-lisp/blob/79d04dc9ca80d98e65c4f28b3d67a24a7829b18f/LICENSE
[common-lisp-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/common-lisp.sample
[commons-lang-licence]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/LICENSE.txt
[commons-lang-pom]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/pom.xml
[commons-lang-range]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/src/main/java/org/apache/commons/lang3/Range.java
[commons-lang-validate]: https://github.com/apache/commons-lang/blob/d8470d57fda340864acf5194bb56305a133fba0e/src/main/java/org/apache/commons/lang3/Validate.java
[composable-licence]: https://github.com/pointfreeco/swift-composable-architecture/blob/bb175b544d019c7f23a2e41ae29d8c70592aeb58/LICENSE
[composable-store]: https://github.com/pointfreeco/swift-composable-architecture/blob/bb175b544d019c7f23a2e41ae29d8c70592aeb58/Sources/ComposableArchitecture/Store.swift
[coroutines-licence]: https://github.com/Kotlin/kotlinx.coroutines/blob/7e0d1b3e9ca83652427ae00dda7d1673f058a743/LICENSE.txt
[coroutines-produce]: https://github.com/Kotlin/kotlinx.coroutines/blob/7e0d1b3e9ca83652427ae00dda7d1673f058a743/kotlinx-coroutines-core/common/src/channels/Produce.kt
[coroutines-state-flow]: https://github.com/Kotlin/kotlinx.coroutines/blob/7e0d1b3e9ca83652427ae00dda7d1673f058a743/kotlinx-coroutines-core/common/src/flow/StateFlow.kt
[cpp-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/cpp.sample
[cranelift-atomics]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch
[cranelift-sve]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch
[cranelift-tests]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch
[csharp-docs-async-wpf]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/async/wpf/MainWindow.xaml.cs
[csharp-docs-bitwise-shift]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/operators/snippets/shared/BitwiseAndShiftOperators.cs
[csharp-docs-closed-hierarchy-patterns]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/operators/snippets/patterns/ClosedHierarchyPatterns.cs
[csharp-docs-delegate-operator]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/operators/snippets/shared/DelegateOperator.cs
[csharp-docs-doc-comments]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/xmldoc/snippets/xmldoc/DocComments.cs
[csharp-docs-events]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/events.cs
[csharp-docs-exception-example]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/statements/snippets/exception-handling-statements/ExampleClass.cs
[csharp-docs-fixed]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/snippets/unsafe-code/FixedKeywordExamples.cs
[csharp-docs-generic-where]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/GenericWhereConstraints.cs
[csharp-docs-goto]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/statements/snippets/jump-statements/GotoStatement.cs
[csharp-docs-group]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/group.cs
[csharp-docs-keywords-contextual]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/csrefKeywordsContextual.cs
[csharp-docs-keywords-modifiers]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/csrefKeywordsModifiers.cs
[csharp-docs-lock]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/statements/snippets/lock/Program.cs
[csharp-docs-raw-string-literal]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/tokens/snippets/raw-string-literal.cs
[csharp-docs-record-type]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/builtin-types/snippets/shared/RecordType.cs
[csharp-docs-string-interpolation]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/tokens/snippets/string-interpolation.cs
[csharp-docs-union-body-members]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/builtin-types/snippets/unions/BodyMembers.cs
[csharp-docs-using-alias]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/keywords/snippets/UsingAliasQualifier.cs
[csharp-docs-value-tuples]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/builtin-types/snippets/shared/ValueTuples.cs
[csharp-docs-var-pattern]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference/operators/snippets/patterns/VarPattern.cs
[csharp-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/csharp.sample
[css-lessjs-at-rules-compressed-evaluation]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-config/at-rules-compressed-evaluation/at-rules-compressed-evaluation.css
[css-lessjs-css-3]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/css-3/css-3.css
[css-lessjs-math-strict]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-config/math/strict/css.css
[css-lessjs-media]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/media/media.css
[css-lessjs-selectors]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/selectors/selectors.css
[css-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/css.sample
[csstools-custom-media-basic]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-custom-media/test/basic.css
[csstools-custom-media-licence]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-custom-media/LICENSE.md
[csstools-custom-selectors-basic]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-custom-selectors/test/basic.css
[csstools-custom-selectors-licence]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-custom-selectors/LICENSE.md
[csstools-mixins-arguments]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-mixins/test/arguments.css
[csstools-mixins-licence]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-mixins/LICENSE.md
[csstools-mixins-spaces]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-mixins/test/spaces.css
[csstools-nesting-at-nest]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-nesting/test/at-nest.css
[csstools-nesting-licence]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-nesting/LICENSE.md
[csstools-nesting-spec-examples]: https://github.com/csstools/postcss-plugins/blob/1d146b524f7ef801e876f0fdcd4e5c572222b6fe/plugins/postcss-nesting/test/spec-examples.css
[dart-args-readme]: https://github.com/dart-lang/core/blob/36f57a994965785d641e436b7dd3aa8a18a26f62/pkgs/args/README.md
[dart-collection-algorithms]: https://github.com/dart-lang/core/blob/36f57a994965785d641e436b7dd3aa8a18a26f62/pkgs/collection/lib/src/algorithms.dart
[dart-core-licence]: https://github.com/dart-lang/core/blob/36f57a994965785d641e436b7dd3aa8a18a26f62/LICENSE
[dart-http-base-request]: https://github.com/dart-lang/http/blob/8c0d66acd2385761182d34b48c4c80ca6cfa4513/pkgs/http/lib/src/base_request.dart
[dart-http-client]: https://github.com/dart-lang/http/blob/8c0d66acd2385761182d34b48c4c80ca6cfa4513/pkgs/http/lib/src/client.dart
[dart-http-licence]: https://github.com/dart-lang/http/blob/8c0d66acd2385761182d34b48c4c80ca6cfa4513/LICENSE
[dart-http-readme]: https://github.com/dart-lang/http/blob/8c0d66acd2385761182d34b48c4c80ca6cfa4513/pkgs/http/README.md
[dart-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/dart.sample
[derivative-vue-classic-grid]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/classic/grid.html
[derivative-vue-classic-todomvc]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/classic/todomvc.html
[derivative-vue-commits]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/commits.html
[derivative-vue-grid]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/grid.html
[derivative-vue-hydration-visible]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/__tests__/e2e/hydration-strat-visible.html
[derivative-vue-markdown]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/markdown.html
[derivative-vue-modal]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/transition/modal.html
[derivative-vue-svg]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/svg.html
[derivative-vue-todomvc]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/todomvc.html
[derivative-vue-transition-list]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/transition/list.html
[derivative-vue-tree]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/vue/examples/composition/tree.html
[device-virtual-licence]: https://github.com/edgexfoundry/device-virtual-go/blob/51a4860f7a496206684d7bc404f1c64d674ea9c4/LICENSE
[device-virtual-readme]: https://github.com/edgexfoundry/device-virtual-go/blob/51a4860f7a496206684d7bc404f1c64d674ea9c4/README.md
[devise]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/lib/devise.rb
[devise-confirmation-instructions]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/app/views/devise/mailer/confirmation_instructions.html.erb
[devise-error-messages]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/app/views/devise/shared/_error_messages.html.erb
[devise-licence]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/MIT-LICENSE
[devise-readme]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/README.md
[devise-registrations-edit]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/app/views/devise/registrations/edit.html.erb
[devise-sessions-new]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/app/views/devise/sessions/new.html.erb
[devise-shared-links]: https://github.com/heartcombo/devise/blob/05811fb8366edd08874111d0f6fb44543e1c91ef/app/views/devise/shared/_links.html.erb
[diff-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/diff.sample
[django-licence]: https://github.com/django/django/blob/b501b8493f31cf8cba31211d59b3b78642b8cb02/LICENSE
[django-tox]: https://github.com/django/django/blob/b501b8493f31cf8cba31211d59b3b78642b8cb02/tox.ini
[docker-install]: https://github.com/docker/docker-install/blob/ada7a567e59b3935bd601bb21e422da50e80e376/install.sh
[docker-install-licence]: https://github.com/docker/docker-install/blob/ada7a567e59b3935bd601bb21e422da50e80e376/LICENSE
[docker-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/docker.sample
[dotnet-docs-licence]: https://github.com/dotnet/docs/blob/d1a7e773f847015b60202a8332d949ac9dbbb279/LICENSE-CODE
[dotnet-install]: https://github.com/dotnet/install-scripts/blob/e5cf1dd2d1540ed05ac84f8eb8c5cdec2807621e/src/dotnet-install.ps1
[dotnet-install-licence]: https://github.com/dotnet/install-scripts/blob/e5cf1dd2d1540ed05ac84f8eb8c5cdec2807621e/LICENSE.TXT
[dotnet-runtime-licence]: https://github.com/dotnet/runtime/blob/6375d63af99b6812f11b4dd8c4a18d227e277d73/LICENSE.TXT
[elixir-keyword]: https://github.com/elixir-lang/elixir/blob/cb8695250e2ff8b27c343e2a5ce91e2da7bbdfde/lib/elixir/lib/keyword.ex
[elixir-lang-licence]: https://github.com/elixir-lang/elixir/blob/cb8695250e2ff8b27c343e2a5ce91e2da7bbdfde/LICENSE
[elixir-map-set]: https://github.com/elixir-lang/elixir/blob/cb8695250e2ff8b27c343e2a5ce91e2da7bbdfde/lib/elixir/lib/map_set.ex
[elixir-range]: https://github.com/elixir-lang/elixir/blob/cb8695250e2ff8b27c343e2a5ce91e2da7bbdfde/lib/elixir/lib/range.ex
[elixir-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/elixir.sample
[erb-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/erb.sample
[eslint-imports]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/docs/src/rules/no-restricted-imports.md
[eslint-licence]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/LICENSE
[eslint-mcp]: https://github.com/eslint/eslint/blob/11c66ba08381cb7cff7679d662c9f6654b6fe712/docs/src/use/mcp.md
[exposed-licence]: https://github.com/JetBrains/Exposed/blob/2bdb13cd7b4b511b07de3dde109878b6f8b06d48/LICENSE.txt
[exposed-op]: https://github.com/JetBrains/Exposed/blob/2bdb13cd7b4b511b07de3dde109878b6f8b06d48/exposed-core/src/main/kotlin/org/jetbrains/exposed/v1/core/Op.kt
[express-application]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/lib/application.js
[express-licence]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/LICENSE
[express-response]: https://github.com/expressjs/express/blob/bcc0551cb7a785b7772f77cb59ce55a2bc94975a/lib/response.js
[fastapi-docker]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/docs/en/docs/deployment/docker.md
[fastapi-licence]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/LICENSE
[fastapi-readme]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/README.md
[fastapi-routing]: https://github.com/tiangolo/fastapi/blob/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0/fastapi/routing.py
[flask-helpers]: https://github.com/pallets/flask/blob/49943b5a94413a530761350f04e896f76e8d620e/src/flask/helpers.py
[flask-licence]: https://github.com/pallets/flask/blob/49943b5a94413a530761350f04e896f76e8d620e/LICENSE.txt
[fmt-c]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/include/fmt/fmt-c.h
[fmt-core-test]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/test/core-test.cc
[fmt-gtest-extra]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/test/gtest-extra.h
[fmt-licence]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/LICENSE
[fmt-ranges]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/include/fmt/ranges.h
[fmt-xchar-test]: https://github.com/fmtlib/fmt/blob/4ed3535092a96ac9f56fab80df83416d51bcc78e/test/xchar-test.cc
[foundation-keyed-archive]: https://github.com/apple/swift-corelibs-foundation/blob/cbfe1c7376ffe869d08b1b5f643d747ef0a16069/Tests/Foundation/Resources/NSKeyedUnarchiver-ConcreteValueTest.plist
[foundation-licence]: https://github.com/apple/swift-corelibs-foundation/blob/cbfe1c7376ffe869d08b1b5f643d747ef0a16069/LICENSE
[gdscript-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/gdscript.sample
[gin-context]: https://github.com/gin-gonic/gin/blob/0f09c3a9b4626d9fe9979cf4d0521d6ae32ab646/context.go
[gin-licence]: https://github.com/gin-gonic/gin/blob/0f09c3a9b4626d9fe9979cf4d0521d6ae32ab646/LICENSE
[gin-readme]: https://github.com/gin-gonic/gin/blob/0f09c3a9b4626d9fe9979cf4d0521d6ae32ab646/README.md
[github-markdown-css]: https://github.com/sindresorhus/github-markdown-css/blob/7f38dcb9054bc5d860fac65291f5c98fef47932c/github-markdown.css
[github-markdown-css-licence]: https://github.com/sindresorhus/github-markdown-css/blob/7f38dcb9054bc5d860fac65291f5c98fef47932c/license
[github-theme]: https://github.com/primer/github-vscode-theme/tree/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf
[github-theme-licence]: https://github.com/primer/github-vscode-theme/blob/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf/LICENSE
[go-json-decode]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/encoding/json/decode.go
[go-printer-declarations]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/go/printer/testdata/declarations.golden
[go-printer-go2numbers]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/go/printer/testdata/go2numbers.golden
[go-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/go.sample
[go-slices]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/slices/slices.go
[go-sort]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/sort/sort.go
[go-test-bug141]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/test/fixedbugs/bug141.go
[go-test-issue15528]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/test/fixedbugs/issue15528.go
[go-types-typeinst1]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/internal/types/testdata/check/typeinst1.go
[go-types-typeparams]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/src/internal/types/testdata/check/typeparams.go
[godot-demos-licence]: https://github.com/godotengine/godot-demo-projects/blob/b761b4cd5718a7118f2c95eb55f181d0a5f004f1/LICENSE.md
[godot-paint-control]: https://github.com/godotengine/godot-demo-projects/blob/b761b4cd5718a7118f2c95eb55f181d0a5f004f1/gui/gd_paint/paint_control.gd
[godot-voxel-chunk]: https://github.com/godotengine/godot-demo-projects/blob/b761b4cd5718a7118f2c95eb55f181d0a5f004f1/3d/voxel/world/chunk.gd
[godot-vscode-example-1]: https://github.com/godotengine/godot-vscode-plugin/blob/3f2860fd76c591a6010f0d35220490933e41d01d/syntaxes/examples/gdscript1.gd
[godot-vscode-example-2]: https://github.com/godotengine/godot-vscode-plugin/blob/3f2860fd76c591a6010f0d35220490933e41d01d/syntaxes/examples/gdscript2.gd
[godot-vscode-licence]: https://github.com/godotengine/godot-vscode-plugin/blob/3f2860fd76c591a6010f0d35220490933e41d01d/LICENSE
[golang-licence]: https://github.com/golang/go/blob/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6/LICENSE
[google-java-format-b380299722]: https://github.com/google/google-java-format/blob/056a0ac101cc5c297499f1ded7062a2a65b0cdcc/core/src/test/resources/com/google/googlejavaformat/java/testdata/B380299722.input
[google-java-format-i574]: https://github.com/google/google-java-format/blob/056a0ac101cc5c297499f1ded7062a2a65b0cdcc/core/src/test/resources/com/google/googlejavaformat/java/testdata/I574.input
[google-java-format-lambda-hug]: https://github.com/google/google-java-format/blob/056a0ac101cc5c297499f1ded7062a2a65b0cdcc/core/src/test/resources/com/google/googlejavaformat/java/testdata/LambdaHug.input
[google-java-format-licence]: https://github.com/google/google-java-format/blob/056a0ac101cc5c297499f1ded7062a2a65b0cdcc/LICENSE
[google-java-format-m1]: https://github.com/google/google-java-format/blob/056a0ac101cc5c297499f1ded7062a2a65b0cdcc/core/src/test/resources/com/google/googlejavaformat/java/testdata/M1.input
[googleapis-http]: https://github.com/googleapis/googleapis/blob/0d0982bd9844aacc37a4e87c1cd43bbdb601884d/google/api/http.proto
[googleapis-licence]: https://github.com/googleapis/googleapis/blob/0d0982bd9844aacc37a4e87c1cd43bbdb601884d/LICENSE
[googleapis-operations]: https://github.com/googleapis/googleapis/blob/0d0982bd9844aacc37a4e87c1cd43bbdb601884d/google/longrunning/operations.proto
[googletest-licence]: https://github.com/google/googletest/blob/a8380b01a25ebdb130097b607b3afed2a58d04ba/LICENSE
[googletest-utils]: https://github.com/google/googletest/blob/a8380b01a25ebdb130097b607b3afed2a58d04ba/googletest/cmake/internal_utils.cmake
[gradle-licence]: https://github.com/gradle/gradle/blob/f9f8f85075be950f7d4e2edc4d57689084481e87/LICENSE
[gradlew-bat]: https://github.com/gradle/gradle/blob/f9f8f85075be950f7d4e2edc4d57689084481e87/gradlew.bat
[graphiql-kitchen-sink]: https://github.com/graphql/graphiql/blob/e7a5e07b6c2cb9b524831544c689e742cdc1b904/packages/codemirror-graphql/src/__tests__/kitchen-sink.graphql
[graphiql-licence]: https://github.com/graphql/graphiql/blob/e7a5e07b6c2cb9b524831544c689e742cdc1b904/LICENSE
[graphiql-merged-query]: https://github.com/graphql/graphiql/blob/e7a5e07b6c2cb9b524831544c689e742cdc1b904/packages/graphiql-toolkit/src/graphql-helpers/__tests__/__queries__/mergedQueryWithSchema.graphql
[graphiql-schema-kitchen-sink]: https://github.com/graphql/graphiql/blob/e7a5e07b6c2cb9b524831544c689e742cdc1b904/packages/codemirror-graphql/src/__tests__/schema-kitchen-sink.graphql
[graphiql-star-wars]: https://github.com/graphql/graphiql/blob/e7a5e07b6c2cb9b524831544c689e742cdc1b904/packages/graphql-language-service-server/src/__tests__/__schema__/StarWarsSchema.graphql
[graphql-js-kitchen-sink]: https://github.com/graphql/graphql-js/blob/eb0bd0801ea5f9f7680e91ae575ed74397f051cb/benchmark/kitchen-sink.graphql
[graphql-js-licence]: https://github.com/graphql/graphql-js/blob/eb0bd0801ea5f9f7680e91ae575ed74397f051cb/LICENSE
[graphql-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/graphql.sample
[grpc-licence]: https://github.com/grpc/grpc/blob/9a4f19513f24b48c0fbfc9c56765e553f9c6e56c/LICENSE
[grpc-route-guide]: https://github.com/grpc/grpc/blob/9a4f19513f24b48c0fbfc9c56765e553f9c6e56c/examples/protos/route_guide.proto
[gson-gson]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/gson/src/main/java/com/google/gson/Gson.java
[gson-json-reader]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/gson/src/main/java/com/google/gson/stream/JsonReader.java
[gson-licence]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/LICENSE
[gson-user-guide]: https://github.com/google/gson/blob/57731049da3e415be3d5d34db1ca3b80f3c73810/UserGuide.md
[guava-immutable-list]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/guava/src/com/google/common/collect/ImmutableList.java
[guava-licence]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/LICENSE
[guava-preconditions]: https://github.com/google/guava/blob/e98ea2340f6acc94ed9e2cca08976ccae68c3412/guava/src/com/google/common/base/Preconditions.java
[h5bp-404]: https://github.com/h5bp/html5-boilerplate/blob/b6597338e695dc4a8165c5abcb2bfffa586c3ee5/src/404.html
[h5bp-index]: https://github.com/h5bp/html5-boilerplate/blob/b6597338e695dc4a8165c5abcb2bfffa586c3ee5/src/index.html
[h5bp-licence]: https://github.com/h5bp/html5-boilerplate/blob/b6597338e695dc4a8165c5abcb2bfffa586c3ee5/LICENSE.txt
[haml-helpers]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/test/haml/templates/helpers.haml
[haml-just-stuff]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/test/haml/templates/just_stuff.haml
[haml-licence]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/MIT-LICENSE
[haml-real-sample]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/benchmark/etc/real_sample.haml
[haml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/haml.sample
[haml-silent-script]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/test/haml/templates/silent_script.haml
[haml-standard]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/test/haml/templates/standard.haml
[haml-whitespace]: https://github.com/haml/haml/blob/4a89a4639a5bb4e6e28e1763227cabfb106e5d8b/test/haml/templates/whitespace_handling.haml
[haskell-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/haskell.sample
[helm-licence]: https://github.com/helm/helm/blob/bc232c5312e784e808ea95ae76f9e82d4fe142fc/LICENSE
[helm-make]: https://github.com/helm/helm/blob/bc232c5312e784e808ea95ae76f9e82d4fe142fc/Makefile
[highlightjs-atom-one-dark]: https://github.com/highlightjs/highlight.js/blob/60e092057673595fa542db0bc2b13a99df5d0551/src/styles/atom-one-dark.css
[highlightjs-github]: https://github.com/highlightjs/highlight.js/blob/60e092057673595fa542db0bc2b13a99df5d0551/src/styles/github.css
[highlightjs-licence]: https://github.com/highlightjs/highlight.js/blob/60e092057673595fa542db0bc2b13a99df5d0551/LICENSE
[highlightjs-readme]: https://github.com/highlightjs/highlight.js/blob/60e092057673595fa542db0bc2b13a99df5d0551/README.md
[home-manager-flake]: https://github.com/nix-community/home-manager/blob/095fef2f265c3c0da4fe315efba8f8a04495ab34/flake.nix
[home-manager-licence]: https://github.com/nix-community/home-manager/blob/095fef2f265c3c0da4fe315efba8f8a04495ab34/LICENSE
[home-manager-tmux]: https://github.com/nix-community/home-manager/blob/095fef2f265c3c0da4fe315efba8f8a04495ab34/modules/programs/tmux.nix
[homebrew-install-licence]: https://github.com/Homebrew/install/blob/959bdd728192d78c47287c177796c3d86d111f09/LICENSE.txt
[homebrew-uninstall]: https://github.com/Homebrew/install/blob/959bdd728192d78c47287c177796c3d86d111f09/uninstall.sh
[horse-commons]: https://github.com/HashLoad/horse/blob/a0ac019be1d29fd9deb20f61ca779d5631718bdb/src/Horse.Commons.pas
[horse-http2-framing]: https://github.com/HashLoad/horse/blob/a0ac019be1d29fd9deb20f61ca779d5631718bdb/src/Horse.Core.Http2.Framing.pas
[horse-licence]: https://github.com/HashLoad/horse/blob/a0ac019be1d29fd9deb20f61ca779d5631718bdb/LICENSE
[horse-websocket-console]: https://github.com/HashLoad/horse/blob/a0ac019be1d29fd9deb20f61ca779d5631718bdb/samples/lazarus/console_websocket/console_websocket.lpr
[html-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/html.sample
[httparser-licence]: https://github.com/antlabs/httparser/blob/622df4409cddf119cf437633f620947235d2d1eb/LICENSE
[httparser-readme]: https://github.com/antlabs/httparser/blob/622df4409cddf119cf437633f620947235d2d1eb/README.md
[ini-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/ini.sample
[inspect-lua]: https://github.com/kikito/inspect.lua/blob/a8ca3120dfec48801036eaeff9335ab7a096dd24/inspect.lua
[inspect-lua-licence]: https://github.com/kikito/inspect.lua/blob/a8ca3120dfec48801036eaeff9335ab7a096dd24/MIT-LICENSE.txt
[install-powershell]: https://github.com/PowerShell/PowerShell/blob/2725d6aaa6e67219b6c9902adf7b74c52e0371a3/tools/install-powershell.ps1
[java-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/java.sample
[javapoet-licence]: https://github.com/square/javapoet/blob/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0/LICENSE.txt
[javapoet-readme]: https://github.com/square/javapoet/blob/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0/README.md
[jekyll-front-matter]: https://github.com/jekyll/jekyll/blob/97c3bdc822df9d5bea4c269b29a28de5a5e29d70/docs/_docs/front-matter.md
[jekyll-licence]: https://github.com/jekyll/jekyll/blob/97c3bdc822df9d5bea4c269b29a28de5a5e29d70/LICENSE
[jekyll-site]: https://github.com/jekyll/jekyll/blob/97c3bdc822df9d5bea4c269b29a28de5a5e29d70/lib/jekyll/site.rb
[jq-licence]: https://github.com/jqlang/jq/blob/5c001b18d7237cce1fdba63608732163f978dd23/COPYING
[jq-print]: https://github.com/jqlang/jq/blob/5c001b18d7237cce1fdba63608732163f978dd23/src/jv_print.c
[jquery-delegatetest]: https://github.com/jquery/jquery/blob/feaa9fcdb68cdeadf013d97d8fd40848d4618578/test/delegatetest.html
[jquery-licence]: https://github.com/jquery/jquery/blob/feaa9fcdb68cdeadf013d97d8fd40848d4618578/LICENSE.txt
[jquery-qunit-fixture]: https://github.com/jquery/jquery/blob/feaa9fcdb68cdeadf013d97d8fd40848d4618578/test/data/qunit-fixture.html
[js-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/javascript.sample
[json-cmake-lists]: https://github.com/nlohmann/json/blob/e32051125faf95dc64d49a129959b6742b9ffee9/CMakeLists.txt
[json-lua]: https://github.com/rxi/json.lua/blob/dbf4b2dd2eb7c23be2773c89eb059dadd6436f94/json.lua
[json-lua-licence]: https://github.com/rxi/json.lua/blob/dbf4b2dd2eb7c23be2773c89eb059dadd6436f94/LICENSE
[json-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/json.sample
[jsonc-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsonc.sample
[jsonl-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsonl.sample
[jsx-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/jsx.sample
[julia-base-abstractset]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/base/abstractset.jl
[julia-base-sort]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/base/sort.jl
[julia-licence]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/LICENSE.md
[julia-manual-functions]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/doc/src/manual/functions.md
[julia-manual-strings]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/doc/src/manual/strings.md
[julia-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/julia.sample
[julia-strings-util]: https://github.com/JuliaLang/julia/blob/aca9097c30df5ff53ee088a90cce5ffcea478f70/base/strings/util.jl
[kafka-consumer]: https://github.com/apache/kafka/blob/da5316228ee604724dbcede405ebbf7f18f1dbf3/config/consumer.properties
[kafka-licence]: https://github.com/apache/kafka/blob/da5316228ee604724dbcede405ebbf7f18f1dbf3/LICENSE
[kafka-server]: https://github.com/apache/kafka/blob/da5316228ee604724dbcede405ebbf7f18f1dbf3/config/server.properties
[kingfisher-common-tasks-cache]: https://github.com/onevcat/Kingfisher/blob/770465258c652c73fdef447c10004765c440e993/Sources/Documentation.docc/CommonTasks/CommonTasks_Cache.md
[kingfisher-image-processor]: https://github.com/onevcat/Kingfisher/blob/770465258c652c73fdef447c10004765c440e993/Sources/Image/ImageProcessor.swift
[kingfisher-licence]: https://github.com/onevcat/Kingfisher/blob/770465258c652c73fdef447c10004765c440e993/LICENSE
[knex-licence]: https://github.com/knex/knex/blob/e25d54bcb707714a17f5a5744eba5c4246bb4d1d/LICENSE
[knex-schema]: https://github.com/knex/knex/blob/e25d54bcb707714a17f5a5744eba5c4246bb4d1d/docs/src/guide/schema-builder.md
[kotlin-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/kotlin.sample
[kotlinx-serialization-licence]: https://github.com/Kotlin/kotlinx.serialization/blob/6f6a2e66c3dbf4a34e86a030368ecb0f7a734d22/LICENSE.txt
[ktor-licence]: https://github.com/ktorio/ktor/blob/b7809b1eb47ec68cbb9c78a52a23880caca710cd/LICENSE
[ktor-logging]: https://github.com/ktorio/ktor/blob/b7809b1eb47ec68cbb9c78a52a23880caca710cd/ktor-client/ktor-client-plugins/ktor-client-logging/common/src/io/ktor/client/plugins/logging/Logging.kt
[kubernetes-licence]: https://github.com/kubernetes/kubernetes/blob/98687f28e4fed347d30fcd483edc8b8ee74c583c/LICENSE
[kubernetes-sets]: https://github.com/kubernetes/kubernetes/blob/98687f28e4fed347d30fcd483edc8b8ee74c583c/staging/src/k8s.io/apimachinery/pkg/util/sets/set.go
[ky-core]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/source/core/Ky.ts
[ky-licence]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/license
[ky-readme]: https://github.com/sindresorhus/ky/blob/3541888878c4d303a8bc2ad61815f4e048f55a5d/readme.md
[laravel-licence]: https://github.com/laravel/framework/blob/64dc24f5d965add4d402b4b8b64f59b561112af5/LICENSE.md
[laravel-postgres-grammar]: https://github.com/laravel/framework/blob/64dc24f5d965add4d402b4b8b64f59b561112af5/src/Illuminate/Database/Query/Grammars/PostgresGrammar.php
[laravel-router]: https://github.com/laravel/framework/blob/64dc24f5d965add4d402b4b8b64f59b561112af5/src/Illuminate/Routing/Router.php
[latexrun-licence]: https://github.com/aclements/latexrun/blob/38ff6ec2815654513c91f64bdf2a5760c85da26e/LICENSE
[latexrun-readme]: https://github.com/aclements/latexrun/blob/38ff6ec2815654513c91f64bdf2a5760c85da26e/README.md
[less-antd-button-mixin]: https://github.com/ant-design/ant-design/blob/7c060247516925f48175d7f29cd43b626632665e/components/button/style/mixin.less
[less-antd-color-palette]: https://github.com/ant-design/ant-design/blob/7c060247516925f48175d7f29cd43b626632665e/components/style/color/colorPalette.less
[less-antd-compact-item]: https://github.com/ant-design/ant-design/blob/7c060247516925f48175d7f29cd43b626632665e/components/style/mixins/compact-item.less
[less-antd-customize]: https://github.com/ant-design/ant-design/blob/7c060247516925f48175d7f29cd43b626632665e/components/style/mixins/customize.less
[less-bootstrap3-forms]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/forms.less
[less-bootstrap3-mixins-buttons]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/mixins/buttons.less
[less-bootstrap3-mixins-gradients]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/mixins/gradients.less
[less-bootstrap3-mixins-grid-framework]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/mixins/grid-framework.less
[less-bootstrap3-navbar]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/navbar.less
[less-bootstrap3-variables]: https://github.com/twbs/bootstrap/blob/68b0d231a13201eb14acd3dc84e51543d16e5f7e/less/variables.less
[less-lessjs-at-rules]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/at-rules/at-rules.less
[less-lessjs-at-rules-compressed]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-config/at-rules-compressed/at-rules-compressed.less
[less-lessjs-container]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/container/container.less
[less-lessjs-css-3]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/css-3/css-3.less
[less-lessjs-extract-and-length]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/extract-and-length/extract-and-length.less
[less-lessjs-functions]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/functions/functions.less
[less-lessjs-functions-each]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/functions-each/functions-each.less
[less-lessjs-import-reference]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/import/import/import-reference.less
[less-lessjs-math-strict]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-config/math-strict/css.less
[less-lessjs-merge]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/merge/merge.less
[less-lessjs-selectors]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/selectors/selectors.less
[less-lessjs-sourcemaps-comprehensive]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-config/sourcemaps/comprehensive/comprehensive.less
[less-lessjs-urls]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/packages/test-data/tests-unit/urls/urls.less
[less-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/less.sample
[lessjs-licence]: https://github.com/less/less.js/blob/713331655e437401dfea6cd233893a69762734e1/LICENSE
[leveldb-licence]: https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/LICENSE
[leveldb-version-set]: https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/version_set.cc
[libuv-core]: https://github.com/libuv/libuv/blob/7228e05f3e407ed3fa59edba920b8cd825b5d83d/src/unix/core.c
[libuv-freertos-startup]: https://github.com/libuv/libuv/blob/7228e05f3e407ed3fa59edba920b8cd825b5d83d/test/freertos/mps2/startup.c
[libuv-licence]: https://github.com/libuv/libuv/blob/7228e05f3e407ed3fa59edba920b8cd825b5d83d/LICENSE
[linenoise]: https://github.com/antirez/linenoise/blob/a473823d74b93eab2ba83480df16ed37617493f2/linenoise.c
[linenoise-licence]: https://github.com/antirez/linenoise/blob/a473823d74b93eab2ba83480df16ed37617493f2/LICENSE
[live-dashboard-layered-graph]: https://github.com/phoenixframework/phoenix_live_dashboard/blob/83a0bd137ed3e4c66b8037b140a2744803a48e13/lib/phoenix/live_dashboard/components/layered_graph_component.ex
[live-dashboard-licence]: https://github.com/phoenixframework/phoenix_live_dashboard/blob/83a0bd137ed3e4c66b8037b140a2744803a48e13/LICENSE.md
[live-dashboard-nav-bar]: https://github.com/phoenixframework/phoenix_live_dashboard/blob/83a0bd137ed3e4c66b8037b140a2744803a48e13/lib/phoenix/live_dashboard/components/nav_bar_component.ex
[live-dashboard-table]: https://github.com/phoenixframework/phoenix_live_dashboard/blob/83a0bd137ed3e4c66b8037b140a2744803a48e13/lib/phoenix/live_dashboard/components/table_component.ex
[lua-licence]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/lua.h
[lua-literals]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/testes/literals.lua
[lua-locals]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/testes/locals.lua
[lua-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/lua.sample
[lua-vm]: https://github.com/lua/lua/blob/8cb93bada561977eb370819e39d7d644e5fd0a3f/lvm.c
[luals-basic]: https://github.com/LuaLS/lua-language-server/blob/a58fc7b01e1746c46275c71b0e61bfed40a6296d/meta/template/basic.lua
[luals-licence]: https://github.com/LuaLS/lua-language-server/blob/a58fc7b01e1746c46275c71b0e61bfed40a6296d/LICENSE
[luals-string]: https://github.com/LuaLS/lua-language-server/blob/a58fc7b01e1746c46275c71b0e61bfed40a6296d/meta/template/string.lua
[luals-table]: https://github.com/LuaLS/lua-language-server/blob/a58fc7b01e1746c46275c71b0e61bfed40a6296d/meta/template/table.lua
[luarocks-licence]: https://github.com/luarocks/luarocks/blob/85bfe36c812465b2f377877939d884a4db829a25/COPYING
[luarocks-util]: https://github.com/luarocks/luarocks/blob/85bfe36c812465b2f377877939d884a4db829a25/src/luarocks/core/util.lua
[lume]: https://github.com/rxi/lume/blob/98847e7812cf28d3d64b289b03fad71dc704547d/lume.lua
[lume-licence]: https://github.com/rxi/lume/blob/98847e7812cf28d3d64b289b03fad71dc704547d/LICENSE
[make-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/make.sample
[markdown-markdownlint-blanks-around-headings-arrays]: https://github.com/DavidAnson/markdownlint/blob/b67636d47f335cf124990faae4a72c5e9fadd613/test/blanks-around-headings-arrays.md
[markdown-markdownlint-link-style-autolink-only]: https://github.com/DavidAnson/markdownlint/blob/b67636d47f335cf124990faae4a72c5e9fadd613/test/link-style-autolink-only.md
[markdown-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/markdown.sample
[markdownlint-licence]: https://github.com/DavidAnson/markdownlint/blob/b67636d47f335cf124990faae4a72c5e9fadd613/LICENSE
[masonry-constraint-maker]: https://github.com/SnapKit/Masonry/blob/8bd77ea92bbe995e14c454f821200b222e5a8804/Masonry/MASConstraintMaker.m
[masonry-licence]: https://github.com/SnapKit/Masonry/blob/8bd77ea92bbe995e14c454f821200b222e5a8804/LICENSE
[mathjax-demos-licence]: https://github.com/mathjax/MathJax-demos-web/blob/82c49422ee92dc58fbeed278532495d52c00aed0/LICENSE
[mathjax-mml-chtml]: https://github.com/mathjax/MathJax-demos-web/blob/82c49422ee92dc58fbeed278532495d52c00aed0/page/mml-chtml.html
[mathjax-tex-mml]: https://github.com/mathjax/MathJax-demos-web/blob/82c49422ee92dc58fbeed278532495d52c00aed0/page/tex-mml.html
[matlab-grammar-arguments-block]: https://github.com/mathworks/MATLAB-Language-grammar/blob/69dbf20c1eb19efd629c0438a2a493e60617e8de/test/t13ArgumentsBlock.m
[matlab-grammar-licence]: https://github.com/mathworks/MATLAB-Language-grammar/blob/69dbf20c1eb19efd629c0438a2a493e60617e8de/license.txt
[matlab-grammar-property-validation]: https://github.com/mathworks/MATLAB-Language-grammar/blob/69dbf20c1eb19efd629c0438a2a493e60617e8de/test/t08PropertyValidation.m
[matlab-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/matlab.sample
[mbprogresshud]: https://github.com/jdg/MBProgressHUD/blob/4a7c5f3e53cdea77c5dcb8578c2ee5acacdf6781/MBProgressHUD.m
[mbprogresshud-licence]: https://github.com/jdg/MBProgressHUD/blob/4a7c5f3e53cdea77c5dcb8578c2ee5acacdf6781/LICENSE
[mermaid-api-sequence]: https://github.com/mermaid-js/mermaid/blob/ea53556f130b397fb6f781818ee0f1e924b429eb/docs/diagrams/mermaid-api-sequence.mmd
[mermaid-flowchart-code-flow]: https://github.com/mermaid-js/mermaid/blob/ea53556f130b397fb6f781818ee0f1e924b429eb/docs/diagrams/flowchart-code-flow.mmd
[mermaid-licence]: https://github.com/mermaid-js/mermaid/blob/ea53556f130b397fb6f781818ee0f1e924b429eb/LICENSE
[mermaid-mindmap-sequence]: https://github.com/mermaid-js/mermaid/blob/ea53556f130b397fb6f781818ee0f1e924b429eb/docs/diagrams/mindmap-implementation-sequence.mmd
[mermaid-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/mermaid.sample
[milligram-css]: https://github.com/milligram/milligram/blob/5e56af462aea08baf5011a045bf1bfaaef72552c/dist/milligram.css
[milligram-licence]: https://github.com/milligram/milligram/blob/5e56af462aea08baf5011a045bf1bfaaef72552c/license
[minikube-licence]: https://github.com/kubernetes/minikube/blob/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996/LICENSE
[minikube-make]: https://github.com/kubernetes/minikube/blob/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996/Makefile
[mir-cast]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/tests/mir-opt/const_prop/cast.main.GVN.diff
[mir-slices]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/tests/mir-opt/gvn.slices.GVN.panic-abort.diff
[moby-docker]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/Dockerfile
[moby-licence]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/LICENSE
[moby-make]: https://github.com/moby/moby/blob/aed2b781abc209a639b79d00f6962b629242aa1a/Makefile
[moshi-licence]: https://github.com/square/moshi/blob/889013ec2edb8d8034902662a1dc8c4f3b3f8111/LICENSE.txt
[moshi-util]: https://github.com/square/moshi/blob/889013ec2edb8d8034902662a1dc8c4f3b3f8111/moshi/src/main/java/com/squareup/moshi/internal/Util.kt
[newtonsoft-json-convert]: https://github.com/JamesNK/Newtonsoft.Json/blob/52fa3aef1f2cadcd3a3f874251eddc98d3efbbaa/Src/Newtonsoft.Json/JsonConvert.cs
[newtonsoft-licence]: https://github.com/JamesNK/Newtonsoft.Json/blob/52fa3aef1f2cadcd3a3f874251eddc98d3efbbaa/LICENSE.md
[next-docker]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker/Dockerfile
[next-docker-export-readme]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker-export-output/README.md
[next-docker-readme]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/examples/with-docker/README.md
[next-licence]: https://github.com/vercel/next.js/blob/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378/license.md
[nio-codec]: https://github.com/apple/swift-nio/blob/2f58d8ef84438d6517fc78bcea2f8527e3c72936/Sources/NIOCore/Codec.swift
[nio-licence]: https://github.com/apple/swift-nio/blob/2f58d8ef84438d6517fc78bcea2f8527e3c72936/LICENSE.txt
[nix-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/nix.sample
[nixpkgs-licence]: https://github.com/NixOS/nixpkgs/blob/084f878659a86f432881f0267c50863df444e925/COPYING
[nixpkgs-lists]: https://github.com/NixOS/nixpkgs/blob/084f878659a86f432881f0267c50863df444e925/lib/lists.nix
[nixpkgs-ripgrep]: https://github.com/NixOS/nixpkgs/blob/084f878659a86f432881f0267c50863df444e925/pkgs/by-name/ri/ripgrep/package.nix
[nlohmann-json-licence]: https://github.com/nlohmann/json/blob/e32051125faf95dc64d49a129959b6742b9ffee9/LICENSE.MIT
[node-licence]: https://github.com/nodejs/node/blob/3556d018a4863a42fba62a927e7e605c1e8cf128/LICENSE
[node-make]: https://github.com/nodejs/node/blob/3556d018a4863a42fba62a927e7e605c1e8cf128/Makefile
[normalize-css]: https://github.com/necolas/normalize.css/blob/fc091cce1534909334c1911709a39c22d406977b/normalize.css
[normalize-licence]: https://github.com/necolas/normalize.css/blob/fc091cce1534909334c1911709a39c22d406977b/LICENSE.md
[nvm]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/nvm.sh
[nvm-licence]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/LICENSE.md
[nvm-make]: https://github.com/nvm-sh/nvm/blob/16df5aa69aeda3814703e14f9d35a4f60220dd37/Makefile
[nvm-make-licence]: https://github.com/nvm-sh/nvm/blob/16df5aa69aeda3814703e14f9d35a4f60220dd37/LICENSE.md
[nvm-readme]: https://github.com/nvm-sh/nvm/blob/5e38bc0caed342356ffd26eb21e0e9b995ac228b/README.md
[objcpp-grammar-licence]: https://github.com/jeff-hykin/better-objcpp-syntax/blob/5a7eb15eee382dd5aa388bc04fdb60a0d2128e14/LICENSE
[objcpp-grammar-misc-002]: https://github.com/jeff-hykin/better-objcpp-syntax/blob/5a7eb15eee382dd5aa388bc04fdb60a0d2128e14/language_examples/misc_002.mm
[objcpp-grammar-misc-003]: https://github.com/jeff-hykin/better-objcpp-syntax/blob/5a7eb15eee382dd5aa388bc04fdb60a0d2128e14/language_examples/misc_003.mm
[objective-c-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/objective-c.sample
[objective-cpp-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/objective-cpp.sample
[ohmyzsh-alias-finder-test]: https://github.com/ohmyzsh/ohmyzsh/blob/b509b1c6ed0c1ba37de12cbd37f30b2b2f61193a/plugins/alias-finder/tests/test_run.sh
[ohmyzsh-licence]: https://github.com/ohmyzsh/ohmyzsh/blob/b509b1c6ed0c1ba37de12cbd37f30b2b2f61193a/LICENSE.txt
[okhttp-build]: https://github.com/square/okhttp/blob/16fb6aef3aee1c4645fc8d24af01f8a4dc128ebf/okhttp/build.gradle.kts
[okhttp-cookie]: https://github.com/square/okhttp/blob/16fb6aef3aee1c4645fc8d24af01f8a4dc128ebf/okhttp/src/commonJvmAndroid/kotlin/okhttp3/Cookie.kt
[okhttp-licence]: https://github.com/square/okhttp/blob/16fb6aef3aee1c4645fc8d24af01f8a4dc128ebf/LICENSE.txt
[okhttp-request]: https://github.com/square/okhttp/blob/16fb6aef3aee1c4645fc8d24af01f8a4dc128ebf/okhttp/src/commonJvmAndroid/kotlin/okhttp3/Request.kt
[one-light]: https://github.com/akamud/vscode-theme-onelight/tree/5866e900db932d580e978a58db42f65cde07998b
[one-light-licence]: https://github.com/akamud/vscode-theme-onelight/blob/5866e900db932d580e978a58db42f65cde07998b/LICENSE
[pascal-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/pascal.sample
[penlight-licence]: https://github.com/lunarmodules/Penlight/blob/7f3351f226c2f643aaa1d3afa8d7293dd8a1d985/LICENSE.md
[penlight-stringx]: https://github.com/lunarmodules/Penlight/blob/7f3351f226c2f643aaa1d3afa8d7293dd8a1d985/lua/pl/stringx.lua
[perl-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/perl.sample
[petstore-licence]: https://github.com/swagger-api/swagger-petstore/blob/5cb909fc7780bc708151bce0014939acba8b212a/LICENSE
[petstore-openapi]: https://github.com/swagger-api/swagger-petstore/blob/5cb909fc7780bc708151bce0014939acba8b212a/src/main/resources/openapi.yaml
[pg-create]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/create_table.sql
[pg-select]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/select.sql
[pg-with]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/src/test/regress/sql/with.sql
[pgvector-licence]: https://github.com/pgvector/pgvector/blob/1d5ef0a160cadec08f754528669ce17e20eab38c/LICENSE
[pgvector-readme]: https://github.com/pgvector/pgvector/blob/1d5ef0a160cadec08f754528669ce17e20eab38c/README.md
[php-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/php.sample
[poetry-faq]: https://github.com/python-poetry/poetry/blob/6ecadf5b71309b1da4f00949cc349a79884cff8f/docs/faq.md
[poetry-licence]: https://github.com/python-poetry/poetry/blob/6ecadf5b71309b1da4f00949cc349a79884cff8f/LICENSE
[poetry-pyproject]: https://github.com/python-poetry/poetry/blob/6ecadf5b71309b1da4f00949cc349a79884cff8f/pyproject.toml
[postcss-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/postcss.sample
[postgres-docker]: https://github.com/docker-library/postgres/blob/dfc76bb299b057785026eb2eea39556b6fe52983/18/bookworm/Dockerfile
[postgres-docker-licence]: https://github.com/docker-library/postgres/blob/dfc76bb299b057785026eb2eea39556b6fe52983/LICENSE
[postgres-licence]: https://github.com/postgres/postgres/blob/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3/COPYRIGHT
[powershell-licence]: https://github.com/PowerShell/PowerShell/blob/2725d6aaa6e67219b6c9902adf7b74c52e0371a3/LICENSE.txt
[powershell-releasing]: https://github.com/PowerShell/PowerShell/blob/2725d6aaa6e67219b6c9902adf7b74c52e0371a3/docs/maintainers/releasing.md
[powershell-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/powershell.sample
[powershell-windows-build]: https://github.com/PowerShell/PowerShell/blob/2725d6aaa6e67219b6c9902adf7b74c52e0371a3/docs/building/windows-core.md
[preact-keys]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/test/browser/keys.test.jsx
[preact-licence]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/LICENSE
[preact-readme]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/README.md
[preact-render]: https://github.com/preactjs/preact/blob/81dbe69d39e948b2305193d2622c225699311083/compat/test/browser/render.test.jsx
[prettier-licence]: https://github.com/prettier/prettier/blob/a02e92be26ec8963a07aa9500a6c20e24599e95b/LICENSE
[prettier-plugins]: https://github.com/prettier/prettier/blob/a02e92be26ec8963a07aa9500a6c20e24599e95b/docs/plugins.md
[prometheus-config]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/documentation/examples/prometheus.yml
[prometheus-docker]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/Dockerfile
[prometheus-licence]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/LICENSE
[prometheus-make]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/Makefile.common
[prometheus-make-licence]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/LICENSE
[prometheus-storage]: https://github.com/prometheus/prometheus/blob/b0c3a30ab2a09b88e64d876017a0134aa2f38f40/storage/interface.go
[proto-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/proto.sample
[pubs]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/samples/databases/northwind-pubs/instpubs.sql
[puma-licence]: https://github.com/puma/puma/blob/c0df0c19dde943c3f5c25cd67caa984dfa84e3da/LICENSE
[puma-server]: https://github.com/puma/puma/blob/c0df0c19dde943c3f5c25cd67caa984dfa84e3da/lib/puma/server.rb
[pyenv-init]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/libexec/pyenv-init
[pyenv-licence]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/LICENSE
[pyenv-readme]: https://github.com/pyenv/pyenv/blob/e6669f0a84df60daba8672c3b3f0caebd0fdcef8/README.md
[pytest-licence]: https://github.com/pytest-dev/pytest/blob/1211245aa72f1efe07bcfce63a7c543156cee976/LICENSE
[pytest-python]: https://github.com/pytest-dev/pytest/blob/1211245aa72f1efe07bcfce63a7c543156cee976/src/_pytest/python.py
[pytest-tox]: https://github.com/pytest-dev/pytest/blob/1211245aa72f1efe07bcfce63a7c543156cee976/tox.ini
[python-black-attribute-access-on-number-literals]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/cases/attribute_access_on_number_literals.py
[python-black-bytes-docstring]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/cases/bytes_docstring.py
[python-black-fmtonoff]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/cases/fmtonoff.py
[python-black-pep-701]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/cases/pep_701.py
[python-black-python2-detection]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/miscellaneous/python2_detection.py
[python-black-string-prefixes]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/cases/string_prefixes.py
[python-black-string-quotes]: https://github.com/psf/black/blob/945d902bef347d2e7e1d26897aa89a9c177fb535/tests/data/miscellaneous/string_quotes.py
[python-docker]: https://github.com/docker-library/python/blob/f4c67ece208dead594e43628d22e3c6027dd97ab/3.13/slim-bookworm/Dockerfile
[python-docker-licence]: https://github.com/docker-library/python/blob/f4c67ece208dead594e43628d22e3c6027dd97ab/LICENSE
[python-ruff-class]: https://github.com/astral-sh/ruff/blob/f10755306e3e932fd127960acb2c892fafb87410/crates/ruff_python_parser/resources/valid/statement/class.py
[python-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/python.sample
[quicklisp-http]: https://github.com/quicklisp/quicklisp-client/blob/6d96fd86c114345bd74d995bb296fae46f1f26ec/quicklisp/http.lisp
[quicklisp-licence]: https://github.com/quicklisp/quicklisp-client/blob/6d96fd86c114345bd74d995bb296fae46f1f26ec/LICENSE.txt
[r8brain-dll]: https://github.com/avaneev/r8brain-free-src/blob/cb2abb9977efe2471979b380ed95daa56ab4fdb9/DLL/r8bsrc.pas
[r8brain-licence]: https://github.com/avaneev/r8brain-free-src/blob/cb2abb9977efe2471979b380ed95daa56ab4fdb9/LICENSE
[rack-licence]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/MIT-LICENSE
[rack-readme]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/README.md
[rack-request]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/lib/rack/request.rb
[rack-response]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/lib/rack/response.rb
[rack-spec-headers]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/test/spec_headers.rb
[rack-spec-lint]: https://github.com/rack/rack/blob/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f/test/spec_lint.rb
[rag-licence]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/LICENSE
[rag-output]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/output.jsonl
[rag-output-50]: https://github.com/jxnl/n-levels-of-rag/blob/2ce1106b0163d11fcf7959187b42f0d3a67fbe05/output-50.jsonl
[rails-app-layout]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/generators/rails/app/templates/app/views/layouts/application.html.erb.tt
[rails-callbacks]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/activesupport/lib/active_support/callbacks.rb
[rails-inflector]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/activesupport/lib/active_support/inflector/methods.rb
[rails-info-notes]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/templates/rails/info/notes.html.erb
[rails-licence]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/activesupport/MIT-LICENSE
[rails-mailer-email]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/templates/rails/mailers/email.html.erb
[rails-pwa-offline]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/generators/rails/app/templates/app/views/pwa/offline.html.erb
[rails-rescues-diagnostics]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/actionpack/lib/action_dispatch/middleware/templates/rescues/diagnostics.html.erb
[rails-rescues-source]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/actionpack/lib/action_dispatch/middleware/templates/rescues/_source.html.erb
[rails-routes-table]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/actionpack/lib/action_dispatch/middleware/templates/routes/_table.html.erb
[rails-scaffold-form]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/generators/erb/scaffold/templates/_form.html.erb.tt
[rails-welcome]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/lib/rails/templates/rails/welcome/index.html.erb
[railties-licence]: https://github.com/rails/rails/blob/f21e7f42b510327f6bc57dcb7fcc9825d4670a91/railties/MIT-LICENSE
[rake-licence]: https://github.com/ruby/rake/blob/3f29d39daa688eb51cd975c06b1501320f10b911/MIT-LICENSE
[rake-reduce-compat-test]: https://github.com/ruby/rake/blob/3f29d39daa688eb51cd975c06b1501320f10b911/test/test_rake_reduce_compat.rb
[rake-task]: https://github.com/ruby/rake/blob/3f29d39daa688eb51cd975c06b1501320f10b911/lib/rake/task.rb
[rapidjson-licence]: https://github.com/Tencent/rapidjson/blob/24b5e7a8b27f42fa16b96fc70aade9106cf7102f/license.txt
[rapidjson-reader]: https://github.com/Tencent/rapidjson/blob/24b5e7a8b27f42fa16b96fc70aade9106cf7102f/include/rapidjson/reader.h
[react-native-component-view-registry]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/Fabric/Mounting/RCTComponentViewRegistry.mm
[react-native-dev-loading-view]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/CoreModules/RCTDevLoadingView.mm
[react-native-device-info]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/CoreModules/RCTDeviceInfo.mm
[react-native-legacy-interop-coordinator]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/ReactCommon/react/renderer/components/legacyviewmanagerinterop/platform/ios/react/renderer/components/legacyviewmanagerinterop/RCTLegacyViewManagerInteropCoordinator.mm
[react-native-licence]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/LICENSE
[react-native-mounting-manager]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/Fabric/Mounting/RCTMountingManager.mm
[react-native-surface-presenter]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/Fabric/RCTSurfacePresenter.mm
[react-native-virtual-view]: https://github.com/facebook/react-native/blob/25d63d471a91cf9c2141bbe8dab6802eb73c2567/packages/react-native/React/Fabric/Mounting/ComponentViews/VirtualView/RCTVirtualViewComponentView.mm
[requests-licence]: https://github.com/psf/requests/blob/3c7baaa05ab4aa49cc819ce3a819d260de19c157/LICENSE
[requests-models]: https://github.com/psf/requests/blob/3c7baaa05ab4aa49cc819ce3a819d260de19c157/src/requests/models.py
[rewrite-licence]: https://github.com/openrewrite/rewrite/blob/03d32965a3cdbf06f2717eb3ca706a162039703c/LICENSE
[rewrite-switch-patterns]: https://github.com/openrewrite/rewrite/blob/03d32965a3cdbf06f2717eb3ca706a162039703c/rewrite-java-tck/src/main/java/org/openrewrite/java/tree/SwitchPatternMatchingTest.java
[rich-console]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/rich/console.py
[rich-licence]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/LICENSE
[rich-readme]: https://github.com/Textualize/rich/blob/9d8f9a372cc5916fd4781fec207ced7ddac2f08f/README.md
[ripgrep-ci]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/.github/workflows/ci.yml
[ripgrep-licence]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/COPYING
[ripgrep-manifest]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/Cargo.toml
[ripgrep-searcher]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/crates/searcher/src/searcher/mod.rs
[rome-at]: https://github.com/rometools/rome/blob/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75/rome-modules/src/test/resources/xml/at.xml
[rome-licence]: https://github.com/rometools/rome/blob/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75/LICENSE
[rome-slashdot]: https://github.com/rometools/rome/blob/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75/rome-modules/src/test/resources/xml/slashdot.xml
[rome-yahoo-top-songs]: https://github.com/rometools/rome/blob/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75/rome-modules/src/test/resources/data/YahooTopSongs.xml
[roslyn-error-method-symbol]: https://github.com/dotnet/roslyn/blob/868edfafce0460b50e1abe82f1bb0872979dbf0a/src/Compilers/VisualBasic/Portable/Symbols/ErrorMethodSymbol.vb
[roslyn-licence]: https://github.com/dotnet/roslyn/blob/868edfafce0460b50e1abe82f1bb0872979dbf0a/License.txt
[roslyn-named-type-binder]: https://github.com/dotnet/roslyn/blob/868edfafce0460b50e1abe82f1bb0872979dbf0a/src/Compilers/VisualBasic/Portable/Binding/NamedTypeBinder.vb
[roslyn-text-span]: https://github.com/dotnet/roslyn/blob/868edfafce0460b50e1abe82f1bb0872979dbf0a/src/Compilers/Core/Portable/Text/TextSpan.cs
[rspec-core-licence]: https://github.com/rspec/rspec-core/blob/aec5f49485d97908183dbe790a7fefb8baaa8091/LICENSE.md
[rspec-example]: https://github.com/rspec/rspec-core/blob/aec5f49485d97908183dbe790a7fefb8baaa8091/lib/rspec/core/example.rb
[rubocop-cop-base]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/lib/rubocop/cop/base.rb
[rubocop-duplicated-gem]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/lib/rubocop/cop/bundler/duplicated_gem.rb
[rubocop-licence]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/LICENSE.txt
[rubocop-line-length-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/layout/line_length_spec.rb
[rubocop-percent-literal-delimiters-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/style/percent_literal_delimiters_spec.rb
[rubocop-quoted-symbols-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/style/quoted_symbols_spec.rb
[rubocop-redundant-percent-q-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/style/redundant_percent_q_spec.rb
[rubocop-regexp-literal-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/style/regexp_literal_spec.rb
[rubocop-symbol-array-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/style/symbol_array_spec.rb
[rubocop-useless-assignment-spec]: https://github.com/rubocop/rubocop/blob/09c38e2526622e5c08a73ebff58fd2f4bed35ffe/spec/rubocop/cop/lint/useless_assignment_spec.rb
[ruby-build]: https://github.com/rbenv/ruby-build/blob/4ef3159f94dd66c8325efb42db2faeb7d5732db0/bin/ruby-build
[ruby-build-licence]: https://github.com/rbenv/ruby-build/blob/4ef3159f94dd66c8325efb42db2faeb7d5732db0/LICENSE
[ruby-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/ruby.sample
[ruff-licence]: https://github.com/astral-sh/ruff/blob/f10755306e3e932fd127960acb2c892fafb87410/LICENSE
[ruff-pyproject]: https://github.com/astral-sh/ruff/blob/f10755306e3e932fd127960acb2c892fafb87410/pyproject.toml
[runtime-linq-where]: https://github.com/dotnet/runtime/blob/6375d63af99b6812f11b4dd8c4a18d227e277d73/src/libraries/System.Linq/src/System/Linq/Where.cs
[runtime-stack]: https://github.com/dotnet/runtime/blob/6375d63af99b6812f11b4dd8c4a18d227e277d73/src/libraries/System.Collections/src/System/Collections/Generic/Stack.cs
[rust-core-macros]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/library/core/src/macros/mod.rs
[rust-licence]: https://github.com/rust-lang/rust/blob/ae36fe22c7e590df40ffd53ac25fea35597c3f3d/LICENSE-MIT
[rust-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/rust.sample
[sass-bulma-functions]: https://github.com/jgthms/bulma/blob/3e00a8e6d0d0e566d507328f0185ef84854effba/sass/utilities/functions.sass
[sass-bulma-generic]: https://github.com/jgthms/bulma/blob/3e00a8e6d0d0e566d507328f0185ef84854effba/sass/base/generic.sass
[sass-bulma-mixins]: https://github.com/jgthms/bulma/blob/3e00a8e6d0d0e566d507328f0185ef84854effba/sass/utilities/mixins.sass
[sass-bulma-navbar]: https://github.com/jgthms/bulma/blob/3e00a8e6d0d0e566d507328f0185ef84854effba/sass/components/navbar.sass
[sass-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/sass.sample
[scala-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/scala.sample
[scala3-boundary]: https://github.com/scala/scala3/blob/d71919d08172571e6a5c9392f186b2a6a4413097/library/src/scala/util/boundary.scala
[scala3-enums]: https://github.com/scala/scala3/blob/d71919d08172571e6a5c9392f186b2a6a4413097/docs/_docs/reference/enums/enums.md
[scala3-givens]: https://github.com/scala/scala3/blob/d71919d08172571e6a5c9392f186b2a6a4413097/docs/_docs/reference/contextual/givens.md
[scala3-licence]: https://github.com/scala/scala3/blob/d71919d08172571e6a5c9392f186b2a6a4413097/LICENSE
[scala3-source-position]: https://github.com/scala/scala3/blob/d71919d08172571e6a5c9392f186b2a6a4413097/compiler/src/dotty/tools/dotc/util/SourcePosition.scala
[scss-angular-menu]: https://github.com/angular/components/blob/6b8d6a47693cf9c862c46919e5de8982fa3fd417/src/material/menu/menu.scss
[scss-bootstrap-button]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/buttons/_button.scss
[scss-bootstrap-functions]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/_functions.scss
[scss-bootstrap-mixins-form-validation]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/mixins/_form-validation.scss
[scss-bootstrap-mixins-grid]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/mixins/_grid.scss
[scss-bootstrap-navbar]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/_navbar.scss
[scss-bootstrap-root]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/_root.scss
[scss-bootstrap-utilities]: https://github.com/twbs/bootstrap/blob/2871f2a55ccf18af64045b8fc37529391b285900/scss/_utilities.scss
[scss-bulma-button]: https://github.com/jgthms/bulma/blob/c5cd04baf0258cfcc77b4e8610488a0d439effa2/sass/elements/button.scss
[scss-bulma-functions]: https://github.com/jgthms/bulma/blob/c5cd04baf0258cfcc77b4e8610488a0d439effa2/sass/utilities/functions.scss
[scss-bulma-mixins]: https://github.com/jgthms/bulma/blob/c5cd04baf0258cfcc77b4e8610488a0d439effa2/sass/utilities/mixins.scss
[scss-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/scss.sample
[sdwebimage-graphics-image-renderer]: https://github.com/SDWebImage/SDWebImage/blob/c3ad5e1a9bf55c9b76d4c362430b5fcded96c502/SDWebImage/Core/SDGraphicsImageRenderer.m
[sdwebimage-image-cache]: https://github.com/SDWebImage/SDWebImage/blob/c3ad5e1a9bf55c9b76d4c362430b5fcded96c502/SDWebImage/Core/SDImageCache.h
[sdwebimage-licence]: https://github.com/SDWebImage/SDWebImage/blob/c3ad5e1a9bf55c9b76d4c362430b5fcded96c502/LICENSE
[sdwebimage-manager]: https://github.com/SDWebImage/SDWebImage/blob/c3ad5e1a9bf55c9b76d4c362430b5fcded96c502/SDWebImage/Core/SDWebImageManager.m
[serde-de]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/serde_core/src/de/mod.rs
[serde-licence]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/LICENSE-MIT
[serde-readme]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/README.md
[serialization-json]: https://github.com/Kotlin/kotlinx.serialization/blob/6f6a2e66c3dbf4a34e86a030368ecb0f7a734d22/formats/json/commonMain/src/kotlinx/serialization/json/Json.kt
[set-release-tag]: https://github.com/PowerShell/PowerShell/blob/2725d6aaa6e67219b6c9902adf7b74c52e0371a3/tools/releaseBuild/setReleaseTag.ps1
[shadcn-chart]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/chart.tsx
[shadcn-combobox]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/combobox.tsx
[shadcn-data-table]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/content/docs/components/radix/data-table.mdx
[shadcn-licence]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/LICENSE.md
[shadcn-message-scroller]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/content/docs/components/radix/message-scroller.mdx
[shadcn-sidebar]: https://github.com/shadcn-ui/ui/blob/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610/apps/v4/registry/new-york-v4/ui/sidebar.tsx
[shell-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/shellscript.sample
[shiki-samples-licence]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/LICENSE
[snapshot-assert-snapshot]: https://github.com/pointfreeco/swift-snapshot-testing/blob/0c6380dd46cd45e00f2bf87cf6002f3478e84452/Sources/SnapshotTesting/AssertSnapshot.swift
[snapshot-testing-licence]: https://github.com/pointfreeco/swift-snapshot-testing/blob/0c6380dd46cd45e00f2bf87cf6002f3478e84452/LICENSE
[spdlog-licence]: https://github.com/gabime/spdlog/blob/6d9805df3f588402b17073dcd13eb569ceda864d/LICENSE
[spdlog-logger]: https://github.com/gabime/spdlog/blob/6d9805df3f588402b17073dcd13eb569ceda864d/include/spdlog/logger.h
[split-sequence-licence]: https://github.com/sharplispers/split-sequence/blob/89a10b4d697f03eb32ade3c373c4fd69800a841a/LICENSE
[split-sequence-list]: https://github.com/sharplispers/split-sequence/blob/89a10b4d697f03eb32ade3c373c4fd69800a841a/list.lisp
[sqitch-engine-pg]: https://github.com/sqitchers/sqitch/blob/b08e5c8a773ee4ab5ea4ab7cf6f7a16c55d6994e/lib/App/Sqitch/Engine/pg.pm
[sqitch-engine-sqlite]: https://github.com/sqitchers/sqitch/blob/b08e5c8a773ee4ab5ea4ab7cf6f7a16c55d6994e/lib/App/Sqitch/Engine/sqlite.pm
[sqitch-licence]: https://github.com/sqitchers/sqitch/blob/b08e5c8a773ee4ab5ea4ab7cf6f7a16c55d6994e/LICENSE.md
[sql-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/sql.sample
[sqlalchemy-licence]: https://github.com/sqlalchemy/sqlalchemy/blob/679372836fe7343683248467d276036a6d6f1673/LICENSE
[sqlalchemy-tox]: https://github.com/sqlalchemy/sqlalchemy/blob/679372836fe7343683248467d276036a6d6f1673/tox.ini
[sqlserver-licence]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/license.txt
[ssms-create]: https://github.com/microsoft/sql-server-samples/blob/3c71696a3942879f0481f3bd65fe112a40023f18/samples/features/ssms-templates/Sql/Table/Create%20Table.sql
[string-processing-algorithms-tests]: https://github.com/swiftlang/swift-experimental-string-processing/blob/93065a071c4bcfc5b23e9ee3dec49f9bc9b257ee/Tests/RegexTests/AlgorithmsTests.swift
[string-processing-custom-tests]: https://github.com/swiftlang/swift-experimental-string-processing/blob/93065a071c4bcfc5b23e9ee3dec49f9bc9b257ee/Tests/RegexBuilderTests/CustomTests.swift
[string-processing-licence]: https://github.com/swiftlang/swift-experimental-string-processing/blob/93065a071c4bcfc5b23e9ee3dec49f9bc9b257ee/LICENSE.txt
[svelte-kit-licence]: https://github.com/sveltejs/kit/blob/decaf66ff3198b70a4fafa0242f60c6a07481ee3/LICENSE
[svelte-kit-root]: https://github.com/sveltejs/kit/blob/decaf66ff3198b70a4fafa0242f60c6a07481ee3/packages/kit/src/runtime/components/root.svelte
[svelte-licence]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/LICENSE.md
[svelte-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/svelte.sample
[svelte-site-kit-docs-contents]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/docs/DocsContents.svelte
[svelte-site-kit-licence]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/LICENSE
[svelte-site-kit-mobile-menu]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/nav/MobileMenu.svelte
[svelte-site-kit-nav]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/nav/Nav.svelte
[svelte-site-kit-search-box]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/search/SearchBox.svelte
[svelte-site-kit-search-result-list]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/search/SearchResultList.svelte
[svelte-site-kit-text]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/components/Text.svelte
[svelte-site-kit-tooltip]: https://github.com/sveltejs/svelte.dev/blob/d3b7b75aadce5184bf52ca211f58ec5ba0130bd8/packages/site-kit/src/lib/docs/Tooltip.svelte
[svelte-tests-const-tag-component]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/packages/svelte/tests/runtime-legacy/samples/const-tag-component/main.svelte
[svelte-tests-destructure-async-assignments]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/packages/svelte/tests/runtime-runes/samples/destructure-async-assignments/main.svelte
[svelte-tests-form-default-value]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/packages/svelte/tests/runtime-runes/samples/form-default-value/main.svelte
[svelte-tests-props-default-value-behavior]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/packages/svelte/tests/runtime-runes/samples/props-default-value-behavior/main.svelte
[svelte-tests-style-directive-mutations]: https://github.com/sveltejs/svelte/blob/628098d2c662fabf4c1f3cf6564c6b20964ad9a8/packages/svelte/tests/runtime-runes/samples/style-directive-mutations/main.svelte
[swift-licence]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/LICENSE.txt
[swift-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/swift.sample
[swift-test-decl-func-keyword-argument-labels]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/decl/func/keyword-argument-labels.swift
[swift-test-decl-func-rethrows]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/decl/func/rethrows.swift
[swift-test-decl-import-import]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/decl/import/import.swift
[swift-test-decl-protocol-protocol-enum-witness]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/decl/protocol/protocol_enum_witness.swift
[swift-test-decl-protocol-protocols]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/decl/protocol/protocols.swift
[swift-test-expr-expressions]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/expr/expressions.swift
[swift-test-expr-primary-selector-selector]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/expr/primary/selector/selector.swift
[swift-test-expr-unary-async-await]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/expr/unary/async_await.swift
[swift-test-parse-conditionalcompilation-basicparseerrors]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/ConditionalCompilation/basicParseErrors.swift
[swift-test-parse-escaped-identifiers]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/escaped_identifiers.swift
[swift-test-parse-operator-decl]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/operator_decl.swift
[swift-test-parse-raw-string]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/raw_string.swift
[swift-test-parse-raw-string-errors]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/raw_string_errors.swift
[swift-test-parse-trailing-comma]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/trailing-comma.swift
[swift-test-parse-type-expr]: https://github.com/swiftlang/swift/blob/f21f2c19ef734859c366829e0d23927813040968/test/Parse/type_expr.swift
[swift-testing-licence]: https://github.com/swiftlang/swift-testing/blob/87ec972e0b5725ee02b31a6e9ed690d85abb460a/LICENSE.txt
[tailwind-licence]: https://github.com/tailwindlabs/tailwindcss/blob/2626ae6ceddf36069ab6d55c53a5857c1e2e360a/LICENSE
[tailwind-preflight]: https://github.com/tailwindlabs/tailwindcss/blob/2626ae6ceddf36069ab6d55c53a5857c1e2e360a/packages/tailwindcss/preflight.css
[testify-assertions]: https://github.com/stretchr/testify/blob/87a7b9d57689f6579db2da795ab8deeab29cb724/assert/assertions.go
[testify-licence]: https://github.com/stretchr/testify/blob/87a7b9d57689f6579db2da795ab8deeab29cb724/LICENSE
[testing-runner-tests]: https://github.com/swiftlang/swift-testing/blob/87ec972e0b5725ee02b31a6e9ed690d85abb460a/Tests/TestingTests/RunnerTests.swift
[thor]: https://github.com/rails/thor/blob/cb11e8e87b3cfe6bc9f6d4b092897f0dfda2476d/lib/thor.rb
[thor-licence]: https://github.com/rails/thor/blob/cb11e8e87b3cfe6bc9f6d4b092897f0dfda2476d/LICENSE.md
[three-animation-keyframes]: https://github.com/mrdoob/three.js/blob/f06340e459f19609d3dd82253f38831f58ab6c84/examples/webgl_animation_keyframes.html
[three-css3d-periodictable]: https://github.com/mrdoob/three.js/blob/f06340e459f19609d3dd82253f38831f58ab6c84/examples/css3d_periodictable.html
[three-geometry-colors]: https://github.com/mrdoob/three.js/blob/f06340e459f19609d3dd82253f38831f58ab6c84/examples/webgl_geometry_colors.html
[three-licence]: https://github.com/mrdoob/three.js/blob/f06340e459f19609d3dd82253f38831f58ab6c84/LICENSE
[three-loader-gltf]: https://github.com/mrdoob/three.js/blob/f06340e459f19609d3dd82253f38831f58ab6c84/examples/webgl_loader_gltf.html
[todomvc-backbone]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/backbone/index.html
[todomvc-jquery]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/jquery/index.html
[todomvc-knockoutjs]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/knockoutjs/index.html
[todomvc-licence]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/license.md
[todomvc-lit]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/lit/index.html
[todomvc-polymer]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/polymer/index.html
[todomvc-polymer-licence]: https://github.com/tastejs/todomvc/blob/ff43b02e59dfa604386bb382034b2cd07c2bcd8a/examples/polymer/LICENSE
[tokio-licence]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/LICENSE
[tokio-manifest]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/tokio/Cargo.toml
[tokio-readme]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/README.md
[tomcat-catalina]: https://github.com/apache/tomcat/blob/1b2785c63ca0e0f9a242745b16f26924b5488ddd/conf/catalina.properties
[tomcat-catalina-bat]: https://github.com/apache/tomcat/blob/1b2785c63ca0e0f9a242745b16f26924b5488ddd/bin/catalina.bat
[tomcat-licence]: https://github.com/apache/tomcat/blob/1b2785c63ca0e0f9a242745b16f26924b5488ddd/LICENSE
[tomcat-setclasspath]: https://github.com/apache/tomcat/blob/1b2785c63ca0e0f9a242745b16f26924b5488ddd/bin/setclasspath.bat
[toml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/toml.sample
[toml-test-inline-table-key-dotted-01]: https://github.com/toml-lang/toml-test/blob/96fd088448ec0f4d9f061c730ed7854399c46faf/tests/valid/inline-table/key-dotted-01.toml
[toml-test-key-quoted-unicode]: https://github.com/toml-lang/toml-test/blob/96fd088448ec0f4d9f061c730ed7854399c46faf/tests/valid/key/quoted-unicode.toml
[toml-test-licence]: https://github.com/toml-lang/toml-test/blob/96fd088448ec0f4d9f061c730ed7854399c46faf/LICENSE
[traefik-docker]: https://github.com/traefik/traefik/blob/38d53dd06c53c7e9bd08e57b217e72686d823610/docs/content/getting-started/docker.md
[traefik-licence]: https://github.com/traefik/traefik/blob/38d53dd06c53c7e9bd08e57b217e72686d823610/LICENSE.md
[ts-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/typescript.sample
[tsc-arbitrary-module-namespace-identifiers-syntax]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/es2022/arbitraryModuleNamespaceIdentifiers/arbitraryModuleNamespaceIdentifiers_syntax.ts
[tsc-assertion-type-predicates1]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/controlFlow/assertionTypePredicates1.ts
[tsc-async-with-var-shadowing-es6]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/async/es6/asyncWithVarShadowing_es6.ts
[tsc-await-using-declarations-1]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.1.ts
[tsc-await-using-declarations-11]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.11.ts
[tsc-check-merged-global-umdsymbol]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/checkMergedGlobalUMDSymbol.ts
[tsc-checker-initialization-crash]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/checkerInitializationCrash.ts
[tsc-co-and-contra-variant-inferences3]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/coAndContraVariantInferences3.ts
[tsc-computed-property-names-contextual-type6-es5]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/es6/computedProperties/computedPropertyNamesContextualType6_ES5.ts
[tsc-conflicting-declarations-import-from-namespace2]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/conflictingDeclarationsImportFromNamespace2.ts
[tsc-const-assertions]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/expressions/typeAssertions/constAssertions.ts
[tsc-constructor-with-incomplete-type-annotation]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/constructorWithIncompleteTypeAnnotation.ts
[tsc-contextual-type-caching]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/contextualTypeCaching.ts
[tsc-contravariant-only-inference-from-annotated-function-js]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/contravariantOnlyInferenceFromAnnotatedFunctionJs.ts
[tsc-control-flow-optional-chain]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/controlFlow/controlFlowOptionalChain.ts
[tsc-correlated-unions]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/correlatedUnions.ts
[tsc-declaration-emit-binding-patterns-unused]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/declarationEmitBindingPatternsUnused.ts
[tsc-declaration-emit-higher-order-retained-generics]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/declarationEmitHigherOrderRetainedGenerics.ts
[tsc-declaration-emit-mapped-type-preserves-type-parameter-constraint]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/declarationEmitMappedTypePreservesTypeParameterConstraint.ts
[tsc-dependent-destructured-variables-from-nested-patterns]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/controlFlow/dependentDestructuredVariablesFromNestedPatterns.ts
[tsc-divergent-accessors-types7]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/divergentAccessorsTypes7.ts
[tsc-duplicate-identifier-binding-element-in-parameter-declaration1]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/duplicateIdentifierBindingElementInParameterDeclaration1.ts
[tsc-erasable-syntax-only]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/erasableSyntaxOnly.ts
[tsc-fix-signature-caching]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/fixSignatureCaching.ts
[tsc-intra-expression-inferences]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/types/typeRelationships/typeInference/intraExpressionInferences.ts
[tsc-jsdoc-disallowed-in-typescript]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/jsdoc/jsdocDisallowedInTypescript.ts
[tsc-jsx-complex-signature-has-applicability-error]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/jsxComplexSignatureHasApplicabilityError.tsx
[tsc-jsx-namespace-prefix-in-name]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/jsxNamespacePrefixInName.tsx
[tsc-parserharness]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/parser/ecmascript5/RealWorld/parserharness.ts
[tsc-plain-jsgrammar-errors]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/salsa/plainJSGrammarErrors.ts
[tsc-source-map-validation-statements]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/sourceMapValidationStatements.ts
[tsc-tsx-react-emit-entities]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/jsx/tsxReactEmitEntities.tsx
[tsc-variadic-tuples1]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/conformance/types/tuple/variadicTuples1.ts
[tsc-yield-expression-inner-comment-emit]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases/compiler/yieldExpressionInnerCommentEmit.ts
[tsx-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/tsx.sample
[turbine-flow-test]: https://github.com/cashapp/turbine/blob/8c4e8c9b391d75ac9d7a168cca34b1bf99ed2cb0/src/commonTest/kotlin/app/cash/turbine/FlowTest.kt
[turbine-licence]: https://github.com/cashapp/turbine/blob/8c4e8c9b391d75ac9d7a168cca34b1bf99ed2cb0/LICENSE.txt
[typeorm-indexes]: https://github.com/typeorm/typeorm/blob/5bdc67882dee5a02d975d5ed140c9c3f91661116/docs/docs/indexes.md
[typeorm-licence]: https://github.com/typeorm/typeorm/blob/5bdc67882dee5a02d975d5ed140c9c3f91661116/LICENSE
[typescript-tests-licence]: https://github.com/microsoft/TypeScript/blob/c63de15a992d37f0d6cec03ac7631872838602cb/LICENSE.txt
[unity-assertions]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/docs/UnityAssertionsReference.md
[unity-getting-started]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/docs/UnityGettingStartedGuide.md
[unity-licence]: https://github.com/ThrowTheSwitch/Unity/blob/54f864cf6027a50f4cc5424616e9e01e1bb729da/LICENSE.txt
[vb-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/vb.sample
[views-activity-entrance]: https://github.com/android/views-widgets-samples/blob/2238cc873501f9cda63605051de11832bb736a8a/ConstraintLayoutExamples/motionlayoutintegrations/src/main/res/layout/activity_entrance.xml
[views-licence]: https://github.com/android/views-widgets-samples/blob/2238cc873501f9cda63605051de11832bb736a8a/LICENSE
[views-lots-of-cards]: https://github.com/android/views-widgets-samples/blob/2238cc873501f9cda63605051de11832bb736a8a/ConstraintLayoutExamples/motionlayoutintegrations/src/main/res/layout/lots_of_cards.xml
[viper-licence]: https://github.com/spf13/viper/blob/528f7416c4b56a4948673984b190bf8713f0c3c4/LICENSE
[viper-viper]: https://github.com/spf13/viper/blob/528f7416c4b56a4948673984b190bf8713f0c3c4/viper.go
[vite-api-plugin]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/docs/guide/api-plugin.md
[vite-features]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/docs/guide/features.md
[vite-licence]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/LICENSE
[vite-ssr]: https://github.com/vitejs/vite/blob/b3cb0575aff63004a71301a1ec798aeea2aa05d7/docs/guide/ssr.md
[vitepress-badge]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPBadge.vue
[vitepress-carbon-ads]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPCarbonAds.vue
[vitepress-image]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPImage.vue
[vitepress-layout]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/Layout.vue
[vitepress-licence]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/LICENSE
[vitepress-local-search-box]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPLocalSearchBox.vue
[vitepress-modal-demo]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/docs/components/ModalDemo.vue
[vitepress-nav-bar-hamburger]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPNavBarHamburger.vue
[vitepress-nav-menu-group]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPNavMenuGroup.vue
[vitepress-not-found]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/NotFound.vue
[vitepress-runtime-api]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/docs/en/reference/runtime-api.md
[vitepress-switch]: https://github.com/vuejs/vitepress/blob/00c9d790543790e054b145db4e00e009f25c15f3/src/client/theme-default/components/VPSwitch.vue
[vitest-jest]: https://github.com/vitest-dev/vitest/blob/7a3844954d8753fc0d5bed76a96ffd3a21716c22/docs/guide/migration/jest.md
[vitest-licence]: https://github.com/vitest-dev/vitest/blob/7a3844954d8753fc0d5bed76a96ffd3a21716c22/LICENSE
[vscode-foundry]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/build/npm/patches/foundry-local-sdk.patch
[vscode-launch]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/.vscode/launch.json
[vscode-licence]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/LICENSE.txt
[vscode-ts-grammar]: https://github.com/microsoft/vscode/blob/a4e689ae13ef7a1fe9b515656b51c9be7c274294/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json
[vue-app]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/runtime-core/src/apiCreateApp.ts
[vue-effect]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages/reactivity/src/effect.ts
[vue-licence]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/LICENSE
[vue-package]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/package.json
[vue-playground-app]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages-private/sfc-playground/src/App.vue
[vue-playground-header]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages-private/sfc-playground/src/Header.vue
[vue-playground-sun]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages-private/sfc-playground/src/icons/Sun.vue
[vue-playground-version-select]: https://github.com/vuejs/core/blob/68f25a4a1957710de71731702bd42e05535f494b/packages-private/sfc-playground/src/VersionSelect.vue
[vue-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/vue.sample
[woo-licence]: https://github.com/fukamachi/woo/blob/51b8cb2653116cd05f6a8d0029719ce6ce106711/LICENSE.txt
[woo-response]: https://github.com/fukamachi/woo/blob/51b8cb2653116cd05f6a8d0029719ce6ce106711/src/response.lisp
[xerces-design]: https://github.com/apache/xerces2-j/blob/cf0c517a41b31b0242b96ab1af9627a3ab07fcd2/design/xml/design.xml
[xerces-licence]: https://github.com/apache/xerces2-j/blob/cf0c517a41b31b0242b96ab1af9627a3ab07fcd2/LICENSE
[xerces-schedule]: https://github.com/apache/xerces2-j/blob/cf0c517a41b31b0242b96ab1af9627a3ab07fcd2/design/xml/schedule.xml
[xerces-xni-config]: https://github.com/apache/xerces2-j/blob/cf0c517a41b31b0242b96ab1af9627a3ab07fcd2/docs/xni-config.xml
[xml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/xml.sample
[xv6-bootasm]: https://github.com/mit-pdos/xv6-public/blob/eeb7b415dbcb12cc362d0783e41c3d1f44066b17/bootasm.S
[xv6-licence]: https://github.com/mit-pdos/xv6-public/blob/eeb7b415dbcb12cc362d0783e41c3d1f44066b17/LICENSE
[xv6-swtch]: https://github.com/mit-pdos/xv6-public/blob/eeb7b415dbcb12cc362d0783e41c3d1f44066b17/swtch.S
[yaml-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/yaml.sample
[yaml-test-suite-c4hz]: https://github.com/yaml/yaml-test-suite/blob/6ad3d2c62885d82fc349026c136ef560838fdf3d/C4HZ/in.yaml
[yaml-test-suite-kk5p]: https://github.com/yaml/yaml-test-suite/blob/6ad3d2c62885d82fc349026c136ef560838fdf3d/KK5P/in.yaml
[yaml-test-suite-licence]: https://github.com/yaml/yaml-test-suite/blob/da267a5c4782e7361e82889e76c0dc7df0e1e870/License
[yaml-test-suite-w4tn]: https://github.com/yaml/yaml-test-suite/blob/6ad3d2c62885d82fc349026c136ef560838fdf3d/W4TN/in.yaml
[yykit-licence]: https://github.com/ibireme/YYKit/blob/4e1bd1cfcdb3331244b219cbd37cc9b1ccb62b7a/LICENSE
[yykit-macro]: https://github.com/ibireme/YYKit/blob/4e1bd1cfcdb3331244b219cbd37cc9b1ccb62b7a/YYKit/Base/YYKitMacro.h
[zap-licence]: https://github.com/uber-go/zap/blob/4892335e05f14bce8a98a69a577fcf3844a42623/LICENSE
[zap-logger]: https://github.com/uber-go/zap/blob/4892335e05f14bce8a98a69a577fcf3844a42623/logger.go
[zig-licence]: https://github.com/ziglang/zig/blob/738d2be9d6b6ef3ff3559130c05159ef53336224/LICENSE
[zig-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/zig.sample
[zig-std-allocator]: https://github.com/ziglang/zig/blob/738d2be9d6b6ef3ff3559130c05159ef53336224/lib/std/mem/Allocator.zig
[zig-std-fmt]: https://github.com/ziglang/zig/blob/738d2be9d6b6ef3ff3559130c05159ef53336224/lib/std/fmt.zig
[zlib-inflate]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/inflate.c
[zlib-licence]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/LICENSE
[zlib-make]: https://github.com/madler/zlib/blob/aaa772ce5c6d0023204c6ee45c59cf70c8372c51/Makefile.in
[zls-build]: https://github.com/zigtools/zls/blob/b3e65123f1bf455cea491ba7069bebd5f93e6839/build.zig
[zls-licence]: https://github.com/zigtools/zls/blob/b3e65123f1bf455cea491ba7069bebd5f93e6839/LICENSE
[zls-offsets]: https://github.com/zigtools/zls/blob/b3e65123f1bf455cea491ba7069bebd5f93e6839/src/offsets.zig
[zod-api]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/docs/content/api.mdx
[zod-licence]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/LICENSE
[zod-package]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/package.json
[zod-readme]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/README.md
[zod-schemas]: https://github.com/colinhacks/zod/blob/2554b70c3d4a80273381e61d527d820720626440/packages/zod/src/v4/classic/schemas.ts
