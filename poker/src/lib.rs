#![allow(dead_code)]
#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use std::str::FromStr;

static RANKS: [&str; 13] = [
    "A", "K", "Q", "J", "10", "9", "8", "7", "6", "5", "4", "3", "2",
];

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Suit {
    Clubs,
    Diamond,
    Hearts,
    Spades,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Card {
    s: Suit,
    r: u8,
}

#[derive(PartialEq, Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    rank: Option<HandRank>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum HandRank {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandRank {
    pub fn iterator() -> Iter<'static, HandRank> {
        use HandRank::*;
        static RANKS: [HandRank; 9] = [
            StraightFlush,
            FourOfAKind,
            FullHouse,
            Flush,
            Straight,
            ThreeOfAKind,
            TwoPair,
            OnePair,
            HighCard,
        ];

        RANKS.iter()
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &HandRank) -> Option<Ordering> {
        let self_idx = HandRank::iterator().position(|r| r == self).unwrap();
        let other_idx = HandRank::iterator().position(|r| r == other).unwrap();
        self_idx.partial_cmp(&other_idx)
    }
}

impl FromStr for Suit {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rv = match s {
            "C" => Suit::Clubs,
            "D" => Suit::Diamond,
            "H" => Suit::Hearts,
            "S" => Suit::Spades,
            _ => return Err(format!("Unkown suit: {}", s)),
        };

        Ok(rv)
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rank = match s[0..s.len() - 1].parse::<String>() {
            Ok(r) => RANKS
                .iter()
                .position(|&elm| elm == r)
                .unwrap_or_else(|| panic!("Unknown rank: {}", r)),
            _ => return Err(format!("Uknown rank: {}", s)),
        };

        let suit = s[s.len() - 1..].parse::<Suit>()?;

        Ok(Card {
            s: suit,
            r: rank as u8,
        })
    }
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = vec![];

        for c in s.split_ascii_whitespace() {
            let c = c.parse::<Card>()?;
            cards.push(c);
        }

        cards.sort_by(|a, b| a.r.cmp(&b.r));

        let mut h = Hand { cards, rank: None };

        h.rank = Some(h.get_rank());
        Ok(h)
    }
}

impl Hand {
    fn all_same_suit(&self) -> bool {
        let suits = self.cards.iter().map(|c| c.s).collect::<HashSet<_>>();
        suits.len() == 1
    }

    fn all_increasing_rank(&self) -> bool {
        let mut ranks = self.cards.iter().map(|c| c.r).collect::<Vec<_>>();
        ranks.sort_unstable();

        for i in 1..self.cards.len() {
            if ranks[i] != ranks[i - 1] + 1 {
                return false;
            }
        }

        true
    }

    fn four_incr_rank_and_ace(&self) -> bool {
        let h_without_ace = Hand {
            cards: self
                .cards
                .clone()
                .into_iter()
                .filter(|c| c.r != 0)
                .collect(),
            rank: None,
        };

        if h_without_ace.cards.len() != 4 {
            return false;
        }

        h_without_ace.all_increasing_rank()
    }

    fn is_four_of_kind(&self) -> bool {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for rank in self.cards.iter().map(|c| c.r) {
            (*counts.entry(rank).or_default()) += 1;
        }

        for (_, count) in counts {
            if count == 4 {
                return true;
            }
        }

        false
    }

    fn is_full_house(&self) -> bool {
        let mut r_counts: HashMap<u8, u8> = HashMap::new();
        for rank in self.cards.iter().map(|c| c.r) {
            (*r_counts.entry(rank).or_default()) += 1;
        }

        let mut r_counts = r_counts.into_iter().map(|(_, c)| c).collect::<Vec<_>>();
        r_counts.sort_unstable();
        r_counts == [2, 3]
    }

    fn is_flush(&self) -> bool {
        let hs = self.cards.iter().map(|c| c.s).collect::<HashSet<_>>();
        hs.len() == 1
    }

    fn is_three_of_a_kind(&self) -> bool {
        let mut r_counts: HashMap<u8, u8> = HashMap::new();
        for rank in self.cards.iter().map(|c| c.r) {
            (*r_counts.entry(rank).or_default()) += 1;
        }

        r_counts.into_iter().map(|(_, c)| c).any(|c| c == 3)
    }

    fn is_two_pair(&self) -> bool {
        let mut r_counts: HashMap<u8, u8> = HashMap::new();
        for rank in self.cards.iter().map(|c| c.r) {
            (*r_counts.entry(rank).or_default()) += 1;
        }

        let mut r_counts = r_counts.into_iter().map(|(_, c)| c).collect::<Vec<_>>();
        r_counts.sort_unstable();
        r_counts == [1, 2, 2]
    }

    fn is_one_pair(&self) -> bool {
        let mut r_counts: HashMap<u8, u8> = HashMap::new();
        for rank in self.cards.iter().map(|c| c.r) {
            (*r_counts.entry(rank).or_default()) += 1;
        }

        let mut r_counts = r_counts.into_iter().map(|(_, c)| c).collect::<Vec<_>>();
        r_counts.sort_unstable();
        r_counts == [1, 1, 1, 2]
    }

