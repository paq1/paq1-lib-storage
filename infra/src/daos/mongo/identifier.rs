pub trait HasIdentifier {
    fn identifier_value(&self) -> &String;
    fn identifier_key() -> String;
}