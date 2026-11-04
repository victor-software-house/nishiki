# The text after __END__, which Ruby reads through DATA. Shiki lexes it as
# Ruby: its rule for the data comes after the one for constants, which takes
# __END__ first.
puts DATA.read

__END__
Some plain text first.
<!DOCTYPE html>
<html lang="en">
  <head>
    <title>#{not_ruby} &amp; more</title>
    <style>body { color: red; }</style>
  </head>
  <body>
    <p class="note">x \n y</p>
    <script>var a = 1; // done</script>
  </body>
</html>
