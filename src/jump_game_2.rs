use std::cmp;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut answer = 0;

    let mut cur_end = 0;
    let mut cur_far = 0;

    for i in 0..nums.len() - 1 {
        cur_far = cmp::max(cur_far, i as i32 + nums[i]);

        if i == cur_end {
            answer += 1;
            cur_end = cur_far as usize;
        }
    }

    answer
}

#[test]
fn jumb_test() {
    let nums1 = vec![2, 3, 1, 1, 4];
    let expected1 = 2;
    assert_eq!(expected1, jump(nums1));

    let nums2 = vec![2, 3, 0, 1, 4];
    let expected2 = 2;
    assert_eq!(expected2, jump(nums2));

    let nums3 = vec![2, 3, 1, 0, 2, 2, 3];
    let expected3 = 3;
    assert_eq!(expected3, jump(nums3));
}
