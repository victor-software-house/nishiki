/**
 * @file coverage.cpp
 * @brief Exercises the C++ grammar's rarer rules.
 * @param[in] value the value, in \c units, see @ref other.
 * @param [in, out] buffer, other the buffers
 * \a arg \em emph \b bold \p param
 * @addtogroup group
 * @attention careful
 * @code
 * int x = 1;
 * @endcode
 * @callgraph
 * NOTE: a note, @todo: later
 */
/** One line with @callgraph, \a arg, \b bold, \c code, \p param, @ref other, @addtogroup g, @param[in,out] a, b the values, @brief short, @code, NOTE: and @fixme: */
/// Line with @callgraph, \a arg, \em emph, \b bold, \c code, \p param, @ref other
/// @addtogroup grp @param[in, out] first, second both @brief short @code
/// TODO: soon @note: later

// Written for nishiki's Shiki oracle: it exercises rules of the cpp and
// cpp-macro TextMate grammars that ordinary code rarely reaches.
// - A `/* c */` sits in every gap where a rule's regex allows an inline
//   comment, so each of those comment groups takes part in a match.
// - Most lines start at column zero: a rule that may begin with whitespace
//   would otherwise match first, at the indentation.
// - Everything from the raw strings to the end of the text that is not C++
//   repeats inside one macro, for cpp-macro, except the directive there.

/* c */ #include <vector> /* c */
/* c */ #include "local.h" /* c */
/* c */ #include /* c */ HEADER_NAME /* c */
#include_next <limits.h>
/* c */ #undef /* c */ OLD_NAME
/* c */ #pragma mark - Section
/* c */ #line 1000 "coverage.cpp"
#warning 'a warning'
#if 0x1'2 || 0b1'0 || 0'17 || 1'0
/* c */ #elif 1
/* c */ #else
/* c */ #endif
#pragma pack 0b1'0
#pragma pack 0'17
#pragma pack 0x1'2.3'4p1'0
#pragma pack 1'0.2'5e1'0
#pragma pack 1'0e1'0
#pragma pack 0x1'2p1'0
#pragma pack 9zz

auto raw_plain = R"(a "quoted" (paren) string)";
auto raw_glsl = R"glsl(void main() { gl_Position = vec4(1.0); })glsl";
auto raw_glsl_upper = u8R"GLSL(void main() {})GLSL";
auto raw_regex = R"re(\d+\.\d*)re";
auto raw_regex2 = LR"regex([a-z]+)regex";
auto raw_regex3 = UR"_r(x|y)_r";
auto raw_delim = u8R"delim(text with )" inside)delim"_sv;
auto raw_reserved = R"x(text)x"s;
auto raw_long = R"abcdefghijklmnopqrstuvwxyz(too long a delimiter)abcdefghijklmnopqrstuvwxyz";
auto raw_multi = R"md(
first line
second line
)md";

void numbers() {
double a = 0x1'2.3'4p1'0;
double b = 1'0.2'5e1'0;
int c = 0b1'0'1u;
int d = 0'17;
int e = 0x1'2p1'0;
int f = 1'0e1'0;
int g = 12zz;
int h = 0x1.8p3;
int i = 017ULL;
int j = 0b11u;
int k = 1.2.3;
}

// A number before `<` or `<<` inside template arguments, where the grammar
// keeps the comparison or shift from opening a nested template.
struct Shifts : Bits<0x1'2.3'4p1'0 << 1, 1'0.2'5e1'0 << 1, 0b1'0'1u << 1, 0'17 << 1, 0x1'2p1'0 << 1, 1'0e1'0 << 1, 1.2.3 << 1, 1 /* c */ << 1> {};
struct Compares : Bits<0x1'2.3'4p1'0 < 1, 1'0.2'5e1'0 < 1, 0b1'0'1u < 1, 0'17 < 1, 0x1'2p1'0 < 1, 1'0e1'0 < 1, 1.2.3 < 1, 1 /* c */ < 1> {};

void strings() {
const char* s = "\?\a\b\e\f\n\p\r\t\v\\\'\"\0\101\4\x41\u00e9\U0001F600";
char c = '\q';
const char* t = u8"\q";
}

void qualified(struct /* c */ Point /* c */ * /* c */ * /* c */ p /* c */ [ /* c */ ] /* c */,
union /* c */ Value /* c */ & /* c */ v /* c */,
enum /* c */ Color /* c */ * /* c */ c /* c */ [ /* c */ ] /* c */,
class /* c */ Shape /* c */ && /* c */ s /* c */);
void qualified_more(struct Point*, union Value&, enum Color&&, class Shape*);

