fn main() {
    let input = "1,3,3,4,5,1,1,1,1,1,1,2,1,4,1,1,1,5,2,2,4,3,1,1,2,5,4,2,2,3,1,2,3,2,1,1,4,4,2,4,4,1,2,4,3,3,3,1,1,3,4,5,2,5,1,2,5,1,1,1,3,2,3,3,1,4,1,1,4,1,4,1,1,1,1,5,4,2,1,2,2,5,5,1,1,1,1,2,1,1,1,1,3,2,3,1,4,3,1,1,3,1,1,1,1,3,3,4,5,1,1,5,4,4,4,4,2,5,1,1,2,5,1,3,4,4,1,4,1,5,5,2,4,5,1,1,3,1,3,1,4,1,3,1,2,2,1,5,1,5,1,3,1,3,1,4,1,4,5,1,4,5,1,1,5,2,2,4,5,1,3,2,4,2,1,1,1,2,1,2,1,3,4,4,2,2,4,2,1,4,1,3,1,3,5,3,1,1,2,2,1,5,2,1,1,1,1,1,5,4,3,5,3,3,1,5,5,4,4,2,1,1,1,2,5,3,3,2,1,1,1,5,5,3,1,4,4,2,4,2,1,1,1,5,1,2,4,1,3,4,4,2,1,4,2,1,3,4,3,3,2,3,1,5,3,1,1,5,1,2,2,4,4,1,2,3,1,2,1,1,2,1,1,1,2,3,5,5,1,2,3,1,3,5,4,2,1,3,3,4";

    let number_of_days = 256;

    let fishes = input
        .split(",")
        .map(|x| {
            return Fish {
                value: x.parse::<u32>().unwrap(),
            };
        })
        .collect::<Vec<_>>();

    let mut turns = Vec::new();

    for day in 1..number_of_days + 1 {
        let turn = Turn {
            day: day,
            fishes_to_add: 0,
        };
        turns.push(turn);
    }

    turns = calculate_turns(&fishes, &turns, 0, number_of_days);

    for day in 1..number_of_days + 1 {
        let current_turn = turns.iter().find(|x| x.day == day).unwrap();
        turns = calculate_turns_2(current_turn.fishes_to_add, 8, &turns, day, number_of_days);
    }

    let total_fish = (fishes.len() as u64) + turns.iter().map(|t| t.fishes_to_add).sum::<u64>();
    println!("Result: {}", total_fish);
}

fn calculate_turns(fishes: &Vec<Fish>, t: &Vec<Turn>, from_day: u32, last_day: u32) -> Vec<Turn> {
    let mut turns = t.clone();

    for fish in fishes {
        let from_day = from_day + fish.value + 1;
        let step = 7;

        for day in (from_day..last_day + 1).step_by(step) {
            let turn = turns.iter_mut().find(|x| x.day == day).unwrap();
            turn.fishes_to_add += 1;
        }
    }

    return turns;
}

fn calculate_turns_2(
    fishes_to_use: u64,
    fish_value: u32,
    t: &Vec<Turn>,
    from_day: u32,
    last_day: u32,
) -> Vec<Turn> {
    let mut turns = t.clone();

    let from_day = from_day + fish_value + 1;
    let step = 7;

    for day in (from_day..last_day + 1).step_by(step) {
        let turn = turns.iter_mut().find(|x| x.day == day).unwrap();
        turn.fishes_to_add += fishes_to_use;
    }

    return turns;
}

#[derive(Debug, Clone)]
struct Fish {
    value: u32,
}

#[derive(Debug, Clone)]
struct Turn {
    day: u32,
    fishes_to_add: u64,
}
