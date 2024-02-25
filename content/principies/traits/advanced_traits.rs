// we can use the associated types, the main differnce with the generic types is that, that we dont
// need to implement the trait for every type, we need only to implement the trait in our struct
// and give the name of the type, with which the trait will work
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// default implementations fot the generic type in trait, this is useful, fot example in operator
// overloading
// This is the implementation from the std::ops library
// here is the default type of the Rhs generic type is Self
trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// if we have a lot of methods in the struct with the same name we need to concretizez it, to not
// disambiguate the Rust:
//
// traits Human,wizard,bird has fly methods.
// by default the method of the variable will be called:
//
// let i = human{};
// i.fly() -> Human fly
// Wizard::fly(&i) -> wizard fly ...
//
// if we have the struct in which we have a data and we want to choose which implementation we want
// to use, of the trait or of the struct we need this syntax:
//
// <Type as Trait>::method() - <Dog as Animal>::baby_name();

// if we want to use the trait funktionality in the trait we can say that the one trait needs
// another one:
//
// trait OutlinePrint: fmt::Display{...}

// if we want to reimplment the trait for the struct that was implemnted in other code, we can do a
// tuple struct, and then implement the trait for this struct
// if we want that all the traits for our struct would be implmented by the data type from rust, we
// can implement the deref trait, that will return the type that we want to be implemented