void declares() {
y = g(static_cast<void (*)(struct /* c */ Point /* c */ * /* c */ & /* c */ p /* c */)>(h));
y = g(static_cast<void (*)(union /* c */ Value /* c */ * /* c */ & /* c */ v /* c */)>(h));
y = g(static_cast<void (*)(enum /* c */ Color /* c */ * /* c */ & /* c */ c /* c */)>(h));
y = g(static_cast<void (*)(class /* c */ Shape /* c */ * /* c */ & /* c */ s /* c */)>(h));
y = g(static_cast<void (*)(struct /* c */ Point /* c */ * /* c */ * /* c */ p /* c */ [ /* c */ ] /* c */)>(h));
y = g(static_cast<void (*)(union /* c */ Value /* c */ * /* c */ * /* c */ v /* c */ [ /* c */ ] /* c */)>(h));
y = g(static_cast<void (*)(enum /* c */ Color /* c */ * /* c */ * /* c */ c /* c */ [ /* c */ ] /* c */)>(h));
y = g(static_cast<void (*)(class /* c */ Shape /* c */ * /* c */ * /* c */ s /* c */ [ /* c */ ] /* c */)>(h));
struct /* c */ Point /* c */ * /* c */ & /* c */ p1 /* c */ ;
struct Point && p2 = q;
union /* c */ Value /* c */ * /* c */ & /* c */ v1 /* c */ ;
union Value && v2 = w;
enum /* c */ Color /* c */ * /* c */ & /* c */ c1 /* c */ ;
enum Color && c2 = d;
class /* c */ Shape /* c */ * /* c */ & /* c */ s1 /* c */ ;
class Shape && s2 = t;
}

typedef class /* c */ DLLEXPORT EXPORTED Widget final /* c */ {
int x;
} *WidgetPtr, &WidgetRef, &&WidgetOdd, WidgetValue;

typedef union /* c */ DLLEXPORT EXPORTED Bits final /* c */ {
int i;
float f;
} *BitsPtr, &BitsRef, &&BitsOdd, BitsValue;

typedef struct /* c */ DLLEXPORT EXPORTED Pair final /* c */ {
int a;
} *PairPtr, &PairRef, &&PairOdd, PairValue;

typedef union {
int i;
} Plain;

typedef class
{
int y;
} PlainClass;

union DLLEXPORT EXPORTED Cell final { int v; };
struct DLLEXPORT EXPORTED Row final { int v; };
class DLLEXPORT EXPORTED Grid final { int v; };
class DLLEXPORT final final { int v; };

Widget::Widget() : x(1), y{2}, z<int>(3) {}
Widget::Widget(int a) = default;
Widget::Widget(const Widget&) = delete;
__cdecl /* c */ ns::Widget /* c */ :: /* c */ Widget /* c */ (int a, int b) : x{a}, y(b) {} ;
outer::inner<int>::Widget::Widget(char c) {}
Widget::~Widget() {}
Widget::~Widget() = default;
/* c */ __stdcall /* c */ ns::Widget /* c */ :: /* c */ ~Widget /* c */ () {} ;
outer::inner<int>::Widget::~Widget() = delete;

void assignments() {
static /* c */ const /* c */ ::ns::Type<int> /* c */ * /* c */ & /* c */ ref /* c */ = value;
thread_local struct Thing && odd = value;
const unsigned long long big = 1;
ns::Type<int>::inner name = 2;
int arr /* c */ [] /* c */ = {1, 2};
__attribute__((aligned(8))) /* c */ struct /* c */ ns::Thing /* c */ * /* c */ * /* c */ items /* c */ [] /* c */ = {};
ns::Thing ctor /* c */ (1, 2);
ns::Thing brace /* c */ {1, 2};
struct ns::Thing<int> /* c */ ctor2 /* c */ (3);
class ns::Thing<int> /* c */ brace2 /* c */ {4};
Point pt{.x = 1, .y = 2};
}

using Alias /* c */ [[deprecated]] = /* c */ const /* c */ struct /* c */ ns::Type<int> /* c */ * /* c */ * /* c */ [3];
using Alias2 = class ns::Thing &;
using Alias3 = unsigned long;

void flow(int x) {
label_one /* c */ :
goto /* c */ label_one /* c */ ;
/* c */ throw /* c */ x;
try {} /* c */ catch (...) {}
/* c */ return /* c */ x;
auto p = /* c */ new /* c */ int;
auto q = /* c */ static_cast /* c */ <long>(x);
auto r = /* c */ this /* c */ ->value;
/* c */ break;
/* c */ continue;
switch (x) { case 1: break; } ;
for (struct Item /* c */ * /* c */ && /* c */ item : items) {}
for (const ns::Item<int>::inner && item : items) {}
for (auto [a, b] : pairs) {}
auto lam = [x]() mutable { return x; };
auto lam2 = []() constexpr { return 1; };
auto lam3 = []() consteval { return 1; };
this->member.call();
/* c */ this /* c */ ->other;
obj.x;
int y = a ? b : c;
}

struct Access {
/* c */ public /* c */ :
};
/* c */ using /* c */ namespace std;
/* c */ inline /* c */ void inl();
/* c */ extern /* c */ int ext;

template</* c */ typename /* c */ T>
void templ(T t) requires (sizeof(T) > 1) {}

