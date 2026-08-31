use std::vec;

mod _169_majority_element;

fn main() {
    let result =
        _169_majority_element::Solution::majority_element(vec![1,2,3,4]);
    println!("{:?}", result);
}
