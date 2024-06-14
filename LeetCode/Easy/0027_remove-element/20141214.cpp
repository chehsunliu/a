class Solution {
public:
    int removeElement(int A[], int n, int elem) {
        if (n == 0)
            return 0;

        int candidate = n - 1;
        int i = 0;

        

        while (i <= candidate) {
            if (A[i] == elem) {
                A[i] = A[candidate];
                --candidate;
            } else {
                ++i;
            }
        }

        return i;
    }
};
