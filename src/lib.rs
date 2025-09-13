pub fn version () -> &'static str {
    "0.0.1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = version();
        assert_eq!(result, "0.0.1");
    }
}
