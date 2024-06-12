class Point(NamedTuple):
    x: int
    h: int


class Solution:
    def trap(self, height: list[int]) -> int:
        unsorted_ps = [Point(x=x, h=h) for x, h in enumerate(height)]
        
        # n*log(n)
        ps = sorted(unsorted_ps, key=lambda p: (p.h, p.x))

        max_h = ps[len(ps) - 1].h
        highest_ps = [p for p in ps if p.h == max_h]
        l_highest_p, r_highest_p = highest_ps[0], highest_ps[len(highest_ps) - 1]
        
        l_group = []
        r_group = []
        for p in ps:
            if p.x < l_highest_p.x:
                l_group.append(p)
            elif p.x > r_highest_p.x:
                r_group.append(p)
                
        total_area = 0
                
        prev_p = l_highest_p
        for p in reversed(l_group):
            if p.x > prev_p.x:
                continue
                
            total_area += (prev_p.x - p.x - 1) * p.h
            for x in range(p.x + 1, prev_p.x):
                total_area -= unsorted_ps[x].h
            
            prev_p = p

        prev_p = r_highest_p
        for p in reversed(r_group):
            if p.x < prev_p.x:
                continue
                
            total_area += (p.x - prev_p.x - 1) * p.h
            for x in range(prev_p.x + 1, p.x):
                total_area -= unsorted_ps[x].h
                
            prev_p = p

        if r_highest_p.x != l_highest_p.x:
            total_area += (r_highest_p.x - l_highest_p.x - 1) * l_highest_p.h
            for x in range(l_highest_p.x + 1, r_highest_p.x):
                total_area -= unsorted_ps[x].h

        return total_area