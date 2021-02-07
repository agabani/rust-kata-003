pub fn fixture(filename: &str) -> Vec<u8> {
    let path = std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join(filename);

    std::fs::read(path).unwrap()
}
