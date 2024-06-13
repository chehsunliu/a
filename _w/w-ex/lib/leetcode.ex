defmodule Solution do
  @spec min_moves_to_seat(seats :: [integer], students :: [integer]) :: integer
  def min_moves_to_seat(seats, students) do
    sorted_seats = seats |> Enum.sort()
    sorted_students = students |> Enum.sort()

    Stream.zip(sorted_seats, sorted_students)
    |> Stream.map(fn {x, y} -> abs(x - y) end)
    |> Enum.reduce(0, fn x, acc -> x + acc end)
  end
end
