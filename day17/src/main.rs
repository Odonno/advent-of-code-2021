use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    let input = "target area: x=201..230, y=-99..-65";

    let target_area = parse_target_area(input);

    let probe_position = Position { x: 0, y: 0 };

    let mut possibilities = Vec::new();

    for y in -250..=250 {
        for x in 0..=700 {
            let mut step = Step {
                position: probe_position.clone(),
                velocity: Vector2 { x, y },
            };

            loop {
                step = apply_velocity(&step);

                if step.position.is_in_area(&target_area) {
                    possibilities.push(Vector2 { x, y });
                    break;
                }

                if step.position.is_below_area(&target_area) {
                    break;
                }
            }
        }
    }

    let mut distinct_possibilies = possibilities
        .iter()
        .unique_by(|v| format!("{},{}", v.x, v.y))
        .collect::<Vec<_>>();

    println!("distinct_possibilies: ");

    distinct_possibilies.sort_by(|a, b| {
        let a_abs = a.x * 1000 + a.y;
        let b_abs = b.x * 1000 + b.y;

        a_abs.cmp(&b_abs)
    });

    for p in &distinct_possibilies {
        println!("{},{}", p.x, p.y);
    }

    println!("possibilities: {}", distinct_possibilies.len());
}

fn parse_target_area(input: &str) -> Area {
    let input = input.replace("target area: ", "");
    let mut parts = input.split(',');

    let x_part = parts.next().unwrap().trim().replace("x=", "");
    let y_part = parts.next().unwrap().trim().replace("y=", "");

    let x_part = x_part.split("..").collect::<Vec<&str>>();
    let y_part = y_part.split("..").collect::<Vec<&str>>();

    Area {
        left: x_part[0].parse::<i32>().unwrap(),
        right: x_part[1].parse::<i32>().unwrap(),
        bottom: y_part[0].parse::<i32>().unwrap(),
        top: y_part[1].parse::<i32>().unwrap(),
    }
}

fn apply_velocity(previous_step: &Step) -> Step {
    let mut next_step = previous_step.clone();

    // apply velocity on probe
    next_step.position.x += next_step.velocity.x;
    next_step.position.y += next_step.velocity.y;

    // apply drag
    match next_step.velocity.x.cmp(&0) {
        Ordering::Less => next_step.velocity.x += 1,
        Ordering::Greater => next_step.velocity.x -= 1,
        Ordering::Equal => {}
    }

    // apply gravity
    next_step.velocity.y -= 1;

    next_step
}

#[derive(Debug, Clone)]
struct Area {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_in_area(&self, target_area: &Area) -> bool {
        return self.x >= target_area.left
            && self.x <= target_area.right
            && self.y >= target_area.bottom
            && self.y <= target_area.top;
    }

    fn is_below_area(&self, target_area: &Area) -> bool {
        return self.y < target_area.bottom;
    }
}

#[derive(Debug, Clone)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Step {
    position: Position,
    velocity: Vector2,
}
