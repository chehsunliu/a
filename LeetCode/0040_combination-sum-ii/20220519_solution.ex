defmodule Solution do
  @spec combination_sum2(candidates :: [integer], target :: integer) :: [[integer]]
  def combination_sum2(candidates, target) do
    f(candidates |> Enum.sort(), target, [])
  end
      
  defp f(nums, 0, answer), do: [answer]
  defp f(nums, target, answer) when target < 0, do: []
  defp f(nums, target, answer) do
    nums
      |> Enum.with_index()
      |> Enum.uniq_by(fn {value, index} -> value end)
      |> Enum.flat_map(fn {value, index} ->
        {_, nums_next} = nums |> Enum.split(index + 1)
        f(nums_next, target - value, answer ++ [value])
      end)
  end
end