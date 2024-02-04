use std::collections::HashMap;

fn main() {
    let mut vec: Vec<i32> = vec![3, 3, 3, 4, 5, 1, 20, 42, 10, 1, 1, 1, 123, 9, 100];
    vec.sort();
    println!("{:?}", vec);
    println!("Mediana is {}", median(&vec));
    let mut map = HashMap::new();
    let mut max_num = vec[0];
    for num in &vec {
        let count = map.entry(num).or_insert(0);
        *count += 1;
        if *count > *map.get(&max_num).unwrap_or(&0) {
            max_num = *num;
        }
    }

    println!("{:?}", map);
    println!("{max_num}");
}

fn median(v: &Vec<i32>) -> i32 {
    v[v.len() / 2]
}
