use std::{path::Path, fs::File, io::{BufReader, BufRead}};

use phf::{phf_map};

static INPUT_MAP: phf::Map<& str, u8> = phf_map! {
    "A" => 1,
    "B" => 2,
    "C" => 3,
};


static OUT_MAP: phf::Map<& str, u8> = phf_map! {
    "X" => 0,
    "Y" => 3,
    "Z" => 6,
};

#[derive(Debug)]
struct Game {
    total_socre: u32
}

impl Game {

    fn new() -> Game {
        Game { total_socre: 0 }
    }

    fn play_round(&mut self, outcome: &str, opps_move: &str) {
        self.update_score(outcome, opps_move)
    }

    fn update_score(&mut self, outcome: &str, opps_move: &str) {
        self.total_socre += self.get_socre(outcome, opps_move)
    }

    fn get_socre(&self, outcome: &str, opps_move: &str) -> u32 {
        let outcome_score = OUT_MAP[outcome];
        let my_move_score = self.get_my_move(&outcome_score, &INPUT_MAP[opps_move]);

        return (my_move_score + outcome_score).into()
    }

    fn get_my_move(&self, outcome_score: &u8, opps_move_score: &u8) -> u8{
        if outcome_score == &3{
            return *opps_move_score;
        }
        let mut my_move_score: u8 = 1;

        if outcome_score == &6 {
            my_move_score = opps_move_score + 1;
            if my_move_score == 4 {
                return  1;
            }
        }
        else if outcome_score == &0 {
            my_move_score = opps_move_score - 1;
            if my_move_score == 0 {
                return 3;
            }
        }
        
        return  my_move_score;
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


