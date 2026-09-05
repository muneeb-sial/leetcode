mod _283_move_zeroes;

fn main() {
    let mut nums = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    _283_move_zeroes::Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
