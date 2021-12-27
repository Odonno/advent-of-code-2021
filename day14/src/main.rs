use std::collections::HashMap;

fn main() {
    let input = "\
BVBNBVPOKVFHBVCSHCFO

SO -> V
PB -> P
HV -> N
VF -> O
KS -> F
BB -> C
SH -> H
SB -> C
FS -> F
PV -> F
BC -> K
SF -> S
NO -> O
SK -> C
PO -> N
VK -> F
FC -> C
VV -> S
SV -> S
HH -> K
FH -> K
HN -> O
NP -> F
PK -> N
VO -> K
NC -> C
KP -> B
CS -> C
KO -> F
BK -> N
OO -> N
CF -> H
KN -> C
BV -> S
OK -> O
CN -> F
OP -> O
VP -> N
OC -> P
NH -> C
VN -> S
VC -> B
NF -> H
FO -> H
CC -> B
KB -> N
CP -> N
HK -> N
FB -> H
BH -> V
BN -> N
KC -> F
CV -> K
SP -> V
VS -> P
KF -> S
CH -> V
NS -> N
HS -> O
CK -> K
NB -> O
OF -> K
VB -> N
PS -> B
KH -> P
BS -> C
VH -> C
KK -> F
FN -> F
BP -> B
HF -> O
HB -> V
OV -> H
NV -> N
HO -> S
OS -> H
SS -> K
BO -> V
OB -> K
HP -> P
CO -> B
PP -> K
HC -> N
BF -> S
NK -> S
ON -> P
PH -> C
FV -> H
CB -> H
PC -> K
FF -> P
PN -> P
NN -> O
PF -> F
SC -> C
FK -> K
SN -> K
KV -> P
FP -> B
OH -> F";

    let mut lines = input.lines();

    let polymer = lines.next().unwrap().to_string();
    lines.next(); // empty line

    let mut rules = HashMap::new();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" -> ").collect();

        let pair = parts[0].to_string();

        let insert_char = parts[1];
        let insert_char = insert_char.chars().collect::<Vec<_>>();
        let insert_char = insert_char[0];

        let mut map = HashMap::new();
        map.insert(insert_char, 1);

        rules.insert((pair, 1), map);
    }

    let mut polymer_map = HashMap::new();

    for c in polymer.chars() {
        let count = polymer_map.entry(c).or_insert(0);
        *count += 1;
    }

    let number_of_steps = 40;

    for i in 0..polymer.len() - 1 {
        let before = polymer.chars().nth(i).unwrap();
        let after = polymer.chars().nth(i + 1).unwrap();

        let map = execute_rule(&mut rules, before, after, number_of_steps);
        for (k, v) in map {
            let count = polymer_map.entry(k).or_insert(0);
            *count += v;
        }
    }

    let mut values = polymer_map.values().map(|f| *f as u64).collect::<Vec<_>>();
    values.sort();

    let least_common_value = values.first().unwrap();
    let most_common_value = values.last().unwrap();

    let result = most_common_value - least_common_value;
    println!("Result: {}", result);
}

fn execute_rule(
    rules: &mut HashMap<(String, i32), HashMap<char, u64>>,
    before: char,
    after: char,
    remaining_steps: i32,
) -> HashMap<char, u64> {
    if remaining_steps == 0 {
        return HashMap::new();
    }

    let pair = format!("{}{}", before, after);
    let rule = rules.get(&(pair.clone(), remaining_steps));

    if rule.is_some() {
        return rule.unwrap().clone();
    }

    let rule = rules.get(&(pair.clone(), 1)).unwrap().clone();

    let inserted_char = rule.keys().next().unwrap().clone();

    let left_map = execute_rule(rules, before, inserted_char, remaining_steps - 1);
    let right_map = execute_rule(rules, inserted_char, after, remaining_steps - 1);

    let mut result = HashMap::new();

    for (k, v) in left_map {
        let count = result.entry(k).or_insert(0);
        *count += v;
    }
    for (k, v) in right_map {
        let count = result.entry(k).or_insert(0);
        *count += v;
    }
    for (k, v) in rule {
        let count = result.entry(k).or_insert(0);
        *count += v;
    }

    rules.insert((pair, remaining_steps), result.clone());

    result
}
