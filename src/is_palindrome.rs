fn reverse_integer(n: i32, mut num: i32) -> i32 {
    if n < 10 {
        return num + n;
    }

    num += n % 10;
    num *= 10;
    return reverse_integer(n / 10, num);
}

pub fn is_palindrome(n: i32) -> bool {
    if n < 0 {
        return false;
    }

    let num = reverse_integer(n, 0);
    num == n
}

#[test]
fn test_reverse_integer() {
    let n1 = 121;
    let n2 = 4321;
    assert_eq!(121, reverse_integer(n1, 0));
    assert_eq!(1234, reverse_integer(n2, 0));
}

#[test]
fn test_is_palindrome() {
    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(-121));
    assert_eq!(false, is_palindrome(10));
}
