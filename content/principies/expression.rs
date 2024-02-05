fn main() {
    let condition = false;
    let _number = if condition { 5 } else { 3 };

    for i in (1..4).rev() {
        println!("{}", i);
    }

    let mut counter = 0;
    // we can names the loops
    let number = 'big: loop {
        counter += 1;

        if counter == 20 {
            break 'big counter - 13;
        }
    };

    println!("The value is {number}");

    if number > 5 {
        println!("True");
    } else if number < 8 {
        println!("Less than 8");
    } else {
        println!("False");
    }
}
