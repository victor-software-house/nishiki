// Objective-C that nishiki's other oracle inputs leave out, written to
// exercise the rest of Shiki's `objective-c` grammar.
/* = Banner block = */
// = Banner line =
	// a comment after a tab
#pragma mark - Section
#pragma mark Helpers

#import <Foundation/Foundation.h>
#include "header.h"
#include_next <next.h>
#error "an error string" with 'quotes'
#warning 'a warning'
#line 42 "coverage.m"
#undef FOO
#pragma clang diagnostic push
API_AVAILABLE(macos(10.15), ios(13.0))
int top_array[];

#if 0
int disabled_at_top;
#endif
#ifdef SOMETHING
int other_at_top;
#endif

@interface Shape : NSObject <NSCopying, NSCoding>
#if 1
- (void)enabledInInterface;
#else
- (void)elseInInterface;
#ifdef NESTED_IN_ELSE
#endif
#pragma mark - In else
#endif
#if 0
- (void)disabledInInterface;
#if NESTED_IN_DISABLED
#endif
#pragma mark In disabled
#else
- (void)enabledAfterDisabled;
#endif
#ifdef FEATURE
- (void)otherInInterface;
#endif
#pragma mark - Interface section
@property (nonatomic, strong, nullable, getter=isShown) NSString *name;
@property (class, readonly) id shared;
@property BOOL flag;
@required
- (id)initWithFrame:(CGRect)frame;
@optional
+ (instancetype)shape;
- (void)a:(int)a b:(NSString *)b;
- (void)commented /* block comment */ // line comment \
    continued;
@end

@interface Shape () {
@public
    int a;
@protected
    int b;
@private
    int c;
@package
    int d;
}
@end

@protocol Drawing <NSObject>
@end

@implementation Shape : NSObject
#if 1
- (void)enabledInImplementation {}
#else
- (void)elseInImplementation {}
#ifndef NESTED_IN_ELSE
#endif
#pragma mark - In implementation else
#endif
#if 0
- (void)disabledInImplementation {}
#ifdef NESTED_IN_DISABLED
#endif
#pragma mark In implementation disabled
#else
- (void)enabledAfterDisabledImplementation {}
#endif
#ifdef FEATURE
- (void)otherInImplementation {}
#endif
@synthesize name = _name;
@dynamic flag;

- (id)initWithFrame:(CGRect)frame {
    self = [super init];
    if (self) {
        SEL s = _cmd;
        Class k = [self class];
        id<NSCopying> c = nil;
        id <Drawing> d = Nil;
        BOOL yes = YES, no = NO;
        IBOutlet UIView *view;
        instancetype other;
        unichar u = 'a';
        IMP imp = NULL;
    }
    return self;
}

- (IBAction)tap:(id)sender {
    @try {
        @throw [NSException exceptionWithName:@"n" reason:@"r" userInfo:nil];
    } @catch (NSException *e) {
        NSLog(@"%@ %1$@ %-5.2@ %*@ %.*@", e, e, e, e, e);
    } @finally {
        @synchronized (self) {
            NS_DURING
            [NSApp terminate:nil];
            NS_HANDLER
            NS_ENDHANDLER
        }
    }
    @encode(int);
    @defs(Shape);
    @selector(initWithFrame:);
    @selector( tap: );
    @class Other;
    @protocol(Drawing);
}