struct Base {
virtual ~Base();
};
struct Derived : virtual public Base, private /* c */ ns::Other<int> /* c */ , protected /* c */ Third {};
class /* c */ alignas(16) __declspec(dllexport) __declspec(align(8)) [[using gnu: packed, aligned(8)]] Vec {
alignas(ns::Type) int a;
__attribute__((section("x"), hot, cold)) int b;
__declspec(deprecated) int c;
alignas(8) int d;
__declspec(property(get = getX)) int e;
public:
Vec() {} ;
~Vec() {} ;
Vec& operator=(Vec&&) {} ;
operator int&&() const;
operator Thing &() const;
};
namespace ns {} ;
class Tail {}
;
union U { int a; } ;
enum class E { a } ;
void after() {} ;

int (*fp)(ns::Type<int>, struct Point*, int) = nullptr;
int (* /* c */ & /* c */ fref)(int) = f;
int (* && fpo)(int) = nullptr;
struct ns::Type<int>::inner (*fp2)(int);
void takes(int (*cb)(int), struct ns::Thing (&& cb2)(int), ns::Type<int> (*cb3)(int));
void params(register int a, const struct Thing b, int arr[3], int&& c, struct Q q);

void operators() {
x = a::b;
y = ns::fn<int>();
z = (a ? b : c);
}

// Declarations whose regexes start at the line, so they stay unindented.
void more_declarations() {
static /* c */ const /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ && /* c */ second /* c */ = value;
x = 0; static /* c */ const /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ & /* c */ other /* c */ = value;
auto made = new struct ::ns::Holder<int>::Point {.x = 1, .y = 2};
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ items[] /* c */ = {};
ns::Thing && dangling;
[[maybe_unused]] struct ::ns::Holder<int>::Item * * & (*callback)(int);
x = y */ z;
for (const auto & [first, second] : pairs) {}
for (const ::ns::Pair<int>::Inner * && item : pairs) {}
for (auto && [first, second] : pairs) {}
if (true) {} else [[maybe_unused]] /* c */ struct /* c */ ns::Holder<int>:: /* c */ Name /* c */ obj (1);
if (true) {} else ::ns::Holder<int>::Name obj (1);
if (true) {} else ::ns::Holder<int>::Name obj {1};
if (true) {} else [[maybe_unused]] /* c */ struct /* c */ ns::Holder<int>:: /* c */ Name /* c */ obj /* c */ {1};
x = 5; typename /* c */ ::outer::inner<int>:: /* c */ Name value;
x = 6; /* c */ done /* c */ :
x = 7; /* c */ goto /* c */ done;
auto box = Box<struct /* c */ ns::Bar<int>:: /* c */ Baz>();
asm
/* c */
("mov %0, %1\n\t\x41\101\q" : "=r" (y));
auto cb = [x, /* c */ this]() {};
auto lam = [](int arr[3], const int, register long = 3) {};
}

using Alias [[deprecated]] = const /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ * /* c */ [3];

error::error() : code(1) {}
error::error(const error&) = delete;
error::~error() {}
error::~error() = default;
line::line(int) = default;
line::~line() = delete;

::ns::Type<int> && make_thing() {}
auto trailing() -> struct ::ns::Type<int> {}
void takes_callback(struct ::ns::Holder<int>::Item * * & (*cb)(int));
void unnamed(const int, register long = 3) {}
void with_default(Point p = ::ns::Holder<int>::Point {.x = 1, .y = 2}) {}
struct ::ns::Vec<int> && operator+(const Vec&, const Vec&);
namespace outer::inner::deep {}
namespace a::b::c {}
template < /* c */ class T, /* c */ typename U>
void two_params(T, U);
int (*with_array)(int arr[3], const int, register long = 3);

struct Derived2 : [[deprecated]] /* c */ public /* c */ ns::Base<int>:: /* c */ Inner {};
struct alignas(ns::Type) alignas(8) Aligned {};
struct alignas((8)) Aligned2 {};
void attributes() {
__declspec(property(get = getX, put = putX)) int a;
[[using gnu: always_inline, hot]] int b;
__attribute__((section("x"), hot)) int c;
}


void macro_basics(int x) {
switch (x) { case 1: break; default: break; }
switch /* c */ (x) {}
char ch = '\x41';
char cu = u8'\101';
const char* esc = "\u00e9\U0001F600\101\x41\x4142\q";
auto s1 = sizeof...(Args);
auto t1 = typeid(x);
auto a1 = alignof(int);
alignas(8) char a2[8];
x--;
auto l1 = [x, &y, z = 1, this](int a = 1) -> int { return a; };
auto l2 = [=]() mutable constexpr -> decltype(auto) { return 0; };
int n = int(3);
int m = (x ? n : 0);
/* a block comment
   across lines */
for (auto & item : items) {}
for (auto & [key, value] : pairs) {}
::ns::Item * ptr;
[[maybe_unused]] struct ns::Item & ref;
asm ("mov %[out], %[in]" : [out] "=r" (y) : [in] "r" (x));
}
enum class Mode : int { a, b };
enum Plain { c, d } plain_value;
union DLLEXPORT Cell2 final { int v; } cell_value;
struct [[nodiscard("checked")]] Checked {};
struct FromRoot : public ::ns::Base {};
ns::Out::Type * operator+(Op, Op);
ns::Owner::operator ns::Inner::Value *() {}
ns::Type & make_ref() {}
template <typename T,
          typename U>
