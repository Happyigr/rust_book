#[test]
fn iterator_demostration() {
    let v1 = vec![1, 2, 3];

    let mut iter1 = v1.iter();

    assert_eq!(iter1.next(), Some(&1));
    assert_eq!(iter1.next(), Some(&2));
    assert_eq!(iter1.next(), Some(&3));
    assert_eq!(iter1.next(), None);
}

#[derive(Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3];

    // we can also call into_iter() if we want to take the ownership
    // and we can call iter_mut() if we want to take an mutable iter
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("got: {}", val);
    }

    // this will icrement all the nums in vector by one
    // instead this returns an iterator that then can be consumed with collect() keyword
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();

    // here we use the iterators that takes the ownership
    let result: i32 = v2.iter().sum();
    // we cant now use the v1_iter, because of the ownersip
    println!("The sum is:{}", result);

    let shoes = vec![
        Shoe {
            size: 3,
            style: String::from("Normal"),
        },
        Shoe {
            size: 4,
            style: String::from("jojo"),
        },
        Shoe {
            size: 3,
            style: String::from("sex"),
        },
    ];

    println!("These are shoes in size 3: {:#?}", shoes_in_size(shoes, 3));
}
