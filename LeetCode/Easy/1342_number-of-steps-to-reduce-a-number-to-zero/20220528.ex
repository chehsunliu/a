defmodule Solution do
  @spec number_of_steps(num :: integer) :: integer
  def number_of_steps(num) do
    f(num, 0)
  end
    
  defp f(0, steps) do
    steps
  end
    
  defp f(num, steps) when rem(num, 2) == 0 do
    f(div(num, 2), steps + 1)
  end
    
  defp f(num, steps) do
    f(num - 1, steps + 1)
  end
end