void two_lines(T, U);
template <typename T = Box<int>> void with_default_arg(T);
using namespace std::chrono;
extern "C" { int c_func(); }
template <typename T> void constrained(T) requires (sizeof(T) > 1);
struct Op {
Op& operator=(const Op&) = default;
~Op() = default;
Op() : a{1}, b(2) {}
int a, b;
};

// Text that is not C++, kept to compare the grammar's other branches with
// Shiki. Each entry reaches a part of the grammar that no valid C++ above
// reaches: an empty #include, three or more `&` in a declarator, `using`
// inside alignas, __declspec and __attribute__, `final` before a class name,
// and specifiers or attributes where C++ allows none.
/* c */ #include /* c */
typedef class /* c */ DLLEXPORT final EXPORTED Widget final /* c */ {
int x;
} *WidgetPtr, &WidgetRef, &&&WidgetOdd, WidgetValue;
typedef union /* c */ DLLEXPORT final EXPORTED Bits final /* c */ {
int i;
float f;
} *BitsPtr, &BitsRef, &&&BitsOdd, BitsValue;
typedef struct /* c */ DLLEXPORT final EXPORTED Pair final /* c */ {
int a;
} *PairPtr, &PairRef, &&&PairOdd, PairValue;
union DLLEXPORT final EXPORTED Cell final { int v; };
struct DLLEXPORT final EXPORTED Row final { int v; };
class /* c */ alignas(16) __declspec(dllexport, align(8)) [[gnu::packed, using ns: attr]] Vec {
alignas(ns::Type, 8) int a;
__attribute__((section("x"), using ns: hot, ns::cold)) int b;
__declspec(using ns: x, ns::attr) int c;
alignas(using ns: x) int d;
__declspec(property(get = getX)) int e;
public:
Vec() {} ;
~Vec() {} ;
Vec& operator=(Vec&&&) {} ;
operator int&&&() const;
operator Thing &() const;
};
void params(register int a, const struct Thing b, int arr[3], int&&& c, struct Q q);
using Alias [[deprecated]] = typename /* c */ const [[maybe_unused]] /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ &&& /* c */ [3];
::ns::Type<int> &&& make_thing() {}
void takes_callback(struct ::ns::Holder<int>::Item & * &&& (*cb)(int));
void with_default(Point p = struct ::ns::Holder<int>::Point {.x = 1, .y = 2}) {}
struct ::ns::Vec<int> &&& operator+(const Vec&, const Vec&);
struct Derived2 : public [[deprecated]] /* c */ struct /* c */ ns::Base<int>:: /* c */ Inner {};
struct alignas(using ns: x, ns::Type, name, (8)) Aligned {};
void attributes() {
f(__declspec(property(get = g), using ns: a, ns::b) 1);
h([[using gnu: always_inline, hot]] 1);
k(__attribute__((section("x"), ns::attr, using ns: hot)) 1);
}
void not_cpp() {
if (struct /* c */ Point /* c */ & /* c */ * /* c */ &&& /* c */ p /* c */ [ /* c */ ] /* c */) {}
if (union /* c */ Value /* c */ & /* c */ * /* c */ &&& /* c */ v /* c */ [ /* c */ ] /* c */) {}
if (enum /* c */ Color /* c */ & /* c */ * /* c */ &&& /* c */ c /* c */ [ /* c */ ] /* c */) {}
if (class /* c */ Shape /* c */ & /* c */ * /* c */ &&& /* c */ s /* c */ [ /* c */ ] /* c */) {}
struct Point &&& p2 = q;
union Value &&& v2 = w;
enum Color &&& c2 = d;
class Shape &&& s2 = t;
thread_local struct Thing &&& odd = value;
int first = 1, static /* c */ const /* c */ [[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ &&& /* c */ second /* c */ = value;
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ &&& /* c */ items[] /* c */ = {};
ns::Thing &&& dangling;
[[maybe_unused]] struct ::ns::Holder<int>::Item & * &&& (*callback)(int);
for (const ::ns::Pair<int>::Inner * &&& [first, second] : pairs) {}
for (const ::ns::Pair<int>::Inner * &&& item : pairs) {}
for (struct ns::Pair [first, second] : pairs) {}
x = 5; typename /* c */ [[nodiscard]] /* c */ struct /* c */ ::outer::inner<int>:: /* c */ Name value;
auto box = Box<[[maybe_unused]] /* c */ struct /* c */ ns::Bar<int>:: /* c */ Baz>();
auto a2 = alignas(8);
}

#define LONG_MACRO(x) \
do { x; } while (0)

#if FOO
/* c */ #endif


// The same constructs again, as the body of a macro.
#define COVERAGE_BODY(a, b, ...) \
auto raw_plain = R"(a "quoted" (paren) string)"; \
auto raw_glsl = R"glsl(void main() { gl_Position = vec4(1.0); })glsl"; \
auto raw_glsl_upper = u8R"GLSL(void main() {})GLSL"; \
auto raw_regex = R"re(\d+\.\d*)re"; \
auto raw_regex2 = LR"regex([a-z]+)regex"; \
auto raw_regex3 = UR"_r(x|y)_r"; \
auto raw_delim = u8R"delim(text with )" inside)delim"_sv; \
auto raw_reserved = R"x(text)x"s; \
auto raw_long = R"abcdefghijklmnopqrstuvwxyz(too long a delimiter)abcdefghijklmnopqrstuvwxyz"; \
auto raw_multi = R"md( \
first line \
second line \
)md"; \
void numbers() { \
double a = 0x1'2.3'4p1'0; \
double b = 1'0.2'5e1'0; \
int c = 0b1'0'1u; \
int d = 0'17; \
int e = 0x1'2p1'0; \
int f = 1'0e1'0; \
int g = 12zz; \
int h = 0x1.8p3; \
int i = 017ULL; \
int j = 0b11u; \
int k = 1.2.3; \
} \
struct Shifts : Bits<0x1'2.3'4p1'0 << 1, 1'0.2'5e1'0 << 1, 0b1'0'1u << 1, 0'17 << 1, 0x1'2p1'0 << 1, 1'0e1'0 << 1, 1.2.3 << 1, 1 /* c */ << 1> {}; \
struct Compares : Bits<0x1'2.3'4p1'0 < 1, 1'0.2'5e1'0 < 1, 0b1'0'1u < 1, 0'17 < 1, 0x1'2p1'0 < 1, 1'0e1'0 < 1, 1.2.3 < 1, 1 /* c */ < 1> {}; \
void strings() { \
const char* s = "\?\a\b\e\f\n\p\r\t\v\\\'\"\0\101\4\x41\u00e9\U0001F600"; \
char c = '\q'; \
const char* t = u8"\q"; \
} \
void qualified(struct /* c */ Point /* c */ * /* c */ * /* c */ p /* c */ [ /* c */ ] /* c */, \
union /* c */ Value /* c */ & /* c */ v /* c */, \
enum /* c */ Color /* c */ * /* c */ c /* c */ [ /* c */ ] /* c */, \
class /* c */ Shape /* c */ && /* c */ s /* c */); \
void qualified_more(struct Point*, union Value&, enum Color&&, class Shape*); \
void declares() { \
y = g(static_cast<void (*)(struct /* c */ Point /* c */ * /* c */ & /* c */ p /* c */)>(h)); \
y = g(static_cast<void (*)(union /* c */ Value /* c */ * /* c */ & /* c */ v /* c */)>(h)); \
y = g(static_cast<void (*)(enum /* c */ Color /* c */ * /* c */ & /* c */ c /* c */)>(h)); \
y = g(static_cast<void (*)(class /* c */ Shape /* c */ * /* c */ & /* c */ s /* c */)>(h)); \
y = g(static_cast<void (*)(struct /* c */ Point /* c */ * /* c */ * /* c */ p /* c */ [ /* c */ ] /* c */)>(h)); \
y = g(static_cast<void (*)(union /* c */ Value /* c */ * /* c */ * /* c */ v /* c */ [ /* c */ ] /* c */)>(h)); \
y = g(static_cast<void (*)(enum /* c */ Color /* c */ * /* c */ * /* c */ c /* c */ [ /* c */ ] /* c */)>(h)); \
y = g(static_cast<void (*)(class /* c */ Shape /* c */ * /* c */ * /* c */ s /* c */ [ /* c */ ] /* c */)>(h)); \
struct /* c */ Point /* c */ * /* c */ & /* c */ p1 /* c */ ; \
struct Point && p2 = q; \
union /* c */ Value /* c */ * /* c */ & /* c */ v1 /* c */ ; \
union Value && v2 = w; \
enum /* c */ Color /* c */ * /* c */ & /* c */ c1 /* c */ ; \
enum Color && c2 = d; \
class /* c */ Shape /* c */ * /* c */ & /* c */ s1 /* c */ ; \
class Shape && s2 = t; \
} \
typedef class /* c */ DLLEXPORT EXPORTED Widget final /* c */ { \
int x; \
} *WidgetPtr, &WidgetRef, &&WidgetOdd, WidgetValue; \
typedef union /* c */ DLLEXPORT EXPORTED Bits final /* c */ { \
int i; \
float f; \
} *BitsPtr, &BitsRef, &&BitsOdd, BitsValue; \
typedef struct /* c */ DLLEXPORT EXPORTED Pair final /* c */ { \
int a; \
} *PairPtr, &PairRef, &&PairOdd, PairValue; \
typedef union { \
int i; \
} Plain; \
typedef class \
{ \
int y; \
} PlainClass; \
union DLLEXPORT EXPORTED Cell final { int v; }; \
struct DLLEXPORT EXPORTED Row final { int v; }; \
class DLLEXPORT EXPORTED Grid final { int v; }; \
class DLLEXPORT final final { int v; }; \
Widget::Widget() : x(1), y{2}, z<int>(3) {} \
Widget::Widget(int a) = default; \
Widget::Widget(const Widget&) = delete; \
__cdecl /* c */ ns::Widget /* c */ :: /* c */ Widget /* c */ (int a, int b) : x{a}, y(b) {} ; \
outer::inner<int>::Widget::Widget(char c) {} \
Widget::~Widget() {} \
Widget::~Widget() = default; \
/* c */ __stdcall /* c */ ns::Widget /* c */ :: /* c */ ~Widget /* c */ () {} ; \
outer::inner<int>::Widget::~Widget() = delete; \
void assignments() { \
static /* c */ const /* c */ ::ns::Type<int> /* c */ * /* c */ & /* c */ ref /* c */ = value; \
thread_local struct Thing && odd = value; \
const unsigned long long big = 1; \
ns::Type<int>::inner name = 2; \
int arr /* c */ [] /* c */ = {1, 2}; \
__attribute__((aligned(8))) /* c */ struct /* c */ ns::Thing /* c */ * /* c */ * /* c */ items /* c */ [] /* c */ = {}; \
ns::Thing ctor /* c */ (1, 2); \
ns::Thing brace /* c */ {1, 2}; \
struct ns::Thing<int> /* c */ ctor2 /* c */ (3); \
class ns::Thing<int> /* c */ brace2 /* c */ {4}; \
Point pt{.x = 1, .y = 2}; \
} \
using Alias /* c */ [[deprecated]] = /* c */ const /* c */ struct /* c */ ns::Type<int> /* c */ * /* c */ * /* c */ [3]; \
using Alias2 = class ns::Thing &; \
using Alias3 = unsigned long; \
void flow(int x) { \
label_one /* c */ : \
goto /* c */ label_one /* c */ ; \
/* c */ throw /* c */ x; \
try {} /* c */ catch (...) {} \
/* c */ return /* c */ x; \
auto p = /* c */ new /* c */ int; \
auto q = /* c */ static_cast /* c */ <long>(x); \
auto r = /* c */ this /* c */ ->value; \
/* c */ break; \
/* c */ continue; \
switch (x) { case 1: break; } ; \
for (struct Item /* c */ * /* c */ && /* c */ item : items) {} \
for (const ns::Item<int>::inner && item : items) {} \
for (auto [a, b] : pairs) {} \
auto lam = [x]() mutable { return x; }; \
auto lam2 = []() constexpr { return 1; }; \
auto lam3 = []() consteval { return 1; }; \
this->member.call(); \
/* c */ this /* c */ ->other; \
obj.x; \
int y = a ? b : c; \
} \
struct Access { \
/* c */ public /* c */ : \
}; \
/* c */ using /* c */ namespace std; \
/* c */ inline /* c */ void inl(); \
/* c */ extern /* c */ int ext; \
template</* c */ typename /* c */ T> \
void templ(T t) requires (sizeof(T) > 1) {} \
struct Base { \
virtual ~Base(); \
}; \
struct Derived : virtual public Base, private /* c */ ns::Other<int> /* c */ , protected /* c */ Third {}; \
class /* c */ alignas(16) __declspec(dllexport) __declspec(align(8)) [[using gnu: packed, aligned(8)]] Vec { \
alignas(ns::Type) int a; \
__attribute__((section("x"), hot, cold)) int b; \
__declspec(deprecated) int c; \
alignas(8) int d; \
__declspec(property(get = getX)) int e; \
public: \
Vec() {} ; \
~Vec() {} ; \
Vec& operator=(Vec&&) {} ; \
operator int&&() const; \
operator Thing &() const; \
}; \
namespace ns {} ; \
class Tail {} \
; \
union U { int a; } ; \
enum class E { a } ; \
void after() {} ; \
int (*fp)(ns::Type<int>, struct Point*, int) = nullptr; \
int (* /* c */ & /* c */ fref)(int) = f; \
int (* && fpo)(int) = nullptr; \
struct ns::Type<int>::inner (*fp2)(int); \
void takes(int (*cb)(int), struct ns::Thing (&& cb2)(int), ns::Type<int> (*cb3)(int)); \
void params(register int a, const struct Thing b, int arr[3], int&& c, struct Q q); \
void operators() { \
x = a::b; \
y = ns::fn<int>(); \
z = (a ? b : c); \
} \
void more_declarations() { \
static /* c */ const /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ && /* c */ second /* c */ = value; \
x = 0; static /* c */ const /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ & /* c */ other /* c */ = value; \
auto made = new struct ::ns::Holder<int>::Point {.x = 1, .y = 2}; \
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ items[] /* c */ = {}; \
ns::Thing && dangling; \
[[maybe_unused]] struct ::ns::Holder<int>::Item * * & (*callback)(int); \
x = y */ z; \
for (const auto & [first, second] : pairs) {} \
for (const ::ns::Pair<int>::Inner * && item : pairs) {} \
for (auto && [first, second] : pairs) {} \
if (true) {} else [[maybe_unused]] /* c */ struct /* c */ ns::Holder<int>:: /* c */ Name /* c */ obj (1); \
if (true) {} else ::ns::Holder<int>::Name obj (1); \
if (true) {} else ::ns::Holder<int>::Name obj {1}; \
if (true) {} else [[maybe_unused]] /* c */ struct /* c */ ns::Holder<int>:: /* c */ Name /* c */ obj /* c */ {1}; \
x = 5; typename /* c */ ::outer::inner<int>:: /* c */ Name value; \
x = 6; /* c */ done /* c */ : \
x = 7; /* c */ goto /* c */ done; \
auto box = Box<struct /* c */ ns::Bar<int>:: /* c */ Baz>(); \
asm \
/* c */ \
("mov %0, %1\n\t\x41\101\q" : "=r" (y)); \
auto cb = [x, /* c */ this]() {}; \
auto lam = [](int arr[3], const int, register long = 3) {}; \
} \
using Alias [[deprecated]] = const /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ * /* c */ [3]; \
error::error() : code(1) {} \
error::error(const error&) = delete; \
error::~error() {} \
error::~error() = default; \
line::line(int) = default; \
line::~line() = delete; \
::ns::Type<int> && make_thing() {} \
auto trailing() -> struct ::ns::Type<int> {} \
void takes_callback(struct ::ns::Holder<int>::Item * * & (*cb)(int)); \
void unnamed(const int, register long = 3) {} \
void with_default(Point p = ::ns::Holder<int>::Point {.x = 1, .y = 2}) {} \
struct ::ns::Vec<int> && operator+(const Vec&, const Vec&); \
namespace outer::inner::deep {} \
namespace a::b::c {} \
template < /* c */ class T, /* c */ typename U> \
void two_params(T, U); \
int (*with_array)(int arr[3], const int, register long = 3); \
struct Derived2 : [[deprecated]] /* c */ public /* c */ ns::Base<int>:: /* c */ Inner {}; \
struct alignas(ns::Type) alignas(8) Aligned {}; \
struct alignas((8)) Aligned2 {}; \
void attributes() { \
__declspec(property(get = getX, put = putX)) int a; \
[[using gnu: always_inline, hot]] int b; \
__attribute__((section("x"), hot)) int c; \
} \
void macro_basics(int x) { \
switch (x) { case 1: break; default: break; } \
switch /* c */ (x) {} \
char ch = '\x41'; \
char cu = u8'\101'; \
const char* esc = "\u00e9\U0001F600\101\x41\x4142\q"; \
auto s1 = sizeof...(Args); \
auto t1 = typeid(x); \
auto a1 = alignof(int); \
alignas(8) char a2[8]; \
x--; \
auto l1 = [x, &y, z = 1, this](int a = 1) -> int { return a; }; \
auto l2 = [=]() mutable constexpr -> decltype(auto) { return 0; }; \
int n = int(3); \
int m = (x ? n : 0); \
/* a block comment \
   across lines */ \
for (auto & item : items) {} \
for (auto & [key, value] : pairs) {} \
::ns::Item * ptr; \
[[maybe_unused]] struct ns::Item & ref; \
asm ("mov %[out], %[in]" : [out] "=r" (y) : [in] "r" (x)); \
} \
enum class Mode : int { a, b }; \
enum Plain { c, d } plain_value; \
union DLLEXPORT Cell2 final { int v; } cell_value; \
struct [[nodiscard("checked")]] Checked {}; \
struct FromRoot : public ::ns::Base {}; \
ns::Out::Type * operator+(Op, Op); \
ns::Owner::operator ns::Inner::Value *() {} \
ns::Type & make_ref() {} \
template <typename T, \
          typename U> \
void two_lines(T, U); \
template <typename T = Box<int>> void with_default_arg(T); \
using namespace std::chrono; \
extern "C" { int c_func(); } \
template <typename T> void constrained(T) requires (sizeof(T) > 1); \
struct Op { \
Op& operator=(const Op&) = default; \
~Op() = default; \
Op() : a{1}, b(2) {} \
int a, b; \
}; \
typedef class /* c */ DLLEXPORT final EXPORTED Widget final /* c */ { \
int x; \
} *WidgetPtr, &WidgetRef, &&&WidgetOdd, WidgetValue; \
typedef union /* c */ DLLEXPORT final EXPORTED Bits final /* c */ { \
int i; \
float f; \
} *BitsPtr, &BitsRef, &&&BitsOdd, BitsValue; \
typedef struct /* c */ DLLEXPORT final EXPORTED Pair final /* c */ { \
int a; \
} *PairPtr, &PairRef, &&&PairOdd, PairValue; \
union DLLEXPORT final EXPORTED Cell final { int v; }; \
struct DLLEXPORT final EXPORTED Row final { int v; }; \
class /* c */ alignas(16) __declspec(dllexport, align(8)) [[gnu::packed, using ns: attr]] Vec { \
alignas(ns::Type, 8) int a; \
__attribute__((section("x"), using ns: hot, ns::cold)) int b; \
__declspec(using ns: x, ns::attr) int c; \
alignas(using ns: x) int d; \
__declspec(property(get = getX)) int e; \
public: \
Vec() {} ; \
~Vec() {} ; \
Vec& operator=(Vec&&&) {} ; \
operator int&&&() const; \
operator Thing &() const; \
}; \
void params(register int a, const struct Thing b, int arr[3], int&&& c, struct Q q); \
using Alias [[deprecated]] = typename /* c */ const [[maybe_unused]] /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ &&& /* c */ [3]; \
::ns::Type<int> &&& make_thing() {} \
void takes_callback(struct ::ns::Holder<int>::Item & * &&& (*cb)(int)); \
void with_default(Point p = struct ::ns::Holder<int>::Point {.x = 1, .y = 2}) {} \
struct ::ns::Vec<int> &&& operator+(const Vec&, const Vec&); \
struct Derived2 : public [[deprecated]] /* c */ struct /* c */ ns::Base<int>:: /* c */ Inner {}; \
struct alignas(using ns: x, ns::Type, name, (8)) Aligned {}; \
void attributes() { \
f(__declspec(property(get = g), using ns: a, ns::b) 1); \
h([[using gnu: always_inline, hot]] 1); \
k(__attribute__((section("x"), ns::attr, using ns: hot)) 1); \
} \
void not_cpp() { \
if (struct /* c */ Point /* c */ & /* c */ * /* c */ &&& /* c */ p /* c */ [ /* c */ ] /* c */) {} \
if (union /* c */ Value /* c */ & /* c */ * /* c */ &&& /* c */ v /* c */ [ /* c */ ] /* c */) {} \
if (enum /* c */ Color /* c */ & /* c */ * /* c */ &&& /* c */ c /* c */ [ /* c */ ] /* c */) {} \
if (class /* c */ Shape /* c */ & /* c */ * /* c */ &&& /* c */ s /* c */ [ /* c */ ] /* c */) {} \
struct Point &&& p2 = q; \
union Value &&& v2 = w; \
enum Color &&& c2 = d; \
class Shape &&& s2 = t; \
thread_local struct Thing &&& odd = value; \
int first = 1, static /* c */ const /* c */ [[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ &&& /* c */ second /* c */ = value; \
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ &&& /* c */ items[] /* c */ = {}; \
ns::Thing &&& dangling; \
[[maybe_unused]] struct ::ns::Holder<int>::Item & * &&& (*callback)(int); \
for (const ::ns::Pair<int>::Inner * &&& [first, second] : pairs) {} \
for (const ::ns::Pair<int>::Inner * &&& item : pairs) {} \
for (struct ns::Pair [first, second] : pairs) {} \
x = 5; typename /* c */ [[nodiscard]] /* c */ struct /* c */ ::outer::inner<int>:: /* c */ Name value; \
auto box = Box<[[maybe_unused]] /* c */ struct /* c */ ns::Bar<int>:: /* c */ Baz>(); \
auto a2 = alignas(8); \
} \
/** One line with @callgraph, \a arg, \b bold, \c code, \p param, @ref other, @addtogroup g, @param[in,out] a, b the values, @brief short, @code, NOTE: and @fixme: */ \
/** \
 * @brief A documented block inside a macro. \
 * @param[in, out] first, second the values \
 * \a arg \em emph \b bold \c code \p param @ref other \
 * @addtogroup group @attention careful @code @callgraph \
 * NOTE: a note, @todo: later \
 */ \
/// Line with @callgraph, \a arg, \em emph, \b bold, \c code, \p param, @ref other \
/// @addtogroup grp @param[in, out] first, second both @brief short @code \
/// TODO: soon @note: later

// Directives inside a macro body. Each runs to the end of the macro, so each
// macro holds one, on its last line.
#define WITH_LINE \
#line 42 "coverage.cpp"
#define WITH_PRAGMA \
#pragma omp parallel for
#define WITH_ERROR \
#error "a quoted message"
#define WITH_WARNING \
#warning 'a single-quoted message'
#define WITH_PLAIN_WARNING \
#warning a plain message
#define WITH_IF \
#if defined(FOO) && BAR
#define WITH_DEFINE \
#define INNER(a, b, ...) a + b
