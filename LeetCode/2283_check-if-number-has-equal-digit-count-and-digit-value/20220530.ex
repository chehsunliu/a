defmodule Solution do
  @spec digit_count(num :: String.t) :: boolean
  def digit_count(num) do
    values = num
              |> to_charlist()
              |> Enum.map(fn x -> x - ?0 end)

    records = 0..9 |> Enum.reduce(%{}, &(Map.put(&2, &1, 0)))
    records = values |> Enum.reduce(records, &(Map.put(&2, &1, Map.get(&2, &1) + 1)))

    values
      |> Enum.with_index(fn value, i -> records[i] == value end)
      |> Enum.reduce(&(&1 && &2))
  end
end