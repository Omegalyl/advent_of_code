use std::{path::Path, fs::File, io::{BufReader, BufRead}};

use phf::{phf_map};

static INPUT_MAP: phf::Map<& str, u8> = phf_map! {
    "A" => 1,
    "B" => 2,
    "C" => 3,
    "X" => 1,
    "Y" => 2,
    "Z" => 3
};

#[derive(Debug)]
struct Game {
    total_socre: u32
}

impl Game {

    fn new() -> Game {
        Game { total_socre: 0 }
    }

    fn play_round(&mut self, my_move: &str, opps_move: &str) {
        self.update_score(my_move, opps_move)
    }

    fn update_score(&mut self, my_move: &str, opps_move: &str) {
        self.total_socre += self.get_socre(my_move, opps_move)
    }

    fn get_socre(&self, my_move: &str, opps_move: &str) -> u32 {
        let my_move_score = INPUT_MAP[my_move];
        let outcome = self.get_outcome(&my_move_score, &INPUT_MAP[opps_move]);

        return (my_move_score + outcome).into()
    }

    fn get_outcome(&self, my_move_score: &u8, opps_move_score: &u8) -> u8{
        if my_move_score == opps_move_score{
            return 3;
        }
        let mut outcome: u8 = 0;

        if my_move_score == &1 && opps_move_score == &3 {
            outcome = 6;
        }
        else if my_move_score == &2 && opps_move_score == &1 {
            outcome = 6;
        }
        else if my_move_score == &3 && opps_move_score == &2 {
            outcome = 6;
        }
        
        return  outcome;
    }
}


fn play_game(file_path: &Path) {
    let mut game = Game::new();
    let fp = File::open(file_path).expect("Unable to open file");
    let fp = BufReader::new(fp);

    for line in fp.lines() {
        let line = line.unwrap();
        let moves: Vec<&str> = line.trim().split(" ").collect();
        game.play_round(moves[1], moves[0])
    }

    println!("{:#?}", game)
}

pub fn init() {
    let file_path = Path::new("./src/day2.in");
    play_game(file_path)
}


