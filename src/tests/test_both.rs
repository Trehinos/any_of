#[cfg(test)]
mod tests {
    use crate::{Both, Map, Swap};

    #[test]
    fn test_both_new() {
        let both = Both::new(10, "right");
        assert_eq!(both.left, 10);
        assert_eq!(both.right, "right");
    }

    #[test]
    fn test_both_from_couple() {
        let tuple = (42, "hello");
        let both = Both::from_couple(tuple);
        assert_eq!(both.left, 42);
        assert_eq!(both.right, "hello");
    }

    #[test]
    fn test_into_couple() {
        let both = Both::new(100, "goodbye");
        let couple = both.into_couple();
        assert_eq!(couple, (100, "goodbye"));
    }

    #[test]
    fn test_into_left() {
        let both = Both::new(1, "left");
        let left = both.into_left();
        assert!(matches!(left, crate::either::Either::Left(1)));
    }

    #[test]
    fn test_into_right() {
        let both = Both::new("right", 2023);
        let right = both.into_right();
        assert!(matches!(right, crate::either::Either::Right(2023)));
    }

    #[test]
    fn test_both_swap() {
        let both = Both::new("first", 5);
        let swapped = both.swap();
        assert_eq!(swapped.left, 5);
        assert_eq!(swapped.right, "first");
    }

    #[test]
    fn test_both_map() {
        let both = Both::new(10, core::f64::consts::PI);
        let mapped = both.map(|l| l * 2, |r| r / 2.0);
        assert_eq!(mapped.left, 20);
        assert!(mapped.right < 1.6);
    }
}
