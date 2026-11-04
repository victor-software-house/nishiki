// The oracle's inputs, each pinned to a commit. tests/oracle/SOURCES.md
// records each one's licence; keep the two lists in step.

/** One source file the oracle tokenizes. */
export type Input = {
  /** Shiki's language id. */
  lang: string;
  /** The fixture's file name, without extension. */
  name: string;
  url: string;
};

const abseil =
  "https://raw.githubusercontent.com/abseil/abseil-cpp/1dc74f6f0aadd77ed263a34afa9af20c91fd35f8";
const aeson =
  "https://raw.githubusercontent.com/haskell/aeson/42190167dbe120578a06d67b13b4d96991b34e01";
const airflow =
  "https://raw.githubusercontent.com/apache/airflow/cfbd70b66d11f5d464eb9c0ba926dd34304e110d";
const alamofire =
  "https://raw.githubusercontent.com/Alamofire/Alamofire/6c2892ea5adb5a03fa81c510933dd782c5ddd9cb";
const angularComponents =
  "https://raw.githubusercontent.com/angular/components/6b8d6a47693cf9c862c46919e5de8982fa3fd417";
const ant =
  "https://raw.githubusercontent.com/apache/ant/dbb01a2d528568b1d7ba0c2d84e533577f7f0876";
const antd =
  "https://raw.githubusercontent.com/ant-design/ant-design/7c060247516925f48175d7f29cd43b626632665e";
const argumentParser =
  "https://raw.githubusercontent.com/apple/swift-argument-parser/38ca4eceb349155836701b88da3e0986eb9d1e1c";
const aspnetcore =
  "https://raw.githubusercontent.com/dotnet/aspnetcore/cd236906b73c6540703e51fa583d55de091ae047";
const axios =
  "https://raw.githubusercontent.com/axios/axios/7324e94d8a7b5156a4baf3c86c8b5daa242daee3";
const benchmark =
  "https://raw.githubusercontent.com/google/benchmark/1813ad9c3a828cee126f399739443c329e75bc79";
const black =
  "https://raw.githubusercontent.com/psf/black/945d902bef347d2e7e1d26897aa89a9c177fb535";
const book =
  "https://raw.githubusercontent.com/rust-lang/book/2771df165f092a4fbb9f46b5b2e7450f19010adb";
const bootstrap =
  "https://raw.githubusercontent.com/twbs/bootstrap/2871f2a55ccf18af64045b8fc37529391b285900";
const bootstrap3 =
  "https://raw.githubusercontent.com/twbs/bootstrap/68b0d231a13201eb14acd3dc84e51543d16e5f7e";
const buildx =
  "https://raw.githubusercontent.com/docker/buildx/6997c2578c3fb0ac66e883ceaa535aae90c1ee3d";
const bulma =
  "https://raw.githubusercontent.com/jgthms/bulma/c5cd04baf0258cfcc77b4e8610488a0d439effa2";
const burntsushiToml =
  "https://raw.githubusercontent.com/BurntSushi/toml/d733fc535e4a9f3c454e421a87b90ade5d49bf31";
const cargo =
  "https://raw.githubusercontent.com/rust-lang/cargo/6238f6fd61bffc158aabeb77b779fc16d6e7b5fc";
const catch2 =
  "https://raw.githubusercontent.com/catchorg/Catch2/a1a991fee5f217b40035cb2f8ebc83864d66408d";
const checkout =
  "https://raw.githubusercontent.com/actions/checkout/f548e57e544e1ff5a4c46bf1e1b8685f8e4a348a";
const circe =
  "https://raw.githubusercontent.com/circe/circe/b38c17d9a9dd9b761a68319cff038d75bbf0cf8d";
const cjson =
  "https://raw.githubusercontent.com/DaveGamble/cJSON/6d9f2443ab071f86e5d9b43025a40929ec41c46c";
const clap =
  "https://raw.githubusercontent.com/clap-rs/clap/db30b39a55c188763666268447050a0dacba3bf0";
const clickMan =
  "https://raw.githubusercontent.com/click-contrib/click-man/be9eac7642e32a43a93ec2c1f5deea7047bb7288";
const cobra =
  "https://raw.githubusercontent.com/spf13/cobra/adbc8813901bba65827259daa8e22ff94ec1f30e";
const codegen =
  "https://raw.githubusercontent.com/dotansimha/graphql-code-generator/11ac951a30b79369d3b0aa78f83a79d309ffcc77";
const commonsLang =
  "https://raw.githubusercontent.com/apache/commons-lang/d8470d57fda340864acf5194bb56305a133fba0e";
const composable =
  "https://raw.githubusercontent.com/pointfreeco/swift-composable-architecture/bb175b544d019c7f23a2e41ae29d8c70592aeb58";
const coroutines =
  "https://raw.githubusercontent.com/Kotlin/kotlinx.coroutines/7e0d1b3e9ca83652427ae00dda7d1673f058a743";
const dartCore =
  "https://raw.githubusercontent.com/dart-lang/core/36f57a994965785d641e436b7dd3aa8a18a26f62";
const dartHttp =
  "https://raw.githubusercontent.com/dart-lang/http/8c0d66acd2385761182d34b48c4c80ca6cfa4513";
const deviceVirtual =
  "https://raw.githubusercontent.com/edgexfoundry/device-virtual-go/51a4860f7a496206684d7bc404f1c64d674ea9c4";
const devise =
  "https://raw.githubusercontent.com/heartcombo/devise/05811fb8366edd08874111d0f6fb44543e1c91ef";
const django =
  "https://raw.githubusercontent.com/django/django/b501b8493f31cf8cba31211d59b3b78642b8cb02";
const dockerPostgres =
  "https://raw.githubusercontent.com/docker-library/postgres/dfc76bb299b057785026eb2eea39556b6fe52983";
const dockerPython =
  "https://raw.githubusercontent.com/docker-library/python/f4c67ece208dead594e43628d22e3c6027dd97ab";
const dotnetDocs =
  "https://raw.githubusercontent.com/dotnet/docs/d1a7e773f847015b60202a8332d949ac9dbbb279/docs/csharp/language-reference";
const dotnetInstall =
  "https://raw.githubusercontent.com/dotnet/install-scripts/e5cf1dd2d1540ed05ac84f8eb8c5cdec2807621e";
const dotnetRuntime =
  "https://raw.githubusercontent.com/dotnet/runtime/6375d63af99b6812f11b4dd8c4a18d227e277d73";
const eslint =
  "https://raw.githubusercontent.com/eslint/eslint/11c66ba08381cb7cff7679d662c9f6654b6fe712";
const exposed =
  "https://raw.githubusercontent.com/JetBrains/Exposed/2bdb13cd7b4b511b07de3dde109878b6f8b06d48";
const express =
  "https://raw.githubusercontent.com/expressjs/express/bcc0551cb7a785b7772f77cb59ce55a2bc94975a";
const fastapi =
  "https://raw.githubusercontent.com/tiangolo/fastapi/5cb6272ed7e3e07eaf6978e1e65b94aca76fa8e0";
const flask =
  "https://raw.githubusercontent.com/pallets/flask/49943b5a94413a530761350f04e896f76e8d620e";
const fmt =
  "https://raw.githubusercontent.com/fmtlib/fmt/4ed3535092a96ac9f56fab80df83416d51bcc78e";
const foundation =
  "https://raw.githubusercontent.com/apple/swift-corelibs-foundation/cbfe1c7376ffe869d08b1b5f643d747ef0a16069";
const gin =
  "https://raw.githubusercontent.com/gin-gonic/gin/0f09c3a9b4626d9fe9979cf4d0521d6ae32ab646";
const githubMarkdownCss =
  "https://raw.githubusercontent.com/sindresorhus/github-markdown-css/7f38dcb9054bc5d860fac65291f5c98fef47932c";
const golang =
  "https://raw.githubusercontent.com/golang/go/736a1652a0d65f0eb8d95db7e8125a2d381a2cc6";
const googleapis =
  "https://raw.githubusercontent.com/googleapis/googleapis/0d0982bd9844aacc37a4e87c1cd43bbdb601884d";
const googletest =
  "https://raw.githubusercontent.com/google/googletest/a8380b01a25ebdb130097b607b3afed2a58d04ba";
const gradle =
  "https://raw.githubusercontent.com/gradle/gradle/f9f8f85075be950f7d4e2edc4d57689084481e87";
const graphiql =
  "https://raw.githubusercontent.com/graphql/graphiql/e7a5e07b6c2cb9b524831544c689e742cdc1b904";
const graphqlJs =
  "https://raw.githubusercontent.com/graphql/graphql-js/eb0bd0801ea5f9f7680e91ae575ed74397f051cb";
const grpc =
  "https://raw.githubusercontent.com/grpc/grpc/9a4f19513f24b48c0fbfc9c56765e553f9c6e56c";
const gson =
  "https://raw.githubusercontent.com/google/gson/57731049da3e415be3d5d34db1ca3b80f3c73810";
const guava =
  "https://raw.githubusercontent.com/google/guava/e98ea2340f6acc94ed9e2cca08976ccae68c3412";
const h5bp =
  "https://raw.githubusercontent.com/h5bp/html5-boilerplate/b6597338e695dc4a8165c5abcb2bfffa586c3ee5";
const helm =
  "https://raw.githubusercontent.com/helm/helm/bc232c5312e784e808ea95ae76f9e82d4fe142fc";
const highlightjs =
  "https://raw.githubusercontent.com/highlightjs/highlight.js/60e092057673595fa542db0bc2b13a99df5d0551";
const homeManager =
  "https://raw.githubusercontent.com/nix-community/home-manager/095fef2f265c3c0da4fe315efba8f8a04495ab34";
const httparser =
  "https://raw.githubusercontent.com/antlabs/httparser/622df4409cddf119cf437633f620947235d2d1eb";
const inspectLua =
  "https://raw.githubusercontent.com/kikito/inspect.lua/a8ca3120dfec48801036eaeff9335ab7a096dd24";
const javapoet =
  "https://raw.githubusercontent.com/square/javapoet/b9017a9503b76e11b4ad4c1a9f050e2d29112cb0";
const jekyll =
  "https://raw.githubusercontent.com/jekyll/jekyll/97c3bdc822df9d5bea4c269b29a28de5a5e29d70";
const jq =
  "https://raw.githubusercontent.com/jqlang/jq/5c001b18d7237cce1fdba63608732163f978dd23";
const jquery =
  "https://raw.githubusercontent.com/jquery/jquery/feaa9fcdb68cdeadf013d97d8fd40848d4618578";
const jsonLua =
  "https://raw.githubusercontent.com/rxi/json.lua/dbf4b2dd2eb7c23be2773c89eb059dadd6436f94";
const julia =
  "https://raw.githubusercontent.com/JuliaLang/julia/aca9097c30df5ff53ee088a90cce5ffcea478f70";
const kafka =
  "https://raw.githubusercontent.com/apache/kafka/da5316228ee604724dbcede405ebbf7f18f1dbf3";
const kingfisher =
  "https://raw.githubusercontent.com/onevcat/Kingfisher/770465258c652c73fdef447c10004765c440e993";
const knex =
  "https://raw.githubusercontent.com/knex/knex/e25d54bcb707714a17f5a5744eba5c4246bb4d1d";
const kotlinxSerialization =
  "https://raw.githubusercontent.com/Kotlin/kotlinx.serialization/6f6a2e66c3dbf4a34e86a030368ecb0f7a734d22";
const ktor =
  "https://raw.githubusercontent.com/ktorio/ktor/b7809b1eb47ec68cbb9c78a52a23880caca710cd";
const kubernetes =
  "https://raw.githubusercontent.com/kubernetes/kubernetes/98687f28e4fed347d30fcd483edc8b8ee74c583c";
const ky =
  "https://raw.githubusercontent.com/sindresorhus/ky/3541888878c4d303a8bc2ad61815f4e048f55a5d";
const latexrun =
  "https://raw.githubusercontent.com/aclements/latexrun/38ff6ec2815654513c91f64bdf2a5760c85da26e";
const lessjs =
  "https://raw.githubusercontent.com/less/less.js/713331655e437401dfea6cd233893a69762734e1";
