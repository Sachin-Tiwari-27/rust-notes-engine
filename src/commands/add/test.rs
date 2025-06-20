#[cfg(test)]

mod tests {
    use super::super::add::add_note;
    use crate::models::Tag;

    #[test]
    fn test_add_notes () {
        let result = add_note("Test Title", "Test Body", &Tag::Work);
        assert!(result.is_ok());
    }
}