
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_calories.cmp(&other.total_calories)
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.total_calories == other.total_calories
    }
}

impl Eq for Elf {}

fn load_from_file(file_path: &Path) -> BinaryHeap<Elf> {
    let fp = File::open(file_path).expect("Unable to open file");
    let fp = BufReader::new(fp);
    let mut heap_elf: BinaryHeap<Elf> = BinaryHeap::new();
    let mut number: u32 = 1;
    let mut elf: Elf = Elf::new();

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
            heap_elf.push(elf);
            number += 1;
            elf = Elf::new();
        }
    }

    elf.number = number;
    elf.update_calories();
    heap_elf.push(elf);

    return heap_elf;
}

pub fn top3() {
    let file_path = Path::new("./src/input.txt");
    println!("new asd {}", file_path.display());
    let mut heap_elf = load_from_file(&file_path);

    let top1 = match heap_elf.pop() {
        Some(elf) => elf.total_calories,
        None => panic!("no Elf left"),
    };

    let top2 = match heap_elf.pop() {
        Some(elf) => elf.total_calories,
        None => panic!("no Elf left"),
    };

    let top3 = match heap_elf.pop() {
        Some(elf) => elf.total_calories,
        None => panic!("no Elf left"),
    };

    println!("{:#?}", top1 + top2 + top3);
}