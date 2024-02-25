// we can use it to make the type annotation smaller
// if we having the long type we can make it smaller
type Kilometers = i32;
// we can also make this, and then we can make this type to change with the type we declare a
// Result type Result<usize>
type Result<T> = std::result::Result<T, std::io::Error>;

fn main() {
    // so will the Kilomoters type have the same behaviour as the i32 type
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("{} {}", x, y);
}

// we have a type that will be never returned (never type) -> !
// this type can be useful in a functions that doesnt returns anything and never returns such a panic!()

// dynamiclly sized types - one example for this type is the str. This is a dst and because this is
// a dst we cant create a variable, that will have a dst value, because of this we have the &str
// value in the variables
// pointers to this types will have an attribute length
// the Sized trait says, that we know the size of the variable on the start
//
// by default all generics are working with the Sized types, but we can overwrite it so ->
// fn generic<T: ?Sized>(t:T){...} so we saying that the type can be Sized or not
