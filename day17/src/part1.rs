use std::cmp::Ordering;

fn main() {
    let input = "target area: x=201..230, y=-99..-65";

    let target_area = parse_target_area(input);

    let probe_position = Position { x: 0, y: 0 };

    let mut target_positions = Vec::new();

    for y in target_area.bottom..=target_area.top {
        for x in target_area.left..=target_area.right {
            let position = Position { x, y };
            target_positions.push(position);
        }
    }

    let mut max_y = -1;
    let mut best_velocity: Option<Vector2> = None;

    for target_position in target_positions {
        let mut current_step = 0;

        loop {
            if current_step > 750 {
                break;
            }
            current_step += 1;

            let current_step_f32 = current_step as f32;

            let velocity_y_f32 = (current_step_f32 - 1.0) / 2.0
                + (target_position.y as f32 - probe_position.y as f32) / current_step_f32;

            if velocity_y_f32.fract() != 0.0 {
                continue;
            }

            let velocity_x = -1 as i32;
            let velocity_y = velocity_y_f32 as i32;

            let possible_highest_y = (1..=current_step)
                .map(|step| {
                    return (velocity_y * step) - (step * (step - 1) / 2);
                })
                .max()
                .unwrap();

            if possible_highest_y > max_y {
                max_y = possible_highest_y;
                best_velocity = Some(Vector2 {
                    x: velocity_x,
                    y: velocity_y,
                });
            }

            continue;

            let a = -1.0 / 2.0;
            let b = velocity_y_f32 + (1.0 / 2.0);
            let c = probe_position.y as f32;

            let delta = b * b - 4.0 * a * c;

            let solutions = match delta {
                delta if delta < 0.0 => vec![],
                0.0 => vec![-b / (2.0 * a)],
                delta => {
                    let delta_sqrt = delta.sqrt();
                    vec![(-b - delta_sqrt) / (2.0 * a), (-b + delta_sqrt) / (2.0 * a)]
                }
            };
            println!("solutions: {:?}", solutions);

            for s in solutions {
                if s.fract() != 0.0 {
                    continue;
                }

                let s = s as i32;

                if s > max_y {
                    max_y = s as i32;
                    best_velocity = Some(Vector2 {
                        x: velocity_x,
                        y: velocity_y,
                    });
                }
            }
        }
    }

    println!("Best velocity: {:?}", best_velocity);
    println!("Highest y: {:?}", max_y);
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

fn find_velocity_x(steps: i32, target_position_x: i32, probe_position_x: i32) -> Option<f32> {
    for o in 1..=2 {
        let remaining_steps = steps - o;

        for g in 0..=remaining_steps {
            let l = remaining_steps - g;

            let g_f32 = g as f32;
            let l_f32 = l as f32;

            let velocity_x_f32 = (g_f32 - l_f32) / 2.0
                + (target_position_x as f32 - probe_position_x as f32) / steps as f32;

            if velocity_x_f32.fract() == 0.0 {
                return Some(velocity_x_f32);
            }
        }
    }

    return None;
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
