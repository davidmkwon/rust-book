#![allow(dead_code)]
#![allow(unused_variables)]
use tests::Point;

// this file has integration tests. these only run if unit tests all pass first
//
// you can run specific integration test file via `cargo test --test FILE_NAME`

#[test]
fn create_point1() {
    let x = Point::new(3, 4);
    assert_eq!(x, Point { x: 3, y: 4 });
}
