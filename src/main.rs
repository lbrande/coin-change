fn minimum_change_naive(denominations: &Vec<u32>, change: u32) -> u32 {
    if change == 0 {
        return 0;
    }
    let mut min_coins = u32::max_value();
    for i in 0..denominations.len() {
        if denominations[i] <= change {
            let coins = 1 + minimum_change_naive(denominations, change - denominations[i]);
            if coins < min_coins {
                min_coins = coins;
            }
        }
    }
    min_coins
}

fn main() {
    println!("{}", minimum_change_naive(&vec![10, 5, 2, 1], 17));
}
