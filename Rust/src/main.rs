mod _27_remove_element;
fn main() {
    let result = _27_remove_element::Solution::remove_element(&mut vec![1,2,3,4,4,3],3);
    println!("{:?}", result);
}
