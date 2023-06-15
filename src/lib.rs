use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut h = HashMap::new();
    for i in &nums {
        h.entry(*i).or_insert(100);
    }
    if h.len() < nums.len() {
        return true;
    } else {
        return false;
    }
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut y = Vec::from(s.into_bytes());
    y.sort();
    let mut z = Vec::from(t.into_bytes());
    z.sort();
    if z == y {
        return true;
    } else {
        return false;
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut v = Vec::new();
    for i in 0..nums.len() {
        for j in 1..nums.len() {
            if (nums[i] + nums[j]) == target && i != j {
                v.push(i as i32);
                v.push(j as i32);
                return v;
            }
        }
    }
    return v;
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut h = HashMap::new();
    let mut av = Vec::new();

    for i in strs {
        let mut count = vec![0; 26];
        for j in i.chars() {
            let v = j as u8 - 97;
            count[v as usize] += 1;
        }
        h.entry(count).or_insert(Vec::new()).push(i.to_string());
    }

    for val in h.values() {
        av.push(val.clone());
    }

    return av;
}

pub fn k_frequent_element(nums: Vec<i32>, k: u32) -> Vec<i32> {
    let mut frequency_map: HashMap<i32, usize> = HashMap::new();
    let mut y = Vec::new();
    for num in nums {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let mut frequency_vec: Vec<(i32, usize)> = frequency_map.into_iter().collect();
    frequency_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let x = frequency_vec
        .iter()
        .take(k as usize)
        .map(|(num, _)| *num)
        .collect::<Vec<_>>();

    for i in 0..k {
        y.push(x[i as usize]);
    }
    return y;
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut v = Vec::new();
    let mut product = 1;
    for i in nums.clone() {
        product *= i;
    }
    for i in nums {
        v.push(division(product, i as u32));
    }
    println!("{v:?}");
    return v;
}

fn division(product: i32, i: u32) -> i32 {
    product / i as i32
}
