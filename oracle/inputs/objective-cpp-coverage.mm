// Written for nishiki: Objective-C++ that reaches the rules the grammar takes
// from C++, which the other inputs leave out. The rules it shares with
// Objective-C are reached by Objective-C's coverage input, which Objective-C++
// reads too.

#import <Foundation/Foundation.h>
#include <string>

using namespace std;
using namespace foo::bar::baz;
using  namespace detail ;
using namespace

namespace widgets {
class Widget;
}
namespace {
int hidden = 0;
}
namespace outer::inner {
}
namespace broken)
namespace other(1)
namespace last;
namespace
{
}

class Base {
public:
    explicit Base(int x);
    virtual ~Base();
    virtual void draw() const override final;
    friend class Helper;
    void stop() noexcept;
protected:
    int mValue;
    int fCount;
private:
    static constexpr int kSize = 4;
    mutable int cache;
    thread_local static int counter;
};

struct Point : public Base, private Mixin, protected Other {
    int x, y;
};

class Derived : public Base
{
};

struct Plain;
class Forward;
struct Anon {
} anon;
class Cut : public Base, protected
{
};

extern "C" {
void c_function(void);
}

extern "C" void single(void);

#ifdef __cplusplus
extern "C" {
#endif
void inside(void);
#ifdef __cplusplus
}
#endif

Base::Base(int x) : mValue(x), fCount(0) {
}

Base::~Base() {
}

Widget::~Widget() noexcept {}
~Plain();
else ~Skipped();
x = ~notDestructor();

template <typename T>
T maximum(T a, T b);

template <typename T, typename U = int, class... Ts>
struct Pack;

template<class T, int N, typename = void>
struct Array;

template < typename T , typename U >
struct Spaced;

template <typename... Args>
void forward(Args... args);

template <int N = 4, bool B = true>
struct Flags;

template
struct Explicit<int>;

void casts(Base *b) {
    auto d = dynamic_cast<Derived *>(b);
    auto s = static_cast<int>(1.5);
    auto r = reinterpret_cast<char *>(b);
    auto c = const_cast<const Base *>(b);
    if (this == nullptr) {
        throw std::runtime_error("x");
    }
    try {
        delete b;
        delete [] items;
        delete[] items;
        int *p = new int[4];
        int *q = new(place) int;
    } catch (const std::exception &e) {
    }
    bool t = a and b or not c;
    t = a bitand b bitor compl c xor d;
    t = a and_eq b or_eq c xor_eq d not_eq e;
    auto id = typeid(b);
    auto size = alignof(int);
    alignas(16) char buf[16];
    decltype(x) y = x;
    wchar_t w = L'w';
    char16_t c16 = u'c';
    char32_t c32 = U'c';
    typename T::value_type v;
    export module m;
}

void strings() {
    const char *a = "plain \"quoted\" \n \t \\ \? \a \b \f \r \v";
    const char *b = "octal \7 \17 \177 \1777";
    const char *c = "hex \x41 \xFFFF \x";
    const char *d = "universal é \U0001F600 \u12";
    const wchar_t *e = L"wide";
    const char16_t *f = u"sixteen";
    const char32_t *g = U"thirty-two";
    const char *h = u8"utf-8";
    const char *i = R"(raw (with parens) inside)";
    const char *j = R"xy(raw with "quotes" and )x" inside)xy";
    const char *k = LR"(wide raw)";
    const char *l = u8R"tag(utf-8 raw)tag";
    const char *m = uR"(sixteen raw)";
    const char *n = UR"(thirty-two raw)";
    const char *o = R"abcdefghijklmnopq(too long a delimiter)abcdefghijklmnopq";
    const char *p = R"abcdefghijklmnop(sixteen exactly)abcdefghijklmnop";
    const char *q = R"multi(
line one
line two
)multi";
    const char *r = R"é(non-ASCII delimiter)é";
}

std::vector<std::string> names;
::std::string global;
a::b::c::d(1);
Outer::Inner::method();
x::y;

// Objective-C and C forms the C++ rules leave to the rules after them.
NSGradient *g = [[NSGradient alloc] init];
NSMapTable *t;
x = NSGarbageCollector;
API_AVAILABLE(macos(10.15)) void f(void);
- (void)f API_AVAILABLE(ios(13.0));
[self foo:1 bar:2];
[NSPredicate predicateWithFormat:@"a CONTAINS %@ AND b IN %@", x, y];
[NSPredicate predicateWithFormat:@"ANY a == NIL" argumentArray:@[x]];
id y = [x bar];
printf("%d %s %5.2f %% \q", 1, "s", 2.0);
hypotf(1, 2);
  sqrt(4);
return memcpy(a, b, n);
int f(int a, (int) b) {
}
#else
#endif
#elif X
#define BLOCK { a; }
#define CALL(a) f((a), (b))
#define CALL2(a) (a) + \
  (b)
int n = 0x1.8p3f;
float m = 1.5e3f;
long q = 017L;
int r = 1'000u;
namespace ns {
Widget::Widget() : mValue(1), fCount(2) {
}
Foo::Foo(int x)
    : x(x) {
}
}
class Klass {
}   
int after;
x = 1e-3;
y = 2.5E-10f;
#define S "abc"
#define T "a\
char c = '\q';
char d = '\\x41';
#define FMT "%d %5.2f %% x"
#define F(a) { a; }
#define G(a) f(a) { b; }
#define H(a) if (a) { b; } else { c; }
#define P "100%"
#define NEST { { a; } }
#define NEST2 do { if (x) { y; } } while (0)