const leveldb =
  "https://raw.githubusercontent.com/google/leveldb/bb74ef739973a70ca9f0d90788a1da89b788c0ad";
const libuv =
  "https://raw.githubusercontent.com/libuv/libuv/7228e05f3e407ed3fa59edba920b8cd825b5d83d";
const linenoise =
  "https://raw.githubusercontent.com/antirez/linenoise/a473823d74b93eab2ba83480df16ed37617493f2";
const lua =
  "https://raw.githubusercontent.com/lua/lua/8cb93bada561977eb370819e39d7d644e5fd0a3f";
const luaLanguageServer =
  "https://raw.githubusercontent.com/LuaLS/lua-language-server/a58fc7b01e1746c46275c71b0e61bfed40a6296d";
const luarocks =
  "https://raw.githubusercontent.com/luarocks/luarocks/85bfe36c812465b2f377877939d884a4db829a25";
const lume =
  "https://raw.githubusercontent.com/rxi/lume/98847e7812cf28d3d64b289b03fad71dc704547d";
const markdownlint =
  "https://raw.githubusercontent.com/DavidAnson/markdownlint/b67636d47f335cf124990faae4a72c5e9fadd613";
const mathjaxDemos =
  "https://raw.githubusercontent.com/mathjax/MathJax-demos-web/82c49422ee92dc58fbeed278532495d52c00aed0";
const mermaid =
  "https://raw.githubusercontent.com/mermaid-js/mermaid/ea53556f130b397fb6f781818ee0f1e924b429eb";
const milligram =
  "https://raw.githubusercontent.com/milligram/milligram/5e56af462aea08baf5011a045bf1bfaaef72552c";
const minikube =
  "https://raw.githubusercontent.com/kubernetes/minikube/1f8cacf5b0bdb6ecdcf1abf16da4f3d596ba7996";
const moby =
  "https://raw.githubusercontent.com/moby/moby/aed2b781abc209a639b79d00f6962b629242aa1a";
const moshi =
  "https://raw.githubusercontent.com/square/moshi/889013ec2edb8d8034902662a1dc8c4f3b3f8111";
const newtonsoft =
  "https://raw.githubusercontent.com/JamesNK/Newtonsoft.Json/52fa3aef1f2cadcd3a3f874251eddc98d3efbbaa";
const nextjs =
  "https://raw.githubusercontent.com/vercel/next.js/5a5a4cdc5263ee3ca5a14378de78d2b97dbc4378";
const nio =
  "https://raw.githubusercontent.com/apple/swift-nio/2f58d8ef84438d6517fc78bcea2f8527e3c72936";
const nixpkgs =
  "https://raw.githubusercontent.com/NixOS/nixpkgs/084f878659a86f432881f0267c50863df444e925";
const nlohmannJson =
  "https://raw.githubusercontent.com/nlohmann/json/e32051125faf95dc64d49a129959b6742b9ffee9";
const nodejs =
  "https://raw.githubusercontent.com/nodejs/node/3556d018a4863a42fba62a927e7e605c1e8cf128";
const normalize =
  "https://raw.githubusercontent.com/necolas/normalize.css/fc091cce1534909334c1911709a39c22d406977b";
const nvm =
  "https://raw.githubusercontent.com/nvm-sh/nvm/5e38bc0caed342356ffd26eb21e0e9b995ac228b";
const nvmMake =
  "https://raw.githubusercontent.com/nvm-sh/nvm/16df5aa69aeda3814703e14f9d35a4f60220dd37";
const okhttp =
  "https://raw.githubusercontent.com/square/okhttp/16fb6aef3aee1c4645fc8d24af01f8a4dc128ebf";
const penlight =
  "https://raw.githubusercontent.com/lunarmodules/Penlight/7f3351f226c2f643aaa1d3afa8d7293dd8a1d985";
const petstore =
  "https://raw.githubusercontent.com/swagger-api/swagger-petstore/5cb909fc7780bc708151bce0014939acba8b212a";
const pgvector =
  "https://raw.githubusercontent.com/pgvector/pgvector/1d5ef0a160cadec08f754528669ce17e20eab38c";
const poetry =
  "https://raw.githubusercontent.com/python-poetry/poetry/6ecadf5b71309b1da4f00949cc349a79884cff8f";
const postgres =
  "https://raw.githubusercontent.com/postgres/postgres/5681f0cfa5acfdc5146e2b74ff5b7e940df039f3";
const powershell =
  "https://raw.githubusercontent.com/PowerShell/PowerShell/2725d6aaa6e67219b6c9902adf7b74c52e0371a3";
const preact =
  "https://raw.githubusercontent.com/preactjs/preact/81dbe69d39e948b2305193d2622c225699311083";
const prettier =
  "https://raw.githubusercontent.com/prettier/prettier/a02e92be26ec8963a07aa9500a6c20e24599e95b";
const prometheus =
  "https://raw.githubusercontent.com/prometheus/prometheus/b0c3a30ab2a09b88e64d876017a0134aa2f38f40";
const prometheusGo =
  "https://raw.githubusercontent.com/prometheus/prometheus/b0c3a30ab2a09b88e64d876017a0134aa2f38f40";
const puma =
  "https://raw.githubusercontent.com/puma/puma/c0df0c19dde943c3f5c25cd67caa984dfa84e3da";
const pyenv =
  "https://raw.githubusercontent.com/pyenv/pyenv/e6669f0a84df60daba8672c3b3f0caebd0fdcef8";
const pytest =
  "https://raw.githubusercontent.com/pytest-dev/pytest/1211245aa72f1efe07bcfce63a7c543156cee976";
const rack =
  "https://raw.githubusercontent.com/rack/rack/da90464e3fa93e3cb49dc2d4ede798e6c6b70b4f";
const rag =
  "https://raw.githubusercontent.com/jxnl/n-levels-of-rag/2ce1106b0163d11fcf7959187b42f0d3a67fbe05";
const rails =
  "https://raw.githubusercontent.com/rails/rails/f21e7f42b510327f6bc57dcb7fcc9825d4670a91";
const rake =
  "https://raw.githubusercontent.com/ruby/rake/3f29d39daa688eb51cd975c06b1501320f10b911";
const rapidjson =
  "https://raw.githubusercontent.com/Tencent/rapidjson/24b5e7a8b27f42fa16b96fc70aade9106cf7102f";
const requests =
  "https://raw.githubusercontent.com/psf/requests/3c7baaa05ab4aa49cc819ce3a819d260de19c157";
const rewrite =
  "https://raw.githubusercontent.com/openrewrite/rewrite/03d32965a3cdbf06f2717eb3ca706a162039703c";
const rich =
  "https://raw.githubusercontent.com/Textualize/rich/9d8f9a372cc5916fd4781fec207ced7ddac2f08f";
const ripgrep =
  "https://raw.githubusercontent.com/BurntSushi/ripgrep/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b";
const rome =
  "https://raw.githubusercontent.com/rometools/rome/6b4a7e89cbf67dab0be32e7517f3d8ae3c2b4d75";
const roslyn =
  "https://raw.githubusercontent.com/dotnet/roslyn/868edfafce0460b50e1abe82f1bb0872979dbf0a";
const rspecCore =
  "https://raw.githubusercontent.com/rspec/rspec-core/aec5f49485d97908183dbe790a7fefb8baaa8091";
const rubocop =
  "https://raw.githubusercontent.com/rubocop/rubocop/09c38e2526622e5c08a73ebff58fd2f4bed35ffe";
const rubyBuild =
  "https://raw.githubusercontent.com/rbenv/ruby-build/4ef3159f94dd66c8325efb42db2faeb7d5732db0";
const ruff =
  "https://raw.githubusercontent.com/astral-sh/ruff/f10755306e3e932fd127960acb2c892fafb87410";
const rustc =
  "https://raw.githubusercontent.com/rust-lang/rust/ae36fe22c7e590df40ffd53ac25fea35597c3f3d";
const samples =
  "https://raw.githubusercontent.com/shikijs/textmate-grammars-themes/8f1828ebccc57930cf0adde9b35ae929584fe191/samples";
const scala3 =
  "https://raw.githubusercontent.com/scala/scala3/d71919d08172571e6a5c9392f186b2a6a4413097";
const serde =
  "https://raw.githubusercontent.com/serde-rs/serde/0db9b3b504bcb0a7fa9cf001429869519afd3e9a";
const shadcn =
  "https://raw.githubusercontent.com/shadcn-ui/ui/4aaea19d9cff3e0908e12a8c1d504b8c5ec34610";
const snapshotTesting =
  "https://raw.githubusercontent.com/pointfreeco/swift-snapshot-testing/0c6380dd46cd45e00f2bf87cf6002f3478e84452";
const spdlog =
  "https://raw.githubusercontent.com/gabime/spdlog/6d9805df3f588402b17073dcd13eb569ceda864d";
const sqlalchemy =
  "https://raw.githubusercontent.com/sqlalchemy/sqlalchemy/679372836fe7343683248467d276036a6d6f1673";
const sqlServerSamples =
  "https://raw.githubusercontent.com/microsoft/sql-server-samples/3c71696a3942879f0481f3bd65fe112a40023f18";
const stringProcessing =
  "https://raw.githubusercontent.com/swiftlang/swift-experimental-string-processing/93065a071c4bcfc5b23e9ee3dec49f9bc9b257ee";
const swiftAlgorithms =
  "https://raw.githubusercontent.com/apple/swift-algorithms/5b7143f8e291dee0e14c118fd0212487f0b37af5";
const swiftTesting =
  "https://raw.githubusercontent.com/swiftlang/swift-testing/87ec972e0b5725ee02b31a6e9ed690d85abb460a";
const swiftTests =
  "https://raw.githubusercontent.com/swiftlang/swift/f21f2c19ef734859c366829e0d23927813040968";
const tailwind =
  "https://raw.githubusercontent.com/tailwindlabs/tailwindcss/2626ae6ceddf36069ab6d55c53a5857c1e2e360a";
const testify =
  "https://raw.githubusercontent.com/stretchr/testify/87a7b9d57689f6579db2da795ab8deeab29cb724";
const thor =
  "https://raw.githubusercontent.com/rails/thor/cb11e8e87b3cfe6bc9f6d4b092897f0dfda2476d";
const threeJs =
  "https://raw.githubusercontent.com/mrdoob/three.js/f06340e459f19609d3dd82253f38831f58ab6c84";
const todomvc =
  "https://raw.githubusercontent.com/tastejs/todomvc/ff43b02e59dfa604386bb382034b2cd07c2bcd8a";
const tokio =
  "https://raw.githubusercontent.com/tokio-rs/tokio/b499257ef62bd631b83a988d371f4330f545670c";
const tomcat =
  "https://raw.githubusercontent.com/apache/tomcat/1b2785c63ca0e0f9a242745b16f26924b5488ddd";
const tomlTest =
  "https://raw.githubusercontent.com/toml-lang/toml-test/96fd088448ec0f4d9f061c730ed7854399c46faf";
const traefik =
  "https://raw.githubusercontent.com/traefik/traefik/38d53dd06c53c7e9bd08e57b217e72686d823610";
const turbine =
  "https://raw.githubusercontent.com/cashapp/turbine/8c4e8c9b391d75ac9d7a168cca34b1bf99ed2cb0";
const typeorm =
  "https://raw.githubusercontent.com/typeorm/typeorm/5bdc67882dee5a02d975d5ed140c9c3f91661116";
const typescriptTests =
  "https://raw.githubusercontent.com/microsoft/TypeScript/c63de15a992d37f0d6cec03ac7631872838602cb/tests/cases";
const unity =
  "https://raw.githubusercontent.com/ThrowTheSwitch/Unity/54f864cf6027a50f4cc5424616e9e01e1bb729da";
const viewsWidgets =
  "https://raw.githubusercontent.com/android/views-widgets-samples/2238cc873501f9cda63605051de11832bb736a8a";
const viper =
  "https://raw.githubusercontent.com/spf13/viper/528f7416c4b56a4948673984b190bf8713f0c3c4";
const vite =
  "https://raw.githubusercontent.com/vitejs/vite/b3cb0575aff63004a71301a1ec798aeea2aa05d7";
const vitest =
  "https://raw.githubusercontent.com/vitest-dev/vitest/7a3844954d8753fc0d5bed76a96ffd3a21716c22";
