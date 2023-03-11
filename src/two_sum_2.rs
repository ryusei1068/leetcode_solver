pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut low = 0;
    let mut hight = numbers.len() as i32 - 1;

    while low < hight {
        let sum = numbers[low as usize] + numbers[hight as usize];

        if sum == target {
            return vec![low + 1, hight + 1];
        } else if sum < target {
            low += 1;
        } else {
            hight -= 1;
        }
    }
    vec![-1, -1]
}

#[test]
fn test_two_sum() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let expected1 = vec![1, 2];
    assert_eq!(expected1, two_sum(nums1, target1));

    let nums2 = vec![2, 3, 4];
    let target2 = 6;
    let expected2 = vec![1, 3];
    assert_eq!(expected2, two_sum(nums2, target2));

    let nums3 = vec![-1, 0];
    let target3 = -1;
    let expected3 = vec![1, 2];
    assert_eq!(expected3, two_sum(nums3, target3));
}
