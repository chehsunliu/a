#include <cstdio>

// max int32 = 2^32 = (2^10)^3*(2^2) ~= 4*10^9
int main() {
  long n;
  scanf("%ld", &n);

  long sum = 0, x;
  for (int i = 0; i < n - 1; i++) {
    scanf("%ld", &x);
    sum += x;
  }

  long ans = (1 + n) * n / 2 - sum;
  printf("%ld\n", ans);

  return 0;
}
