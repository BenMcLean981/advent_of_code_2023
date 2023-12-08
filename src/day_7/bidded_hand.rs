use std::str::FromStr;

use super::hand::Hand;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct BiddedHand {
    pub hand: Hand,
    pub bid: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBiddedHandError;

impl FromStr for BiddedHand {
    type Err = ParseBiddedHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        let hand = Hand::from_str(&split[0]).unwrap();
        let bid = u32::from_str(&split[1]).unwrap();

        return Ok(BiddedHand { hand, bid });
    }
}

impl PartialOrd for BiddedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return self.hand.partial_cmp(&other.hand);
    }
}

impl Ord for BiddedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.hand.cmp(&other.hand);
    }
}
