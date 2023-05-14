use crate::hashsets::hashset::Hashset;
/// define the struct for the Hashlist
/// conatins a Vector, that Contains a Hashset

pub struct Hashlist<T> {
    // this Vector contains Hashsets
    vec: Vec<Hashset<T>>,
}

/// This si the Implementation for the Hashlist
impl<T> Hashlist<T> {
    /// Creates a Hashlist
    pub async fn new() -> Hashlist<T> {
        Self { vec: Vec::new() }
    }
    //  put an element into the vector sortet to enamble binary search throug the vector
}
