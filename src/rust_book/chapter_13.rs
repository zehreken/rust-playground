use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn run() {
    // Closures
    let simulated_use_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_use_specified_value, simulated_random_number);

    // Iterators
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let mut v1_iter = v1.iter();
    println!("Next: {}", v1_iter.next().unwrap());

    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // sum takes ownership of the iterator
    println!("Sum: {}", total);

    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter.map(|x| x * x).collect();
    println!("v1: {:?}, v2: {:?}", v1, v2);

    // Using closures that capture their environment
    // Filter by size
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!("{:?}", in_my_size);

    // Implementing own iterators
    let mut counter = Counter::new();
    println!("{}", counter.next().unwrap()); // 1
    println!("{}", counter.next().unwrap()); // 2
    println!("{}", counter.next().unwrap()); // 3
    for i in counter {
        println!("{}", i); // 4, 5
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // zip produces only for pairs
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("Sum: {}", sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num: u32| -> u32 // More verbose definition
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            )
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
    map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if !self.map.contains_key(&arg) {
            self.map.insert(arg, (self.calculation)(arg));
        }

        *self.map.get(&arg).unwrap()
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
