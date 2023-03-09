use list::IntList;

#[test]
fn test_new() {
    let list = IntList::new(vec![1, 2, 3]);
    assert_eq!(list.list, vec![1, 2, 3]);
}

// Fibonacci tests
#[test]
fn test_new_fib() {
    let int_list = IntList::new_fib(0);
    assert_eq!(int_list.list, vec![]);

    let int_list = IntList::new_fib(1);
    assert_eq!(int_list.list, vec![0]);

    let int_list = IntList::new_fib(2);
    assert_eq!(int_list.list, vec![0, 1]);

    let int_list = IntList::new_fib(5);
    assert_eq!(int_list.list, vec![0, 1, 1, 2, 3]);

    let fib = IntList::new_fib(15);
    assert_eq!(
        fib.list,
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]
    );
}

// Even sum tests
#[test]
fn test_sum_evens_with_no_evens() {
    let list = IntList::new(vec![1, 3, 5]);
    assert_eq!(list.sum_evens(), 0);

    let list = IntList::new(vec![2, 4, 6]);
    assert_eq!(list.sum_evens(), 12);

    let list = IntList::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(list.sum_evens(), 6);

    let list = IntList::new(vec![2]);
    assert_eq!(list.sum_evens(), 2);
}

// Divisible by 3 tests
#[test]
fn test_sum_divisible_by_three() {
    let list = IntList::new(vec![]);
    assert_eq!(list.sum_divisible_by_three(), 0);

    let list = IntList::new(vec![1, 2, 4, 5]);
    assert_eq!(list.sum_divisible_by_three(), 0);

    let list = IntList::new(vec![3]);
    assert_eq!(list.sum_divisible_by_three(), 3);

    let list = IntList::new(vec![3, 6, 9, 12, 15]);
    assert_eq!(list.sum_divisible_by_three(), 45);

    let list = IntList::new(vec![1, 3, 4, 6, 8, 9]);
    assert_eq!(list.sum_divisible_by_three(), 18);
}

// Second largest tests
#[test]
fn test_second_largest() {
    let list = IntList::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(*list.second_largest().unwrap(), 4);

    let list = IntList::new(vec![5, 4, 3, 2, 1]);
    assert_eq!(*list.second_largest().unwrap(), 4);

    let list = IntList::new(vec![2, 2, 1, 1, 1]);
    assert_eq!(*list.second_largest().unwrap(), 1);

    let list = IntList::new(vec![3, 1, 2, 1, 1]);
    assert_eq!(*list.second_largest().unwrap(), 2);

    let list = IntList::new(vec![1, 1, 1, 1, 1]);
    assert_eq!(list.second_largest(), None);

    let list = IntList::new(vec![1]);
    assert_eq!(list.second_largest(), None);
}