- (void)cocoa {
    NSRect r = NSMakeRect(0, 0, 1, 1);
    NSRect q = NSRectFromCGRect(CGRectZero);
    id m = NSMakeCollectable(nil);
    CGFloat f = 1.0;
    CGPoint p;
    CGSize sz;
    CIImage *image;
    CALayer *layer;
    CAAnimation *animation;
    NSComparisonResult o = NSOrderedSame;
    NSUInteger nf = NSNotFound;
    NSString *n1 = NSWindowDidResizeNotification;
    NSString *n2 = NSMenuDidBeginTrackingNotification;
    NSString *n3 = NSViewDidUpdateTrackingAreasNotification;
    NSInteger t = NSRuleEditorRowTypeSimple;
    NSInteger g = NSGradientDrawsBeforeStartingLocation;
    NSInteger w = NSWindowAbove;
    NSString *s = NSStringFromClass([self class]);
    API_AVAILABLE(ios(13.0)) id x;
    NS_SWIFT_NAME(foo(bar:)) id y;
    NS_AVAILABLE_IOS(8_0) id z;
    NSPredicate *pr = [NSPredicate predicateWithFormat:@"name CONTAINS[cd] %@ AND age BETWEEN %@ OR NOT x IN %@", a, b, c];
    NSPredicate *p2 = [NSPredicate predicateWithFormat:@"ANY a == NULL OR ALL b != NIL OR SOME c > SELF OR NONE d < TRUE" argumentArray:@[]];
    NSPredicate *p3 = [NSPredicate predicateWithFormat:@"x MATCHES y CI \n \x41 \101 \q" arguments:args];
    NSPredicate *p4 = [NSPredicate predicateWithFormat:@"SUBQUERY(x, $y, $y.z == 1) OR CAST(1, 'NSDate') OR TRUEPREDICATE OR FALSEPREDICATE OR ANYKEY OR CASEINSENSITIVE OR BEGINSWITH OR ENDSWITH OR FIRST OR LAST OR SIZE OR YES OR NO OR FALSE"];
    [obj performSelector:@selector(foo) withObject:nil afterDelay:0];
    [self foo:helper(1) bar: other (2) baz:sqrt(4)];
    NSPredicate *p5 = [NSPredicate predicateWithFormat:@"a == %@" other:x];
    NSString *percent = @"50% \q";
}

@end

static inline int numbers(void) {
    int a = 0x1.8p3;
    double b = 0x.8p-2;
    double c = 0X1.p+4f;
    double c2 = 0x1'2.3'4p1'0;
    int d = 0'17;
    int e = 017;
    int e2 = 017ULL;
    int f = 0b1010;
    int g = 0B1'0'1u;
    int h = 1'000'000;
    int i = 0x12'34'56;
    int i2 = 0x1p1'0;
    int i3 = 0x1p+3L;
    int i4 = 0x1p-3;
    int j = 0x12zz;
    int k = 123abc;
    long n = 1e+5;
    long n2 = 1e1'0;
    double o = 1.5e-3;
    double o2 = 1'0.2'5e1'0;
    double o3 = 1.5e+3f;
    double o4 = .5;
    return a;
}

void strings(void) {
    printf("%d %5.2f %% 100% %lu %llx %hhd %zd %vhd %hld %,d %1$s %-+ #0d");
    printf("%" PRId64 "\n", x);
    printf("bad \q escape \y \777 é \U0001F600 \x4");
    char c = '\x41';
    char d = '\101';
    char e = '\'';
    const char *s = "\?\a\b\f\n\r\t\v\\\'\"";
    NSString *o = @"objc \" escape %@ \n";
}

int ops(int x) {
    int y = (x ? 1 : 2);
    int z = sizeof(int) + sizeof x;
    y++; --y; y += 1; y -= 1; y *= 2; y /= 2; y %= 3;
    y <<= 1; y >>= 1; y &= 1; y ^= 1; y |= 1;
    z = y << 1 >> 1;
    z = y != 1 && y <= 2 || y >= 3 || y == 4 || y < 5 || y > 6 || !y;
    z = y & 1 | 2 ^ 3;
    z = ~y;
    z = y + 1 - 2 * 3 / 4 % 5;
    int arr[ ] = { 1, 2 };
    int arr2[] = { 3 };
    x = arr[1];
    switch (x) {
    case 1:
        break;
    case kFoo:
        goto done;
    default:
        break;
    }
    switch(x) {}
    switch (x) {} ;
    x = (a : b);
    x = a->b->c;
    long ul = 10UL;
    printf("50%");
    switch (x) /* c */ {
    default:;
    }
    ;
done:
    return y;
}

