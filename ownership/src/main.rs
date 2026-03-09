fn main() {
    let s = String::from("hello");
    println!("{}", return_borrowed(&s));
}

fn return_borrowed(s: &str) -> &str {
    s
}
