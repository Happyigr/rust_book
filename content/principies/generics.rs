// T is a generic type
// and here we can only make a Point with one type (if it int than both vars must be int, and so on)
struct Point<T> {
    x: T,
    y: T,
}
// This will implement new type T and then we can return it, and will work with all the Points
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// this will be used only for POint structor, that are implementing the type f32
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
// but we can use multiple generic types
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let num_list = [1, 4, 124, 100, 80, 91];
    println!("The larest number in this array is {}", largest(&num_list));

    let char_list = ['y', 'g', 'k', 'a', 'o'];
    println!("The largest char in this array is {}", largest(&char_list));

    let p1 = Point2 { x: 4, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("P3 is now: {:?}", p3);
}
