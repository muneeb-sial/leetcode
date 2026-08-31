use std::vec;

mod _238_product_of_array_except_self;

fn main() {
    let result =
        _238_product_of_array_except_self::Solution::product_except_self(vec![1,2,3,4]);
    println!("{:?}", result);
}
