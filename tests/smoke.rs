extern crate postcss;

#[test]
fn test_no_name() {
    assert_eq!("It compiles", postcss::lol());
}

