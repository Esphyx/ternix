pub trait Parsing {
    fn parse<T: Into<String>>(input: T) -> Result<Self, String>
    where
        Self: Sized;
}
