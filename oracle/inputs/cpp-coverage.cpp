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

void /* c */ numbers /* c */ () {
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
struct Shifts /* c */ : Bits<0x1'2.3'4p1'0 << 1, 1'0.2'5e1'0 << 1, 0b1'0'1u << 1, 0'17 << 1, 0x1'2p1'0 << 1, 1'0e1'0 << 1, 1.2.3 << 1, 1 /* c */ << 1> {};
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

typedef /* c */ class /* c */ DLLEXPORT EXPORTED Widget /* c */ final /* c */ {
int /* c */ x /* c */ ;
} *WidgetPtr, &WidgetRef, &&WidgetOdd, WidgetValue;

typedef /* c */ union /* c */ DLLEXPORT EXPORTED Bits /* c */ final /* c */ {
int i;
float f;
} *BitsPtr, &BitsRef, &&BitsOdd, BitsValue;

typedef /* c */ struct /* c */ DLLEXPORT EXPORTED Pair final /* c */ {
int a;
} *PairPtr, &PairRef, &&PairOdd, PairValue;

typedef union {
int i;
} /* c */ Plain;

typedef class
{
int y;
} /* c */ PlainClass;

union DLLEXPORT EXPORTED Cell final { int v; };
struct DLLEXPORT EXPORTED Row final { int v; };
class DLLEXPORT EXPORTED Grid final { int v; };
class DLLEXPORT final /* c */ final { int v; };

Widget /* c */ ::Widget() : x(1), y{2}, z< /* c */ int>(3) {}
Widget::Widget(int a) = default;
Widget::Widget(const Widget&) = delete;
__cdecl /* c */ ns::Widget /* c */ :: /* c */ Widget /* c */ (int a /* c */ , int b) : x{a}, y(b) {} ;
outer::inner< /* c */ int>::Widget::Widget(char c) {}
Widget::~Widget() {}
Widget::~Widget() = default;
/* c */ __stdcall /* c */ ns::Widget /* c */ :: /* c */ ~Widget /* c */ () {} ;
outer::inner< /* c */ int>::Widget::~Widget() = delete;

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
switch (x) { /* c */ case 1: break; } ;
for ( /* c */ struct /* c */ Item /* c */ * /* c */ && /* c */ item /* c */ : items) {}
for ( /* c */ const /* c */ ns::Item<int>:: /* c */ inner && item : items) {}
for ( /* c */ auto /* c */ [ /* c */ a /* c */ , /* c */ b /* c */ ] /* c */ : pairs) {}
auto lam = [x /* c */ ]() mutable { return x; };
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
void templ(T t) requires /* c */ (sizeof /* c */ (T) > 1) {}

struct Base {
virtual /* c */ ~Base();
};
struct Derived : virtual public Base, private /* c */ ns::Other<int> /* c */ , protected /* c */ Third {};
class /* c */ alignas(16) /* c */ __declspec(dllexport) __declspec(align(8)) [[using gnu: packed, aligned(8)]] Vec {
alignas /* c */ (ns::Type) int a;
__attribute__((section("x"), hot, cold)) int b;
__declspec(deprecated) int c;
alignas(8) int d;
__declspec(property(get = getX)) int e;
public:
Vec() {} ;
~Vec() {} ;
Vec /* c */ & /* c */ operator /* c */ = /* c */ (Vec&&) {} ;
operator int /* c */ && /* c */ () const;
operator Thing &() const;
};
namespace ns {} ;
class Tail {}
;
union U { int a; } ;
enum class E { a } ;
void after() {} ;

int /* c */ (*fp)(ns::Type<int>, struct Point*, int) /* c */ = nullptr;
int (* /* c */ & /* c */ fref)(int) = f;
int (* && fpo)(int) = nullptr;
struct ns /* c */ ::Type< /* c */ int>::inner (*fp2)(int);
void takes(int /* c */ (*cb)(int) /* c */ , struct ns::Thing (&& cb2)(int), ns:: /* c */ Type<int> (*cb3)(int));
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
ns:: /* c */ Thing /* c */ && dangling;
[[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ & (*callback)(int);
x = y */ z;
for ( /* c */ const /* c */ auto /* c */ & [first, second] : pairs) {}
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
auto lam = [](int arr[3], const /* c */ int /* c */ , register long = 3) {};
}

using Alias [[deprecated]] = const /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ * /* c */ [3];

error::error() : code(1) {}
error::error(const error&) = /* c */ delete;
error::~error() {}
error::~error() = /* c */ default;
line::line(int) = default;
line::~line() = delete;

::ns:: /* c */ Type<int> /* c */ && make_thing() {}
auto trailing() -> /* c */ struct /* c */ ::ns:: /* c */ Type<int> {}
void takes_callback(struct /* c */ ::ns::Holder<int>::Item /* c */ * /* c */ * /* c */ & (*cb)(int));
void unnamed(const /* c */ int /* c */ , register long = 3) {}
void with_default(Point p = :: /* c */ ns::Holder<int>::Point {.x = 1, .y = 2}) {}
struct /* c */ ::ns:: /* c */ Vec<int> && operator+(const Vec&, const Vec&);
namespace /* c */ outer::inner::deep {}
namespace a::b::c {}
template < /* c */ class T, /* c */ typename U>
void two_params(T, U);
int ( /* c */ *with_array)(int arr[3], const int, register long = 3);

struct Derived2 : [[deprecated]] /* c */ public /* c */ ns::Base<int>:: /* c */ Inner {};
struct alignas(ns::Type) /* c */ alignas(8) Aligned {};
struct alignas((8)) Aligned2 {};
void attributes() {
__declspec(property(get = getX, put = putX)) /* c */ int a;
[[using gnu: always_inline, hot]] int b;
__attribute__((section("x"), hot)) int c;
}


// Rules and groups the constructs above leave out.
struct Foo { __cdecl Foo(); };
struct Foo { /* c */ Foo& __fastcall /* c */ operator+(int); };
struct Foo { int operator""_km(unsigned long long); };
void __cdecl /* c */ g() {}
[[nodiscard]] /* c */ void k() {}
enum E2 : ::ns::T< /* c */ int>::U { B };
namespace d = ns::Foo< /* c */ int>::e;
extern "C" { int z; };
int (*gp[3])(int) const;
void construct_objects() { Thing fooBar{1}; }
void method_chains() { x = this->f(); x = fooBar.f(); x = FOO_BAR.f(); x = obj.fooBar.f(); x = obj.FOO_BAR.f(); x = obj.thing.f(); }
void hex_exponents() { double n1 = 0x1p+3; double n2 = 0x1p-3; double n3 = 0x1.8p3f; int n4 = 0x1p3_km; }
void suffixes() { double n5 = 1.5e+3; int n6 = 1.5_km; int n7 = 12_km; int n8 = 0b101_km; int n9 = 0x1F_km; }
void float_exponents() { double n10 = 0x1.8p+3; double n11 = 0x1.8p-3; double n12 = 1e+5; }
void range_attribute() { for ([[maybe_unused]] auto& v : vs) {} }
/* === banner === */
void template_numbers() { f<0x1.8p+3, 0x1.8p-3, 0x1.8p3f, 0x1.8p3_km, 1.5e+3, 1.5e-3, 1.5f, 1.5_km, 0b1_km, 0b1u, 07_km, 07u, 0x1Fu, 0x1F_km, 0x1p+3, 0x1p-3, 1e+3, 1e-3, 12u, 12_km>(x); }
void literal_suffixes() { auto s1 = "abc"sv; auto s2 = 'a'sv; auto s3 = 'a'_x; }
typedef struct [[deprecated]] /* c */ S : Base { int x; } S_t;
typedef class [[deprecated]] /* c */ C : Base { int x; } C_t;
typedef union [[deprecated]] /* c */ U { int x; } U_t;
typedef struct S2 { int x; };
union [[deprecated]] /* c */ U2 : Base { int x; };
using namespace ns::inner< /* c */ int>::deep;
using namespace ::ns::inner;
extern template class TT<int>;
[[nodiscard]] /* c */ Foo operator+(Foo a, Foo b);
[[nodiscard]] Foo ns::Thing<int>::operator+(Foo a) {}
class Less : public Base<0x1.8p+3 < 1, 0x1.8p-3 < 1, 0x1.8p3f < 1, 0x1.8p3_km < 1, 1.5e+3 < 1, 1.5e-3 < 1, 1.5f < 1, 1.5_km < 1, 0b1u < 1, 0b1_km < 1, 07u < 1, 07_km < 1, 0x1p+3 < 1, 0x1p-3 < 1, 0x1Fu < 1, 0x1F_km < 1, 1e+3 < 1, 1e-3 < 1, 12u < 1, 12_km < 1> {};
class Shift : public Base<0x1.8p+3 << 1, 0x1.8p-3 << 1, 0x1.8p3f << 1, 0x1.8p3_km << 1, 1.5e+3 << 1, 1.5e-3 << 1, 1.5f << 1, 1.5_km << 1, 0b1u << 1, 0b1_km << 1, 07u << 1, 07_km << 1, 0x1p+3 << 1, 0x1p-3 << 1, 0x1Fu << 1, 0x1F_km << 1, 1e+3 << 1, 1e-3 << 1, 12u << 1, 12_km << 1> {};
void fpp(int (*cb[3])(int));
struct Foo { operator /* c */ const char*(); };
int operator"" /* c */ _mi(unsigned long long);
static_assert /* c */ (true, "m");
void noexcept_gap() { bool b = noexcept /* c */ (f()); }
void decltype_gap() { decltype /* c */ (x) y = x; }
void upper_calls() { MAKE /* c */ <int>(x); ns::MAKE<int>(x); ::MAKE(x); }
void attr_assign() {
[[maybe_unused]] /* c */ int v = 1;
}
void static_decl() {
static /* c */ int z1;
}
template /* c */ void tf(int);
unsigned /* c */ int uf() {}
struct Ctors {
Ctors() = /* c */ default;
__cdecl /* c */ Ctors(int);
__cdecl /* c */ ~Ctors();
constexpr /* c */ Ctors(char) {}
};
void gnu_param(__attribute__((unused)) /* c */ int (*cb)(int));
void fp_specs() {
int (*fpc)(int) const /* c */ noexcept;
}
void chains() {
x = obj.fooBar.y;
x = obj.FOO_BAR.y;
x = obj.Mixed_ok.y;
x = obj.fooBar.f();
x = obj.FOO_BAR.f();
}
void name_chains() {
x = obj.fooBar.y;
x = obj.FOO.y;
x = obj.Foo.y;
x = obj.FOO.f();
x = FOO.f();
}
namespace d2 = ::a::B<int>::C<int>::e;
using namespace ::a::B<int>::C<int>::d;
template <typename T> void a::B<T>::C<T>::g() {}
void arrays_empty() {
int ARR[] = {1};
int Foo[] = {1};
}
typedef class C2 { int x; };
typedef union U3 { int x; };
typedef union U4 : Base { int x; } U4_t;
struct Del {
~Del() = delete;
};
void scoped_param_comments(a::B</* c */ int>::C<int>::T p);
void call_comments() {
f</* c */ int>(x);
MAKE</* c */ int>(x);
}
enum class[[deprecated]] Attr1 : int { A1 };
enum Attr3 {
C1 [[maybe_unused]] = 1,
C2 [[maybe_unused]],
};
enum E5 : a::B<int>::C<int>::D { X5 };
static __attribute__((used)) int gnu_fn() {}
auto trail2() -> a::B<int>::C<int>::D;
a::B<int>::C<int> ret_scoped() {}
void scoped_decl() {
a::B<int>::C<int> x;
}
a::B<int>::C<int> operator%(a::B<int> l, int r);
void ranged() {
for (__attribute__((unused)) auto [a, b] : m) {}
}
void brace_upper() {
Thing FOO{1};
}
void scoped_upper_call() {
x = a::B<int>::MAKE(1);
}
template <typename T = a /* c */ ::B<int>::C<int>::D> struct TD;
template <typename T = a::B</* c */ int>::C<int>::D> struct TD2;
void tcall() {
x = f<a::B /* c */ <int>::C<int>::D>(1);
x = f<a::B</* c */ int>::C<int>::D>(1);
}
struct Lead {
/* c */ ~Lead();
/* c */ static_assert(true);
/* c */ int m = 1;
/* c */ int n;
};
void asm_raw() {
asm(R"(nop)");
}
struct InitT {
InitT() : x</* c */ int>(1) {}
};
void spec_loop() {
static /* c */ const int q;
}
template <template <typename T> class TT> struct TT3;
void macro_names() {
int snake_case = 1;
int camelCase = 2;
int UPPER = 3;
int Plain = 4;
}
void macro_decls() {
int snake_d;
int camelD;
int UPPER_D;
int Plain_d;
}
namespace  g::inline h {
}
void macro_literals() {
auto s4 = "abc"_x;
char cx = '\x4142';
}
struct DelOps {
DelOps() = delete;
DelOps& operator=(const DelOps&) = delete;
};
void asm_volatile() {
asm volatile ("nop");
}
void decl_names() {
int UPPERD;
int Plaind;
}
void unknown_chain() {
x = obj.Foo.f();
}
ns::B<int>::Widget /* c */ ::Widget(int a) : x{a} {}
ns::B<int>::Widget /* c */ ::~Widget() {}
struct Conv {
operator ns::Other<int>::Inner();
};
void type_calls() {
x = uint_least32_t (1) + pthread_mutex_t () + foo_t ();
}
void unnamed_types(const uint_least32_t, const pthread_mutex_t, const foo_t, const Widget);
auto lam_types = [](const uint_least32_t, const pthread_mutex_t, const foo_t, const Widget) {};
auto lam_defaults = [](int a = 1, int b = 2) {};
struct PostSpecs {
bool operator==(const PostSpecs&) const /* c */ noexcept;
~PostSpecs() /* c */ noexcept /* c */ override;
};
void macro_basics(int x) {
switch (x) { case 1: break; /* c */ default: break; }
switch /* c */ (x) {}
char ch = '\x41';
char cu = u8'\101';
const char* esc = "\u00e9\U0001F600\101\x41\x4142\q";
auto s1 = sizeof... /* c */ (Args);
auto t1 = typeid /* c */ (x);
auto a1 = alignof /* c */ (int);
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
[[maybe_unused]] struct /* c */ ns::Item & ref;
asm ("mov %[out], %[in]" : [ /* c */ out /* c */ ] "=r" (y) : [in] "r" (x));
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
Op& operator=(const Op&) = /* c */ default;
~Op() = /* c */ default;
Op() : a{1}, b(2) {}
int a, b;
};

// Text that is not C++, kept to compare the grammar's other branches with
// Shiki. Each entry reaches a part of the grammar that no valid C++ above
// reaches: an empty #include, three or more `&` in a declarator, `using`
// inside alignas, __declspec and __attribute__, `final` before a class name,
// and specifiers or attributes where C++ allows none.
/* c */ #include /* c */
typedef class /* c */ DLLEXPORT final /* c */ EXPORTED Widget /* c */ final /* c */ {
int x;
} /* c */ *WidgetPtr, &WidgetRef, && /* c */ &WidgetOdd, WidgetValue;
typedef union /* c */ DLLEXPORT final /* c */ EXPORTED Bits /* c */ final /* c */ {
int i;
float f;
} /* c */ *BitsPtr, &BitsRef, && /* c */ &BitsOdd, BitsValue;
typedef struct /* c */ DLLEXPORT final /* c */ EXPORTED Pair /* c */ final /* c */ {
int a;
} /* c */ * /* c */ PairPtr, &PairRef, && /* c */ &PairOdd, PairValue;
union DLLEXPORT final /* c */ EXPORTED Cell final { int v; };
struct DLLEXPORT final /* c */ EXPORTED Row final { int v; };
class /* c */ alignas(16) __declspec(dllexport, align(8)) [[gnu::packed, using ns: attr]] Vec {
alignas(ns::Type, 8) int a;
__attribute__((section("x"), using ns: hot, ns::cold)) int b;
__declspec(using ns: x, ns::attr) int c;
alignas(using ns: x) int d;
__declspec(property(get = getX)) int e;
public:
Vec() {} ;
~Vec() {} ;
Vec& operator=(Vec&& /* c */ &) {} ;
operator int&& /* c */ &() const;
operator Thing &() const;
};
void params(register int a, const struct Thing b, int arr[3], int&& /* c */ & c, struct Q q);
using Alias [[deprecated]] = typename /* c */ const [[maybe_unused]] /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ && /* c */ & /* c */ [3];
::ns::Type<int> && /* c */ & make_thing() {}
void takes_callback(struct ::ns:: /* c */ Holder<int>::Item & * && /* c */ & (*cb)(int));
void with_default(Point p = struct /* c */ ::ns::Holder<int>:: /* c */ Point /* c */ {.x = 1, .y = 2}) {}
struct ::ns::Vec<int> && /* c */ & operator+(const Vec&, const Vec&);
struct Derived2 : public [[deprecated]] /* c */ struct /* c */ ns::Base<int>:: /* c */ Inner {};
struct alignas(using ns: x, ns::Type, name, (8)) Aligned {};
void attributes() {
f(__declspec(property(get = g), using ns: a, ns::b) 1);
h([[using gnu: always_inline, hot]] 1);
k(__attribute__((section("x"), ns::attr, using ns: hot)) 1);
}
void not_cpp() {
if (struct /* c */ Point /* c */ & /* c */ * /* c */ && /* c */ & /* c */ p /* c */ [ /* c */ ] /* c */) {}
if (union /* c */ Value /* c */ & /* c */ * /* c */ && /* c */ & /* c */ v /* c */ [ /* c */ ] /* c */) {}
if (enum /* c */ Color /* c */ & /* c */ * /* c */ && /* c */ & /* c */ c /* c */ [ /* c */ ] /* c */) {}
if (class /* c */ Shape /* c */ & /* c */ * /* c */ && /* c */ & /* c */ s /* c */ [ /* c */ ] /* c */) {}
struct Point && /* c */ & p2 = q;
union Value && /* c */ & v2 = w;
enum Color && /* c */ & c2 = d;
class Shape && /* c */ & s2 = t;
thread_local struct Thing && /* c */ & odd = value;
int first = 1, static /* c */ const /* c */ [[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ && /* c */ & /* c */ second /* c */ = value;
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ && /* c */ & /* c */ items[] /* c */ = {};
ns::Thing && /* c */ & dangling;
[[maybe_unused]] struct ::ns::Holder<int>::Item & * && /* c */ & (*callback)(int);
for (const ::ns::Pair<int>:: /* c */ Inner * /* c */ && /* c */ & [first, second] : pairs) {}
for (const ::ns::Pair<int>::Inner * && /* c */ & item : pairs) {}
for (struct /* c */ ns::Pair [first, second] : pairs) {}
x = 5; typename /* c */ [[nodiscard]] /* c */ struct /* c */ ::outer::inner<int>:: /* c */ Name value;
auto box = Box<[[maybe_unused]] /* c */ struct /* c */ ns::Bar<int>:: /* c */ Baz>();
auto a2 = alignas(8);
}

void this_chains() {
x = obj.this.y;
x = obj.this.f();
}
void typed_compound() {
int y += 1;
int z &= 1;
}
auto trail() -> [[x]] /* c */ int;
namespace a::b< /* c */ int>::c<int>::d {}
void typed_continuation() {
int a = 1, int snake_case = 2, int camelCase = 3, int UPPER = 4, int Plain = 5;
int b = 1, int c += 2, int d &= 3;
}
void notcpp_arrays() {
int arr[] += {1};
int arr2[] &= {1};
}
struct ConvArr {
operator Foo[]();
};
void curly_param(__attribute__((x)) /* c */ Thing{1}) {}
#define LONG_MACRO(x) \
do { x; } while (0)

#if FOO
/* c */ #endif


#if 0x1p+3 > 0x1p-3 && 1.5e+3 > 1.5e-3f && 0x1.8p3f && 1u && 0x1Fu && 0b1u && 07u && 1e+3 && 1e-3 && 0x1p3 && 2ULL
#endif
#if 0x1.8p+3 && 0x1.8p-3
#endif
/* c */ #define LEADING 1
/* c */ #pragma pack(1)
/* c */ #warning leading
/* c */ #if 0
#endif
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
void /* c */ numbers /* c */ () { \
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
struct Shifts /* c */ : Bits<0x1'2.3'4p1'0 << 1, 1'0.2'5e1'0 << 1, 0b1'0'1u << 1, 0'17 << 1, 0x1'2p1'0 << 1, 1'0e1'0 << 1, 1.2.3 << 1, 1 /* c */ << 1> {}; \
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
typedef /* c */ class /* c */ DLLEXPORT EXPORTED Widget /* c */ final /* c */ { \
int /* c */ x /* c */ ; \
} *WidgetPtr, &WidgetRef, &&WidgetOdd, WidgetValue; \
typedef /* c */ union /* c */ DLLEXPORT EXPORTED Bits /* c */ final /* c */ { \
int i; \
float f; \
} *BitsPtr, &BitsRef, &&BitsOdd, BitsValue; \
typedef /* c */ struct /* c */ DLLEXPORT EXPORTED Pair final /* c */ { \
int a; \
} *PairPtr, &PairRef, &&PairOdd, PairValue; \
typedef union { \
int i; \
} /* c */ Plain; \
typedef class \
{ \
int y; \
} /* c */ PlainClass; \
union DLLEXPORT EXPORTED Cell final { int v; }; \
struct DLLEXPORT EXPORTED Row final { int v; }; \
class DLLEXPORT EXPORTED Grid final { int v; }; \
class DLLEXPORT final /* c */ final { int v; }; \
Widget /* c */ ::Widget() : x(1), y{2}, z< /* c */ int>(3) {} \
Widget::Widget(int a) = default; \
Widget::Widget(const Widget&) = delete; \
__cdecl /* c */ ns::Widget /* c */ :: /* c */ Widget /* c */ (int a /* c */ , int b) : x{a}, y(b) {} ; \
outer::inner< /* c */ int>::Widget::Widget(char c) {} \
Widget::~Widget() {} \
Widget::~Widget() = default; \
/* c */ __stdcall /* c */ ns::Widget /* c */ :: /* c */ ~Widget /* c */ () {} ; \
outer::inner< /* c */ int>::Widget::~Widget() = delete; \
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
switch (x) { /* c */ case 1: break; } ; \
for ( /* c */ struct /* c */ Item /* c */ * /* c */ && /* c */ item /* c */ : items) {} \
for ( /* c */ const /* c */ ns::Item<int>:: /* c */ inner && item : items) {} \
for ( /* c */ auto /* c */ [ /* c */ a /* c */ , /* c */ b /* c */ ] /* c */ : pairs) {} \
auto lam = [x /* c */ ]() mutable { return x; }; \
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
void templ(T t) requires /* c */ (sizeof /* c */ (T) > 1) {} \
struct Base { \
virtual /* c */ ~Base(); \
}; \
struct Derived : virtual public Base, private /* c */ ns::Other<int> /* c */ , protected /* c */ Third {}; \
class /* c */ alignas(16) /* c */ __declspec(dllexport) __declspec(align(8)) [[using gnu: packed, aligned(8)]] Vec { \
alignas /* c */ (ns::Type) int a; \
__attribute__((section("x"), hot, cold)) int b; \
__declspec(deprecated) int c; \
alignas(8) int d; \
__declspec(property(get = getX)) int e; \
public: \
Vec() {} ; \
~Vec() {} ; \
Vec /* c */ & /* c */ operator /* c */ = /* c */ (Vec&&) {} ; \
operator int /* c */ && /* c */ () const; \
operator Thing &() const; \
}; \
namespace ns {} ; \
class Tail {} \
; \
union U { int a; } ; \
enum class E { a } ; \
void after() {} ; \
int /* c */ (*fp)(ns::Type<int>, struct Point*, int) /* c */ = nullptr; \
int (* /* c */ & /* c */ fref)(int) = f; \
int (* && fpo)(int) = nullptr; \
struct ns /* c */ ::Type< /* c */ int>::inner (*fp2)(int); \
void takes(int /* c */ (*cb)(int) /* c */ , struct ns::Thing (&& cb2)(int), ns:: /* c */ Type<int> (*cb3)(int)); \
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
ns:: /* c */ Thing /* c */ && dangling; \
[[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ * /* c */ * /* c */ & (*callback)(int); \
x = y */ z; \
for ( /* c */ const /* c */ auto /* c */ & [first, second] : pairs) {} \
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
auto lam = [](int arr[3], const /* c */ int /* c */ , register long = 3) {}; \
} \
using Alias [[deprecated]] = const /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ * /* c */ [3]; \
error::error() : code(1) {} \
error::error(const error&) = /* c */ delete; \
error::~error() {} \
error::~error() = /* c */ default; \
line::line(int) = default; \
line::~line() = delete; \
::ns:: /* c */ Type<int> /* c */ && make_thing() {} \
auto trailing() -> /* c */ struct /* c */ ::ns:: /* c */ Type<int> {} \
void takes_callback(struct /* c */ ::ns::Holder<int>::Item /* c */ * /* c */ * /* c */ & (*cb)(int)); \
void unnamed(const /* c */ int /* c */ , register long = 3) {} \
void with_default(Point p = :: /* c */ ns::Holder<int>::Point {.x = 1, .y = 2}) {} \
struct /* c */ ::ns:: /* c */ Vec<int> && operator+(const Vec&, const Vec&); \
namespace /* c */ outer::inner::deep {} \
namespace a::b::c {} \
template < /* c */ class T, /* c */ typename U> \
void two_params(T, U); \
int ( /* c */ *with_array)(int arr[3], const int, register long = 3); \
struct Derived2 : [[deprecated]] /* c */ public /* c */ ns::Base<int>:: /* c */ Inner {}; \
struct alignas(ns::Type) /* c */ alignas(8) Aligned {}; \
struct alignas((8)) Aligned2 {}; \
void attributes() { \
__declspec(property(get = getX, put = putX)) /* c */ int a; \
[[using gnu: always_inline, hot]] int b; \
__attribute__((section("x"), hot)) int c; \
} \
struct Foo { __cdecl Foo(); }; \
struct Foo { /* c */ Foo& __fastcall /* c */ operator+(int); }; \
struct Foo { int operator""_km(unsigned long long); }; \
void __cdecl /* c */ g() {} \
[[nodiscard]] /* c */ void k() {} \
enum E2 : ::ns::T< /* c */ int>::U { B }; \
namespace d = ns::Foo< /* c */ int>::e; \
extern "C" { int z; }; \
int (*gp[3])(int) const; \
void construct_objects() { Thing fooBar{1}; } \
void method_chains() { x = this->f(); x = fooBar.f(); x = FOO_BAR.f(); x = obj.fooBar.f(); x = obj.FOO_BAR.f(); x = obj.thing.f(); } \
void hex_exponents() { double n1 = 0x1p+3; double n2 = 0x1p-3; double n3 = 0x1.8p3f; int n4 = 0x1p3_km; } \
void suffixes() { double n5 = 1.5e+3; int n6 = 1.5_km; int n7 = 12_km; int n8 = 0b101_km; int n9 = 0x1F_km; } \
void float_exponents() { double n10 = 0x1.8p+3; double n11 = 0x1.8p-3; double n12 = 1e+5; } \
void range_attribute() { for ([[maybe_unused]] auto& v : vs) {} } \
/* === banner === */ \
void template_numbers() { f<0x1.8p+3, 0x1.8p-3, 0x1.8p3f, 0x1.8p3_km, 1.5e+3, 1.5e-3, 1.5f, 1.5_km, 0b1_km, 0b1u, 07_km, 07u, 0x1Fu, 0x1F_km, 0x1p+3, 0x1p-3, 1e+3, 1e-3, 12u, 12_km>(x); } \
void literal_suffixes() { auto s1 = "abc"sv; auto s2 = 'a'sv; auto s3 = 'a'_x; } \
typedef struct [[deprecated]] /* c */ S : Base { int x; } S_t; \
typedef class [[deprecated]] /* c */ C : Base { int x; } C_t; \
typedef union [[deprecated]] /* c */ U { int x; } U_t; \
typedef struct S2 { int x; }; \
union [[deprecated]] /* c */ U2 : Base { int x; }; \
using namespace ns::inner< /* c */ int>::deep; \
using namespace ::ns::inner; \
extern template class TT<int>; \
[[nodiscard]] /* c */ Foo operator+(Foo a, Foo b); \
[[nodiscard]] Foo ns::Thing<int>::operator+(Foo a) {} \
class Less : public Base<0x1.8p+3 < 1, 0x1.8p-3 < 1, 0x1.8p3f < 1, 0x1.8p3_km < 1, 1.5e+3 < 1, 1.5e-3 < 1, 1.5f < 1, 1.5_km < 1, 0b1u < 1, 0b1_km < 1, 07u < 1, 07_km < 1, 0x1p+3 < 1, 0x1p-3 < 1, 0x1Fu < 1, 0x1F_km < 1, 1e+3 < 1, 1e-3 < 1, 12u < 1, 12_km < 1> {}; \
class Shift : public Base<0x1.8p+3 << 1, 0x1.8p-3 << 1, 0x1.8p3f << 1, 0x1.8p3_km << 1, 1.5e+3 << 1, 1.5e-3 << 1, 1.5f << 1, 1.5_km << 1, 0b1u << 1, 0b1_km << 1, 07u << 1, 07_km << 1, 0x1p+3 << 1, 0x1p-3 << 1, 0x1Fu << 1, 0x1F_km << 1, 1e+3 << 1, 1e-3 << 1, 12u << 1, 12_km << 1> {}; \
void fpp(int (*cb[3])(int)); \
struct Foo { operator /* c */ const char*(); }; \
int operator"" /* c */ _mi(unsigned long long); \
static_assert /* c */ (true, "m"); \
void noexcept_gap() { bool b = noexcept /* c */ (f()); } \
void decltype_gap() { decltype /* c */ (x) y = x; } \
void upper_calls() { MAKE /* c */ <int>(x); ns::MAKE<int>(x); ::MAKE(x); } \
void attr_assign() { \
[[maybe_unused]] /* c */ int v = 1; \
} \
void static_decl() { \
static /* c */ int z1; \
} \
template /* c */ void tf(int); \
unsigned /* c */ int uf() {} \
struct Ctors { \
Ctors() = /* c */ default; \
__cdecl /* c */ Ctors(int); \
__cdecl /* c */ ~Ctors(); \
constexpr /* c */ Ctors(char) {} \
}; \
void gnu_param(__attribute__((unused)) /* c */ int (*cb)(int)); \
void fp_specs() { \
int (*fpc)(int) const /* c */ noexcept; \
} \
void chains() { \
x = obj.fooBar.y; \
x = obj.FOO_BAR.y; \
x = obj.Mixed_ok.y; \
x = obj.fooBar.f(); \
x = obj.FOO_BAR.f(); \
} \
void name_chains() { \
x = obj.fooBar.y; \
x = obj.FOO.y; \
x = obj.Foo.y; \
x = obj.FOO.f(); \
x = FOO.f(); \
} \
namespace d2 = ::a::B<int>::C<int>::e; \
using namespace ::a::B<int>::C<int>::d; \
template <typename T> void a::B<T>::C<T>::g() {} \
void arrays_empty() { \
int ARR[] = {1}; \
int Foo[] = {1}; \
} \
typedef class C2 { int x; }; \
typedef union U3 { int x; }; \
typedef union U4 : Base { int x; } U4_t; \
struct Del { \
~Del() = delete; \
}; \
void scoped_param_comments(a::B</* c */ int>::C<int>::T p); \
void call_comments() { \
f</* c */ int>(x); \
MAKE</* c */ int>(x); \
} \
enum class[[deprecated]] Attr1 : int { A1 }; \
enum Attr3 { \
C1 [[maybe_unused]] = 1, \
C2 [[maybe_unused]], \
}; \
enum E5 : a::B<int>::C<int>::D { X5 }; \
static __attribute__((used)) int gnu_fn() {} \
auto trail2() -> a::B<int>::C<int>::D; \
a::B<int>::C<int> ret_scoped() {} \
void scoped_decl() { \
a::B<int>::C<int> x; \
} \
a::B<int>::C<int> operator%(a::B<int> l, int r); \
void ranged() { \
for (__attribute__((unused)) auto [a, b] : m) {} \
} \
void brace_upper() { \
Thing FOO{1}; \
} \
void scoped_upper_call() { \
x = a::B<int>::MAKE(1); \
} \
template <typename T = a /* c */ ::B<int>::C<int>::D> struct TD; \
template <typename T = a::B</* c */ int>::C<int>::D> struct TD2; \
void tcall() { \
x = f<a::B /* c */ <int>::C<int>::D>(1); \
x = f<a::B</* c */ int>::C<int>::D>(1); \
} \
struct Lead { \
/* c */ ~Lead(); \
/* c */ static_assert(true); \
/* c */ int m = 1; \
/* c */ int n; \
}; \
void asm_raw() { \
asm(R"(nop)"); \
} \
struct InitT { \
InitT() : x</* c */ int>(1) {} \
}; \
void spec_loop() { \
static /* c */ const int q; \
} \
template <template <typename T> class TT> struct TT3; \
void macro_names() { \
int snake_case = 1; \
int camelCase = 2; \
int UPPER = 3; \
int Plain = 4; \
} \
void macro_decls() { \
int snake_d; \
int camelD; \
int UPPER_D; \
int Plain_d; \
} \
namespace  g::inline h { \
} \
void macro_literals() { \
auto s4 = "abc"_x; \
char cx = '\x4142'; \
} \
struct DelOps { \
DelOps() = delete; \
DelOps& operator=(const DelOps&) = delete; \
}; \
void asm_volatile() { \
asm volatile ("nop"); \
} \
void decl_names() { \
int UPPERD; \
int Plaind; \
} \
void unknown_chain() { \
x = obj.Foo.f(); \
} \
ns::B<int>::Widget /* c */ ::Widget(int a) : x{a} {} \
ns::B<int>::Widget /* c */ ::~Widget() {} \
struct Conv { \
operator ns::Other<int>::Inner(); \
}; \
void type_calls() { \
x = uint_least32_t (1) + pthread_mutex_t () + foo_t (); \
} \
void unnamed_types(const uint_least32_t, const pthread_mutex_t, const foo_t, const Widget); \
auto lam_types = [](const uint_least32_t, const pthread_mutex_t, const foo_t, const Widget) {}; \
auto lam_defaults = [](int a = 1, int b = 2) {}; \
struct PostSpecs { \
bool operator==(const PostSpecs&) const /* c */ noexcept; \
~PostSpecs() /* c */ noexcept /* c */ override; \
};void macro_basics(int x) { \
switch (x) { case 1: break; /* c */ default: break; } \
switch /* c */ (x) {} \
char ch = '\x41'; \
char cu = u8'\101'; \
const char* esc = "\u00e9\U0001F600\101\x41\x4142\q"; \
auto s1 = sizeof... /* c */ (Args); \
auto t1 = typeid /* c */ (x); \
auto a1 = alignof /* c */ (int); \
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
[[maybe_unused]] struct /* c */ ns::Item & ref; \
asm ("mov %[out], %[in]" : [ /* c */ out /* c */ ] "=r" (y) : [in] "r" (x)); \
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
Op& operator=(const Op&) = /* c */ default; \
~Op() = /* c */ default; \
Op() : a{1}, b(2) {} \
int a, b; \
}; \
typedef class /* c */ DLLEXPORT final /* c */ EXPORTED Widget /* c */ final /* c */ { \
int x; \
} /* c */ *WidgetPtr, &WidgetRef, && /* c */ &WidgetOdd, WidgetValue; \
typedef union /* c */ DLLEXPORT final /* c */ EXPORTED Bits /* c */ final /* c */ { \
int i; \
float f; \
} /* c */ *BitsPtr, &BitsRef, && /* c */ &BitsOdd, BitsValue; \
typedef struct /* c */ DLLEXPORT final /* c */ EXPORTED Pair /* c */ final /* c */ { \
int a; \
} /* c */ * /* c */ PairPtr, &PairRef, && /* c */ &PairOdd, PairValue; \
union DLLEXPORT final /* c */ EXPORTED Cell final { int v; }; \
struct DLLEXPORT final /* c */ EXPORTED Row final { int v; }; \
class /* c */ alignas(16) __declspec(dllexport, align(8)) [[gnu::packed, using ns: attr]] Vec { \
alignas(ns::Type, 8) int a; \
__attribute__((section("x"), using ns: hot, ns::cold)) int b; \
__declspec(using ns: x, ns::attr) int c; \
alignas(using ns: x) int d; \
__declspec(property(get = getX)) int e; \
public: \
Vec() {} ; \
~Vec() {} ; \
Vec& operator=(Vec&& /* c */ &) {} ; \
operator int&& /* c */ &() const; \
operator Thing &() const; \
}; \
void params(register int a, const struct Thing b, int arr[3], int&& /* c */ & c, struct Q q); \
using Alias [[deprecated]] = typename /* c */ const [[maybe_unused]] /* c */ struct /* c */ ::ns::Type<int>:: /* c */ Inner /* c */ * /* c */ && /* c */ & /* c */ [3]; \
::ns::Type<int> && /* c */ & make_thing() {} \
void takes_callback(struct ::ns:: /* c */ Holder<int>::Item & * && /* c */ & (*cb)(int)); \
void with_default(Point p = struct /* c */ ::ns::Holder<int>:: /* c */ Point /* c */ {.x = 1, .y = 2}) {} \
struct ::ns::Vec<int> && /* c */ & operator+(const Vec&, const Vec&); \
struct Derived2 : public [[deprecated]] /* c */ struct /* c */ ns::Base<int>:: /* c */ Inner {}; \
struct alignas(using ns: x, ns::Type, name, (8)) Aligned {}; \
void attributes() { \
f(__declspec(property(get = g), using ns: a, ns::b) 1); \
h([[using gnu: always_inline, hot]] 1); \
k(__attribute__((section("x"), ns::attr, using ns: hot)) 1); \
} \
void not_cpp() { \
if (struct /* c */ Point /* c */ & /* c */ * /* c */ && /* c */ & /* c */ p /* c */ [ /* c */ ] /* c */) {} \
if (union /* c */ Value /* c */ & /* c */ * /* c */ && /* c */ & /* c */ v /* c */ [ /* c */ ] /* c */) {} \
if (enum /* c */ Color /* c */ & /* c */ * /* c */ && /* c */ & /* c */ c /* c */ [ /* c */ ] /* c */) {} \
if (class /* c */ Shape /* c */ & /* c */ * /* c */ && /* c */ & /* c */ s /* c */ [ /* c */ ] /* c */) {} \
struct Point && /* c */ & p2 = q; \
union Value && /* c */ & v2 = w; \
enum Color && /* c */ & c2 = d; \
class Shape && /* c */ & s2 = t; \
thread_local struct Thing && /* c */ & odd = value; \
int first = 1, static /* c */ const /* c */ [[maybe_unused]] /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ && /* c */ & /* c */ second /* c */ = value; \
__attribute__((used)) /* c */ struct /* c */ ::ns::Holder<int>:: /* c */ Item /* c */ & /* c */ * /* c */ && /* c */ & /* c */ items[] /* c */ = {}; \
ns::Thing && /* c */ & dangling; \
[[maybe_unused]] struct ::ns::Holder<int>::Item & * && /* c */ & (*callback)(int); \
for (const ::ns::Pair<int>:: /* c */ Inner * /* c */ && /* c */ & [first, second] : pairs) {} \
for (const ::ns::Pair<int>::Inner * && /* c */ & item : pairs) {} \
for (struct /* c */ ns::Pair [first, second] : pairs) {} \
x = 5; typename /* c */ [[nodiscard]] /* c */ struct /* c */ ::outer::inner<int>:: /* c */ Name value; \
auto box = Box<[[maybe_unused]] /* c */ struct /* c */ ns::Bar<int>:: /* c */ Baz>(); \
auto a2 = alignas(8); \
} \
void this_chains() { \
x = obj.this.y; \
x = obj.this.f(); \
} \
void typed_compound() { \
int y += 1; \
int z &= 1; \
} \
auto trail() -> [[x]] /* c */ int; \
namespace a::b< /* c */ int>::c<int>::d {} \
void typed_continuation() { \
int a = 1, int snake_case = 2, int camelCase = 3, int UPPER = 4, int Plain = 5; \
int b = 1, int c += 2, int d &= 3; \
} \
void notcpp_arrays() { \
int arr[] += {1}; \
int arr2[] &= {1}; \
} \
struct ConvArr { \
operator Foo[](); \
}; \
void curly_param(__attribute__((x)) /* c */ Thing{1}) {} \
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

#define LEAD_LINE \
/* c */ #line 43 "coverage.cpp"
#define LEAD_PRAGMA \
/* c */ #pragma once
#define LEAD_WARNING \
/* c */ #warning lead
#define LEAD_IF \
/* c */ #if 1
#define LEAD_DEFINE \
/* c */ #define INNER2 1
#define LEAD_CTOR \
struct S { \
/* c */ S(); }
#define FP_SPECS void fpp_specs(int (*cb)(int) const /* c */ noexcept
// Constructs that leave Shiki inside a rule until the end of the file, so they come last.

void fpp_specs(int (*cb)(int) const /* c */ noexcept
);
template <int N = (1) < /* c */ 2> struct RelT;
