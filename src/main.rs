mod lib;

fn main() {
    let nums = vec![1];
    let k = 1;
    lib::k_frequent_element(nums.clone(), k);
    lib::contains_duplicate(nums.clone());
    lib::is_anagram("anagram".to_string(), "gramsan".to_string());
    lib::two_sum(nums.clone(), k as i32);
    lib::group_anagrams(vec![
        "eat".to_string(),
        "get".to_string(),
        "let".to_string(),
        "teg".to_string(),
    ]);
}
