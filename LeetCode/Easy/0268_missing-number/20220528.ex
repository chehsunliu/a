defmodule Solution do
  @spec missing_number(nums :: [integer]) :: integer
  def missing_number(nums) do
    n = length(nums)
    expected_sum = div((n - 0) * (n + 1), 2)
    nums |> Enum.reduce(expected_sum, fn num, remaining -> remaining - num end)
  end
end