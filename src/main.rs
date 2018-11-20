fn minimum_change_naive(denominations: &Vec<u32>, change: u32) -> Option<u32> {
    if change == 0 {
        return Some(0);
    }
    let mut min_coins = None;
    for i in 0..denominations.len() {
        if denominations[i] <= change {
            if let Some(coins) = minimum_change_naive(denominations, change - denominations[i]) {
                if min_coins == None || coins + 1 < min_coins.unwrap() {
                    min_coins = Some(coins + 1);
                }
            }
        }
    }
    min_coins
}

fn main() {
    println!("{:?}", minimum_change_naive(&vec![10, 4, 2], 17));
}
