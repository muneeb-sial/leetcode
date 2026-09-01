mod _121_best_time_to_buy_and_sell_stock;

fn main() {
    let prices = vec![7,1];
    // let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", _121_best_time_to_buy_and_sell_stock::Solution::max_profit(prices));
}
