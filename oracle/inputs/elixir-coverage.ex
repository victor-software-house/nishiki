## Reaches the units of the `elixir` grammar that the other inputs miss.
## Written for nishiki.

defprotocol Size do
  def size(data)
end

defimpl Size, for: BitString do
  def size(string), do: byte_size(string)
end

defmodule Coverage do
  @moduledoc ~S"""
  Literal documentation, with #{no} interpolation.
  """

  @doc '''
  Charlist documentation, with #{1 + 1} interpolation.
  '''

  @typedoc ~S'''
  Literal charlist documentation.
  '''

  @doc "One line of documentation, with a \t tab."

  def public(a) when is_atom a do
    {__MODULE__, __ENV__, a}
  end

  defp private(a, b \\ 1) when is_list a do
    numbers = [0x1F, 0x1_f, 1.5e10, 2.0, 0b101, 0o17, 1_000]
    atoms = [:'single quoted', :"double #{a} quoted"]
    chars = [?a, ?\n, ?\x41, ?\s]
    {numbers, atoms, chars, b}
  end

  def heredocs do
    """
    A string heredoc with "quotes" and #{interpolation}.
    """

    '''
    A charlist heredoc.
    '''
  end

  def sigils do
    lower = [~s"""
    a heredoc sigil
    """, ~s{braces}, ~s[brackets], ~s<angles>, ~s(parens), ~s|bars|]
    upper = [~S"""
    a literal heredoc sigil
    """, ~S{braces}, ~S[brackets], ~S<angles>, ~S(parens), ~S|bars|]
    {lower, upper, 'charlist', "a\nb"}
  end

  def operators(a, b) do
    a ||| b
    a ^^^ b
    a : b
    a; b
    "#{%{key: a}}"
  end

  def anonymous do
    Enum.map([1], fn(x) -> x end)
    Enum.map([1], fn x when x > 0 -> x end)
  end

  def clauses(list) do
    Enum.map(list, fn x ->
      x
    end)
  end
end

## A section comment
## that runs on.
    # A comment after blanks.

  foo(a) -> a

## Not valid Elixir: a `)` ends an anonymous function's head, as while the
## text is being edited.
f = fn a) -> a
