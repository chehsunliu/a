defmodule Solution do
  @spec reverse_words(s :: String.t) :: String.t
  def reverse_words(s) do
    s |> String.trim()
      |> String.split()
      |> Enum.reverse()
      |> Enum.join(" ")
  end
end