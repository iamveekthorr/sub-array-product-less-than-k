fn main() {
    // Example 3: 713. Subarray Product Less Than K.

    // Given an array of positive integers nums and an integer k, return the number of subarrays where the product of all the elements in the subarray is strictly less than k.

    // For example, given the input nums = [10, 5, 2, 6], k = 100, the answer is 8. The subarrays with products less than k are:

    // [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6]

    let nums = [10, 5, 2, 6];
    let k = 100;

    let result = product_of_subarray_less_than_k(&nums, &k);
    println!("result = {}", result);
}

fn product_of_subarray_less_than_k(nums: &[u32], k: &u32) -> usize {
    if *k <= 1 {
        return 0;
    }

    let mut ans = 0;
    let mut left = 0;
    // because we are multiplying, current has to be 1.
    let mut current = 1;

    for right in 0..nums.len() {
        current *= nums[right];

        while current >= *k {
            current /= nums[left];
            left += 1;
        }

        ans += right - left + 1;
    }

    return ans;
}
