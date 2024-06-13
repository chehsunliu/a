defmodule SolutionTest do
  use ExUnit.Case

  test "try the basic test cases" do
    assert Solution.min_moves_to_seat([3, 1, 5], [2, 7, 4]) == 4
    assert Solution.min_moves_to_seat([4, 1, 5, 9], [1, 3, 2, 6]) == 7
    assert Solution.min_moves_to_seat([2, 2, 6, 6], [1, 3, 2, 6]) == 4
  end
end
