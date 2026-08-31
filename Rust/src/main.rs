use std::vec;

mod _347_top_k_frequent_elements;

fn main() {
    let result =
        _347_top_k_frequent_elements::Solution::top_k_frequent(vec![1,1,1,2,2,3], 2);
    println!("{:?}", result);
}
