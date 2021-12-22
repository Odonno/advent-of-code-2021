struct Player {
    position: u32,
    max_position: u32,
    score: u32,
}

struct Dice {
    value: u32,
    max_side: u32,
    rolls: u32,
}

impl Player {
    fn r#move(&mut self, position: u32) {
        self.position = ((self.position - 1 + position) % self.max_position) + 1;
        self.score += self.position;
    }

    fn has_won(&self) -> bool {
        return self.score >= 1000;
    }
}

impl Dice {
    fn roll(&mut self) -> u32 {
        self.value += 1;

        if self.value > self.max_side {
            self.value = 1;
        }

        self.rolls += 1;

        return self.value;
    }
}

fn main() {
    let mut turn = 0;

    let mut dice = Dice {
        value: 0,
        max_side: 100,
        rolls: 0,
    };

    let start_position_player_one = 7;
    let start_position_player_two = 3;

    let mut player_one = Player {
        position: start_position_player_one,
        max_position: 10,
        score: 0,
    };
    let mut player_two = Player {
        position: start_position_player_two,
        max_position: 10,
        score: 0,
    };

    loop {
        turn += 1;

        let player = if turn % 2 == 1 {
            &mut player_one
        } else {
            &mut player_two
        };

        let turn_score = dice.roll() + dice.roll() + dice.roll();
        player.r#move(turn_score);

        if player.has_won() {
            break;
        }
    }

    let losing_player = if player_one.has_won() {
        &player_two
    } else {
        &player_one
    };

    let result = losing_player.score * dice.rolls;
    println!("Result: {}", result);
}
