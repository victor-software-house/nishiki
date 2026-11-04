# Here documents named for YAML, which nishiki lexes as YAML, with
# interpolation where a YAML step begins at it.
config = <<~YAML
  #{a}: b
  key: #{value}
  key: plain #{value} more
  key: "q #{x}" #{y}
  key: 'q #{x}'
  list:
    - #{item}
    - a #{b}
  block: |
    text #{x}
  # comment #{c}
  key: a \n b
  anchor: &a value
  ref: *a
YAML

settings = <<-YML
#{name}: value
a#{b}: c
key: e#{f}
- #{g}
\n: c
---
nested:
  deeper: [one, two]
YML

pair = [<<~YAML, <<~SQL]
  item: #{item}
YAML
  SELECT 1
SQL
