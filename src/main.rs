use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct Elf {
    food_items: Vec<u32>,
    number: u32,
    total_calories: u32,
}

impl Elf {
    fn new() -> Self {
        Elf {
            number: 0,
            total_calories: 0,
            food_items: Vec::new(),
        }
    }

    fn update_calories(&mut self) {
        self.total_calories = 0;
        for item in self.food_items.iter() {
            self.total_calories += item;
        }
    }
}

fn load_from_file(file_path: &Path) -> Vec<Elf> {
    let fp = File::open(file_path).expect("Unable to open file");
    let fp = BufReader::new(fp);
    let mut vec_elf: Vec<Elf> = Vec::new();
    let mut number: u32 = 1;
    let mut elf: Elf = Elf::new();
    let mut max_calories: u32 = 0;

    for line in fp.lines() {
        let line = line.expect("Unable to read line");
        let value = match line.trim().parse() {
            Ok(num) => num,
            _ => 0,
        };

        if value != 0 {
            elf.food_items.push(value);
        } else {
            elf.number = number;
            elf.update_calories();

            if elf.total_calories >= max_calories {
                max_calories = elf.total_calories;
            }
            vec_elf.push(elf);
            number += 1;
            elf = Elf::new();
        }
    }

    elf.number = number;
    elf.update_calories();
    if elf.total_calories >= max_calories {
        max_calories = elf.total_calories;
    }
    println!("{} ", max_calories);
    vec_elf.push(elf);

    return vec_elf;
}

fn main() {
    let file_path = Path::new("./src/input.txt");
    println!("new asd {}", file_path.display());
    let mut vec_elf = load_from_file(&file_path);

    vec_elf[3].food_items[1] -= 1000;
}
