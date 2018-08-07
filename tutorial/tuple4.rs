fn make_tuple() -> (u32, String) {
    (70, "hello".to_string())
}


fn main() {
    let (number, word) = make_tuple();

    println!("number is {}", number);
    println!("word is {}", word);
}
