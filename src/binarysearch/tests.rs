#[cfg(test)]
mod test {
    use crate::binarysearch;

    struct Test<T> {
        vec: Vec<T>,
        target: T,
        expected: bool,
    }

    fn run_test<T: PartialEq + PartialOrd>(t: Test<T>) {
        assert_eq!(t.expected, binarysearch::search(&t.vec, t.target))
    }

    #[test]
    fn test_empty_vec() {
        run_test(Test{vec: Vec::<i32>::new(), target: 1, expected: false});
    }
    
    #[test]
    fn test_single_vec_true() {
        run_test(Test{vec: vec![0], target: 0, expected: true});
    }

    #[test]
    fn test_single_vec_false() {
        run_test(Test{vec: vec![0], target: 1, expected: false});
    }

    #[test]
    fn test_two_vec_false() {
        run_test(Test{vec: vec![0, 1], target: 2, expected: false});
    }

    #[test]
    fn test_two_vec_true() {
        run_test(Test{vec: vec![0, 1], target: 1, expected: true});
    }
    #[test]
    fn test_three_vec_false() {
        run_test(Test{vec: vec![0, 1, 2], target: 3, expected: false});
    }
    #[test]
    fn test_three_vec_true() {
        run_test(Test{vec: vec![0, 1, 2], target: 1, expected: true});
    }

}