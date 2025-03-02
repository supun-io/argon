fn main() {
    let source_file = "./app.argon";
    let source_code = std::fs::read_to_string(source_file).unwrap();

    println!("{}", source_code);
}
