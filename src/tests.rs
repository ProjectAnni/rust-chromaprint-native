use version;

#[test]
fn get_version() {
    let version = version();
    assert_eq!(version, "1.4.4");
}