const vscode =
  "https://raw.githubusercontent.com/microsoft/vscode/a4e689ae13ef7a1fe9b515656b51c9be7c274294";
const vue =
  "https://raw.githubusercontent.com/vuejs/core/68f25a4a1957710de71731702bd42e05535f494b";
const xerces =
  "https://raw.githubusercontent.com/apache/xerces2-j/cf0c517a41b31b0242b96ab1af9627a3ab07fcd2";
const yamlTestSuite =
  "https://raw.githubusercontent.com/yaml/yaml-test-suite/6ad3d2c62885d82fc349026c136ef560838fdf3d";
const zap =
  "https://raw.githubusercontent.com/uber-go/zap/4892335e05f14bce8a98a69a577fcf3844a42623";
const zig =
  "https://raw.githubusercontent.com/ziglang/zig/738d2be9d6b6ef3ff3559130c05159ef53336224";
const zlib =
  "https://raw.githubusercontent.com/madler/zlib/aaa772ce5c6d0023204c6ee45c59cf70c8372c51";
const zls =
  "https://raw.githubusercontent.com/zigtools/zls/b3e65123f1bf455cea491ba7069bebd5f93e6839";
const zod =
  "https://raw.githubusercontent.com/colinhacks/zod/2554b70c3d4a80273381e61d527d820720626440";

