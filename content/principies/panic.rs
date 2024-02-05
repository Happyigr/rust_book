// when to use panic?
//
// if we want that the programm will be broken after error we should use panic
// examples: prototyping, testing, e.t.c.
// if the code will be insecure or will return another type of data (safety reasons)
//
// if we want that the errord will be manadged from the code we should use result method
// examples: if the error is expected and will be handled out of the function or the programm
//
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

// we can use ? only in the function that returns the values that are matching to the Result
// logic(Option<> some or none...)
fn last_char_in_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_username_from_file() -> Result<String, io::Error> {
    // with ? on the end it will return Ok(var) to the var if it was Ok
    // and Err(error) from the function if it was not Ok
    // This will also wworks with our self-defined types of errors
    // let mut username = String::new();
    // File::open("username.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // this is the function from rust library that we wrote upper in this code
    fs::read_to_string("username.txt")
}
// these are the same
//fn read_username_from_file() -> Result<String, io::Error> {
// let username_file_result = File::open("username.txt");
//
// let mut username_file = match username_file_result {
//     Ok(file) => file,
//     Err(e) => return Err(e),
// };
//
// let mut username = String::new();
//
// match username_file.read_to_string(&mut username) {
//     Ok(_) => Ok(username),
//     Err(e) => Err(e),
// }

fn main() -> Result<(), Box<dyn Error>> {
    // this will return error in case of error to the main function
    let file = File::open("hello.txt")?;
    // creates the panic macro for us
    let file_result = File::open("hello.txt").unwrap();
    // or with panic message, that we want
    let file_result = File::open("hello.txt").expect("The file hello.txt must be created");

    // this returns the Ok Result in our main function
    Ok(())

    // with match
    //
    // let file = match file_result {
    //     Ok(f) => f,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(err) => panic!("Error by creating a file: {:?}", err),
    //         },
    //         other_error => panic!("Open file error: {:?}", other_error),
    //     },
    // };

    // or with unwrap_or_else
    //
    // let file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem by creating a file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem by opening the file:{:?}", error);
    //     }
    // });
}
