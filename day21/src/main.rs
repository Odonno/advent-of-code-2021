use std::cmp;

#[derive(Debug, Clone)]
struct Player {
    position: u32,
    score: u32,
}

struct DiceCopy {
    value: u32,
    copies: u32,
}

struct QuantumDice {}

struct Universe {
    turn: u32,
    player_one: Player,
    player_two: Player,
    copies: u64,
}

const MAX_PLAYER_POSITION: u32 = 10;

impl Player {
    fn r#move(&self, position: u32) -> Player {
        let position = ((self.position - 1 + position) % MAX_PLAYER_POSITION) + 1;
        let score = self.score + position;

        return Player { position, score };
    }

    fn has_won(&self) -> bool {
        return self.score >= 21;
    }
}

const QUANTUM_DICE_VALUES: [u32; 3] = [1, 2, 3];
const THREE_CONSECUTIVE_ROLLS: [u32; 27] = [
    3, 4, 5, 4, 5, 6, 5, 6, 7, 4, 5, 6, 5, 6, 7, 6, 7, 8, 5, 6, 7, 6, 7, 8, 7, 8, 9,
];
const THREE_CONSECUTIVE_ROLLS_AS_COPIES: [DiceCopy; 7] = [
    DiceCopy {
        value: 3,
        copies: 1,
    },
    DiceCopy {
        value: 4,
        copies: 3,
    },
    DiceCopy {
        value: 5,
        copies: 6,
    },
    DiceCopy {
        value: 6,
        copies: 7,
    },
    DiceCopy {
        value: 7,
        copies: 6,
    },
    DiceCopy {
        value: 8,
        copies: 3,
    },
    DiceCopy {
        value: 9,
        copies: 1,
    },
];

impl QuantumDice {
    fn three_consecutive_rolls_as_copies(&self) -> [DiceCopy; 7] {
        return THREE_CONSECUTIVE_ROLLS_AS_COPIES;
    }
}

fn main() {
    let quantum_dice = QuantumDice {};

    let mut current_playing_universes: Vec<Universe> = Vec::new();
    let mut ended_universes: Vec<Universe> = Vec::new();

    let start_position_player_one = 7;
    let start_position_player_two = 3;

    let player_one = Player {
        position: start_position_player_one,
        score: 0,
    };
    let player_two = Player {
        position: start_position_player_two,
        score: 0,
    };

    let rolls_as_copies = &quantum_dice.three_consecutive_rolls_as_copies();

    for dice_copy in rolls_as_copies {
        current_playing_universes.push(Universe {
            turn: 1,
            player_one: player_one.r#move(dice_copy.value),
            player_two: player_two.clone(),
            copies: dice_copy.copies as u64,
        });
    }

    loop {
        // next move on current playing universes
        let mut next_playing_universes: Vec<Universe> = Vec::new();

        for universe in &current_playing_universes {
            let next_turn = universe.turn + 1;
            let is_player_one_turn = is_player_one_turn(next_turn);
            let current_player = if is_player_one_turn {
                &universe.player_one
            } else {
                &universe.player_two
            };

            for dice_copy in rolls_as_copies {
                let next_player = current_player.r#move(dice_copy.value);

                if !next_player.has_won() {
                    if is_player_one_turn {
                        next_playing_universes.push(Universe {
                            turn: next_turn,
                            player_one: next_player,
                            player_two: universe.player_two.clone(),
                            copies: universe.copies * dice_copy.copies as u64,
                        });
                    } else {
                        next_playing_universes.push(Universe {
                            turn: next_turn,
                            player_one: universe.player_one.clone(),
                            player_two: next_player,
                            copies: universe.copies * dice_copy.copies as u64,
                        });
                    }
                } else {
                    if is_player_one_turn {
                        ended_universes.push(Universe {
                            turn: universe.turn,
                            player_one: next_player,
                            player_two: universe.player_two.clone(),
                            copies: universe.copies * dice_copy.copies as u64,
                        });
                    } else {
                        ended_universes.push(Universe {
                            turn: universe.turn,
                            player_one: universe.player_one.clone(),
                            player_two: next_player,
                            copies: universe.copies * dice_copy.copies as u64,
                        });
                    }
                }
            }
        }

        current_playing_universes = next_playing_universes;

        if current_playing_universes.len() == 0 {
            break;
        }
    }

    let player_one_wins = ended_universes
        .iter()
        .filter(|u| u.player_one.has_won())
        .map(|u| u.copies)
        .sum::<u64>();
    let player_two_wins = ended_universes
        .iter()
        .filter(|u| u.player_two.has_won())
        .map(|u| u.copies)
        .sum::<u64>();

    let result = cmp::max(player_one_wins, player_two_wins);
    println!("Result: {}", result);
}

fn is_player_one_turn(turn: u32) -> bool {
    return turn % 2 == 1;
}
