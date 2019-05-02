use test_note;

mod common;

#[test]
fn it_add_three() {
    common::test_setup();
    assert_eq!(122, test_note::lib_add_three(2));
}
