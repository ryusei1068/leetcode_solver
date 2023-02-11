use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index_num = HashMap::new();

    let mut i = 0;
    for num in &nums {
        let diff = target - num;
        if index_num.contains_key(&diff) {
            let index = index_num.get(&diff);
            return vec![i, *index.unwrap()];
        }
        index_num.insert(num, i);
        i += 1;
    }
    return vec![0, 0];
}

#[test]
fn test_two_sum() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let expected1 = vec![1, 0];
    assert_eq!(expected1, two_sum(nums1, target1));

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let expected2 = vec![2, 1];
    assert_eq!(expected2, two_sum(nums2, target2));
}
