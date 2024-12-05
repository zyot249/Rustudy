use rand::{
    distributions::{Alphanumeric, Standard, Uniform},
    prelude::Distribution,
    Rng,
};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        Point {
            x: rng.gen_range(-100..100),
            y: rng.gen_range(-100..100),
        }
    }
}

pub fn main() {
    let mut rng = rand::thread_rng();

    println!("------------------------Random a number------------------------");
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    println!("------------------------Random a number in a range------------------------");
    println!("Random integer: {}", rng.gen_range(0..10));
    println!("Random float: {}", rng.gen_range(0.0..10.0));

    println!("------------------------Use Uniform distribution------------------------");
    let die = Uniform::from(1..7);
    let mut count = 0;
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        count += 1;
        if count >= 2 {
            break;
        }
    }

    println!("------------------------Random value of a custom struct------------------------");
    println!("Random Point: {:?}", rng.gen::<Point>());

    println!("------------------------Random a password------------------------");
    let random_str = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect::<String>();
    println!("Random password: {}", random_str);
}
