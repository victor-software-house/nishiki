# Here documents named for XML, which nishiki lexes as XML, with
# interpolation and escapes where the XML has nothing open.
feed = <<~XML
  <?xml version="1.0" encoding="UTF-8"?>
  <feed xmlns="http://www.w3.org/2005/Atom">
    x #{a} <b>y</b> &amp;
    <entry id="#{entry.id}">
      <title type="text"
             lang="#{locale}">#{title}</title>
    </entry>
    <!-- c #{c} --> #{d}
    <![CDATA[ #{e} ]]> #{f}
    <!-- a comment
         across lines #{g} -->
    a \n b #@ivar
  </feed>
XML

config = <<-APP_XML
  <setting name="timeout" value="#{timeout}"/>
APP_XML

pair = [<<~XML, <<~SQL]
  <item>#{item}</item>
XML
  SELECT 1
SQL
