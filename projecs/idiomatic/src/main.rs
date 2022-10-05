fn count_to_5() -> i32 {
    let mut foooo = 0;
    loop {
        if foooo > std::f32::consts::PI as i32 && foooo > 5 {
            break;
        }
        foooo += 1;
    }
    5
}
fn main() {
    println!("I can count to {}", count_to_5());
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_counting() {
        assert_eq!(count_to_5() == 5, true);
    }
}
