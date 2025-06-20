#[cfg(test)]

mod test {
    use super::super::update::update_note;
    use crate::models::tag;

     #[test]
    fn test_update_note () {
        let result = update_note("Test Title", "Test Body Updated", &Tag::Work);
        assert!(result.is_ok());
    }
}
