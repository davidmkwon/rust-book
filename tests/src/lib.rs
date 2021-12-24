#![allow(dead_code)]

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        if x < 0 && y < 0 {
            panic!("can't have negative values in the point");
        }

        Point { x, y }
    }

    pub fn dist(&self, p: &Point) -> f64 {
        (((self.x - p.x).pow(2) + (self.y - p.y).pow(2)) as f64).sqrt()
    }
}

/*
 * Tests
 *
 * Tests work by being run by a test runner binary that runs all the functions with the #[test]
 * attribute. The test will fail if the function panics, otherwise it passes
 */

// note that this attribute here means that the code under here will only be compiled into the
// binary if a specific cargo config is run (in this case, cargo `test`, hence cfg(test)). this way
// when you make the binary it's much smaller because it doesn't include the tests.
#[cfg(test)]
mod tests {
    // use super::Point; // also works
    use crate::Point;

    // assert!() checks that the condition is true, panics otherwise
    #[test]
    fn test_dist() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert!(p1.dist(&p2) == 5.0);
    }

    // assert_eq!() checks that the first arg is equal to the second, if it's not it panics and
    // shows the value of the left and right arg
    #[test]
    fn test_dist2() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_eq!(p1.dist(&p2), 5.0);
    }

    // assert_ne!() does same as above but checks not equal
    #[test]
    fn test_dist3() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_ne!(p1.dist(&p2), 4.0);
    }

    // to assert equal the arguments must impl Debug and PartialEq
    #[test]
    fn test_dist4() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_ne!(p1, p2);
    }

    // you can have your own error message pop up, pass in a string with {}'s and the arguments
    // just like you do in format!() (in fact the stuff you pass is basically forwarded to the
    // format macro
    #[test]
    fn test_dist5() {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 3, y: 4 };
        assert_ne!(
            p1, p2,
            "p1 and p2 weren't equal because p1 = {:?} and p2 = {:?}",
            p1, p2
        );
    }

    // you can make sure that functions *should* panic
    #[test]
    #[should_panic]
    fn test_dist6() {
        let _p1 = Point::new(-1, -1);
    }

    // sometimes the panic message might not be the one we want to happen, but a panic that happens
    // from something else. in this case we can specify a string we expect to be in the panic
    // messatge
    #[test]
    #[should_panic(expected = "can't have negative values in the point")]
    fn test_dist7() {
        let _p1 = Point::new(-1, -1);
    }

    // you can also have tests that fail when a Result Err() is returned
    #[test]
    fn test_dist8() -> Result<(), String> {
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { ..p1 };
        //let p2 = Point { x: 1, y: 0 };
        if p1 == p2 {
            Ok(())
        } else {
            Err("points are just not equal, man :(".to_string())
        }
    }

    #[test]
    #[ignore]
    fn test_dist9() {
        panic!("lol this test is ignored lmaoooo");
    }

    /*
     * Controlling how tests are run
     *
     * This section basically talks about ways to configure the execution of tests.
     *
     * `cargo test -- --test-threads=N` will run your code on N threads. this means if you want
     * your tests rely on some shared state and it matters the order in which the tests run, then
     * just run in on N=1 thread.
     *
     * `cargo test -- --show-output` will show the things you printed to stdout in your tests in
     * the terminal. otherwise, it will only show these statements in tests that fail, and
     * captures/hides the stdout output for tests that succeed
     *
     * `cargo test WORD` will run tests that contain WORD in their name.
     *
     * `cargo test -- --ignored` will run tests with the attribute #[ignore]
     */

    /*
     * Unit Tests and Integration Tests
     *
     * Unit tests are meant to test individual parts of your code, like functionailty of specific
     * functions, etc.
     *
     * Integration tests are meant to test your code like an external user would
     * use (only through the public API) and that the general functionality/flow of your library is
     * correct. these test that different parts of the library work together correctly
     */

    // testing using other crates in this workspace
    #[test]
    fn test_workspace() {
        use minigrep::search;

        let query = "hiya";
        let contents = "\
hiya first line
hiya second line
third line";

        assert_eq!(
            search(query, contents),
            vec!["hiya first line", "hiya second line"]
        );
    }
}
