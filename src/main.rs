fn main() {
    let f = std::fs::File::open(".").unwrap();
    print!(" {:?}", f);
}
