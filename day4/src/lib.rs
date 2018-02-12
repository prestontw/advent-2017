use std::collections::HashSet;

fn str_to_passphrase(i: &str) -> Vec<&str> {
    i.split_whitespace().collect()
}

fn str_to_passphrases(i: &str) -> Vec<Vec<&str>> {
    i.lines().map(|l| str_to_passphrase(l)).collect()
}

fn to_hashset<'a, I>(i: I) -> HashSet<&'a str> where I: IntoIterator<Item = &'a str> {
    i.into_iter().collect()
}

fn num_matching_lengths<'a, I>(i: I) -> usize where I: IntoIterator<Item = Vec<&'a str>> + Clone {
    let temp = i.clone().into_iter().map(to_hashset);
    i.into_iter().zip(temp)
        .map(|(a, b)| a.len() == b.len())
        .map(|b| if b {1} else {0})
        .sum()
}