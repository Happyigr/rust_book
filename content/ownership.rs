fn main() {
    // HEAP
    //
    // we dont know the size of the variables
    // this string allocated a memory with
    // ptr = memory part start
    // len = length of the string
    // capapcity = the all memory that this var can use
    let s1 = String::from("Hello");
    // copying of the ptr
    let s2 = s1;
    // s1 variable is now empty, because only 1 variable can be the owner of the memory part
    takes_ownership(s2);
    // functions taking ownership, so we cant use the variable that are stored in heap after the
    // passing this var in a function

    let s1 = gives_ownership();
    // &var called reference, so it is the reference to the ptr of the var variable
    make_smth_without_ownersip_taking(&s1);
    let mut s1 = s1;
    let r_s1 = &mut s1;
    // we cant make two mutable refernces
    // we cant make a unmutable reference, while we have the mutable ref
    change_the_reference(r_s1);
    // so we made a deep copying and s1 s2 is now two unequal parts of memory with string
    let s2 = s1.clone();

    // STACK
    //
    // we know the size of the variable from start, beacuase of it it has another behavior
    // these types of vars have te COPY trait, which makes the copying with standart behavior
    let x = 5;
    let y = x;
    // both of the variables have the own value, and not
}

fn change_the_reference(s: &mut String) {
    s.push_str(", world!");
}

fn make_smth_without_ownersip_taking(some_string: &String) {
    println!("The string is {}", some_string);
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