    fn get_one_pair(&self) -> Option<[Card; 2]> {
        for card in &self.cards {
            let mut cards_same_rank = self
                .cards
                .iter()
                .filter(|c| c.r == card.r)
                .copied()
                .collect::<Vec<_>>();

            if cards_same_rank.len() == 2 {
                cards_same_rank.sort_by(|a, b| a.r.cmp(&b.r));
                return Some([cards_same_rank[0], cards_same_rank[1]]);
            }
        }

        None
    }

    fn get_two_pairs(&self) -> Option<[[Card; 2]; 2]> {
        let mut rv = vec![];
        let mut seen_rank: Vec<u8> = vec![];

        for card in &self.cards {
            let mut cards_same_rank = self
                .cards
                .iter()
                .filter(|c| !seen_rank.contains(&c.r) && c.r == card.r)
                .copied()
                .collect::<Vec<_>>();

            if cards_same_rank.len() == 2 {
                seen_rank.push(cards_same_rank[0].r);
                cards_same_rank.sort_by(|a, b| a.r.cmp(&b.r));
                let pair: [Card; 2] = [cards_same_rank[0], cards_same_rank[1]];
                rv.push(pair);
            }
        }

        match rv.len() {
            2 => Some([rv[0], rv[1]]),
            _ => None,
        }
    }

    fn get_triplet(&self) -> ([Card; 3], [Card; 2]) {
        let mut kicker = vec![];
        let mut triplet = vec![];
        let cards_by_rank: HashMap<u8, Vec<Card>> =
            self.cards.iter().fold(HashMap::new(), |mut acc, item| {
                (*acc.entry(item.r).or_default()).push(*item);
                acc
            });

        for (_rank, cards) in cards_by_rank {
            if cards.len() == 3 {
                triplet = cards;
            } else {
                kicker.extend(cards);
            }
        }

        triplet.sort_by(|a, b| a.r.cmp(&b.r));
        kicker.sort_by(|a, b| a.r.cmp(&b.r));

        (triplet.try_into().unwrap(), kicker.try_into().unwrap())
    }

    fn get_rank(&self) -> HandRank {
        if self.all_same_suit() && self.all_increasing_rank() {
            return HandRank::StraightFlush;
        }

        if self.is_four_of_kind() {
            return HandRank::FourOfAKind;
        }

        if self.is_full_house() {
            return HandRank::FullHouse;
        }

        if self.is_flush() {
            return HandRank::Flush;
        }

        if self.all_increasing_rank() || self.four_incr_rank_and_ace() {
            return HandRank::Straight;
        }

        if self.is_two_pair() {
            return HandRank::TwoPair;
        }

        if self.is_three_of_a_kind() {
            return HandRank::ThreeOfAKind;
        }

        if self.is_one_pair() {
            return HandRank::OnePair;
        }

        HandRank::HighCard
    }

    fn get_full_house_cards(&self) -> ([Card; 3], [Card; 2]) {
        let cards_by_rank: HashMap<u8, Vec<Card>> =
            self.cards.iter().fold(HashMap::new(), |mut acc, item| {
                (*acc.entry(item.r).or_default()).push(*item);
                acc
            });

        let triplet = cards_by_rank
            .iter()
            .filter_map(|(_rank, cards)| (cards.len() == 3).then(|| cards.clone()))
            .flatten()
            .collect::<Vec<_>>();

        let pair = cards_by_rank
            .into_iter()
            .filter_map(|(_rank, cards)| (cards.len() == 2).then(|| cards))
            .flatten()
            .collect::<Vec<_>>();

        (triplet.try_into().unwrap(), pair.try_into().unwrap())
    }

    fn get_four_of_a_kind_cards(&self) -> ([Card; 4], [Card; 1]) {
        let cards_by_rank: HashMap<u8, Vec<Card>> =
            self.cards.iter().fold(HashMap::new(), |mut acc, item| {
                (*acc.entry(item.r).or_default()).push(*item);
                acc
            });

        let four = cards_by_rank
            .iter()
            .filter_map(|(_rank, cards)| (cards.len() == 4).then(|| cards.clone()))
            .flatten()
            .collect::<Vec<_>>();

        let one = cards_by_rank
            .into_iter()
            .filter_map(|(_rank, cards)| (cards.len() == 1).then(|| cards))
            .flatten()
            .collect::<Vec<_>>();

        (four.try_into().unwrap(), one.try_into().unwrap())
    }
}

