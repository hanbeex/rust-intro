fn main() {
    let rect = (30, 50);

    let mut area = area(rect);

    // brw is a mutable borrow of area. Even though the scope ends here,
    // it still changed area's value because it's dereferenced first
    {
        let brw = &mut area;
        *brw = 900;
    }

    println!("The area of the rectangle is {} square pixels.", area);

    {
        let mut new = &mut area;
        *new = 5000;
        let mut other = 2000;
        new = &mut other;
        println!("{}", new);
    }

    println!("The area of the rectangle is {} square pixels.", area);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
