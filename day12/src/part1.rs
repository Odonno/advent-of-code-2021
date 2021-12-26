fn main() {
    let input = "\
CI-hb
IK-lr
vr-tf
lr-end
XP-tf
start-vr
lr-io
hb-qi
end-CI
tf-YK
end-YK
XP-lr
XP-vr
lr-EU
tf-CI
EU-vr
start-tf
YK-hb
YK-vr
start-EU
lr-CI
hb-XP
XP-io
tf-EU";

    let lines = input.lines();

    let mut cave_relations = Vec::new();

    for line in lines {
        let splitted = line.split('-').collect::<Vec<_>>();

        let from_name = splitted[0];
        let to_name = splitted[1];

        let from = Cave {
            name: from_name.to_string(),
            cave_type: if from_name == from_name.to_lowercase() {
                CaveType::Small
            } else {
                CaveType::Big
            },
        };

        let to = Cave {
            name: to_name.to_string(),
            cave_type: if to_name == to_name.to_lowercase() {
                CaveType::Small
            } else {
                CaveType::Big
            },
        };

        cave_relations.push(CaveRelation { from, to });
    }

    let mut current_working_paths: Vec<Path> = Vec::new();
    let mut final_paths: Vec<Path> = Vec::new();

    let from_start_relations = &cave_relations
        .iter()
        .filter(|cave_relation| {
            cave_relation.from.name == "start" || cave_relation.to.name == "start"
        })
        .collect::<Vec<_>>();

    for relation in from_start_relations {
        let path = Path {
            caves: vec![relation.from.clone(), relation.to.clone()],
        };
        current_working_paths.push(path);
    }

    display_paths(&current_working_paths);

    loop {
        let mut new_working_path = Vec::new();

        for working_path in current_working_paths {
            let last_cave = working_path.caves.last().unwrap();

            let new_cave_relations = &cave_relations
                .iter()
                .map(|cave_relation| {
                    let is_from = cave_relation.from.name == last_cave.name;
                    let is_to = cave_relation.to.name == last_cave.name;

                    if is_from {
                        if cave_relation.to.cave_type == CaveType::Big {
                            return Some(cave_relation.to.clone());
                        }

                        let is_already_in_path = working_path
                            .caves
                            .iter()
                            .any(|cave| cave.name == cave_relation.to.name);

                        if !is_already_in_path {
                            return Some(cave_relation.to.clone());
                        }
                    }
                    if is_to {
                        if cave_relation.from.cave_type == CaveType::Big {
                            return Some(cave_relation.from.clone());
                        }

                        let is_already_in_path = working_path
                            .caves
                            .iter()
                            .any(|cave| cave.name == cave_relation.from.name);

                        if !is_already_in_path {
                            return Some(cave_relation.from.clone());
                        }
                    }

                    return None;
                })
                .filter(|cave| cave.is_some())
                .map(|cave| cave.unwrap())
                .collect::<Vec<_>>();

            for cave in new_cave_relations {
                let mut new_path = working_path.clone();
                new_path.caves.push(cave.clone());

                if cave.name == "end" {
                    final_paths.push(new_path);
                } else {
                    new_working_path.push(new_path);
                }
            }
        }

        current_working_paths = new_working_path;

        if current_working_paths.is_empty() {
            break;
        }
    }

    let number_of_possible_paths = final_paths.iter().count();

    println!("Number of possible paths: {}", number_of_possible_paths);
}

fn display_paths(paths: &Vec<Path>) {
    for path in paths.iter() {
        let caves = path
            .caves
            .iter()
            .map(|cave| cave.name.clone())
            .collect::<Vec<_>>();
        println!("{:?}", caves);
    }
}

#[derive(Debug, Clone)]
struct Path {
    caves: Vec<Cave>,
}

struct CaveRelation {
    from: Cave,
    to: Cave,
}

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    cave_type: CaveType,
}

#[derive(Debug, Clone, PartialEq)]
enum CaveType {
    Small,
    Big,
}
