// the drop trait lets you modify what will happen when the value goes out of the scope

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustumSmartPointer with data {}", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");

    // we cant drop the values with the method drop(), instead of it we should use the
    // std::mem::drop function

    drop(c);
    println!("smth");
}
