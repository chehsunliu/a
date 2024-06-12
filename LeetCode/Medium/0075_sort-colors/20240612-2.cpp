#include <vector>

using namespace std;

class Solution {
public:
    void sortColors(vector<int> &nums) {
        int counts[] = {0, 0, 0};

        for (int i = 0; i < nums.size(); i++) {
            counts[nums[i]]++;
        }

        int offset = 0;
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < counts[i]; j++) {
                nums[j + offset] = i;
            }
            offset += counts[i];
        }
    }
};
