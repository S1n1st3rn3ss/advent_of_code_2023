use std::cmp::{Ordering, Reverse};
// use std::collections::HashMap;
use hashbrown::HashMap;

const CARD: [(char, u16); 13] = [
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('T', 10),
    ('J', 11),
    ('Q', 12),
    ('K', 13),
    ('A', 14),
];
#[derive(Debug, Eq, PartialEq, Default)]
struct Player {
    hand: Vec<u16>,
    combo: u16,
    bid: u16,
}
impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.combo == other.combo {
            return self.hand.cmp(&other.hand).reverse()
        }
        self.combo.cmp(&other.combo).reverse()
    }
}
impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}
pub fn run(input: &str) -> u64 {
    let card_hash: HashMap<char, u16> = HashMap::<char, u16>::from(CARD);
    let mut input = parse(input);
    input.sort();
    let mut sum: u64 = 0;
    for (idx, player) in input.iter().enumerate() {
        let rank = idx + 1;
        sum += (rank * player.bid as usize) as u64;
    }
    sum
}
fn parse(input: &str) -> Vec<Player> {
    let card_hash: HashMap<char, u16> = HashMap::<char, u16>::from(CARD);
    input
        .trim()
        .lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(cards, bid)| Player {hand: cards.chars().map(|x| card_hash.get(&x).unwrap().to_owned()).collect(), bid: bid.parse::<u16>().unwrap(), combo: 0})
        .map(|x| set_hand_combo(x, &card_hash)).collect()
}
fn set_hand_combo(input: Player, card_hash: &HashMap<char, u16>) -> Player {
    let mut cards: HashMap<u16, u16> = HashMap::new();
    for i in &input.hand{
        cards.entry(*i).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut player: Player = Default::default();
    for (card, count) in &cards {
        if cards.len() == 5 {
            player.combo = 1;
            break;
        }
        match count {
            5 => {
                player.combo = 7;
                break;
            },
            4 => {
                player.combo = 6;
                break;
            }
            3 => {
                match cards.len() {
                    2 => {
                        player.combo = 5;
                        break;
                    }
                    3 => {
                        player.combo = 4;
                        break;
                    }
                    _ => {panic!("???")}
                }
            }
            2 => {
                match cards.len() {
                    2 => {
                        player.combo = 5;
                        break;
                    }
                    3 => {
                        player.combo = 3;
                        break;
                    }
                    4 => {
                        player.combo = 2;
                        break;
                    }
                    _ => panic!("???")
                }
            }
            1 => player.combo = 11,
            _ => {}
        }
    }
    player.hand = input.hand;
    player.bid = input.bid;
    player
}