class Task:
    def __init__(self, index: int, t_enqueue: int, t_processing: int):
        self.index = index
        self.t_enqueue = t_enqueue
        self.t_processing = t_processing


class Solution:
    def getOrder(self, tasks: List[List[int]]) -> List[int]:
        tasks = sorted([Task(i, task[0], task[1]) for i, task in enumerate(tasks)], key=lambda t: t.t_enqueue)

        t = 1
        index = 0
        ans = []
        available_tasks = []

        while len(ans) < len(tasks):
            while index < len(tasks) and tasks[index].t_enqueue <= t:
                heapq.heappush(available_tasks, (tasks[index].t_processing, tasks[index].index, tasks[index]))
                index += 1

            if len(available_tasks) > 0:
                task = heapq.heappop(available_tasks)[2]
                ans.append(task.index)
                t += task.t_processing
            else:
                t  = tasks[index].t_enqueue

        return ans