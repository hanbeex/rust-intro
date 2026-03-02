fn main() {
    let tup = (500, 6.4, 1);
    println!("The value of first element is: {}", tup.0);
    println!("The value of second element is: {}", tup.1);
    println!("The value of third element is: {}", tup.2);

    let (x, y, z) = tup;
    println!("The value of first element is: {}", x);
    println!("The value of second element is: {}", y);
    println!("The value of third element is: {}", z);

    let array = [1, 2, 3, 4, 5];
    for value in array {
        println!("{}", value);
    }
}
