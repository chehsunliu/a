class Solution {
public:
    int removeDuplicates(int A[], int n) {
        if (n == 0)
            return 0;

        int dup = 0;

        for (int i = 1; i < n; ++i) {
            if (A[i] == A[dup]) {
            } else {
                ++dup;
                A[dup] = A[i];
            }   
        }

        return dup + 1;
    }
};
