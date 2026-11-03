# Here documents named for JavaScript, which nishiki lexes as JavaScript,
# with interpolation and escapes where the JavaScript has nothing open.
script = <<~JS
  const user = { id: #{user.id}, name: "#{name}" };
  document.querySelector(`#${id}`).addEventListener("click", () => {
    console.log('clicked', /#{pattern}/g.test(user.name)); // #{note}
  });
  /* a comment
     across lines #{x} */
  const html = `
    <p>#{body}</p>
  `;
  #{statements} \n #@ivar
JS

module_js = <<-JAVASCRIPT
  export default function greet(who) { return `Hello, ${who}!`; }
JAVASCRIPT

widget = <<~WIDGET_JS
  let total = 0; for (const n of [#{numbers}]) total += n;
WIDGET_JS

inline = [<<~JS, <<~SQL]
  if (a < b) { #{branch} }
JS
  SELECT 1
SQL
