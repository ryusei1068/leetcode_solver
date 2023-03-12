pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut last_pos = (nums.len() - 1) as i32;
    let mut i = (nums.len() - 1) as i32;

    for num in nums.iter().rev() {
        if i + num >= last_pos {
            last_pos = i;
        }
        i -= 1;
    }
    last_pos == 0
}

#[test]
fn jumb_test() {
    let nums1 = vec![2, 3, 1, 1, 4];
    assert_eq!(true, can_jump(nums1));

    let nums2 = vec![3, 2, 1, 0, 4];
    assert_eq!(false, can_jump(nums2));
}