fn cmp_cards_seq<'a, 'b>(
    a: impl IntoIterator<Item = &'a Card>,
    b: impl IntoIterator<Item = &'b Card>,
) -> Option<Ordering> {
    for (a_card, b_card) in a.into_iter().zip(b.into_iter()) {
        match a_card.r.cmp(&b_card.r) {
            Ordering::Equal => continue,
            ordering => return Some(ordering),
        }
    }

    Some(Ordering::Equal)
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let self_rank = self.rank.unwrap();
        let other_rank = other.rank.unwrap();

        if self_rank != other_rank {
            return self_rank.partial_cmp(&other_rank);
        }

        match self.rank.unwrap() {
            HandRank::HighCard => cmp_cards_seq(&self.cards, &other.cards),

            HandRank::OnePair => cmp_cards_seq(
                &self.get_one_pair().expect("Hand doesn't contain one pair."),
                &other
                    .get_one_pair()
                    .expect("Hand doesn't contain one pair."),
            ),

            HandRank::TwoPair => {
                let self_pairs = self.get_two_pairs().expect("Couldn't find two pairs.");
                let oth_pairs = other.get_two_pairs().expect("Couldn't find two pairs.");

                for (pair_a, pair_b) in self_pairs.iter().zip(oth_pairs.iter()) {
                    match cmp_cards_seq(pair_a, pair_b) {
                        Some(Ordering::Equal) => continue,
                        rv => return rv,
                    }
                }

                let kicker_a = (&self.cards.iter().copied().collect::<HashSet<Card>>()
                    - &(self_pairs.into_iter().flatten().collect::<HashSet<Card>>()))
                    .into_iter()
                    .next()
                    .unwrap();

                let kicker_b = (&other.cards.iter().copied().collect::<HashSet<Card>>()
                    - &(oth_pairs.into_iter().flatten().collect::<HashSet<Card>>()))
                    .into_iter()
                    .next()
                    .unwrap();

                Some(kicker_a.r.cmp(&kicker_b.r))
            }

            HandRank::ThreeOfAKind => {
                let (a_triplet, a_kicker) = self.get_triplet();
                let (b_triplet, b_kicker) = other.get_triplet();

                match cmp_cards_seq(&a_triplet, &b_triplet) {
                    Some(Ordering::Equal) => cmp_cards_seq(&a_kicker, &b_kicker),

                    o => o,
                }
            }

            HandRank::Straight => match (self.all_increasing_rank(), other.all_increasing_rank()) {
                (true, true) => cmp_cards_seq(&self.cards, &other.cards),
                _ => {
                    let is_five_high_straight =
                        |cards: &Vec<Card>| cards[0].r == 0 && cards[cards.len() - 1].r == 12;

                    if is_five_high_straight(&self.cards) && is_five_high_straight(&other.cards) {
                        return Some(Ordering::Equal);
                    }

                    if is_five_high_straight(&self.cards) {
                        return Some(Ordering::Greater);
                    }

                    if is_five_high_straight(&other.cards) {
                        return Some(Ordering::Less);
                    }

                    Some(self.cards[0].r.cmp(&other.cards[0].r))
                }
            },

            HandRank::FullHouse => {
                let (self_triplet, self_pair) = self.get_full_house_cards();
                let (other_triplet, other_pair) = other.get_full_house_cards();

                match self_triplet[0].r.cmp(&other_triplet[0].r) {
                    Ordering::Equal => Some(self_pair[0].r.cmp(&other_pair[0].r)),
                    ordering => Some(ordering),
                }
            }

            HandRank::FourOfAKind => {
                let (self_four, self_one) = self.get_four_of_a_kind_cards();
                let (other_four, other_one) = other.get_four_of_a_kind_cards();

                match self_four[0].r.cmp(&other_four[0].r) {
                    Ordering::Equal => Some(self_one[0].r.cmp(&other_one[0].r)),
                    ordering => Some(ordering),
                }
            }

            _ => Some(self.cards[0].r.cmp(&other.cards[0].r)),
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_with_ranks = hands
        .iter()
        .map(|&h| (h.parse::<Hand>().unwrap(), h))
        .collect::<Vec<_>>();

    hands_with_ranks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let best_hand = hands_with_ranks[0].0.clone();
    hands_with_ranks
        .into_iter()
        .filter(|h| h.0.partial_cmp(&best_hand).unwrap() == Ordering::Equal)
        .map(|h| h.1)
        .collect::<Vec<_>>()
}

mod tests {
    use crate::*;

    #[test]
    fn test_flush() {
        let h = "JS 10S 9S 8S 7S".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::StraightFlush);
    }

    #[test]
    fn test_four_of_kind() {
        let h = "5C 5D 5H 5S 2D".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::FourOfAKind);
    }

    #[test]
    fn test_full_house() {
        let h = "6S 6H 6D KC KH".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::FullHouse);
    }

    #[test]
    fn test_is_flush() {
        let h = "JD 9D 8D 4D 3D".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::Flush);
    }

    #[test]
    fn is_straight() {
        let h = "10D 9S 8H 7D 6S".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::Straight);
    }

    #[test]
    fn is_three_of_a_kind() {
        let h = "QC QS QH 9H 2S".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::ThreeOfAKind);
    }

    #[test]
    fn is_two_pair() {
        let h = "2S 8H 2H 8D JH".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::TwoPair);

        let h = "JH JS 3C 3S 2H".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::TwoPair);
    }

    #[test]
    fn is_one_pair() {
        let h = "10H 10S 8S 7H 4C".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::OnePair);
    }

    #[test]
    fn is_high_card() {
        let h = "KD QD 7S 4S 3H".parse::<Hand>().unwrap();
        assert_eq!(h.get_rank(), HandRank::HighCard);
    }
}
