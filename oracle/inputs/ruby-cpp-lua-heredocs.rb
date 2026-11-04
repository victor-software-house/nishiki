# Here documents named for C++ and Lua, which nishiki lexes in those
# languages, with interpolation and escapes where the language has nothing
# open.
source = <<~CPP
  #include <vector>
  template <typename T> class Box { T value = #{initial}; };
  int main() { std::vector<int> v{#{items}}; return v.size() > 0 ? 0 : 1; }
  auto s = "#{name}\n"; auto r = R"(raw #{text})";
  /* a comment
     across lines #{x} */
  #{statement} \x
  #define TWICE(x) ((x) * 2)
CPP

header = <<-MY_CPP
  namespace app { struct Point { double x, y; }; }
MY_CPP

script = <<~LUA
  local count = #{count}
  local function greet(name) return "hello " .. name .. " #{suffix}" end
  local text = [[
    long #{body}
  ]]
  --[[ a comment
       across lines #{x} ]]
  for i = 1, #items do print(i) end -- #{note}
  #{statements} \n
LUA

pair = [<<~CPP, <<~LUA]
  if (a < b) { #{branch} }
CPP
  if a < b then #{branch} end
LUA
