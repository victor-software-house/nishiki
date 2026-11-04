# Here documents named for HTML, which nishiki lexes as HTML, with
# interpolation and escapes where the HTML has nothing open.
page = <<~HTML
  <!DOCTYPE html>
  x #{a} <b>y</b> &amp; &copy;
  <a title="#{x}">t</a>
  <a href="x"
     data-y="#{y}">z</a>
  <script>var a = #{x};</script>
  <style>p { color: #{color}; }</style>
  <!-- a comment
       across lines #{c} -->
  a \n b < c #@ivar
  #{content}
HTML

mail = <<-MAIL_HTML
  <p class="greeting">Hello, #{user.name}!</p>
MAIL_HTML

pair = [<<~HTML, <<~SQL]
  <li>#{item}</li>
HTML
  SELECT 1
SQL
