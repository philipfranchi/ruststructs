#[cfg(test)]

#[test]
fn test_empty_vec() {
    assert_eq!(false, crate::binarysearch::search(&Vec::<i32>::new(), 1));
}

#[test]
fn test_single_vec_true() {
    assert_eq!(true, crate::binarysearch::search(&vec![0], 0));
}

#[test]
fn test_single_vec_false() {
    assert_eq!(false, crate::binarysearch::search(&vec![0], 1));
}

#[test]
fn test_two_vec_false() {
    assert_eq!(false, crate::binarysearch::search(&vec![0, 1], 2));
}

#[test]
fn test_two_vec_true() {
    assert_eq!(true, crate::binarysearch::search(&vec![0, 1], 1));
}
#[test]
fn test_three_vec_false() {
    assert_eq!(false, crate::binarysearch::search(&vec![0, 1, 2], 3));
}
#[test]
fn test_three_vec_true() {
    assert_eq!(true, crate::binarysearch::search(&vec![0, 1, 2], 1));
}

