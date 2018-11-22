use std::collections::BTreeSet;

fn minimum_change_naive(denominations: &Vec<u32>, change: u32) -> Option<u32> {
    if change == 0 {
        Some(0)
    } else {
        let mut min_coins = None;
        for &denomination in denominations.iter() {
            if change >= denomination {
                if let Some(coins) = minimum_change_naive(denominations, change - denomination)
                {
                    if min_coins == None || coins + 1 < min_coins.unwrap() {
                        min_coins = Some(coins + 1);
                    }
                }
            }
        }
        min_coins
    }
}

fn minimum_change_greedy(denominations: &BTreeSet<u32>, mut change: u32) -> Option<u32> {
    let mut coins = 0;
    for &denomination in denominations.iter().rev() {
        while change >= denomination {
            change -= denomination;
            coins += 1;
        }
    }
    if change == 0 {
        Some(coins)
    } else {
        None
    }
}

fn main() {
    let denominations_vec = vec![7, 6, 3, 1];
    let mut denominations_set = BTreeSet::new();
    denominations_set.insert(7);
    denominations_set.insert(6);
    denominations_set.insert(3);
    denominations_set.insert(1);
    println!("{:?}", minimum_change_naive(&denominations_vec, 9));
    println!("{:?}", minimum_change_greedy(&denominations_set, 9));
}
