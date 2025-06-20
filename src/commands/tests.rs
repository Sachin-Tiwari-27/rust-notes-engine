#[cfg(test)]

mod tests {
    use super::super::add::add_note;
    use super::super::delete::delete_note;
    use super::super::search::search_note;
    use crate::models::Tag;

    #[test]
    fn test_add_notes() {
        let result = add_note("Test Title", "Test Body", &Tag::Work);
        assert!(result.is_ok());
    }

    #[test]
    fn test_search() {
        let query = String::from("work");
        let result = search_note(&query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_notes() {
        let result = delete_note("Test Title");
        assert!(result.is_ok());
    }
}
