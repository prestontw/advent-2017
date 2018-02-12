use std::collections::HashSet;

fn str_to_passphrase(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}

pub fn str_to_passphrases(i: &str) -> Vec<Vec<&str>> {
    i.lines().map(|l| str_to_passphrase(l)).collect()
}

fn to_hashset<'a, I, S>(i: I) -> HashSet<S> where I: IntoIterator<Item = S>,
S: Into<&'a str> + Eq + std::hash::Hash {
    i.into_iter().collect()
}

fn to_hashset2<'a, I, S>(i: I) -> HashSet<S> where I: IntoIterator<Item = S>,
S: Into<String> + Eq + std::hash::Hash {
    i.into_iter().collect()
}

pub fn num_matching_lengths<'a, I>(i: I) -> usize where I: IntoIterator<Item = Vec<&'a str>> + Clone {
    let temp = i.clone().into_iter().map(to_hashset2);
    i.into_iter().zip(temp)
        .map(|(a, b)| a.len() == b.len())
        .map(|b| if b {1} else {0})
        .sum()
}

fn sort_str<'a, 'b: 'a>(i: &'a str) -> String {
    let mut chars: Vec<char> = i.chars().collect();
    chars.sort();
    let s = chars.into_iter().fold(String::new(), |mut acc, cur|
        { acc.push(cur); acc });
    s
}

pub fn part_two<'a, I>(i: I) -> usize where I: IntoIterator<Item = Vec<&'a str>> + Clone {
    let mut count = 0;
    for line in i {
        let temp = line.clone();
        let sorted_word_vec = temp.into_iter().map(sort_str);
        let hashed = to_hashset2(sorted_word_vec);
        if line.len() == hashed.len() { count += 1; }
    }
    count
}