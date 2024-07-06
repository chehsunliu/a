#include <algorithm>
#include <cassert>
#include <cstdio>
#include <climits>

#define BUFFER_LENGTH 102
// #define DEBUG

void debug(int arr[BUFFER_LENGTH][BUFFER_LENGTH], int n) {
#ifdef DEBUG
    for (int i = 0; i <= n; i++) {
        for (int j = 0; j <= n; j++) {
            printf("%3d ", arr[i][j]);
        }
        printf("\n");
    }
#endif
}

int main() {
    int n;
    int values[BUFFER_LENGTH][BUFFER_LENGTH];

    std::scanf("%d", &n);
    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= n; j++) {
            std::scanf("%d", &values[i][j]);
        }
    }

    int sum0[BUFFER_LENGTH][BUFFER_LENGTH];
    for (int i = 0; i <= n; i++) {
        sum0[0][i] = 0;
        sum0[i][0] = 0;
    }

    for (int r = 1; r <= n; r++) {
        int row_acc = 0;
        for (int c = 1; c <= n; c++) {
            row_acc += values[r][c];
            sum0[r][c] = sum0[r - 1][c] + row_acc;
        }
    }

    int ans = INT_MIN;
    for (int r0 = 1; r0 <= n; r0++) {
        for (int c0 = 1; c0 <= n; c0++) {
            for (int r1 = r0; r1 <= n; r1 ++) {
                for (int c1 = c0; c1 <= n; c1++) {
                    int sum = sum0[r1][c1] - sum0[r0 - 1][c1] - sum0[r1][c0 - 1] + sum0[r0 - 1][c0 - 1];
                    ans = std::max(ans, sum);
                }
            }
        }
    }

    printf("%d\n", ans);

    return 0;
}
