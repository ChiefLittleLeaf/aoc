use std::fs::File;
fn main() {
    let file = File::open("../resources/day3.input").expect("file does not exist");
    println!("{:?}", file);
}
