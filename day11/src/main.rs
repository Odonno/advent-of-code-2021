#[derive(Debug, Clone, Copy)]
struct Octopus {
    x: i32,
    y: i32,
    energy: i32,
}

struct StepOutput {
    number_of_flashes: i32,
    octopuses: Vec<Octopus>,
}

fn main() {
    let input = "3113284886
2851876144
2774664484
6715112578
7146272153
6256656367
3148666245
3857446528
7322422833
8152175168";
    
    // get list of octopuses
    let mut octopuses = Vec::new();

    let mut y = 0;

    for line in input.lines() {
        let mut x = 0;

        for c in line.chars() {
            let energy = c.to_digit(10).unwrap() as i32;

            let octopus = Octopus {
                x,
                y,
                energy,
            };

            octopuses.push(octopus);

            x += 1;
        }

        y += 1;
    }

    let number_of_steps = 100;
    let mut number_of_flashes = 0;

    for _ in 0..number_of_steps {
        let output = execute_step(octopuses);

        number_of_flashes += output.number_of_flashes;
        octopuses = output.octopuses;
    }

    println!("Number of flashes: {}", number_of_flashes);
}

fn execute_step(o: Vec<Octopus>) -> StepOutput {
    let mut octopuses = o.clone();

    // energize all octopuses
    for octopus in &mut octopuses {
        octopus.energy += 1;
    }

    // loop on octopuses to detect those who flashed due to adjacent octopuses
    loop {
        let mut flashed_octopuses = Vec::new();

        let cloned_octopuses = octopuses.clone();

        for octopus in &mut octopuses {
            let flashed_neighbours = cloned_octopuses.iter()
                .filter(|o| 
                    (o.x == octopus.x - 1 || o.x == octopus.x + 1) &&
                    (o.y == octopus.y - 1 || o.y == octopus.y + 1)
                )
                .filter(|o| o.energy == -1);

            if (octopus.energy + flashed_neighbours.count() as i32) > 9 {
                octopus.energy = -1;
                flashed_octopuses.push(octopus.clone());
            }
        }

        // increase energy of adjacent octopuses
        for flashed_octopus in &flashed_octopuses {
            for octopus in &mut octopuses {
                if octopus.energy == -1 {
                    continue;
                }

                if (flashed_octopus.x == octopus.x - 1 || flashed_octopus.x == octopus.x + 1) &&
                    (flashed_octopus.y == octopus.y - 1 || flashed_octopus.y == octopus.y + 1) {
                    octopus.energy += 1;
                }
            }
        }

        if flashed_octopuses.is_empty() {
            break;
        }
    }

    let number_of_flashes = octopuses.iter().filter(|o| o.energy == -1).count() as i32;

    // reset octopuses who have flashed
    for octopus in &mut octopuses {
        if octopus.energy == -1 {
            octopus.energy = 0;
        }
    }

    return StepOutput { number_of_flashes, octopuses };
}