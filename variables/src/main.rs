fn main() {
    // let condition = true;

    // if condition {
    //     5
    // } else {
    //     "six"
    // }; //wtf is rust why does this return an error.
    //they aren't being tied to a variable.

    let mut count = 0;
    'counting_up: while count < 10 {
        println!("count = {count}");
        let mut remaining = 10;
        while remaining > 5 {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'counting_up;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
