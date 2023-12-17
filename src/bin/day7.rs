use std::{collections::HashMap, fs};

use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum HandType {
    HIGH_CARD,
    ONE_PAIR,
    TWO_PAIR,
    THREE_OF_A_KIND,
    FULL_HOUSE,
    FOUR_OF_A_KIND,
    FIVE_OF_A_KIND,
}

fn calculate_type(hand: &Vec<u32>) -> HandType {
    let mut map = HashMap::new();
    for c in hand {
        map.entry(c).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut v = map.values().collect::<Vec<_>>();
    v.sort();
    if *v[v.len() - 1] == 5 {
        return HandType::FIVE_OF_A_KIND;
    } else if *v[v.len() - 1] == 4 {
        return HandType::FOUR_OF_A_KIND;
    } else if v.len() == 2 && *v[v.len() - 1] == 3 && *v[0] == 2 {
        return HandType::FULL_HOUSE;
    } else if *v[v.len() - 1] == 3 {
        return HandType::THREE_OF_A_KIND;
    } else if *v[v.len() - 1] == 2 && *v[v.len() - 2] == 2 {
        return HandType::TWO_PAIR;
    } else if *v[v.len() - 1] == 2 {
        return HandType::ONE_PAIR;
    } else {
        return HandType::HIGH_CARD;
    }
}

fn calculate_type2(hand: &Vec<u32>) -> HandType {
    let mut map = HashMap::new();
    let mut jokers = 0;
    for c in hand {
        if *c == 1 {
            jokers += 1;
        } else {
            map.entry(*c).and_modify(|x| *x += 1).or_insert(1);
        }
    }
    let mut v = map.values().map(|x| *x).collect::<Vec<_>>();
    v.sort();
    let l = v.len();
    if v.is_empty() {
        v.push(jokers);
    } else {
        v[l - 1] = v[l - 1] + jokers;
    }
    if v[v.len() - 1] == 5 {
        return HandType::FIVE_OF_A_KIND;
    } else if v[v.len() - 1] == 4 {
        return HandType::FOUR_OF_A_KIND;
    } else if v.len() == 2 && v[v.len() - 1] == 3 && v[0] == 2 {
        return HandType::FULL_HOUSE;
    } else if v[v.len() - 1] == 3 {
        return HandType::THREE_OF_A_KIND;
    } else if v[v.len() - 1] == 2 && v[v.len() - 2] == 2 {
        return HandType::TWO_PAIR;
    } else if v[v.len() - 1] == 2 {
        return HandType::ONE_PAIR;
    } else {
        return HandType::HIGH_CARD;
    }
}

fn get_card_value(card: char) -> u32 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unexpected card {card}"),
    }
}

fn get_card_value2(card: char) -> u32 {
    match card {
        '2'..='9' => card.to_digit(10).unwrap(),
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("unexpected card {card}"),
    }
}

fn compare_cards(cards1: &Vec<u32>, cards2: &Vec<u32>) -> std::cmp::Ordering {
    for i in 0..cards1.len() {
        let c1 = cards1[i];
        let c2 = cards2[i];
        let compare = c1.cmp(&c2);
        if compare != std::cmp::Ordering::Equal {
            return compare;
        }
    }
    std::cmp::Ordering::Equal
}

fn part1(lines: &Vec<&str>) -> u64 {
    let mut hands = lines
        .iter()
        .map(|x| {
            let tokens = x.split(" ").collect::<Vec<_>>();
            (
                tokens[0]
                    .chars()
                    .map(|c| get_card_value(c))
                    .collect::<Vec<_>>(),
                tokens[1].parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    hands.sort_by(|t1, t2| {
        let type1 = calculate_type(&t1.0).ordinal();
        let type2 = calculate_type(&t2.0).ordinal();
        if type1 < type2 {
            std::cmp::Ordering::Less
        } else if type1 > type2 {
            std::cmp::Ordering::Greater
        } else {
            compare_cards(&t1.0, &t2.0)
        }
    });
    for i in 0..hands.len() {
        res += (i + 1) as u64 * hands[i].1;
    }
    res
}

fn part2(lines: &Vec<&str>) -> u64 {
    let mut hands = lines
        .iter()
        .map(|x| {
            let tokens = x.split(" ").collect::<Vec<_>>();
            (
                tokens[0]
                    .chars()
                    .map(|c| get_card_value2(c))
                    .collect::<Vec<_>>(),
                tokens[1].parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut res = 0;
    hands.sort_by(|t1, t2| {
        let type1 = calculate_type2(&t1.0).ordinal();
        let type2 = calculate_type2(&t2.0).ordinal();
        if type1 < type2 {
            std::cmp::Ordering::Less
        } else if type1 > type2 {
            std::cmp::Ordering::Greater
        } else {
            compare_cards(&t1.0, &t2.0)
        }
    });
    for i in 0..hands.len() {
        res += (i + 1) as u64 * hands[i].1;
    }
    res
}

fn main() {
    let content = fs::read_to_string("inputs/day7.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    // part 1
    // println!("{}", part1(&lines));

    // part 2
    println!("{}", part2(&lines));
}
