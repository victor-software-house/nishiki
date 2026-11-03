# Here documents named for the languages Ruby embeds that nishiki lexes,
# with interpolation and escapes where the embedded language has nothing open.
query = <<~SQL
  SELECT id, name FROM users WHERE id = #{user.id} AND name = 'a #{b}'
  -- #@note and #@@count and #$global
  ORDER BY name \n LIMIT 10
SQL

schema = <<-GRAPHQL
  query Find($id: ID!) { user(id: $id) { name #{fields} } }
GRAPHQL

styles = <<~CSS
  .button { color: #{color}; margin: 0 \1 2px; }
  a::before { content: "\201C"; }
CSS

source = <<~C
  #include <stdio.h>
  int main(void) { printf("%d\n", #{count}); return 0; } \
  /* done */
  #{type} value = \x;
C

run = <<~SHELL
  echo "hello #{name}" \$HOME
  for f in *.rb; do ruby -c "$f"; done \
    && echo ok
  #{command} --flag \x
SHELL

sh = <<~'SH'
  ls -la | grep '#{literal}'
SH

jq = <<~JQUERY
  $("#id").hide();
JQUERY

view = <<~SLIM
  p = #{text}
SLIM

eval <<~RUBY, __FILE__, __LINE__ + 1
  def #{name}
    @value = "#{value}\n"
  end
RUBY

list = [<<~SQL, <<~CSS]
  SELECT 1
SQL
  b { }
CSS

trailing = <<~SQL
  SELECT 2
  SQL  
SQL
