defmodule Solution do  
  def two_oldest_ages(ages) do
    Enum.reduce(ages, [0, 0], &z(&1, &2))
  end
  
  defp z(x, y) do
    Enum.sort([x | y]) |> Enum.slice(1, 2)
  end
end