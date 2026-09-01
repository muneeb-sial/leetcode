mod _75_sort_colors;

fn main() {
    let mut nums = vec![2,0,2,1,1,0];
    _75_sort_colors::Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
