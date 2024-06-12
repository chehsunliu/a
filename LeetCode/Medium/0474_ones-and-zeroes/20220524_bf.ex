defmodule Solution do
  @spec find_max_form(strs :: [String.t], m :: integer, n :: integer) :: integer
  def find_max_form(strs, m, n) do
    counters = strs |> Enum.map(fn s -> to_counter(s) end)
    f(counters, m, n)
  end
  
  defp to_counter(s) do
    s |> String.to_charlist
      |> Enum.reduce(%{:zeros => 0, :ones => 0}, fn x, counter ->
        case x do
          ?0 -> Map.put(counter, :zeros, counter.zeros + 1)
          ?1 -> Map.put(counter, :ones, counter.ones + 1)
        end
      end)
  end
  
  defp f([], zeros, ones), do: 0
  defp f(counters, zeros, ones) do
    [c | tail] = counters
    
    cond do
      zeros >= c.zeros && ones >= c.ones ->
        ans0 = f(tail, zeros, ones)
        ans1 = f(tail, zeros - c.zeros, ones - c.ones) + 1
        max(ans0, ans1)
      true ->
        f(tail, zeros, ones)
    end
  end
end