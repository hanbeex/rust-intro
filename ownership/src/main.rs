fn main() {
    let s = String::from("Hello, world!");

    return_borrowed(s);
    println!("{}", s);
}

fn return_borrowed(x: String) -> String {
    x
}
