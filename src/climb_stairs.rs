pub fn climb_stairs(n: i32) -> i32 {
    let mut cache: Vec<i32> = vec![0; n as usize + 1];
    cache[0] = 1;
    cache[1] = 1;

    let mut i: i32 = 2;
    while i <= n {
        cache[i as usize] = cache[i as usize - 1] + cache[i as usize - 2];
        i += 1;
    }
    cache[n as usize]
}

#[test]
fn test_climb_stairs() {
    assert_eq!(2, climb_stairs(2));
    assert_eq!(3, climb_stairs(3));
}
