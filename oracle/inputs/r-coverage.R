# R written for nishiki's oracle, to reach every expression of the grammar.
library(dplyr)
require("stats")

#' Add two numbers
#'
#' Uses **bold**, *italic*, __bold words__, _italic words_ and `code`.
#' Links: [stats::median()], [base:::sum], [mean()], [print] and <https://r-project.org>.
#' Escapes: @@ and \code{x} and \eqn{a + b}.
#' @param x,y Numbers to add.
#' @field value A field.
#' @slot data A slot.
#' @return The sum.
#' @export
#' ```r
#' add(1, 2)
#' ```
#' @examples
#' add(1, 2)
#' if (TRUE) { add(x = 1, y = 2) } [1] \alpha @seealso
#' @examplesIf interactive()
#' add(3, 4)
add <- function(x, y = 1, ...) {
  x + y
}

f <- function (a, b) NULL
g <- \(x) x + 1
h <- function() invisible()

x <- c(1L, 2.5, .5, 1e10, 1e-3, 2i, 0x1F, 0xFFp2L, 0x10i, 1.e5)
y <- TRUE && FALSE || NA
z <- list(NULL, Inf, NaN, NA_integer_, NA_real_, NA_complex_, NA_character_)

s1 <- 'single \' quote \\ \n \t \x41 \101 é \U{1F600} \q'
s2 <- "double \" quote \a \b \e \f \r \v \` \1"
r1 <- r"(raw (string))"
r2 <- R"--[raw ]-- string]--"
r3 <- r'{raw '' string}'
r4 <- R'-(raw)-'
r5 <- r"[raw]"
r6 <- r'[raw]'
r7 <- r"{raw}"
r8 <- R'(raw)'
`odd name` <- 1
`with \` tick` <- 2

if (x > 1) {
  print("yes")
} else if (x == 2) {
  print("no")
} else {
  next
}
for (i in seq_len(10)) break
while (FALSE) repeat {}

stats::median(x)
base:::sum(x)
pkg.name::fun.name(arg = 1, other=2)
df[x == 1, col = 2]
df[[1]]

a <- 1; a <<- 2; 3 -> b; 4 ->> d; e = 5
x %in% y; x %>% f(); x %% 2; x %/% 2
x |> f(); \(x) x; function(x) x => 1
x := 1; x != 2; x >= 3; x <= 4; x == 5
x ** 2; x * 3; x / 4; x ^ 5; !x; -x; +x
~ x; x ? y; x@slot; x$name; x & y; x | y
x : y; x < y; x > y

setGroupGeneric("g"); setRefClass("C"); setGeneric("f"); NextMethod()
setMethod("f", "C", identity); UseMethod("f"); tryCatch(stop("e"), error = identity)
setClass("C"); warning("w"); R6Class("C"); return(1); switch(x, a = 1)
attach(df); detach(df); source("f.R"); try(1)
