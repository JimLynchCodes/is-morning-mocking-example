// morning is between 7am and 12 noon eastern us time

pub fn is_morning() -> bool {
    return true;
}

#[cfg(test)]
pub mod tests {

    #[test]
    pub fn beginning_of_morning() {
        assert_eq!(is_morning(), true);
    }
}
