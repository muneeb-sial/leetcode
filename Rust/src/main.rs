mod _242_valid_anagram;

fn main() {
    let result =
        _242_valid_anagram::Solution::is_anagram("anagram".to_string(),"nagaram".to_string());
    println!("{:?}", result);
}