export const inputs: Input[] = [
  // Batch runs on the TextMate engine. Shiki's Markdown grammar has no Batch
  // fence, so no Markdown input embeds it.
  { lang: "bat", name: "gradlew", url: `${gradle}/gradlew.bat` },
  { lang: "bat", name: "sample", url: `${samples}/bat.sample` },
  { lang: "bat", name: "tomcat-catalina", url: `${tomcat}/bin/catalina.bat` },
  { lang: "bat", name: "tomcat-setclasspath", url: `${tomcat}/bin/setclasspath.bat` },
  { lang: "c", name: "cjson", url: `${cjson}/cJSON.c` },
  {
    lang: "c",
    name: "coverage",
    url: new URL("inputs/c-coverage.c", import.meta.url).href,
  },
  { lang: "c", name: "jq-print", url: `${jq}/src/jv_print.c` },
  { lang: "c", name: "libuv-core", url: `${libuv}/src/unix/core.c` },
  { lang: "c", name: "libuv-freertos-startup", url: `${libuv}/test/freertos/mps2/startup.c` },
  { lang: "c", name: "linenoise", url: `${linenoise}/linenoise.c` },
  { lang: "c", name: "lua-vm", url: `${lua}/lvm.c` },
  { lang: "c", name: "sample", url: `${samples}/c.sample` },
  { lang: "c", name: "zlib-inflate", url: `${zlib}/inflate.c` },
  // CMake runs on the TextMate engine. Shiki's Markdown grammar has no CMake
  // fence, so no Markdown input embeds it.
  { lang: "cmake", name: "catch2-lists", url: `${catch2}/CMakeLists.txt` },
  { lang: "cmake", name: "googletest-utils", url: `${googletest}/googletest/cmake/internal_utils.cmake` },
  { lang: "cmake", name: "json-lists", url: `${nlohmannJson}/CMakeLists.txt` },
  { lang: "cmake", name: "sample", url: `${samples}/cmake.sample` },
  { lang: "cpp", name: "absl-any-invocable", url: `${abseil}/absl/functional/internal/any_invocable.h` },
  { lang: "cpp", name: "absl-any-invocable-test", url: `${abseil}/absl/functional/any_invocable_test.h` },
  { lang: "cpp", name: "absl-check-op", url: `${abseil}/absl/log/internal/check_op.h` },
  { lang: "cpp", name: "absl-cpu-detect", url: `${abseil}/absl/base/internal/cpu_detect.cc` },
  { lang: "cpp", name: "absl-crc-x86-arm-combined", url: `${abseil}/absl/crc/internal/crc_x86_arm_combined.cc` },
  { lang: "cpp", name: "absl-crc32c", url: `${abseil}/absl/crc/crc32c.h` },
  { lang: "cpp", name: "absl-dynamic-annotations", url: `${abseil}/absl/base/internal/dynamic_annotations.h` },
  { lang: "cpp", name: "absl-exception-safety-testing", url: `${abseil}/absl/base/internal/exception_safety_testing.h` },
  { lang: "cpp", name: "absl-flag", url: `${abseil}/absl/flags/flag.h` },
  { lang: "cpp", name: "absl-flag-internal", url: `${abseil}/absl/flags/internal/flag.h` },
  { lang: "cpp", name: "absl-flag-test", url: `${abseil}/absl/flags/flag_test.cc` },
  { lang: "cpp", name: "absl-hash-benchmark", url: `${abseil}/absl/hash/hash_benchmark.cc` },
  { lang: "cpp", name: "absl-mutex", url: `${abseil}/absl/synchronization/mutex.h` },
  { lang: "cpp", name: "absl-profile-builder", url: `${abseil}/absl/profiling/internal/profile_builder.cc` },
  { lang: "cpp", name: "absl-status-matchers", url: `${abseil}/absl/status/internal/status_matchers.h` },
  { lang: "cpp", name: "absl-str-split", url: `${abseil}/absl/strings/str_split.h` },
  { lang: "cpp", name: "absl-symbolize-test", url: `${abseil}/absl/debugging/symbolize_test.cc` },
  { lang: "cpp", name: "benchmark-core", url: `${benchmark}/src/benchmark.cc` },
  { lang: "cpp", name: "benchmark-sysinfo", url: `${benchmark}/src/sysinfo.cc` },
  {
    lang: "cpp",
    name: "coverage",
    url: new URL("inputs/cpp-coverage.cpp", import.meta.url).href,
  },
  { lang: "cpp", name: "fmt-c", url: `${fmt}/include/fmt/fmt-c.h` },
  { lang: "cpp", name: "fmt-core-test", url: `${fmt}/test/core-test.cc` },
  { lang: "cpp", name: "fmt-gtest-extra", url: `${fmt}/test/gtest-extra.h` },
  { lang: "cpp", name: "fmt-ranges", url: `${fmt}/include/fmt/ranges.h` },
  { lang: "cpp", name: "fmt-xchar-test", url: `${fmt}/test/xchar-test.cc` },
  { lang: "cpp", name: "leveldb-version-set", url: `${leveldb}/db/version_set.cc` },
  { lang: "cpp", name: "rapidjson-reader", url: `${rapidjson}/include/rapidjson/reader.h` },
  { lang: "cpp", name: "sample", url: `${samples}/cpp.sample` },
  { lang: "cpp", name: "spdlog-logger", url: `${spdlog}/include/spdlog/logger.h` },
  {
    lang: "csharp",
    name: "coverage",
    url: new URL("inputs/csharp-coverage.cs", import.meta.url).href,
  },
  { lang: "csharp", name: "docs-async-wpf", url: `${dotnetDocs}/keywords/snippets/async/wpf/MainWindow.xaml.cs` },
  { lang: "csharp", name: "docs-bitwise-shift", url: `${dotnetDocs}/operators/snippets/shared/BitwiseAndShiftOperators.cs` },
  { lang: "csharp", name: "docs-closed-hierarchy-patterns", url: `${dotnetDocs}/operators/snippets/patterns/ClosedHierarchyPatterns.cs` },
  { lang: "csharp", name: "docs-delegate-operator", url: `${dotnetDocs}/operators/snippets/shared/DelegateOperator.cs` },
  { lang: "csharp", name: "docs-doc-comments", url: `${dotnetDocs}/xmldoc/snippets/xmldoc/DocComments.cs` },
  { lang: "csharp", name: "docs-events", url: `${dotnetDocs}/keywords/snippets/events.cs` },
  { lang: "csharp", name: "docs-exception-example", url: `${dotnetDocs}/statements/snippets/exception-handling-statements/ExampleClass.cs` },
  { lang: "csharp", name: "docs-fixed", url: `${dotnetDocs}/snippets/unsafe-code/FixedKeywordExamples.cs` },
  { lang: "csharp", name: "docs-generic-where", url: `${dotnetDocs}/keywords/snippets/GenericWhereConstraints.cs` },
  { lang: "csharp", name: "docs-goto", url: `${dotnetDocs}/statements/snippets/jump-statements/GotoStatement.cs` },
  { lang: "csharp", name: "docs-group", url: `${dotnetDocs}/keywords/snippets/group.cs` },
  { lang: "csharp", name: "docs-keywords-contextual", url: `${dotnetDocs}/keywords/snippets/csrefKeywordsContextual.cs` },
  { lang: "csharp", name: "docs-keywords-modifiers", url: `${dotnetDocs}/keywords/snippets/csrefKeywordsModifiers.cs` },
  { lang: "csharp", name: "docs-lock", url: `${dotnetDocs}/statements/snippets/lock/Program.cs` },
  { lang: "csharp", name: "docs-raw-string-literal", url: `${dotnetDocs}/tokens/snippets/raw-string-literal.cs` },
  { lang: "csharp", name: "docs-record-type", url: `${dotnetDocs}/builtin-types/snippets/shared/RecordType.cs` },
  { lang: "csharp", name: "docs-string-interpolation", url: `${dotnetDocs}/tokens/snippets/string-interpolation.cs` },
  { lang: "csharp", name: "docs-union-body-members", url: `${dotnetDocs}/builtin-types/snippets/unions/BodyMembers.cs` },
  { lang: "csharp", name: "docs-using-alias", url: `${dotnetDocs}/keywords/snippets/UsingAliasQualifier.cs` },
  { lang: "csharp", name: "docs-value-tuples", url: `${dotnetDocs}/builtin-types/snippets/shared/ValueTuples.cs` },
  { lang: "csharp", name: "docs-var-pattern", url: `${dotnetDocs}/operators/snippets/patterns/VarPattern.cs` },
  { lang: "csharp", name: "newtonsoft-json-convert", url: `${newtonsoft}/Src/Newtonsoft.Json/JsonConvert.cs` },
  { lang: "csharp", name: "roslyn-text-span", url: `${roslyn}/src/Compilers/Core/Portable/Text/TextSpan.cs` },
  { lang: "csharp", name: "runtime-linq-where", url: `${dotnetRuntime}/src/libraries/System.Linq/src/System/Linq/Where.cs` },
  { lang: "csharp", name: "runtime-stack", url: `${dotnetRuntime}/src/libraries/System.Collections/src/System/Collections/Generic/Stack.cs` },
  { lang: "csharp", name: "sample", url: `${samples}/csharp.sample` },
  {
    lang: "css",
    name: "charset-no-space",
    url: new URL("inputs/css-charset-no-space.css", import.meta.url).href,
  },
  {
    lang: "css",
    name: "charset-unclosed",
    url: new URL("inputs/css-charset-unclosed.css", import.meta.url).href,
  },
  {
    lang: "css",
    name: "charset-unquoted",
    url: new URL("inputs/css-charset-unquoted.css", import.meta.url).href,
  },
  {
    lang: "css",
    name: "coverage",
    url: new URL("inputs/css-coverage.css", import.meta.url).href,
  },
  { lang: "css", name: "github-markdown", url: `${githubMarkdownCss}/github-markdown.css` },
  { lang: "css", name: "highlightjs-atom-one-dark", url: `${highlightjs}/src/styles/atom-one-dark.css` },
  { lang: "css", name: "highlightjs-github", url: `${highlightjs}/src/styles/github.css` },
  { lang: "css", name: "lessjs-at-rules-compressed-evaluation", url: `${lessjs}/packages/test-data/tests-config/at-rules-compressed-evaluation/at-rules-compressed-evaluation.css` },
  { lang: "css", name: "lessjs-css-3", url: `${lessjs}/packages/test-data/tests-unit/css-3/css-3.css` },
  { lang: "css", name: "lessjs-math-strict", url: `${lessjs}/packages/test-data/tests-config/math/strict/css.css` },
  { lang: "css", name: "lessjs-media", url: `${lessjs}/packages/test-data/tests-unit/media/media.css` },
  { lang: "css", name: "lessjs-selectors", url: `${lessjs}/packages/test-data/tests-unit/selectors/selectors.css` },
  { lang: "css", name: "milligram", url: `${milligram}/dist/milligram.css` },
  { lang: "css", name: "normalize", url: `${normalize}/normalize.css` },
  { lang: "css", name: "sample", url: `${samples}/css.sample` },
  { lang: "css", name: "tailwind-preflight", url: `${tailwind}/packages/tailwindcss/preflight.css` },
  // Dart runs on the TextMate engine.
  { lang: "dart", name: "collection-algorithms", url: `${dartCore}/pkgs/collection/lib/src/algorithms.dart` },
  { lang: "dart", name: "http-base-request", url: `${dartHttp}/pkgs/http/lib/src/base_request.dart` },
  { lang: "dart", name: "http-client", url: `${dartHttp}/pkgs/http/lib/src/client.dart` },
  { lang: "dart", name: "sample", url: `${samples}/dart.sample` },
  {
    lang: "diff",
    name: "cranelift-stdarch-sve",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0001-stdarch-Disable-SVE.patch`,
  },
  {
    lang: "diff",
    name: "cranelift-stdlib-atomics",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0027-stdlib-128bit-atomic-operations.patch`,
  },
  {
    lang: "diff",
    name: "cranelift-sysroot-tests",
    url: `${rustc}/compiler/rustc_codegen_cranelift/patches/0028-sysroot_tests-Disable-long-running-tests.patch`,
  },
  {
    lang: "diff",
    name: "mir-opt-gvn-cast",
    url: `${rustc}/tests/mir-opt/const_prop/cast.main.GVN.diff`,
  },
  {
    lang: "diff",
    name: "mir-opt-gvn-slices",
    url: `${rustc}/tests/mir-opt/gvn.slices.GVN.panic-abort.diff`,
  },
  { lang: "diff", name: "sample", url: `${samples}/diff.sample` },
  {
    lang: "diff",
    name: "vscode-foundry-local-sdk",
    url: `${vscode}/build/npm/patches/foundry-local-sdk.patch`,
  },
  { lang: "docker", name: "buildx", url: `${buildx}/Dockerfile` },
  { lang: "docker", name: "moby", url: `${moby}/Dockerfile` },
  { lang: "docker", name: "next-with-docker", url: `${nextjs}/examples/with-docker/Dockerfile` },
  { lang: "docker", name: "postgres-bookworm", url: `${dockerPostgres}/18/bookworm/Dockerfile` },
  { lang: "docker", name: "prometheus", url: `${prometheus}/Dockerfile` },
  { lang: "docker", name: "python-slim-bookworm", url: `${dockerPython}/3.13/slim-bookworm/Dockerfile` },
  { lang: "docker", name: "sample", url: `${samples}/docker.sample` },
  { lang: "erb", name: "devise-confirmation-instructions", url: `${devise}/app/views/devise/mailer/confirmation_instructions.html.erb` },
  { lang: "erb", name: "devise-error-messages", url: `${devise}/app/views/devise/shared/_error_messages.html.erb` },
  { lang: "erb", name: "devise-registrations-edit", url: `${devise}/app/views/devise/registrations/edit.html.erb` },
  { lang: "erb", name: "devise-sessions-new", url: `${devise}/app/views/devise/sessions/new.html.erb` },
  { lang: "erb", name: "devise-shared-links", url: `${devise}/app/views/devise/shared/_links.html.erb` },
  { lang: "erb", name: "rails-app-layout", url: `${rails}/railties/lib/rails/generators/rails/app/templates/app/views/layouts/application.html.erb.tt` },
  { lang: "erb", name: "rails-info-notes", url: `${rails}/railties/lib/rails/templates/rails/info/notes.html.erb` },
  { lang: "erb", name: "rails-mailer-email", url: `${rails}/railties/lib/rails/templates/rails/mailers/email.html.erb` },
  { lang: "erb", name: "rails-pwa-offline", url: `${rails}/railties/lib/rails/generators/rails/app/templates/app/views/pwa/offline.html.erb` },
  { lang: "erb", name: "rails-rescues-diagnostics", url: `${rails}/actionpack/lib/action_dispatch/middleware/templates/rescues/diagnostics.html.erb` },
  { lang: "erb", name: "rails-rescues-source", url: `${rails}/actionpack/lib/action_dispatch/middleware/templates/rescues/_source.html.erb` },
  { lang: "erb", name: "rails-routes-table", url: `${rails}/actionpack/lib/action_dispatch/middleware/templates/routes/_table.html.erb` },
  { lang: "erb", name: "rails-scaffold-form", url: `${rails}/railties/lib/rails/generators/erb/scaffold/templates/_form.html.erb.tt` },
  { lang: "erb", name: "rails-welcome", url: `${rails}/railties/lib/rails/templates/rails/welcome/index.html.erb` },
  { lang: "erb", name: "sample", url: `${samples}/erb.sample` },
  { lang: "go", name: "cobra-command", url: `${cobra}/command.go` },
  { lang: "go", name: "gin-context", url: `${gin}/context.go` },
  { lang: "go", name: "go-json-decode", url: `${golang}/src/encoding/json/decode.go` },
  { lang: "go", name: "go-slices", url: `${golang}/src/slices/slices.go` },
  { lang: "go", name: "go-sort", url: `${golang}/src/sort/sort.go` },
  { lang: "go", name: "kubernetes-sets", url: `${kubernetes}/staging/src/k8s.io/apimachinery/pkg/util/sets/set.go` },
  { lang: "go", name: "prometheus-storage", url: `${prometheusGo}/storage/interface.go` },
  { lang: "go", name: "sample", url: `${samples}/go.sample` },
  { lang: "go", name: "testify-assertions", url: `${testify}/assert/assertions.go` },
  { lang: "go", name: "viper-viper", url: `${viper}/viper.go` },
  { lang: "go", name: "zap-logger", url: `${zap}/logger.go` },
  { lang: "graphql", name: "codegen-apollo-tooling", url: `${codegen}/dev-test/apollo-tooling/schema.graphql` },
  { lang: "graphql", name: "graphiql-kitchen-sink", url: `${graphiql}/packages/codemirror-graphql/src/__tests__/kitchen-sink.graphql` },
  { lang: "graphql", name: "graphiql-merged-query", url: `${graphiql}/packages/graphiql-toolkit/src/graphql-helpers/__tests__/__queries__/mergedQueryWithSchema.graphql` },
  { lang: "graphql", name: "graphiql-schema-kitchen-sink", url: `${graphiql}/packages/codemirror-graphql/src/__tests__/schema-kitchen-sink.graphql` },
  { lang: "graphql", name: "graphiql-star-wars", url: `${graphiql}/packages/graphql-language-service-server/src/__tests__/__schema__/StarWarsSchema.graphql` },
  { lang: "graphql", name: "graphql-js-kitchen-sink", url: `${graphqlJs}/benchmark/kitchen-sink.graphql` },
  { lang: "graphql", name: "sample", url: `${samples}/graphql.sample` },
  // Haskell runs on the TextMate engine. Shiki's Markdown grammar has no
  // Haskell fence, so no Markdown input embeds it.
  { lang: "haskell", name: "aeson-builder", url: `${aeson}/src/Data/Aeson/Encoding/Builder.hs` },
  { lang: "haskell", name: "aeson-decoding-text", url: `${aeson}/src/Data/Aeson/Decoding/Text.hs` },
  { lang: "haskell", name: "aeson-github", url: `${aeson}/examples/src/GitHub.hs` },
  { lang: "haskell", name: "sample", url: `${samples}/haskell.sample` },
  { lang: "html", name: "bootstrap3-visual-modal", url: `${bootstrap3}/js/tests/visual/modal.html` },
  { lang: "html", name: "h5bp-404", url: `${h5bp}/src/404.html` },
  { lang: "html", name: "h5bp-index", url: `${h5bp}/src/index.html` },
  { lang: "html", name: "jquery-delegatetest", url: `${jquery}/test/delegatetest.html` },
  { lang: "html", name: "jquery-qunit-fixture", url: `${jquery}/test/data/qunit-fixture.html` },
  { lang: "html", name: "mathjax-mml-chtml", url: `${mathjaxDemos}/page/mml-chtml.html` },
  { lang: "html", name: "mathjax-tex-mml", url: `${mathjaxDemos}/page/tex-mml.html` },
  { lang: "html", name: "sample", url: `${samples}/html.sample` },
  { lang: "html", name: "three-animation-keyframes", url: `${threeJs}/examples/webgl_animation_keyframes.html` },
  { lang: "html", name: "three-css3d-periodictable", url: `${threeJs}/examples/css3d_periodictable.html` },
  { lang: "html", name: "three-geometry-colors", url: `${threeJs}/examples/webgl_geometry_colors.html` },
  { lang: "html", name: "three-loader-gltf", url: `${threeJs}/examples/webgl_loader_gltf.html` },
  { lang: "html", name: "todomvc-backbone", url: `${todomvc}/examples/backbone/index.html` },
  { lang: "html", name: "todomvc-jquery", url: `${todomvc}/examples/jquery/index.html` },
  { lang: "html", name: "todomvc-knockoutjs", url: `${todomvc}/examples/knockoutjs/index.html` },
  { lang: "html", name: "todomvc-lit", url: `${todomvc}/examples/lit/index.html` },
  { lang: "html", name: "todomvc-polymer", url: `${todomvc}/examples/polymer/index.html` },
  { lang: "html-derivative", name: "vue-classic-grid", url: `${vue}/packages/vue/examples/classic/grid.html` },
  { lang: "html-derivative", name: "vue-classic-todomvc", url: `${vue}/packages/vue/examples/classic/todomvc.html` },
  { lang: "html-derivative", name: "vue-commits", url: `${vue}/packages/vue/examples/composition/commits.html` },
  { lang: "html-derivative", name: "vue-grid", url: `${vue}/packages/vue/examples/composition/grid.html` },
  { lang: "html-derivative", name: "vue-hydration-visible", url: `${vue}/packages/vue/__tests__/e2e/hydration-strat-visible.html` },
  { lang: "html-derivative", name: "vue-markdown", url: `${vue}/packages/vue/examples/composition/markdown.html` },
  { lang: "html-derivative", name: "vue-modal", url: `${vue}/packages/vue/examples/transition/modal.html` },
  { lang: "html-derivative", name: "vue-svg", url: `${vue}/packages/vue/examples/composition/svg.html` },
  { lang: "html-derivative", name: "vue-todomvc", url: `${vue}/packages/vue/examples/composition/todomvc.html` },
  { lang: "html-derivative", name: "vue-transition-list", url: `${vue}/packages/vue/examples/transition/list.html` },
  { lang: "html-derivative", name: "vue-tree", url: `${vue}/packages/vue/examples/composition/tree.html` },
  { lang: "ini", name: "black-tox", url: `${black}/tox.ini` },
  { lang: "ini", name: "django-tox", url: `${django}/tox.ini` },
  { lang: "ini", name: "kafka-consumer", url: `${kafka}/config/consumer.properties` },
  { lang: "ini", name: "kafka-server", url: `${kafka}/config/server.properties` },
  { lang: "ini", name: "pytest-tox", url: `${pytest}/tox.ini` },
  { lang: "ini", name: "sample", url: `${samples}/ini.sample` },
  { lang: "ini", name: "sqlalchemy-tox", url: `${sqlalchemy}/tox.ini` },
  { lang: "ini", name: "tomcat-catalina", url: `${tomcat}/conf/catalina.properties` },
  {
    lang: "java",
    name: "commons-lang-range",
    url: `${commonsLang}/src/main/java/org/apache/commons/lang3/Range.java`,
  },
  {
    lang: "java",
    name: "commons-lang-validate",
    url: `${commonsLang}/src/main/java/org/apache/commons/lang3/Validate.java`,
  },
  {
    lang: "java",
    name: "gson-gson",
    url: `${gson}/gson/src/main/java/com/google/gson/Gson.java`,
  },
  {
    lang: "java",
    name: "gson-json-reader",
    url: `${gson}/gson/src/main/java/com/google/gson/stream/JsonReader.java`,
  },
  {
    lang: "java",
    name: "guava-immutable-list",
    url: `${guava}/guava/src/com/google/common/collect/ImmutableList.java`,
  },
  {
    lang: "java",
    name: "guava-preconditions",
    url: `${guava}/guava/src/com/google/common/base/Preconditions.java`,
  },
  {
    lang: "java",
    name: "rewrite-switch-patterns",
    url: `${rewrite}/rewrite-java-tck/src/main/java/org/openrewrite/java/tree/SwitchPatternMatchingTest.java`,
  },
  { lang: "java", name: "sample", url: `${samples}/java.sample` },
  { lang: "javascript", name: "axios-xhr", url: `${axios}/lib/adapters/xhr.js` },
  {
    lang: "javascript",
    name: "coverage",
    url: new URL("inputs/typescript-coverage.ts", import.meta.url).href,
  },
  { lang: "javascript", name: "express-application", url: `${express}/lib/application.js` },
  { lang: "javascript", name: "express-response", url: `${express}/lib/response.js` },
  { lang: "javascript", name: "sample", url: `${samples}/javascript.sample` },
  { lang: "javascript", name: "tsc-await-using-declarations-11", url: `${typescriptTests}/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.11.ts` },
  { lang: "javascript", name: "tsc-check-merged-global-umdsymbol", url: `${typescriptTests}/compiler/checkMergedGlobalUMDSymbol.ts` },
  { lang: "javascript", name: "tsc-contextual-type-caching", url: `${typescriptTests}/compiler/contextualTypeCaching.ts` },
  { lang: "javascript", name: "tsc-contravariant-only-inference-from-annotated-function-js", url: `${typescriptTests}/compiler/contravariantOnlyInferenceFromAnnotatedFunctionJs.ts` },
  { lang: "javascript", name: "tsc-declaration-emit-higher-order-retained-generics", url: `${typescriptTests}/compiler/declarationEmitHigherOrderRetainedGenerics.ts` },
  { lang: "javascript", name: "tsc-divergent-accessors-types7", url: `${typescriptTests}/compiler/divergentAccessorsTypes7.ts` },
  { lang: "javascript", name: "tsc-fix-signature-caching", url: `${typescriptTests}/conformance/fixSignatureCaching.ts` },
  { lang: "javascript", name: "tsc-jsdoc-disallowed-in-typescript", url: `${typescriptTests}/conformance/jsdoc/jsdocDisallowedInTypescript.ts` },
  { lang: "javascript", name: "tsc-jsx-namespace-prefix-in-name", url: `${typescriptTests}/compiler/jsxNamespacePrefixInName.tsx` },
  { lang: "javascript", name: "tsc-parserharness", url: `${typescriptTests}/conformance/parser/ecmascript5/RealWorld/parserharness.ts` },
  { lang: "javascript", name: "tsc-plain-jsgrammar-errors", url: `${typescriptTests}/conformance/salsa/plainJSGrammarErrors.ts` },
  { lang: "javascript", name: "tsc-source-map-validation-statements", url: `${typescriptTests}/compiler/sourceMapValidationStatements.ts` },
  { lang: "javascript", name: "tsc-tsx-react-emit-entities", url: `${typescriptTests}/conformance/jsx/tsxReactEmitEntities.tsx` },
  { lang: "javascript", name: "tsc-variadic-tuples1", url: `${typescriptTests}/conformance/types/tuple/variadicTuples1.ts` },
  { lang: "javascript", name: "tsc-yield-expression-inner-comment-emit", url: `${typescriptTests}/compiler/yieldExpressionInnerCommentEmit.ts` },
  { lang: "json", name: "sample", url: `${samples}/json.sample` },
  {
    lang: "json",
    name: "vscode-typescript-grammar",
    url: `${vscode}/extensions/typescript-basics/syntaxes/TypeScript.tmLanguage.json`,
  },
  { lang: "json", name: "vue-package", url: `${vue}/package.json` },
  { lang: "json", name: "zod-package", url: `${zod}/packages/zod/package.json` },
  { lang: "jsonc", name: "sample", url: `${samples}/jsonc.sample` },
  { lang: "jsonc", name: "vscode-launch", url: `${vscode}/.vscode/launch.json` },
  { lang: "jsonl", name: "rag-output", url: `${rag}/output.jsonl` },
  { lang: "jsonl", name: "rag-output-50", url: `${rag}/output-50.jsonl` },
  { lang: "jsonl", name: "sample", url: `${samples}/jsonl.sample` },
  {
    lang: "jsx",
    name: "coverage",
    url: new URL("inputs/typescript-coverage.ts", import.meta.url).href,
  },
  { lang: "jsx", name: "preact-keys", url: `${preact}/test/browser/keys.test.jsx` },
  { lang: "jsx", name: "preact-render", url: `${preact}/compat/test/browser/render.test.jsx` },
  { lang: "jsx", name: "sample", url: `${samples}/jsx.sample` },
  // Julia runs on the TextMate engine.
  { lang: "jsx", name: "tsc-arbitrary-module-namespace-identifiers-syntax", url: `${typescriptTests}/conformance/es2022/arbitraryModuleNamespaceIdentifiers/arbitraryModuleNamespaceIdentifiers_syntax.ts` },
  { lang: "jsx", name: "tsc-async-with-var-shadowing-es6", url: `${typescriptTests}/conformance/async/es6/asyncWithVarShadowing_es6.ts` },
  { lang: "jsx", name: "tsc-await-using-declarations-1", url: `${typescriptTests}/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.1.ts` },
  { lang: "jsx", name: "tsc-checker-initialization-crash", url: `${typescriptTests}/compiler/checkerInitializationCrash.ts` },
  { lang: "jsx", name: "tsc-computed-property-names-contextual-type6-es5", url: `${typescriptTests}/conformance/es6/computedProperties/computedPropertyNamesContextualType6_ES5.ts` },
  { lang: "jsx", name: "tsc-contravariant-only-inference-from-annotated-function-js", url: `${typescriptTests}/compiler/contravariantOnlyInferenceFromAnnotatedFunctionJs.ts` },
  { lang: "jsx", name: "tsc-control-flow-optional-chain", url: `${typescriptTests}/conformance/controlFlow/controlFlowOptionalChain.ts` },
  { lang: "jsx", name: "tsc-correlated-unions", url: `${typescriptTests}/compiler/correlatedUnions.ts` },
  { lang: "jsx", name: "tsc-declaration-emit-binding-patterns-unused", url: `${typescriptTests}/compiler/declarationEmitBindingPatternsUnused.ts` },
  { lang: "jsx", name: "tsc-declaration-emit-higher-order-retained-generics", url: `${typescriptTests}/compiler/declarationEmitHigherOrderRetainedGenerics.ts` },
  { lang: "jsx", name: "tsc-divergent-accessors-types7", url: `${typescriptTests}/compiler/divergentAccessorsTypes7.ts` },
  { lang: "jsx", name: "tsc-fix-signature-caching", url: `${typescriptTests}/conformance/fixSignatureCaching.ts` },
  { lang: "jsx", name: "tsc-jsdoc-disallowed-in-typescript", url: `${typescriptTests}/conformance/jsdoc/jsdocDisallowedInTypescript.ts` },
  { lang: "jsx", name: "tsc-jsx-complex-signature-has-applicability-error", url: `${typescriptTests}/compiler/jsxComplexSignatureHasApplicabilityError.tsx` },
  { lang: "jsx", name: "tsc-jsx-namespace-prefix-in-name", url: `${typescriptTests}/compiler/jsxNamespacePrefixInName.tsx` },
  { lang: "jsx", name: "tsc-parserharness", url: `${typescriptTests}/conformance/parser/ecmascript5/RealWorld/parserharness.ts` },
  { lang: "jsx", name: "tsc-source-map-validation-statements", url: `${typescriptTests}/compiler/sourceMapValidationStatements.ts` },
  { lang: "jsx", name: "tsc-tsx-react-emit-entities", url: `${typescriptTests}/conformance/jsx/tsxReactEmitEntities.tsx` },
  { lang: "jsx", name: "tsc-yield-expression-inner-comment-emit", url: `${typescriptTests}/compiler/yieldExpressionInnerCommentEmit.ts` },
  { lang: "julia", name: "base-abstractset", url: `${julia}/base/abstractset.jl` },
  { lang: "julia", name: "base-sort", url: `${julia}/base/sort.jl` },
  { lang: "julia", name: "sample", url: `${samples}/julia.sample` },
  { lang: "julia", name: "strings-util", url: `${julia}/base/strings/util.jl` },
  {
    lang: "kotlin",
    name: "coroutines-produce",
    url: `${coroutines}/kotlinx-coroutines-core/common/src/channels/Produce.kt`,
  },
  {
    lang: "kotlin",
    name: "coroutines-state-flow",
    url: `${coroutines}/kotlinx-coroutines-core/common/src/flow/StateFlow.kt`,
  },
  {
    lang: "kotlin",
    name: "exposed-op",
    url: `${exposed}/exposed-core/src/main/kotlin/org/jetbrains/exposed/v1/core/Op.kt`,
  },
  {
    lang: "kotlin",
    name: "ktor-logging",
    url: `${ktor}/ktor-client/ktor-client-plugins/ktor-client-logging/common/src/io/ktor/client/plugins/logging/Logging.kt`,
  },
  {
    lang: "kotlin",
    name: "moshi-util",
    url: `${moshi}/moshi/src/main/java/com/squareup/moshi/internal/Util.kt`,
  },
  {
    lang: "kotlin",
    name: "okhttp-build",
    url: `${okhttp}/okhttp/build.gradle.kts`,
  },
  {
    lang: "kotlin",
    name: "okhttp-cookie",
    url: `${okhttp}/okhttp/src/commonJvmAndroid/kotlin/okhttp3/Cookie.kt`,
  },
  {
    lang: "kotlin",
    name: "okhttp-request",
    url: `${okhttp}/okhttp/src/commonJvmAndroid/kotlin/okhttp3/Request.kt`,
  },
  { lang: "kotlin", name: "sample", url: `${samples}/kotlin.sample` },
  {
    lang: "kotlin",
    name: "serialization-json",
    url: `${kotlinxSerialization}/formats/json/commonMain/src/kotlinx/serialization/json/Json.kt`,
  },
  {
    lang: "kotlin",
    name: "turbine-flow-test",
    url: `${turbine}/src/commonTest/kotlin/app/cash/turbine/FlowTest.kt`,
  },
  { lang: "less", name: "antd-button-mixin", url: `${antd}/components/button/style/mixin.less` },
  { lang: "less", name: "antd-color-palette", url: `${antd}/components/style/color/colorPalette.less` },
  { lang: "less", name: "antd-compact-item", url: `${antd}/components/style/mixins/compact-item.less` },
  { lang: "less", name: "antd-customize", url: `${antd}/components/style/mixins/customize.less` },
  { lang: "less", name: "bootstrap3-forms", url: `${bootstrap3}/less/forms.less` },
  { lang: "less", name: "bootstrap3-mixins-buttons", url: `${bootstrap3}/less/mixins/buttons.less` },
  { lang: "less", name: "bootstrap3-mixins-gradients", url: `${bootstrap3}/less/mixins/gradients.less` },
  { lang: "less", name: "bootstrap3-mixins-grid-framework", url: `${bootstrap3}/less/mixins/grid-framework.less` },
  { lang: "less", name: "bootstrap3-navbar", url: `${bootstrap3}/less/navbar.less` },
  { lang: "less", name: "bootstrap3-variables", url: `${bootstrap3}/less/variables.less` },
  {
    lang: "less",
    name: "coverage",
    url: new URL("inputs/less-coverage.less", import.meta.url).href,
  },
  { lang: "less", name: "lessjs-at-rules", url: `${lessjs}/packages/test-data/tests-unit/at-rules/at-rules.less` },
  { lang: "less", name: "lessjs-at-rules-compressed", url: `${lessjs}/packages/test-data/tests-config/at-rules-compressed/at-rules-compressed.less` },
  { lang: "less", name: "lessjs-container", url: `${lessjs}/packages/test-data/tests-unit/container/container.less` },
  { lang: "less", name: "lessjs-css-3", url: `${lessjs}/packages/test-data/tests-unit/css-3/css-3.less` },
  { lang: "less", name: "lessjs-extract-and-length", url: `${lessjs}/packages/test-data/tests-unit/extract-and-length/extract-and-length.less` },
  { lang: "less", name: "lessjs-functions", url: `${lessjs}/packages/test-data/tests-unit/functions/functions.less` },
  { lang: "less", name: "lessjs-functions-each", url: `${lessjs}/packages/test-data/tests-unit/functions-each/functions-each.less` },
  { lang: "less", name: "lessjs-import-reference", url: `${lessjs}/packages/test-data/tests-unit/import/import/import-reference.less` },
  { lang: "less", name: "lessjs-math-strict", url: `${lessjs}/packages/test-data/tests-config/math-strict/css.less` },
  { lang: "less", name: "lessjs-merge", url: `${lessjs}/packages/test-data/tests-unit/merge/merge.less` },
  { lang: "less", name: "lessjs-selectors", url: `${lessjs}/packages/test-data/tests-unit/selectors/selectors.less` },
  { lang: "less", name: "lessjs-sourcemaps-comprehensive", url: `${lessjs}/packages/test-data/tests-config/sourcemaps/comprehensive/comprehensive.less` },
  { lang: "less", name: "lessjs-urls", url: `${lessjs}/packages/test-data/tests-unit/urls/urls.less` },
  { lang: "less", name: "sample", url: `${samples}/less.sample` },
  { lang: "lua", name: "inspect", url: `${inspectLua}/inspect.lua` },
  { lang: "lua", name: "json", url: `${jsonLua}/json.lua` },
  { lang: "lua", name: "lua-literals", url: `${lua}/testes/literals.lua` },
  { lang: "lua", name: "luals-string", url: `${luaLanguageServer}/meta/template/string.lua` },
  { lang: "lua", name: "luals-table", url: `${luaLanguageServer}/meta/template/table.lua` },
  { lang: "lua", name: "luarocks-util", url: `${luarocks}/src/luarocks/core/util.lua` },
  { lang: "lua", name: "lume", url: `${lume}/lume.lua` },
  { lang: "lua", name: "penlight-stringx", url: `${penlight}/lua/pl/stringx.lua` },
  { lang: "lua", name: "sample", url: `${samples}/lua.sample` },
  { lang: "make", name: "cjson", url: `${cjson}/Makefile` },
  { lang: "make", name: "helm", url: `${helm}/Makefile` },
  { lang: "make", name: "minikube", url: `${minikube}/Makefile` },
  { lang: "make", name: "moby", url: `${moby}/Makefile` },
  { lang: "make", name: "node", url: `${nodejs}/Makefile` },
  { lang: "make", name: "nvm", url: `${nvmMake}/Makefile` },
  { lang: "make", name: "prometheus-common", url: `${prometheus}/Makefile.common` },
  { lang: "make", name: "sample", url: `${samples}/make.sample` },
  { lang: "make", name: "zlib", url: `${zlib}/Makefile.in` },
  {
    lang: "markdown",
    name: "argument-parser-declaring-arguments",
    url: `${argumentParser}/Sources/ArgumentParser/Documentation.docc/Articles/DeclaringArguments.md`,
  },
  {
    lang: "markdown",
    name: "argument-parser-getting-started",
    url: `${argumentParser}/Sources/ArgumentParser/Documentation.docc/Articles/GettingStarted.md`,
  },
  { lang: "markdown", name: "benchmark-user-guide", url: `${benchmark}/docs/user_guide.md` },
  { lang: "markdown", name: "black-other-tools", url: `${black}/docs/guides/using_black_with_other_tools.md` },
  {
    lang: "markdown",
    name: "book-ownership",
    url: `${book}/src/ch04-01-what-is-ownership.md`,
  },
  {
    lang: "markdown",
    name: "book-strings",
    url: `${book}/src/ch08-02-strings.md`,
  },
  {
    lang: "markdown",
    name: "cargo-release",
    url: `${cargo}/doc/contrib/src/process/release.md`,
  },
  { lang: "markdown", name: "catch2-tutorial", url: `${catch2}/docs/tutorial.md` },
  { lang: "markdown", name: "cjson-readme", url: `${cjson}/README.md` },
  { lang: "markdown", name: "clap-readme", url: `${clap}/README.md` },
  { lang: "markdown", name: "click-man-readme", url: `${clickMan}/README.md` },
  { lang: "markdown", name: "cobra-user-guide", url: `${cobra}/site/content/user_guide.md` },
  {
    lang: "markdown",
    name: "coverage",
    url: new URL("inputs/markdown-coverage.md", import.meta.url).href,
  },
  { lang: "markdown", name: "dart-args-readme", url: `${dartCore}/pkgs/args/README.md` },
  { lang: "markdown", name: "dart-http-readme", url: `${dartHttp}/pkgs/http/README.md` },
  { lang: "markdown", name: "device-virtual-readme", url: `${deviceVirtual}/README.md` },
  { lang: "markdown", name: "devise-readme", url: `${devise}/README.md` },
  { lang: "markdown", name: "eslint-mcp", url: `${eslint}/docs/src/use/mcp.md` },
  {
    lang: "markdown",
    name: "eslint-no-restricted-imports",
    url: `${eslint}/docs/src/rules/no-restricted-imports.md`,
  },
  { lang: "markdown", name: "fastapi-docker", url: `${fastapi}/docs/en/docs/deployment/docker.md` },
  { lang: "markdown", name: "fastapi-readme", url: `${fastapi}/README.md` },
  { lang: "markdown", name: "gin-readme", url: `${gin}/README.md` },
  { lang: "markdown", name: "gson-user-guide", url: `${gson}/UserGuide.md` },
  { lang: "markdown", name: "highlightjs-readme", url: `${highlightjs}/README.md` },
  { lang: "markdown", name: "httparser-readme", url: `${httparser}/README.md` },
  { lang: "markdown", name: "javapoet-readme", url: `${javapoet}/README.md` },
  {
    lang: "markdown",
    name: "jekyll-front-matter",
    url: `${jekyll}/docs/_docs/front-matter.md`,
  },
  { lang: "markdown", name: "julia-functions", url: `${julia}/doc/src/manual/functions.md` },
  { lang: "markdown", name: "julia-strings", url: `${julia}/doc/src/manual/strings.md` },
  {
    lang: "markdown",
    name: "kingfisher-common-tasks-cache",
    url: `${kingfisher}/Sources/Documentation.docc/CommonTasks/CommonTasks_Cache.md`,
  },
  { lang: "markdown", name: "knex-schema-builder", url: `${knex}/docs/src/guide/schema-builder.md` },
  { lang: "markdown", name: "ky-readme", url: `${ky}/readme.md` },
  { lang: "markdown", name: "latexrun-readme", url: `${latexrun}/README.md` },
  { lang: "markdown", name: "markdownlint-blanks-around-headings-arrays", url: `${markdownlint}/test/blanks-around-headings-arrays.md` },
  { lang: "markdown", name: "markdownlint-link-style-autolink-only", url: `${markdownlint}/test/link-style-autolink-only.md` },
  { lang: "markdown", name: "next-with-docker-export-readme", url: `${nextjs}/examples/with-docker-export-output/README.md` },
  { lang: "markdown", name: "next-with-docker-readme", url: `${nextjs}/examples/with-docker/README.md` },
  { lang: "markdown", name: "nvm-readme", url: `${nvm}/README.md` },
  { lang: "markdown", name: "pgvector-readme", url: `${pgvector}/README.md` },
  { lang: "markdown", name: "poetry-faq", url: `${poetry}/docs/faq.md` },
  { lang: "markdown", name: "powershell-releasing", url: `${powershell}/docs/maintainers/releasing.md` },
  { lang: "markdown", name: "powershell-windows-build", url: `${powershell}/docs/building/windows-core.md` },
  { lang: "markdown", name: "preact-readme", url: `${preact}/README.md` },
  { lang: "markdown", name: "prettier-plugins", url: `${prettier}/docs/plugins.md` },
  { lang: "markdown", name: "pyenv-readme", url: `${pyenv}/README.md` },
  { lang: "markdown", name: "rack-readme", url: `${rack}/README.md` },
  { lang: "markdown", name: "rich-readme", url: `${rich}/README.md` },
  { lang: "markdown", name: "sample", url: `${samples}/markdown.sample` },
  { lang: "markdown", name: "scala3-enums", url: `${scala3}/docs/_docs/reference/enums/enums.md` },
  { lang: "markdown", name: "scala3-givens", url: `${scala3}/docs/_docs/reference/contextual/givens.md` },
  { lang: "markdown", name: "serde-readme", url: `${serde}/README.md` },
  { lang: "markdown", name: "shadcn-data-table", url: `${shadcn}/apps/v4/content/docs/components/radix/data-table.mdx` },
  { lang: "markdown", name: "shadcn-message-scroller", url: `${shadcn}/apps/v4/content/docs/components/radix/message-scroller.mdx` },
  { lang: "markdown", name: "tokio-readme", url: `${tokio}/README.md` },
  {
    lang: "markdown",
    name: "traefik-docker",
    url: `${traefik}/docs/content/getting-started/docker.md`,
  },
  { lang: "markdown", name: "typeorm-indexes", url: `${typeorm}/docs/docs/indexes.md` },
  {
    lang: "markdown",
    name: "unity-assertions",
    url: `${unity}/docs/UnityAssertionsReference.md`,
  },
  {
    lang: "markdown",
    name: "unity-getting-started",
    url: `${unity}/docs/UnityGettingStartedGuide.md`,
  },
  {
    lang: "markdown",
    name: "vite-api-plugin",
    url: `${vite}/docs/guide/api-plugin.md`,
  },
  {
    lang: "markdown",
    name: "vite-features",
    url: `${vite}/docs/guide/features.md`,
  },
  { lang: "markdown", name: "vite-ssr", url: `${vite}/docs/guide/ssr.md` },
  {
    lang: "markdown",
    name: "vitest-migration-jest",
    url: `${vitest}/docs/guide/migration/jest.md`,
  },
  {
    lang: "markdown",
    name: "zod-api",
    url: `${zod}/packages/docs/content/api.mdx`,
  },
  {
    lang: "markdown",
    name: "zod-readme",
    url: `${zod}/packages/zod/README.md`,
  },
  // Mermaid runs on the TextMate engine. Shiki's Markdown grammar has no
  // Mermaid fence, so no Markdown input embeds it.
  { lang: "mermaid", name: "api-sequence", url: `${mermaid}/docs/diagrams/mermaid-api-sequence.mmd` },
  { lang: "mermaid", name: "flowchart-code-flow", url: `${mermaid}/docs/diagrams/flowchart-code-flow.mmd` },
  { lang: "mermaid", name: "mindmap-sequence", url: `${mermaid}/docs/diagrams/mindmap-implementation-sequence.mmd` },
  { lang: "mermaid", name: "sample", url: `${samples}/mermaid.sample` },
  // Nix runs on the TextMate engine. Shiki's Markdown grammar has no Nix
  // fence, so no Markdown input embeds it.
  { lang: "nix", name: "home-manager-flake", url: `${homeManager}/flake.nix` },
  { lang: "nix", name: "home-manager-tmux", url: `${homeManager}/modules/programs/tmux.nix` },
  { lang: "nix", name: "nixpkgs-lists", url: `${nixpkgs}/lib/lists.nix` },
  { lang: "nix", name: "nixpkgs-ripgrep", url: `${nixpkgs}/pkgs/by-name/ri/ripgrep/package.nix` },
  { lang: "nix", name: "sample", url: `${samples}/nix.sample` },
  // PowerShell runs on the TextMate engine.
  { lang: "powershell", name: "dotnet-install", url: `${dotnetInstall}/src/dotnet-install.ps1` },
  { lang: "powershell", name: "install-powershell", url: `${powershell}/tools/install-powershell.ps1` },
  { lang: "powershell", name: "sample", url: `${samples}/powershell.sample` },
  { lang: "powershell", name: "set-release-tag", url: `${powershell}/tools/releaseBuild/setReleaseTag.ps1` },
  // Protobuf runs on the TextMate engine. Shiki's Markdown grammar has no
  // Protobuf fence, so no Markdown input embeds it.
  { lang: "proto", name: "googleapis-http", url: `${googleapis}/google/api/http.proto` },
  { lang: "proto", name: "googleapis-operations", url: `${googleapis}/google/longrunning/operations.proto` },
  { lang: "proto", name: "grpc-route-guide", url: `${grpc}/examples/protos/route_guide.proto` },
  { lang: "proto", name: "sample", url: `${samples}/proto.sample` },
  { lang: "python", name: "black-attribute-access-on-number-literals", url: `${black}/tests/data/cases/attribute_access_on_number_literals.py` },
  { lang: "python", name: "black-bytes-docstring", url: `${black}/tests/data/cases/bytes_docstring.py` },
  { lang: "python", name: "black-fmtonoff", url: `${black}/tests/data/cases/fmtonoff.py` },
  { lang: "python", name: "black-pep-701", url: `${black}/tests/data/cases/pep_701.py` },
  { lang: "python", name: "black-python2-detection", url: `${black}/tests/data/miscellaneous/python2_detection.py` },
  { lang: "python", name: "black-string-prefixes", url: `${black}/tests/data/cases/string_prefixes.py` },
  { lang: "python", name: "black-string-quotes", url: `${black}/tests/data/miscellaneous/string_quotes.py` },
  { lang: "python", name: "black-strings", url: `${black}/src/black/strings.py` },
  {
    lang: "python",
    name: "coverage",
    url: new URL("inputs/python-coverage.py", import.meta.url).href,
  },
  { lang: "python", name: "fastapi-routing", url: `${fastapi}/fastapi/routing.py` },
  { lang: "python", name: "flask-helpers", url: `${flask}/src/flask/helpers.py` },
  { lang: "python", name: "pytest-python", url: `${pytest}/src/_pytest/python.py` },
  { lang: "python", name: "requests-models", url: `${requests}/src/requests/models.py` },
  { lang: "python", name: "rich-console", url: `${rich}/rich/console.py` },
  { lang: "python", name: "ruff-class", url: `${ruff}/crates/ruff_python_parser/resources/valid/statement/class.py` },
  { lang: "python", name: "sample", url: `${samples}/python.sample` },
  {
    lang: "ruby",
    name: "cpp-lua-heredocs",
    url: new URL("inputs/ruby-cpp-lua-heredocs.rb", import.meta.url).href,
  },
  { lang: "ruby", name: "devise", url: `${devise}/lib/devise.rb` },
  {
    lang: "ruby",
    name: "end-html",
    url: new URL("inputs/ruby-end-html.rb", import.meta.url).href,
  },
  {
    lang: "ruby",
    name: "heredocs",
    url: new URL("inputs/ruby-heredocs.rb", import.meta.url).href,
  },
  {
    lang: "ruby",
    name: "html-heredocs",
    url: new URL("inputs/ruby-html-heredocs.rb", import.meta.url).href,
  },
  { lang: "ruby", name: "jekyll-site", url: `${jekyll}/lib/jekyll/site.rb` },
  {
    lang: "ruby",
    name: "js-heredocs",
    url: new URL("inputs/ruby-js-heredocs.rb", import.meta.url).href,
  },
  { lang: "ruby", name: "puma-server", url: `${puma}/lib/puma/server.rb` },
  { lang: "ruby", name: "rack-request", url: `${rack}/lib/rack/request.rb` },
  { lang: "ruby", name: "rails-callbacks", url: `${rails}/activesupport/lib/active_support/callbacks.rb` },
  {
    lang: "ruby",
    name: "rails-inflector",
    url: `${rails}/activesupport/lib/active_support/inflector/methods.rb`,
  },
  { lang: "ruby", name: "rake-task", url: `${rake}/lib/rake/task.rb` },
  { lang: "ruby", name: "rspec-example", url: `${rspecCore}/lib/rspec/core/example.rb` },
  { lang: "ruby", name: "rubocop-cop-base", url: `${rubocop}/lib/rubocop/cop/base.rb` },
  { lang: "ruby", name: "sample", url: `${samples}/ruby.sample` },
  { lang: "ruby", name: "thor", url: `${thor}/lib/thor.rb` },
  {
    lang: "ruby",
    name: "xml-heredocs",
    url: new URL("inputs/ruby-xml-heredocs.rb", import.meta.url).href,
  },
  {
    lang: "ruby",
    name: "yaml-heredocs",
    url: new URL("inputs/ruby-yaml-heredocs.rb", import.meta.url).href,
  },
  { lang: "rust", name: "core-macros", url: `${rustc}/library/core/src/macros/mod.rs` },
  {
    lang: "rust",
    name: "coverage",
    url: new URL("inputs/rust-coverage.rs", import.meta.url).href,
  },
  {
    lang: "rust",
    name: "ripgrep-searcher",
    url: `${ripgrep}/crates/searcher/src/searcher/mod.rs`,
  },
  { lang: "rust", name: "sample", url: `${samples}/rust.sample` },
  { lang: "rust", name: "serde-de", url: `${serde}/serde_core/src/de/mod.rs` },
  // Scala runs on the TextMate engine.
  { lang: "scala", name: "circe-build", url: `${circe}/build.sbt` },
  { lang: "scala", name: "circe-json", url: `${circe}/modules/core/shared/src/main/scala/io/circe/Json.scala` },
  { lang: "scala", name: "sample", url: `${samples}/scala.sample` },
  { lang: "scala", name: "scala3-boundary", url: `${scala3}/library/src/scala/util/boundary.scala` },
  { lang: "scala", name: "scala3-source-position", url: `${scala3}/compiler/src/dotty/tools/dotc/util/SourcePosition.scala` },
  { lang: "scss", name: "angular-menu", url: `${angularComponents}/src/material/menu/menu.scss` },
  { lang: "scss", name: "bootstrap-button", url: `${bootstrap}/scss/buttons/_button.scss` },
  { lang: "scss", name: "bootstrap-functions", url: `${bootstrap}/scss/_functions.scss` },
  { lang: "scss", name: "bootstrap-mixins-form-validation", url: `${bootstrap}/scss/mixins/_form-validation.scss` },
  { lang: "scss", name: "bootstrap-mixins-grid", url: `${bootstrap}/scss/mixins/_grid.scss` },
  { lang: "scss", name: "bootstrap-navbar", url: `${bootstrap}/scss/_navbar.scss` },
  { lang: "scss", name: "bootstrap-root", url: `${bootstrap}/scss/_root.scss` },
  { lang: "scss", name: "bootstrap-utilities", url: `${bootstrap}/scss/_utilities.scss` },
  { lang: "scss", name: "bulma-button", url: `${bulma}/sass/elements/button.scss` },
  { lang: "scss", name: "bulma-functions", url: `${bulma}/sass/utilities/functions.scss` },
  { lang: "scss", name: "bulma-mixins", url: `${bulma}/sass/utilities/mixins.scss` },
  {
    lang: "scss",
    name: "coverage",
    url: new URL("inputs/scss-coverage.scss", import.meta.url).href,
  },
  { lang: "scss", name: "sample", url: `${samples}/scss.sample` },
  { lang: "shellscript", name: "nvm", url: `${nvm}/nvm.sh` },
  { lang: "shellscript", name: "pyenv-init", url: `${pyenv}/libexec/pyenv-init` },
  { lang: "shellscript", name: "ruby-build", url: `${rubyBuild}/bin/ruby-build` },
  { lang: "shellscript", name: "sample", url: `${samples}/shellscript.sample` },
  {
    lang: "shellscript",
    name: "unicode-words",
    url: new URL("inputs/shellscript-unicode-words.sh", import.meta.url).href,
  },
  {
    lang: "sql",
    name: "northwind-pubs",
    url: `${sqlServerSamples}/samples/databases/northwind-pubs/instpubs.sql`,
  },
  {
    lang: "sql",
    name: "postgres-create-table",
    url: `${postgres}/src/test/regress/sql/create_table.sql`,
  },
  { lang: "sql", name: "postgres-select", url: `${postgres}/src/test/regress/sql/select.sql` },
  { lang: "sql", name: "postgres-with", url: `${postgres}/src/test/regress/sql/with.sql` },
  { lang: "sql", name: "sample", url: `${samples}/sql.sample` },
  {
    lang: "sql",
    name: "ssms-create-table",
    url: `${sqlServerSamples}/samples/features/ssms-templates/Sql/Table/Create%20Table.sql`,
  },
  {
    lang: "swift",
    name: "alamofire-af-error",
    url: `${alamofire}/Source/Core/AFError.swift`,
  },
  {
    lang: "swift",
    name: "algorithms-chunked",
    url: `${swiftAlgorithms}/Sources/Algorithms/Chunked.swift`,
  },
  {
    lang: "swift",
    name: "argument-parser-argument",
    url: `${argumentParser}/Sources/ArgumentParser/Parsable%20Properties/Argument.swift`,
  },
  {
    lang: "swift",
    name: "argument-parser-command-parser",
    url: `${argumentParser}/Sources/ArgumentParser/Parsing/CommandParser.swift`,
  },
  {
    lang: "swift",
    name: "composable-store",
    url: `${composable}/Sources/ComposableArchitecture/Store.swift`,
  },
  {
    lang: "swift",
    name: "coverage",
    url: new URL("inputs/swift-coverage.swift", import.meta.url).href,
  },
  {
    lang: "swift",
    name: "kingfisher-image-processor",
    url: `${kingfisher}/Sources/Image/ImageProcessor.swift`,
  },
  {
    lang: "swift",
    name: "nio-codec",
    url: `${nio}/Sources/NIOCore/Codec.swift`,
  },
  { lang: "swift", name: "sample", url: `${samples}/swift.sample` },
  {
    lang: "swift",
    name: "snapshot-assert-snapshot",
    url: `${snapshotTesting}/Sources/SnapshotTesting/AssertSnapshot.swift`,
  },
  {
    lang: "swift",
    name: "string-processing-algorithms-tests",
    url: `${stringProcessing}/Tests/RegexTests/AlgorithmsTests.swift`,
  },
  {
    lang: "swift",
    name: "string-processing-custom-tests",
    url: `${stringProcessing}/Tests/RegexBuilderTests/CustomTests.swift`,
  },
  { lang: "swift", name: "swift-test-decl-func-keyword-argument-labels", url: `${swiftTests}/test/decl/func/keyword-argument-labels.swift` },
  { lang: "swift", name: "swift-test-decl-func-rethrows", url: `${swiftTests}/test/decl/func/rethrows.swift` },
  { lang: "swift", name: "swift-test-decl-import-import", url: `${swiftTests}/test/decl/import/import.swift` },
  { lang: "swift", name: "swift-test-decl-protocol-protocol-enum-witness", url: `${swiftTests}/test/decl/protocol/protocol_enum_witness.swift` },
  { lang: "swift", name: "swift-test-decl-protocol-protocols", url: `${swiftTests}/test/decl/protocol/protocols.swift` },
  { lang: "swift", name: "swift-test-expr-expressions", url: `${swiftTests}/test/expr/expressions.swift` },
  { lang: "swift", name: "swift-test-expr-primary-selector-selector", url: `${swiftTests}/test/expr/primary/selector/selector.swift` },
  { lang: "swift", name: "swift-test-expr-unary-async-await", url: `${swiftTests}/test/expr/unary/async_await.swift` },
  { lang: "swift", name: "swift-test-parse-conditionalcompilation-basicparseerrors", url: `${swiftTests}/test/Parse/ConditionalCompilation/basicParseErrors.swift` },
  { lang: "swift", name: "swift-test-parse-escaped-identifiers", url: `${swiftTests}/test/Parse/escaped_identifiers.swift` },
  { lang: "swift", name: "swift-test-parse-operator-decl", url: `${swiftTests}/test/Parse/operator_decl.swift` },
  { lang: "swift", name: "swift-test-parse-raw-string", url: `${swiftTests}/test/Parse/raw_string.swift` },
  { lang: "swift", name: "swift-test-parse-raw-string-errors", url: `${swiftTests}/test/Parse/raw_string_errors.swift` },
  { lang: "swift", name: "swift-test-parse-trailing-comma", url: `${swiftTests}/test/Parse/trailing-comma.swift` },
  { lang: "swift", name: "swift-test-parse-type-expr", url: `${swiftTests}/test/Parse/type_expr.swift` },
  {
    lang: "swift",
    name: "testing-runner-tests",
    url: `${swiftTesting}/Tests/TestingTests/RunnerTests.swift`,
  },
  { lang: "toml", name: "burntsushi-example", url: `${burntsushiToml}/_example/example.toml` },
  { lang: "toml", name: "cargo-manifest", url: `${cargo}/Cargo.toml` },
  { lang: "toml", name: "clap-manifest", url: `${clap}/Cargo.toml` },
  {
    lang: "toml",
    name: "coverage",
    url: new URL("inputs/toml-coverage.toml", import.meta.url).href,
  },
  { lang: "toml", name: "poetry-pyproject", url: `${poetry}/pyproject.toml` },
  { lang: "toml", name: "ripgrep-manifest", url: `${ripgrep}/Cargo.toml` },
  { lang: "toml", name: "ruff-pyproject", url: `${ruff}/pyproject.toml` },
  { lang: "toml", name: "sample", url: `${samples}/toml.sample` },
  { lang: "toml", name: "tokio-manifest", url: `${tokio}/tokio/Cargo.toml` },
  { lang: "toml", name: "toml-test-inline-table-key-dotted-01", url: `${tomlTest}/tests/valid/inline-table/key-dotted-01.toml` },
  { lang: "toml", name: "toml-test-key-quoted-unicode", url: `${tomlTest}/tests/valid/key/quoted-unicode.toml` },
  {
    lang: "tsx",
    name: "coverage",
    url: new URL("inputs/typescript-coverage.ts", import.meta.url).href,
  },
  { lang: "tsx", name: "sample", url: `${samples}/tsx.sample` },
  { lang: "tsx", name: "shadcn-chart", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/chart.tsx` },
  { lang: "tsx", name: "shadcn-combobox", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/combobox.tsx` },
  { lang: "tsx", name: "shadcn-sidebar", url: `${shadcn}/apps/v4/registry/new-york-v4/ui/sidebar.tsx` },
  { lang: "tsx", name: "tsc-async-with-var-shadowing-es6", url: `${typescriptTests}/conformance/async/es6/asyncWithVarShadowing_es6.ts` },
  { lang: "tsx", name: "tsc-await-using-declarations-11", url: `${typescriptTests}/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.11.ts` },
  { lang: "tsx", name: "tsc-co-and-contra-variant-inferences3", url: `${typescriptTests}/compiler/coAndContraVariantInferences3.ts` },
  { lang: "tsx", name: "tsc-conflicting-declarations-import-from-namespace2", url: `${typescriptTests}/compiler/conflictingDeclarationsImportFromNamespace2.ts` },
  { lang: "tsx", name: "tsc-contextual-type-caching", url: `${typescriptTests}/compiler/contextualTypeCaching.ts` },
  { lang: "tsx", name: "tsc-contravariant-only-inference-from-annotated-function-js", url: `${typescriptTests}/compiler/contravariantOnlyInferenceFromAnnotatedFunctionJs.ts` },
  { lang: "tsx", name: "tsc-declaration-emit-higher-order-retained-generics", url: `${typescriptTests}/compiler/declarationEmitHigherOrderRetainedGenerics.ts` },
  { lang: "tsx", name: "tsc-dependent-destructured-variables-from-nested-patterns", url: `${typescriptTests}/conformance/controlFlow/dependentDestructuredVariablesFromNestedPatterns.ts` },
  { lang: "tsx", name: "tsc-divergent-accessors-types7", url: `${typescriptTests}/compiler/divergentAccessorsTypes7.ts` },
  { lang: "tsx", name: "tsc-fix-signature-caching", url: `${typescriptTests}/conformance/fixSignatureCaching.ts` },
  { lang: "tsx", name: "tsc-jsdoc-disallowed-in-typescript", url: `${typescriptTests}/conformance/jsdoc/jsdocDisallowedInTypescript.ts` },
  { lang: "tsx", name: "tsc-jsx-namespace-prefix-in-name", url: `${typescriptTests}/compiler/jsxNamespacePrefixInName.tsx` },
  { lang: "tsx", name: "tsc-parserharness", url: `${typescriptTests}/conformance/parser/ecmascript5/RealWorld/parserharness.ts` },
  { lang: "tsx", name: "tsc-plain-jsgrammar-errors", url: `${typescriptTests}/conformance/salsa/plainJSGrammarErrors.ts` },
  { lang: "tsx", name: "tsc-source-map-validation-statements", url: `${typescriptTests}/compiler/sourceMapValidationStatements.ts` },
  { lang: "tsx", name: "tsc-tsx-react-emit-entities", url: `${typescriptTests}/conformance/jsx/tsxReactEmitEntities.tsx` },
  { lang: "tsx", name: "tsc-yield-expression-inner-comment-emit", url: `${typescriptTests}/compiler/yieldExpressionInnerCommentEmit.ts` },
  {
    lang: "typescript",
    name: "coverage",
    url: new URL("inputs/typescript-coverage.ts", import.meta.url).href,
  },
  { lang: "typescript", name: "ky-core", url: `${ky}/source/core/Ky.ts` },
  { lang: "typescript", name: "sample", url: `${samples}/typescript.sample` },
  { lang: "typescript", name: "tsc-assertion-type-predicates1", url: `${typescriptTests}/conformance/controlFlow/assertionTypePredicates1.ts` },
  { lang: "typescript", name: "tsc-async-with-var-shadowing-es6", url: `${typescriptTests}/conformance/async/es6/asyncWithVarShadowing_es6.ts` },
  { lang: "typescript", name: "tsc-await-using-declarations-11", url: `${typescriptTests}/conformance/statements/VariableStatements/usingDeclarations/awaitUsingDeclarations.11.ts` },
  { lang: "typescript", name: "tsc-check-merged-global-umdsymbol", url: `${typescriptTests}/compiler/checkMergedGlobalUMDSymbol.ts` },
  { lang: "typescript", name: "tsc-const-assertions", url: `${typescriptTests}/conformance/expressions/typeAssertions/constAssertions.ts` },
  { lang: "typescript", name: "tsc-constructor-with-incomplete-type-annotation", url: `${typescriptTests}/compiler/constructorWithIncompleteTypeAnnotation.ts` },
  { lang: "typescript", name: "tsc-contravariant-only-inference-from-annotated-function-js", url: `${typescriptTests}/compiler/contravariantOnlyInferenceFromAnnotatedFunctionJs.ts` },
  { lang: "typescript", name: "tsc-declaration-emit-higher-order-retained-generics", url: `${typescriptTests}/compiler/declarationEmitHigherOrderRetainedGenerics.ts` },
  { lang: "typescript", name: "tsc-declaration-emit-mapped-type-preserves-type-parameter-constraint", url: `${typescriptTests}/compiler/declarationEmitMappedTypePreservesTypeParameterConstraint.ts` },
  { lang: "typescript", name: "tsc-duplicate-identifier-binding-element-in-parameter-declaration1", url: `${typescriptTests}/compiler/duplicateIdentifierBindingElementInParameterDeclaration1.ts` },
  { lang: "typescript", name: "tsc-erasable-syntax-only", url: `${typescriptTests}/compiler/erasableSyntaxOnly.ts` },
  { lang: "typescript", name: "tsc-fix-signature-caching", url: `${typescriptTests}/conformance/fixSignatureCaching.ts` },
  { lang: "typescript", name: "tsc-intra-expression-inferences", url: `${typescriptTests}/conformance/types/typeRelationships/typeInference/intraExpressionInferences.ts` },
  { lang: "typescript", name: "tsc-jsdoc-disallowed-in-typescript", url: `${typescriptTests}/conformance/jsdoc/jsdocDisallowedInTypescript.ts` },
  { lang: "typescript", name: "tsc-source-map-validation-statements", url: `${typescriptTests}/compiler/sourceMapValidationStatements.ts` },
  { lang: "typescript", name: "tsc-yield-expression-inner-comment-emit", url: `${typescriptTests}/compiler/yieldExpressionInnerCommentEmit.ts` },
  {
    lang: "typescript",
    name: "vue-api-create-app",
    url: `${vue}/packages/runtime-core/src/apiCreateApp.ts`,
  },
  {
    lang: "typescript",
    name: "vue-effect",
    url: `${vue}/packages/reactivity/src/effect.ts`,
  },
  {
    lang: "typescript",
    name: "zod-classic-schemas",
    url: `${zod}/packages/zod/src/v4/classic/schemas.ts`,
  },
  { lang: "xml", name: "ant-build", url: `${ant}/build.xml` },
  { lang: "xml", name: "ant-checkstyle-frames", url: `${ant}/src/etc/checkstyle/checkstyle-frames.xsl` },
  { lang: "xml", name: "ant-junit-frames", url: `${ant}/src/etc/junit-frames.xsl` },
  { lang: "xml", name: "ant-junit-noframes", url: `${ant}/src/etc/junit-noframes.xsl` },
  { lang: "xml", name: "bootstrap-logo", url: `${bootstrap}/site/static/docs/%5Bversion%5D/assets/brand/bootstrap-logo.svg` },
  { lang: "xml", name: "commons-lang-pom", url: `${commonsLang}/pom.xml` },
  { lang: "xml", name: "foundation-keyed-archive", url: `${foundation}/Tests/Foundation/Resources/NSKeyedUnarchiver-ConcreteValueTest.plist` },
  { lang: "xml", name: "rome-at", url: `${rome}/rome-modules/src/test/resources/xml/at.xml` },
  { lang: "xml", name: "rome-slashdot", url: `${rome}/rome-modules/src/test/resources/xml/slashdot.xml` },
  { lang: "xml", name: "rome-yahoo-top-songs", url: `${rome}/rome-modules/src/test/resources/data/YahooTopSongs.xml` },
  { lang: "xml", name: "sample", url: `${samples}/xml.sample` },
  { lang: "xml", name: "views-activity-entrance", url: `${viewsWidgets}/ConstraintLayoutExamples/motionlayoutintegrations/src/main/res/layout/activity_entrance.xml` },
  { lang: "xml", name: "views-lots-of-cards", url: `${viewsWidgets}/ConstraintLayoutExamples/motionlayoutintegrations/src/main/res/layout/lots_of_cards.xml` },
  { lang: "xml", name: "xerces-design", url: `${xerces}/design/xml/design.xml` },
  { lang: "xml", name: "xerces-schedule", url: `${xerces}/design/xml/schedule.xml` },
  { lang: "xml", name: "xerces-xni-config", url: `${xerces}/docs/xni-config.xml` },
  {
    lang: "yaml",
    name: "airflow-compose",
    url: `${airflow}/airflow-core/docs/howto/docker-compose/docker-compose.yaml`,
  },
  {
    lang: "yaml",
    name: "checkout-test",
    url: `${checkout}/.github/workflows/test.yml`,
  },
  {
    lang: "yaml",
    name: "coverage",
    url: new URL("inputs/yaml-coverage.yaml", import.meta.url).href,
  },
  {
    lang: "yaml",
    name: "petstore-openapi",
    url: `${petstore}/src/main/resources/openapi.yaml`,
  },
  {
    lang: "yaml",
    name: "prometheus-config",
    url: `${prometheus}/documentation/examples/prometheus.yml`,
  },
  {
    lang: "yaml",
    name: "ripgrep-ci",
    url: `${ripgrep}/.github/workflows/ci.yml`,
  },
  { lang: "yaml", name: "sample", url: `${samples}/yaml.sample` },
  { lang: "yaml", name: "yaml-test-suite-c4hz", url: `${yamlTestSuite}/C4HZ/in.yaml` },
  { lang: "yaml", name: "yaml-test-suite-kk5p", url: `${yamlTestSuite}/KK5P/in.yaml` },
  { lang: "yaml", name: "yaml-test-suite-w4tn", url: `${yamlTestSuite}/W4TN/in.yaml` },
  // Zig runs on the TextMate engine. Shiki's Markdown grammar has no Zig
  // fence, so no Markdown input embeds it.
  { lang: "zig", name: "sample", url: `${samples}/zig.sample` },
  { lang: "zig", name: "std-allocator", url: `${zig}/lib/std/mem/Allocator.zig` },
  { lang: "zig", name: "std-fmt", url: `${zig}/lib/std/fmt.zig` },
  { lang: "zig", name: "zls-build", url: `${zls}/build.zig` },
  { lang: "zig", name: "zls-offsets", url: `${zls}/src/offsets.zig` },
];
