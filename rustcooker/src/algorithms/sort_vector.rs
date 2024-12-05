#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct City {
    name: String,
    population: u64,
}

impl City {
    fn new(name: String, population: u64) -> Self {
        Self { name, population }
    }
}

pub fn main() {
    println!("------------------------Sort a vector------------------------");
    let mut vec1 = vec![1, 5, 3, 2, 4];
    vec1.sort();
    println!("Sorted integer vector: {:?}", vec1);

    let mut vec2 = vec![1.0, 5.0, 3.0, 2.0, 4.0];
    vec2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Sorted float vector: {:?}", vec2);

    println!("------------------------Sort a vector of custom struct------------------------");
    let mut cities = vec![
        City::new("Beijing".to_string(), 2154),
        City::new("Shanghai".to_string(), 2418),
        City::new("Tianjin".to_string(), 1560),
    ];
    cities.sort();
    println!("Sorted cities with default implememtation of comparison: {:?}", cities);
}
