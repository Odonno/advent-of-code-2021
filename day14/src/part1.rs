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

    let mut polymer = lines.next().unwrap().to_string();
    lines.next(); // empty line

    let mut rules: Vec<InsertionRule> = Vec::new();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" -> ").collect();

        let from = parts[0];
        let from = from.chars().collect::<Vec<_>>();

        let to = parts[1];
        let to = to.chars().collect::<Vec<_>>();

        let before = from[0];
        let after = from[1];
        let insert_char = to[0];

        let rule = InsertionRule {
            before,
            after,
            insert_char,
        };
        rules.push(rule);
    }

    let number_of_steps = 10;

    for _ in 1..=number_of_steps {
        let copied_polymer = polymer.to_string();
        let polymer_len = polymer.len();

        for i in 0..polymer_len - 1 {
            let before = copied_polymer.chars().nth(i).unwrap();
            let after = copied_polymer.chars().nth(i + 1).unwrap();

            let rule = rules
                .iter()
                .find(|rule| rule.before == before && rule.after == after)
                .unwrap();

            polymer.insert((i * 2) + 1, rule.insert_char);
        }
    }

    let mut polymer_map = HashMap::new();

    for c in polymer.chars() {
        let current_value = polymer_map.entry(c).or_insert(0);
        *current_value += 1;
    }

    let least_common_value = polymer_map.iter().min_by_key(|(_, v)| *v).unwrap().1;
    let most_common_value = polymer_map.iter().max_by_key(|(_, v)| *v).unwrap().1;

    let result = most_common_value - least_common_value;
    println!("Result: {}", result);
}

struct InsertionRule {
    before: char,
    after: char,
    insert_char: char,
}
