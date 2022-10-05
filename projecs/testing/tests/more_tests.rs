use testing::splish;
use testing::sploosh;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_conditions() {
        assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
    }
}
