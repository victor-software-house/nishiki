-include deps/%.d
vpath %.c src lib
here := ${CURDIR}
kind := ${origin CC} $(flavor LD)
objects := ${patsubst %.c,%.o,${wildcard src/*.c}}
long := ${subst a,b,\
  $(sources)}
more := $(sources\
  )
brace := ${name\
  }
override define RECIPE = # the recipe
	@echo building $@
endef

all: $(objects)
	$(RECIPE)
