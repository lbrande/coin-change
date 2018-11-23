use std::collections::BTreeSet;
use std::collections::HashMap;

fn minimum_change_naive(denoms: &Vec<u32>, change: u32) -> Option<u32> {
    if change == 0 {
        Some(0)
    } else {
        let mut min_coins = None;
        for &denom in denoms.iter() {
            if change >= denom {
                if let Some(coins) = minimum_change_naive(denoms, change - denom) {
                    if min_coins == None || coins + 1 < min_coins.unwrap() {
                        min_coins = Some(coins + 1);
                    }
                }
            }
        }
        min_coins
    }
}

fn minimum_change_greedy(denoms: &BTreeSet<u32>, mut change: u32) -> Option<u32> {
    let mut coins = 0;
    for &denom in denoms.iter().rev() {
        while change >= denom {
            change -= denom;
            coins += 1;
        }
    }
    if change == 0 {
        Some(coins)
    } else {
        None
    }
}

fn minimum_change_dp(
    denoms: &Vec<u32>,
    change: u32,
    memo: &mut HashMap<u32, Option<u32>>,
) -> Option<u32> {
    if change == 0 {
        return Some(0);
    } else if let Some(&coins) = memo.get(&change) {
        return coins;
    } else {
        let mut min_coins = None;
        for &denom in denoms.iter() {
            if change >= denom {
                if let Some(coins) = minimum_change_dp(denoms, change - denom, memo) {
                    if min_coins == None || coins + 1 < min_coins.unwrap() {
                        min_coins = Some(coins + 1);
                    }
                }
            }
        }
        memo.insert(change, min_coins);
        min_coins
    }
}

fn minimum_change_dp_bottom_up(denoms: &Vec<u32>, change: u32) -> Option<u32> {
    let mut table = Vec::with_capacity((change + 1) as usize);
    table.push(Some(0));
    for c in 1..=change {
        let mut min_coins = None;
        for &denom in denoms.iter() {
            if c >= denom {
                if let Some(coins) = table[(c - denom) as usize] {
                    if min_coins == None || coins + 1 < min_coins.unwrap() {
                        min_coins = Some(coins + 1);
                    }
                }
            }
        }
        table.push(min_coins);
    }
    table[change as usize]
}

fn main() {
    let denoms_vec = vec![7, 6, 3, 1];
    let mut denoms_set = BTreeSet::new();
    denoms_set.insert(7);
    denoms_set.insert(6);
    denoms_set.insert(3);
    denoms_set.insert(1);
    println!("{:?}", minimum_change_naive(&denoms_vec, 9));
    println!("{:?}", minimum_change_greedy(&denoms_set, 9));
    println!(
        "{:?}",
        minimum_change_dp(&denoms_vec, 9, &mut HashMap::new())
    );
    println!("{:?}", minimum_change_dp_bottom_up(&denoms_vec, 9));
}
