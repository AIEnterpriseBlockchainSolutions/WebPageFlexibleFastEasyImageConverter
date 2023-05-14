/// A Struct called Set it contains a Value
/// The type is defined by the Generic T
pub struct Set<T> {
    value: T,
}
/// Implement for the Set Struct
impl<T> Set<T> {
    pub async fn new(value: T) -> Self {
        Self { value }
    }

    pub async fn get_value(&self) -> &T {
        &self.value
    }
}
