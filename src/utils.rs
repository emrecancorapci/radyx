/// Finds the first differing index between two strings
pub(crate) fn longest_common_prefix(s1: &str, s2: &str) -> usize {
    let mut iter1 = s1.chars();
    let mut iter2 = s2.chars();

    for i in 0.. {
        match (iter1.next(), iter2.next()) {
            (Some(ch1), Some(ch2)) if ch1 == ch2 => continue,
            _ => return i,
        }
    }

    0
}

#[test]
fn lcp_commutativity() {
    let str1 = "/Mainframe";
    let str2 = "/Maintenance";
    let str3 = "/s";
    let str4 = "/f";

    assert_eq!(
        longest_common_prefix(str2, str1),
        longest_common_prefix(str1, str2)
    );
    assert_eq!(
        longest_common_prefix(str3, str4),
        longest_common_prefix(str4, str3)
    );
}
