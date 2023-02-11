pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let mid = (left + right) / 2;
        if nums[mid] > nums[mid + 1] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left as i32
}

#[test]
fn test_find_peak_element() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(2, find_peak_element(nums));

    let nums1 = vec![1, 2, 1, 3, 5, 6, 4];
    assert_eq!(5, find_peak_element(nums1));
}
