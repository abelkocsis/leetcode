pub fn run() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    println!("{:?}", Solution::group_anagrams(strs));
}

struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut sorted_elems: Vec<(String, usize)> = strs
            .iter()
            .enumerate()
            .map(|(i, str)| {
                let mut char_vec: Vec<char> = str.chars().collect();
                char_vec.sort();
                (char_vec.iter().collect::<String>(), i)
            })
            .collect();

        sorted_elems.sort_by(|(str_1, _), (str_2, _)| str_1.cmp(str_2));

        let mut first = sorted_elems.first().unwrap().0.clone();
        let mut res: Vec<Vec<String>> = vec![vec![]];
        sorted_elems.iter().for_each(|(str, i)| {
            if *str == first {
                res.last_mut().unwrap().push(strs[*i].clone());
            } else {
                first = str.clone();
                res.push(vec![strs[*i].clone()]);
            }
        });

        res
    }
}
