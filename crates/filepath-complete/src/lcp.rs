/// Longest Common Prefix

/// Return the lcp of all strings
pub fn longest_common_prefix(mut data: Vec<String>) -> String {
    let mut ret = String::new();

    if data.is_empty() {
        return ret;
    }

    data.sort();

    let mut first = data.first().unwrap().chars();
    let mut last = data.last().unwrap().chars();

    loop {
        match (first.next(), last.next()) {
            (Some(c1), Some(c2)) if c1 == c2 => {
                ret.push(c1);
            }
            _ => return ret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    fn make_strings(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn check_common_prefix() {
        assert_eq!(longest_common_prefix(make_strings(vec!["aab"])), "aab");

        assert_eq!(
            longest_common_prefix(make_strings(vec!["aab", "aac", "aah"])),
            "aa"
        );

        assert_eq!(
            longest_common_prefix(make_strings(vec!["", "aac", "aah"])),
            ""
        );

        assert_eq!(
            longest_common_prefix(make_strings(vec!["bbab", "bbab22", "bbab23"])),
            "bbab"
        );
    }
}