int bits(int a, int b) {
    return a not_eq(b) bitand(a) or_eq(b) xor_eq(a);
}

void calls(void) {
    hypotf(1, 2);
    strlen ("x");
      sqrt(2.0);
    printf("x");
    memcpy(a, b, n);
    a::b(c);
    ::f(x);
    obj.method(1);
    obj.member.other(2);
    obj->field->size(3);
    obj.integer;
    obj.int8_t;
    a.b->c.d;
    [x y].z;
    f(x).w;
    (int)x;
    sizeof(pthread_t) + sizeof(u_char) + sizeof(int32_t);
}

typedef struct node_s {
    struct node_s *next;
} node_t;
enum /* c */ color { RED, GREEN };
union /* c */ value { int i; float f; };
typedef enum : NSInteger { A, B } E;

int printf(const char *fmt, ...);
void varargs(int a, ...);
static int sCount = 0;
int gGlobal = 1;
const int kLimit = 10;
extern NSString *const kName;
register volatile int rv;
restrict int *rp;
__block int bv;
u_char uc;
u_short us;
size_t sz;
ssize_t ssz;
pthread_mutex_t mutex;
pthread_t thread;
pthread_key_t key;
int8_t i8;
uint64_t u64;
int32_t i32;
uint_least16_t ul16;
Boolean flag;
OSStatus status = noErr;
OSType type = kNilOptions;
UInt32 ui = kInvalidID;
Float32 f32;
my_type_t mine;
_Bool bb;
_Complex double cd;
atomic_int ai;
memory_order mo;

#define FOO 1
#define BAR(a, b) ((a) + (b))
#define BAZ(a, ...) a
#define QUX(a ...) a
#define ALL(...) 1
#define $DOLLAR 1
#define STR(x) #x
#define CHR 'c'
#define ERR "unterminated
#define SQ 'x
#define BLOCK { int x; }
#define CALL f(x) + g(y)
#define LONG(a, b) \
    do { \
        a = b; \
    } while (0)
#define DEF(x) if (x) { \
    } else

#if defined(FOO) + 1
#endif
#if defined(FOO) && BAR || !defined BAZ
#endif
#if defined
#endif
#if defined(FOO) // trailing
#endif
#if defined FOO ? A : B
#endif
#if TRUE || FALSE || NULL || true || false
#endif

#if 1
int enabled_one;
#elif defined FOO
int enabled_elif;
#else
int enabled_else;
#endif

#if 1 // comment
int enabled_two;
#else
#if FOO
int nested;
#endif
#endif

#if 0
int disabled;
#ifdef NESTED
int nested_disabled;
#endif
#elif 1
int enabled_after_disabled;
#elif FOO
int other;
#else
int last;
#endif

#if 0
int disabled_again;
#elif 0
int disabled_elif;
#elif BAR
int bar;
#endif

#if FOO
#elif 1
int enabled_elif_line;
#else
int enabled_elif_else;
#endif

#if FOO
#elif 0
int disabled_elif_line;
#elif BAR
int bar_line;
#endif

void blocks(void) {
#if 1
    int inside_enabled;
#elif FOO
    int inside_elif;
#else
    int inside_else;
#endif
#if 0
    int inside_disabled;
#elif 1
    int inside_enabled_elif;
#else
    int inside_enabled_else;
#endif
#if 0
    int inside_disabled_two;
#elif 0
    int inside_disabled_elif;
#endif
#if FOO
    int generic_if;
#elif BAR
    int generic_elif;
#else
    int generic_else;
#endif
#if 1
    int one;
#elif 1
    int one_elif;
#elif BAR
    int elif_after_elif;
#endif
#if FOO
#elif 1
    int enabled_elif_in_block;
#elif BAR
    int generic_after_enabled_elif;
#endif
#if 0
    int zero;
#elif BAR
    int generic_after_zero;
#endif
}

void stray_directive(int x) {
    x = (1
#endif
    );
}

#if FOO
void split(void) {
#else
void split(int x) {
#endif
    return;
}
