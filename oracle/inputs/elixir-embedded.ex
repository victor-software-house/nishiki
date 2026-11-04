defmodule DemoWeb.Components do
  @moduledoc """
  Components whose templates hand HTML to its lexer.
  """
  use Phoenix.Component

  attr :title, :string, required: true
  slot :inner_block

  def card(assigns) do
    ~H"""
    <section class="card" id={@id}>
      <h2><%= @title %></h2>
      <p :if={@note}>{@note}</p>
      <!-- a comment -->
      <ul>
        <li :for={item <- @items}>{item.name}</li>
      </ul>
      <%= render_slot(@inner_block) %>
    </section>
    """
  end

  def legacy(assigns) do
    ~L"""
    <div class="legacy">
      <%= for row <- @rows do %>
        <span data-id="<%= row.id %>"><%= row.label %></span>
      <% end %>
    </div>
    """
  end

  def inline(assigns), do: ~H"""
  <b>bold</b>
  """

  def plain(name) when is_binary(name) do
    greeting = ~s(hello #{name})
    pattern = ~r/^[a-z]+$/i
    %{greeting: greeting, match?: Regex.match?(pattern, name), at: ~D[2026-11-04]}
  end
end
