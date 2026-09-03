mod _84_largest_rectangle_in_histogram;

fn main() {
    let heights = vec![2,1,5,6,2,3];
    let result = _84_largest_rectangle_in_histogram::Solution::largest_rectangle_area(heights);
    println!("{:?}", result);
}