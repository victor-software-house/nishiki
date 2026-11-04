// C that nishiki's other oracle inputs leave out, written to exercise the
// rest of Shiki's `c` grammar.

/**
 * @file coverage.c
 * @brief Exercises the C grammar's rarer rules.
 * @param[in] value the value, in \c units, see @ref other.
 * @param [in, out] buffer a buffer
 * \b bold \a arg \e emph \p param \c code
 * @code
 * int x = 1;
 * @endcode
 * @see other
 * @note A note. TODO: something
 * @deprecated
 * \n
 * @addtogroup group
 * @defgroup grp Group
 * @copydoc other
 * @warning careful
 */
/*! Short qt style comment @return nothing */
/** A one-liner with @a arg and \b bold. */
/*!< after a member */
/// A line doc with @brief and \param name
/// @param[out] out the result
/// \b strong @c code @p param \a arg
//! Another doc line with \return value and @todo later
//!< trailing member doc
// = Banner =
/* plain block */

#pragma mark - Section
#pragma mark Helpers

#line 42 "coverage.c"
#line 43

#include "header.h"
#include <stdio.h>
#error "an error string" with 'quotes'
#warning 'a warning'

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

#if defined BAR && (A ? B : C)
#endif

#define STR(x) #x
#define CHR 'c'
#define ERR "unterminated
#define SQ 'x

struct /* comment */ point /* another */ {
    int x : 4;
    int y : 4;
};
struct
point2 {
    int z;
};
struct /* lead */
named {
    int w;
};
typedef struct node_s {
    struct node_s *next;
} node_t;
enum /* c */ color { RED, GREEN };
union /* c */ value { int i; float f; };

int printf(const char *fmt, ...);
static int sCount = 0;
int gGlobal = 1;
const int kLimit = 10;
u_char uc;
pthread_mutex_t mutex;
pthread_t thread;
int32_t i32;
uint_least16_t ul16;
Boolean flag;
OSStatus status = noErr;
Float32 f32;

void numbers(void) {
    int a = 0x1.8p3;
    double b = 0x.8p-2;
    double c = 0X1.p+4f;
    int d = 0'17;
    int e = 017;
    int f = 0b1010;
    int g = 0B1'0'1;
    int h = 1'000'000;
    int i = 0x12'34'56;
    int j = 0x12zz;
    int k = 123abc;
    int l = 017ULL;
    int m = 0b11u;
    long n = 1e+5;
    double o = 1.5e-3;
}

void strings(void) {
    printf("%d %5.2f %% 100%");
    printf("%" PRId64 "\n", x);
    printf("bad \q escape \y");
    char c = '\x41';
    char d = '\101';
    char e = 'é';
    char f = '\U0001F600';
    char g = '\a';
    const char *s = "\?\a\b\f\n\r\t\v\\\'\"";
}

int bits(int a, int b) {
    return a not_eq(b) bitand compl(a) or not(b);
}

int ops(int x) {
    int y = (x ? 1 : 2);
    int z = x and y;
    switch (x) /* c */ {
    case 1 /* c */:
        return __FILE__[0] + __LINE__ + __MY_MACRO__;
    default /* c */:
        break;
    }
    switch /* c */ (x) {
    default:;
    }
    ;
    return y;
}

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
}

/** One line with @code, @param [in] x, \c y, @addtogroup g, @cite ref, TODO: and \callgraph @endcode */
/// Line with @code, @addtogroup grp, @anchor here, \callgraph and FIXME: soon @endcode

#if defined(FOO) + 1
#endif

 /* lead **/#define LEAD(a, b, ...) a
/* lead **/struct lead_s { int x; };

void more_blocks(int x) {
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
    x = 1; /* c **/switch (x) {
    case 0:
        break; /* c **/case 1:
        break; /* c **/default:
        break;
    } ;
    switch (x) {} ;
    switch /* c **/(x) {}
}

void numbers_more(void) {
    double a = 1'0.2'5e1'0;
    double b = 0x1'2.3'4p1'0;
    int c = 0x1p1'0;
    int d = 1e1'0;
}

void assembly(int x) {
    int y;
    __asm__ volatile ("mov %[out], %[in]\n\t\x41\101" : [out] "=r" (y) : [in] "r" (x) : "memory");
    __asm__ volatile /* c **/("nop" : [ /* c **/ name /* c **/ ] "=r" (y));
    asm
    /* c **/
    ("nop");
    size_t n = sizeof(pthread_t) + sizeof(u_char) + sizeof(int32_t);
}

#if FOO
void split(void) {
#else
void split(int x) {
#endif
    return;
}
