L See LaTeX run.  Run latexrun.
T 29 text.html.markdown meta.paragraph.markdown
L 
L latexrun fits LaTeX into a modern build environment.  It hides LaTeX's
T 70 text.html.markdown meta.paragraph.markdown
L circular dependencies, surfaces errors in a standard and user-friendly
T 70 text.html.markdown meta.paragraph.markdown
L format, and generally enables other tools to do what they do best.
T 66 text.html.markdown meta.paragraph.markdown
L 
L 
L Features
T 8 text.html.markdown meta.paragraph.markdown
L ========
T 8 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L * Runs latex the right number of times.  LaTeX's iterative approach is
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   a poor match for build tools that expect to run a task once and be
T 2 text.html.markdown markup.list.unnumbered.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   done with it.  latexrun hides this complexity by running LaTeX (and
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   BibTeX) as many times as necessary and no more.  Only the results
T 2 text.html.markdown markup.list.unnumbered.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   from the final run are shown, making it act like a standard,
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   single-run build task.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 24 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Surfaces error messages and warnings.  LaTeX and related tools bury
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   errors and useful warnings in vast wastelands of output noise.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   latexrun prints only the messages that matter, in a format
T 2 text.html.markdown markup.list.unnumbered.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   understood by modern tools.  latexrun even figures out file names
T 2 text.html.markdown markup.list.unnumbered.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   and line numbers for many BibTeX errors that usually don't indicate
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   their source.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 15 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L         paper.tex:140: Overfull \hbox (15pt too wide)
T 4 text.html.markdown markup.list.unnumbered.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 53 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L         paper.tex:400: Reference `sec:eval' on page 5 undefined
T 4 text.html.markdown markup.list.unnumbered.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L         local.bib:230: empty booktitle in clements:commutativity
T 4 text.html.markdown markup.list.unnumbered.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Incremental progress reporting.  latexrun keeps you informed of
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   LaTeX's progress, without overwhelming you with output.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 57 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Cleaning.  LaTeX's output files are legion.  latexrun keeps track of
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   them and can clean them up for you.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 37 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Atomic commit.  latexrun updates output files atomically, which
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   means your PDF reader will no longer crash or complain about broken
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   xref tables when it catches latex in the middle of writing out a
T 2 text.html.markdown markup.list.unnumbered.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   PDF.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 6 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Easy {.git,.hg,svn:}ignore.  Just ignore `latex.out/`.  Done!
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 43 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 54 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 55 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 63 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Self-contained.  latexrun is a single, self-contained Python script
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   that can be copied in to your source tree so your collaborators
T 2 text.html.markdown markup.list.unnumbered.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   don't have to install it.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L 
L Non-features
T 12 text.html.markdown meta.paragraph.markdown
L ------------
T 12 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L Kitchen sink not included.  latexrun is not a build system.  It will
T 68 text.html.markdown meta.paragraph.markdown
L not convert your graphics behind your back.  It will not continuously
T 69 text.html.markdown meta.paragraph.markdown
L monitor your files for changes.  It will not start your previewer for
T 69 text.html.markdown meta.paragraph.markdown
L you.  latexrun is designed to be *part* of your build system and let
T 33 text.html.markdown meta.paragraph.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 68 text.html.markdown meta.paragraph.markdown
L other tools do what they do well.
T 33 text.html.markdown meta.paragraph.markdown
L 
L 
L Integrating with make
T 21 text.html.markdown meta.paragraph.markdown
L =====================
T 21 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L latexrun does its own dependency tracking (at a finer granularity than
T 70 text.html.markdown meta.paragraph.markdown
L `make`).  Since it also does nothing if no dependencies have changed,
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 69 text.html.markdown meta.paragraph.markdown
L it's easy to integrate with `make` using phony targets.  Here's a
T 28 text.html.markdown meta.paragraph.markdown
T 29 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 33 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 34 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 65 text.html.markdown meta.paragraph.markdown
L complete example:
T 17 text.html.markdown meta.paragraph.markdown
L 
L ```Makefile
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 11 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language.markdown
L .PHONY: FORCE
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile support.function.target.PHONY.makefile
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile punctuation.separator.key-value.makefile
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile meta.scope.prerequisites.makefile
L paper.pdf: FORCE {files that need to be generated, if any}
T 9 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile entity.name.function.target.makefile
T 10 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile punctuation.separator.key-value.makefile
T 58 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile meta.scope.prerequisites.makefile
L 	latexrun paper.tex
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
T 18 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
T 19 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
L 
L .PHONY: clean
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile support.function.target.PHONY.makefile
T 7 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile punctuation.separator.key-value.makefile
T 13 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile meta.scope.prerequisites.makefile
L clean:
T 5 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile entity.name.function.target.makefile
T 6 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.target.makefile punctuation.separator.key-value.makefile
L 	latexrun --clean-all
T 1 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
T 20 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
T 21 text.html.markdown markup.fenced_code.block.markdown meta.embedded.block.makefile meta.scope.recipe.makefile
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L 
L Note that `paper.pdf` depends on a phony target, but is not itself
T 10 text.html.markdown meta.paragraph.markdown
T 11 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 20 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 21 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown
L phony, since this would cause `make` to consider anything that
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown
L *depended* on `paper.pdf` to always be out of date.  Instead, `make`
T 1 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 9 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 10 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 14 text.html.markdown meta.paragraph.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 24 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 25 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 62 text.html.markdown meta.paragraph.markdown
T 63 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L only considers targets that depend on `paper.pdf` out of date if
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown meta.paragraph.markdown
L latexrun actually updates `paper.pdf`.
T 26 text.html.markdown meta.paragraph.markdown
T 27 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 36 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 38 text.html.markdown meta.paragraph.markdown
L 
L 
L Comparison with other tools
T 27 text.html.markdown meta.paragraph.markdown
L ===========================
T 27 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L Several other tools achieve the same basic goal as latexrun: to
T 63 text.html.markdown meta.paragraph.markdown
L automatically run LaTeX and BibTeX the right number of times.
T 61 text.html.markdown meta.paragraph.markdown
L 
L Rubber
T 6 text.html.markdown meta.paragraph.markdown
L ------
T 6 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L Rubber inspired many features of latexrun.  Rubber showed that it's
T 67 text.html.markdown meta.paragraph.markdown
L possible to abstract away many of LaTeX's eccentricities like
T 61 text.html.markdown meta.paragraph.markdown
L iterative builds and user-hostile logs.  latexrun is a response to
T 66 text.html.markdown meta.paragraph.markdown
L Rubber: it attempts to capture its best features while further
T 62 text.html.markdown meta.paragraph.markdown
L modernizing the LaTeX build process and adhering to the Unix
T 60 text.html.markdown meta.paragraph.markdown
L philosophy of "do one thing well."
T 34 text.html.markdown meta.paragraph.markdown
L 
L latexrun has more user-friendly and modern defaults than Rubber.  It
T 68 text.html.markdown meta.paragraph.markdown
L defaults to reporting important warnings like unknown references,
T 65 text.html.markdown meta.paragraph.markdown
L rather than suppressing all warnings.  It defaults to PDF output,
T 65 text.html.markdown meta.paragraph.markdown
L rather than DVI output.  It defaults to putting all intermediate
T 64 text.html.markdown meta.paragraph.markdown
L output in a directory, rather than inheriting LaTeX's default behavior
T 70 text.html.markdown meta.paragraph.markdown
L of filling your source tree with intermediate files.  It also has
T 65 text.html.markdown meta.paragraph.markdown
L features that interact better with modern tools, like atomic commit
T 67 text.html.markdown meta.paragraph.markdown
L for PDF viewers that use file system notification to automatically
T 66 text.html.markdown meta.paragraph.markdown
L refresh.
T 8 text.html.markdown meta.paragraph.markdown
L 
L latexrun has robust post-execution dependency analysis, while Rubber
T 68 text.html.markdown meta.paragraph.markdown
L works by parsing your TeX code.  TeX is, famously, the only thing that
T 70 text.html.markdown meta.paragraph.markdown
L can correctly parse TeX code and as a result Rubber's dependency
T 64 text.html.markdown meta.paragraph.markdown
L analysis is fragile.  Simple things like
T 40 text.html.markdown meta.paragraph.markdown
L ```latex
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
T 8 text.html.markdown markup.fenced_code.block.markdown fenced_code.block.language
L     \newcommand{\includegraph}[1]{\includegraphics{graph/#1.pdf}}
T 65 text.html.markdown markup.fenced_code.block.markdown
L     \includegraph{results}
T 26 text.html.markdown markup.fenced_code.block.markdown
L ```
T 3 text.html.markdown markup.fenced_code.block.markdown punctuation.definition.markdown
L break Rubber's dependency analysis.
T 35 text.html.markdown meta.paragraph.markdown
L 
L latexrun is simple.  It's intentionally a single, self-contained, and
T 69 text.html.markdown meta.paragraph.markdown
L reasonably short Python script that you can drop in next to your LaTeX
T 70 text.html.markdown meta.paragraph.markdown
L sources so your collaborators don't have to install it.  Rubber is
T 66 text.html.markdown meta.paragraph.markdown
L modular, which is admirable but results in a non-zero setup cost.
T 65 text.html.markdown meta.paragraph.markdown
L 
L latexrun fits in to your toolchain, while Rubber tries to replace it.
T 69 text.html.markdown meta.paragraph.markdown
L Rubber goes beyond building LaTeX code with "features" like automatic
T 69 text.html.markdown meta.paragraph.markdown
L graphics conversion based on a weighted shortest-path walk through
T 66 text.html.markdown meta.paragraph.markdown
L known converters.  This makes its build process unpredictable and
T 65 text.html.markdown meta.paragraph.markdown
L means that other build tools like `make` integrate poorly with it.
T 34 text.html.markdown meta.paragraph.markdown
T 35 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 40 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown
L latexrun prefers explicit over implicit, and tries to make latex act
T 68 text.html.markdown meta.paragraph.markdown
L like a modern tool in your toolchain to enable other tools to do what
T 69 text.html.markdown meta.paragraph.markdown
L *they* do best.
T 1 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 5 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 6 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 15 text.html.markdown meta.paragraph.markdown
L 
L latexmk
T 7 text.html.markdown meta.paragraph.markdown
L -------
T 7 text.html.markdown meta.paragraph.markdown markup.heading.setext.2.markdown
L 
L latexmk is a venerable LaTeX wrapper that has amassed an impressive
T 67 text.html.markdown meta.paragraph.markdown
L array of options, flags, and features.  It includes many features that
T 70 text.html.markdown meta.paragraph.markdown
L latexrun leaves to other tools---like starting viewers, printing, and
T 69 text.html.markdown meta.paragraph.markdown
L conversion to PDF from DVI and PostScript---or simply omits---like
T 66 text.html.markdown meta.paragraph.markdown
L adding banners and automatically invoking `make` for missing files.
T 42 text.html.markdown meta.paragraph.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 47 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 48 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 67 text.html.markdown meta.paragraph.markdown
L 
L Like latexrun, latexmk can use the "file recorder" feature of modern
T 68 text.html.markdown meta.paragraph.markdown
L TeX engines to compute precise file dependencies.  However, latexrun
T 68 text.html.markdown meta.paragraph.markdown
L also tracks non-file dependencies like environment variables and knows
T 70 text.html.markdown meta.paragraph.markdown
L how to track filtered file contents (like only the lines that actually
T 70 text.html.markdown meta.paragraph.markdown
L matter to BibTeX).
T 18 text.html.markdown meta.paragraph.markdown
L 
L latexmk simply passes through LaTeX's log spew, while latexrun
T 62 text.html.markdown meta.paragraph.markdown
L extracts errors and warnings and displays them in a friendly, modern
T 68 text.html.markdown meta.paragraph.markdown
L format.
T 7 text.html.markdown meta.paragraph.markdown
L 
L 
L How latexrun works
T 18 text.html.markdown meta.paragraph.markdown
L ==================
T 18 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L latexrun views the world as a cyclic graph of deterministic functions
T 69 text.html.markdown meta.paragraph.markdown
L of system state:
T 16 text.html.markdown meta.paragraph.markdown
L 
L          .tex → ┌───────┐ ─────────────────────────→ .pdf
T 9 text.html.markdown meta.paragraph.markdown
T 129 text.html.markdown meta.paragraph.markdown
L     ╭─── .aux → │       │ → .aux ──╮
T 4 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown
L     │╭── .toc → │ latex │ → .toc ─╮│
T 4 text.html.markdown meta.paragraph.markdown
T 58 text.html.markdown meta.paragraph.markdown
L     ││╭─ .bbl → │       │ → .log  ││
T 4 text.html.markdown meta.paragraph.markdown
T 56 text.html.markdown meta.paragraph.markdown
L     │││     … → └───────┘ → …     ││
T 4 text.html.markdown meta.paragraph.markdown
T 72 text.html.markdown meta.paragraph.markdown
L     │╰────────────────────────────╯│
T 4 text.html.markdown meta.paragraph.markdown
T 100 text.html.markdown meta.paragraph.markdown
L     ╰──────────────────────────────┤
T 4 text.html.markdown meta.paragraph.markdown
T 100 text.html.markdown meta.paragraph.markdown
L       │                            ↓
T 6 text.html.markdown meta.paragraph.markdown
T 40 text.html.markdown meta.paragraph.markdown
L       │                 .bst → ┌────────┐ → .blg
T 6 text.html.markdown meta.paragraph.markdown
T 74 text.html.markdown meta.paragraph.markdown
L       │                 .bib → │ bibtex │ → .bbl ─╮
T 6 text.html.markdown meta.paragraph.markdown
T 65 text.html.markdown meta.paragraph.markdown
L       │                        └────────┘         │
T 6 text.html.markdown meta.paragraph.markdown
T 75 text.html.markdown meta.paragraph.markdown
L       ╰───────────────────────────────────────────╯
T 6 text.html.markdown meta.paragraph.markdown
T 141 text.html.markdown meta.paragraph.markdown
L 
L latexrun's goal is to find the *fixed-point* of this computation.  Put
T 31 text.html.markdown meta.paragraph.markdown
T 32 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 43 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 44 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 70 text.html.markdown meta.paragraph.markdown
L simply, it runs latex and bibtex repeatedly until it can guarantee
T 66 text.html.markdown meta.paragraph.markdown
L that additional runs will not change the output.
T 48 text.html.markdown meta.paragraph.markdown
L 
L A direct approach could be to check the output of latex and bibtex
T 66 text.html.markdown meta.paragraph.markdown
L after each run and stop when the output is the same from one run to
T 67 text.html.markdown meta.paragraph.markdown
L the next.  This works, but runs latex too many times; it will have
T 66 text.html.markdown meta.paragraph.markdown
L already produced its final output one iteration before this approach
T 68 text.html.markdown meta.paragraph.markdown
L can figure out that it's done.
T 30 text.html.markdown meta.paragraph.markdown
L 
L Instead, latexrun models latex and bibtex as deterministic functions
T 68 text.html.markdown meta.paragraph.markdown
L of the file system state, their command-line arguments, and certain
T 67 text.html.markdown meta.paragraph.markdown
L environment variables.  Hence, if the *inputs* to latex and bibtex do
T 38 text.html.markdown meta.paragraph.markdown
T 39 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 45 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 46 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 69 text.html.markdown meta.paragraph.markdown
L not change over one iteration, then latexrun can guarantee that the
T 67 text.html.markdown meta.paragraph.markdown
L output also will not change, and stop.
T 38 text.html.markdown meta.paragraph.markdown
L 
L 
L Known issues
T 12 text.html.markdown meta.paragraph.markdown
L ============
T 12 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L When a missing file is silently ignored by latex (e.g., `\openin`,
T 56 text.html.markdown meta.paragraph.markdown
T 57 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 64 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 65 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 66 text.html.markdown meta.paragraph.markdown
L `\IfFileExists`), latexrun can't track that the *lack* of that file
T 1 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 14 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 15 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 48 text.html.markdown meta.paragraph.markdown
T 49 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 53 text.html.markdown meta.paragraph.markdown markup.italic.markdown
T 54 text.html.markdown meta.paragraph.markdown markup.italic.markdown punctuation.definition.italic.markdown
T 67 text.html.markdown meta.paragraph.markdown
L was important.  Hence, if a missing file is later created, latexrun
T 67 text.html.markdown meta.paragraph.markdown
L may not re-run latex.  However, missing files that cause latex to
T 65 text.html.markdown meta.paragraph.markdown
L abort (e.g., missing files in `\input`) as well as missing `\include`
T 30 text.html.markdown meta.paragraph.markdown
T 31 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 37 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 38 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 59 text.html.markdown meta.paragraph.markdown
T 60 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 69 text.html.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
L files (which do not cause aborts) are handled.
T 46 text.html.markdown meta.paragraph.markdown
L 
L Command-line usage errors when calling latex appear before "This is
T 67 text.html.markdown meta.paragraph.markdown
L pdfTeX" and are not logged and therefore often not reported.
T 60 text.html.markdown meta.paragraph.markdown
L 
L 
L To do
T 5 text.html.markdown meta.paragraph.markdown
L =====
T 5 text.html.markdown meta.paragraph.markdown markup.heading.setext.1.markdown
L 
L * Solve the problem of missing input files.  LaTeX doesn't record
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 65 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   inputs that it couldn't read, so we don't know about them even
T 2 text.html.markdown markup.list.unnumbered.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   though they affect the computation (often seriously!).  Possible
T 2 text.html.markdown markup.list.unnumbered.markdown
T 66 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   solutions include ptrace, FUSE, and fanotify, but none of these are
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   portable or easily accessible from Python.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 44 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Provide a way to disable output filters for things that do their own
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 70 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   output parsing (e.g., AUCTeX).
T 2 text.html.markdown markup.list.unnumbered.markdown
T 32 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Integrate even better with `make`.  Phony rules are okay, but will
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 29 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 30 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 34 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 35 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   force `make` dependencies to be generated even if latexrun
T 2 text.html.markdown markup.list.unnumbered.markdown
T 8 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 9 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 13 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 14 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 60 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   ultimately does nothing.  Since latexrun's dependencies are
T 2 text.html.markdown markup.list.unnumbered.markdown
T 61 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   finer-grained than `make`'s, it might be necessary to shell out to
T 2 text.html.markdown markup.list.unnumbered.markdown
T 21 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 26 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown
T 27 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown markup.inline.raw.string.markdown punctuation.definition.raw.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   latexrun to do this.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 22 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Separate clean data by source file so you can clean a single input
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 68 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   file's outputs.
T 2 text.html.markdown markup.list.unnumbered.markdown
T 17 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Some important things like undefined references are only considered
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   warnings since they don't stop compilation.  Maybe distinguish
T 2 text.html.markdown markup.list.unnumbered.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   document-mangling warnings (almost but not quite errors) and format
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   warnings (can be ignored if justified).
T 2 text.html.markdown markup.list.unnumbered.markdown
T 41 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
L * Perhaps separate out the LaTeX output filter so it can be used on
T 1 text.html.markdown markup.list.unnumbered.markdown punctuation.definition.list.begin.markdown
T 2 text.html.markdown markup.list.unnumbered.markdown
T 67 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   its own as a pipe filter.  It could even be used on high-level
T 2 text.html.markdown markup.list.unnumbered.markdown
T 64 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   output, like from make, by acting as a pass-through until the "This
T 2 text.html.markdown markup.list.unnumbered.markdown
T 69 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   is (pdf)TeX" line up to the "Transcript written on" line (or
T 2 text.html.markdown markup.list.unnumbered.markdown
T 62 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L   anything else that can end TeX's log output).
T 2 text.html.markdown markup.list.unnumbered.markdown
T 47 text.html.markdown markup.list.unnumbered.markdown meta.paragraph.markdown
L 
