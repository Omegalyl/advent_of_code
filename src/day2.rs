use phf::{phf_map};

static SOCRE_MAP: phf::Map<& str, u8> = phf_map! {
    "1" => 6,
    "0" => 0,
    "-1" => 3
};

static INPUT_MAP: phf::Map<& str, u8> = phf_map! {
    "A" => 1,
    "B" => 2,
    "C" => 3,
    "X" => 1,
    "Y" => 2,
    "Z" => 3
};


static WIN_MAP: phf::Map<& str, u8> = phf_map! {
    "1" => 1,
    "0" => 2,
    "2" => 3,
};

#[derive(Debug)]
struct Game<'a> {
    moves: Vec<(&'a str, &'a str)>,
    total_socre: u32
}

impl Game<'_> {

    fn new() -> Game<'static>{
        Game { moves: Vec::new(), total_socre: 0 }
    }

    fn update_score(&mut self) {
        let _move = self.moves.pop().expect("No moves left");
        self.total_socre += self.get_socre(_move.0, _move.1)
    }

    fn get_socre(&self, opps_move: &str, my_move: &str) -> u32 {
        let my_move_score = INPUT_MAP[my_move];
        let outcome = ((my_move_score + INPUT_MAP[opps_move])%3).to_string();
        let socre = SOCRE_MAP[&outcome];

        return socre.into()
    }
}

pub fn init() {
    let mut game = Game::new();

    game.moves.push(("A", "Y"));
    game.moves.push(("B", "X"));
    game.moves.push(("C", "Z"));

    game.update_score();
    //game.update_score();
    //game.update_score();

    println!("{:#?}", game)
}


