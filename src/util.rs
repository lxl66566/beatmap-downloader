use std::collections::BTreeSet;
pub fn calculate_api_number(s: &BTreeSet<usize>) -> usize {
    s.iter().fold(0, |acc, x| acc + (1 << (x - 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_api_number() {
        let mut set1 = BTreeSet::new();
        set1.insert(1);
        set1.insert(3);
        set1.insert(4);
        assert_eq!(calculate_api_number(&set1), 1 + 4 + 8);
    }
}
