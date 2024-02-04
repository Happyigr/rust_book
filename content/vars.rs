fn main() {
    let mut x = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");

    // if we make spaces mut we cant change the type of it, but with let we cn do it;
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The spaces are {spaces} length");

    let tup: (u32, f64, i8) = (500, 10.1, 1);
    let (_x, _y, _z) = tup;
    let _x = tup.0;

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];

    let _first = _a[0];
}
