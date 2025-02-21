pub trait HasIdentifier {
    fn identifier(&self) -> &String;

    // TODO : add identifier path ?
}