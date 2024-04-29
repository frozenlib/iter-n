#[test]
fn iter_2() {
    assert_eq!(f(0).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(f(1).collect::<Vec<_>>(), vec![2, 3]);

    fn f(x: i32) -> impl Iterator<Item = i32> {
        use iter_n::iter2::*;
        if x % 2 == 0 {
            [0, 1].iter().map(|y| y + 1).into_iter0()
        } else {
            [0, 1].iter().map(|y| y + 2).into_iter1()
        }
    }
}

#[test]
fn iter_3() {
    assert_eq!(f(0).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(f(1).collect::<Vec<_>>(), vec![2, 3]);
    assert_eq!(f(2).collect::<Vec<_>>(), vec![3, 4]);

    fn f(x: i32) -> impl Iterator<Item = i32> {
        use iter_n::iter3::*;
        if x % 3 == 0 {
            [0, 1].iter().map(|y| y + 1).into_iter0()
        } else if x % 3 == 1 {
            [0, 1].iter().map(|y| y + 2).into_iter1()
        } else {
            [0, 1].iter().map(|y| y + 3).into_iter2()
        }
    }
}
