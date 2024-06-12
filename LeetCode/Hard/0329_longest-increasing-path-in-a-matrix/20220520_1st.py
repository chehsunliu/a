class Node:
    def __init__(self, value: int, steps: Optional[int] = None) -> None:
        self.value = value
        self.steps = steps
        
    def __repr__(self) -> str:
        return f"Node(v={self.value}, s={self.steps})"


class Solution:
    def longestIncreasingPath(self, matrix: list[list[int]]) -> int:
        self.answer = 0

        nodes = [[Node(value) for value in values] for values in matrix]
        
        for i in range(len(matrix)):
            for j in range(len(matrix[i])):
                self.f(nodes, i, j)

        return self.answer

    def f(self, nodes: list[list[Node]], i: int, j: int) -> None:
        queue = []

        if i - 1 >= 0:
            queue.append((i - 1, j))
        
        if i + 1 < len(nodes):
            queue.append((i + 1, j))
        
        if j - 1 >= 0:
            queue.append((i, j - 1))
        
        if j + 1 < len(nodes[i]):
            queue.append((i, j + 1))

        max_steps = 0
        current_node = nodes[i][j]
        for i0, j0 in queue:
            neighbor_node = nodes[i0][j0]

            if neighbor_node.value > current_node.value:
                if neighbor_node.steps is None:
                    self.f(nodes, i0, j0)
                max_steps = max(max_steps, neighbor_node.steps)
            
        current_node.steps = 1 + max_steps
        self.answer = max(self.answer, current_node.steps)
