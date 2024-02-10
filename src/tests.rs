#[cfg(test)]

use crate::fenwick::Fenwick;

#[test]
fn test_init() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let f = Fenwick::new(&arr);

    assert_eq!(f.query(0), 1);
    assert_eq!(f.query(1), 3);
    assert_eq!(f.query(2), 6);
    assert_eq!(f.query(3), 10);
    assert_eq!(f.query_range(3, 4), 9);
}

#[test]
fn test_update() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7];
    let mut f = Fenwick::new(&arr);
    // arr[1] += 8;
    f.update(1, 8);

    assert_eq!(f.query(1), 11);
}
