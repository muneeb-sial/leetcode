mod _49_group_anagrams;

fn main() {
    let result =
        _49_group_anagrams::Solution::group_anagrams(vec![
            "eat".to_string(),"tea".to_string(),"tan".to_string(),"ate".to_string(),"nat".to_string(),"bat".to_string()
            ]);
    println!("{:?}", result);
}
