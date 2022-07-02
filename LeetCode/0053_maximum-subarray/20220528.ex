defmodule Solution do
  @spec max_sub_array(nums :: [integer]) :: integer
  def max_sub_array(nums) do
    nums
      |> Enum.reverse()
      |> Enum.reduce([], fn num, dp ->
          case dp do
            [] ->
              [num]
            _ ->
              [next_optimal | remainings] = dp
              [max(num, num + next_optimal), next_optimal] ++ remainings
          end
        end)
      |> Enum.reduce(&(max(&1, &2)))
  end
end