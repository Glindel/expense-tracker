pub trait LocalizedError {
    fn localized_description(&self) -> &str;
}