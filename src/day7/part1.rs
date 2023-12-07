use std::collections::HashMap;
static CARD: HashMap<char, u16> = HashMap::<char, u16>::from([
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
]);
enum CardCombinations {
    HighCard(u16),
    Pair(u16),
    TwoPairs(u16, u16),
    ThreeOfAKind(u16),
    FullHouse(u16, u16),
    FourOfAKind(u16),
    FiveOfAKind(u16),
}
#[derive(Debug)]
struct Player{
    hand: String,
    combo: u16,
    high: u16,
    bid: u16,
}
pub fn run(input: &str) -> u64 {
    let a = parse(input);
    set_hand_combo(a[0]);
    dbg!(a);
    0
}
fn parse(input: &str) -> Vec<Player> {
    input.lines()
        .map(|x| x.split_once(" ").unwrap())
        .map(|(cards, bid)| Player {hand: cards.to_string(), bid: bid.parse::<u16>().unwrap(), combo: 0, high: 0})
        .collect()
}
fn set_hand_combo(player: Player) {
    for i in player.hand.chars() {
        println!("{}", i);
    }
}