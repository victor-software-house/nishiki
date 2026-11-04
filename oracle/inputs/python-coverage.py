import re

single = r'(?#note)(?(1)yes|no)(?=x)(?!x)(?<=x)(?<!x)(?:x)(x)[a-z][^a-z][]a][^]a]\1(?P=name)(?i)a'
double = r"(?#note)(?(1)yes|no)(?=x)(?!x)(?<=x)(?<!x)(?:x)(x)[a-z][^a-z][]a][^]a]\1(?P=name)(?i)a"
single_three = r'''(?x)
    (?#note) (?(1)yes|no) (?=x) (?!x) (?<=x) (?<!x) (?:x) (x)  # a comment
    [a-z] [^a-z] []a] [^]a] [^
    a] \1 (?P=name) (?ims)
'''
double_three = r"""(?x)
    (?#note) (?(1)yes|no) (?=x) (?!x) (?<=x) (?<!x) (?:x) (x)  # a comment
    [a-z] [^a-z] []a] [^]a] \1 (?P=name) (?ims)
"""
bytes_regex = br'(?=x)' + rb"(?!x)" + Br'''(?<=x)'''

raw_multi = R'''raw {%name%} and {a b}
text'''
raw_unicode = uR"""raw"""
raw_bytes = Rb'''bytes''' + bR"""bytes"""
plain_brace = R'{%name%}' + R'{a b}' + R"{%name%}" + R"{a b}"
unicode_brace = '{%name%}' + "{%name%}" + u'{%name%}'
multi_unicode_brace = '''{%name%}''' + """{%name%}"""
multi_raw_brace = R'''{a b}''' + R"""{%x%}"""

bad_prefix = uf"value" + bf'''value''' + Bf"value" + uF'''value'''
raw_f = rf'''{name!r} and {width}
more''' + fr"""{x}"""
width = 10
formatted = f"{name!r} {name=!s} {value:>10} {value:{width}.2f} {value=:^20} a } b"
formatted_multi = f'''{name!a} {value:<{width}} {value=!r:{width}} {value:.3}
{value:{width}d}'''
spec = f"{value:{width}.{precision}}" + f"{value!r:{width}}"


named = r'(?P<word>x)\x41\101\u0041\U00000041 a[]' + r"a[^]"
escapes = r'\d\w\s\b[\101\d]'
named_three = r'''(?P<word>x) [] ''' + r"""[^]"""
deprecated = ur'x' + Ur"x" + ur'''x''' + ur"""x""" + br"""x"""
odd_prefix = rU'x' + rU'''x''' + U'''x''' + uR'x' + fu'x' + fb'''x'''
braces = f"{}" + f"{ }" + f'''{value=} {value=!r} {value:{width}} {value:+#{width},.2f} {value:{width}d}'''
single_tail = f"{value:+#{width},d}"
nested = f"{'''a
'''}"
formats = "{0:>10}".format(value) + "{0!r:{w}}".format(value, w=1)
typed = []  # type: List[int]
print(end="")
call("key"=1)
tails = f'''{value!r:>10} {value:{width}10} {value:{width}z}''' + f"{value:{width}10}"
odd = lambda x, lambda: x
arrow = a -> b
bad = a && b || c ++ d -- e $ f ?
semi = 1;

@dec !x
@call(1) junk
def None(): pass
class True: pass
ref = obj.None + obj.import
def oops:
    pass
with open(path) as handle:
    pass
try:
    pass
except ValueError as err:
    pass

class Meta(Base, metaclass=ABCMeta):
    pass

class Wrapped(make_base(Base)):
    pass

with ctx as value
set_one = r'a[]'
set_three = r'''[]'''
positional = lambda a, /, b, *args, **kwargs: a
nested_lambda = lambda a=lambda, b: a
call(lambda)
items = [lambda, lambda]
obj.lambda
bad_number = 123abc
lambda = 1
