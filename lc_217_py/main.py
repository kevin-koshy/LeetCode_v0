from typing import List


class Solution:
    def containsDuplicate(self, nums: List[int]) -> bool:
        x = set(nums)
        if len(x) != len(nums):
            return True;
        else:
            return False;


result = Solution()
print(result.containsDuplicate([1,2,3,1]))