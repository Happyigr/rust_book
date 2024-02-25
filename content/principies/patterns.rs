// match expression
// match x {
//      PATTERN => EXPRESSION,
// }
//
// if let expression
// let age: Result<u8, _> = "34".parse();
// if let Ok(age) = age { smth with age }
//
// while let expression
// while let Some(top) = stack.pop(){...}
//
// for expression
// for PATTERN in EXPRESSION
//
// let expression
// let PATTERN = EXPRESSION
//
// function params expressions
// fn(PATTERN: EXPRESSION){}

// Refutability
// irrefutable are the patterns that matches everything
// and not everywhere we can use the irrefutable patterns and the same is with refutable patterns

// Pattern syntax
// in match arms we can make a slice of strings 'k'..'z' and compare it with the value
//
// we can also destract a struct in vars
// struct Point{ x: usize, y: usize, }
// let p = Point{x:10,y:3};
// let Point {x: a, y: b} = p{ make smth with a and b}
//
// we can write it easier
// let Point {x,y} = p { make smth with x and y}
//
// we can use destructors in match arms:
// match p { Point {x: 0, y} => do smth when x = 0 and y is any of the abilities}
//
// if we want also use the variable x we need to bind it
// match p { Point {x: getted @ 0..1, y} => then we can use getted and y
//
// we can destruct enums also
//
// enum Mssg{ Move {x: usize,y: usize}, }
//
// match msg { Mssg::Move {x,y} => do smth with this values}
//
// we can do it multiple times (in the cases when our structure has nested types
// Msg::ChangeColor(Color(r,g,b)))
//
// how we doing it with _ when ignoring some values we can ignore the stack of the values with ..
// p = Point{x:1,y:3,z:9}
// match p { Point {x, ..} => do smth with x}
