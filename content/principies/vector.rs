fn main() {
    enum Sheet {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let v = vec![
        Sheet::Int(20),
        Sheet::Float(30.1),
        Sheet::Text(String::from("Hi")),
    ];
    // Auto creating of val types
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    let third = &v[2];
    v.push(4);
    v.push(5);
    // the difference between ref and ,get mrhod is how the error will be managing
    // with ref it will panic
    // with .get it will return None

    // println!("Third element is {}", third);
    // will throw an error due to mutable borrow before immutable
    println!("Third element is {}", &v[2]);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("Third element is {}", third),
        None => println!("There are no third element"),
    }
    for i in &mut v {
        *i += 1;
    }
    for i in &v {
        println!("{i}");
    }
}
