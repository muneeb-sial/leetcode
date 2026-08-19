mod _4_median_of_two_sorted_arrays;

fn main() {
    let result =
        _4_median_of_two_sorted_arrays::Solution::find_median_sorted_arrays(vec![2,2,4,4], vec![2,2,2,4,4]);
    println!("{:?}", result);
}
