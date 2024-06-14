#include <algorithm>
#include <cassert>
#include <cstdio>
#include <unordered_map>

// #define CHEAT
#define CHEAT_START 1
#define CHEAT_END   (CHEAT_START + 3)

#ifndef CHEAT
int64_t pre_dp[] = {0,1,2};
#endif

std::unordered_map<int64_t, int64_t> dp;

int64_t f(int64_t n) {
    if (n == 1) {
        return 1;
    }

#ifndef CHEAT
    if (CHEAT_START <= n && n <= CHEAT_END) {
        return pre_dp[n - CHEAT_START + 1];
    }
#endif

    std::unordered_map<int64_t, int64_t>::const_iterator dp_count = dp.find(n);
    if (dp_count != dp.end()) {
        return dp_count->second;
    }

    int64_t count = 1 + (n % 2 == 1 ? f(3 * n + 1) : f(n / 2));
    dp.insert(std::make_pair(n, count));
    return count;
}

int main() {
#ifndef CHEAT

    int64_t i, j;
    while (std::scanf("%lld %lld", &i, &j) != EOF) {
        assert(i <= 1000000 && j <= 1000000);

        int64_t ans = 0;
        for (int64_t k = std::min(i, j); k <= std::max(i, j); k++) {
            ans = std::max(ans, f(k));
        }
        std::printf("%lld %lld %lld\n", i, j, ans);
    }

#else

    int64_t i = CHEAT_START, j = CHEAT_END;
    printf("{0");
    for (int64_t k = i; k <= j; k++) {
        int64_t v = f(k);
        printf(",%lld", v);
    }
    printf("};");

#endif

    return 0;
}
