use super::set::Set;
use crate::hashsets::hash::Hash;
use crate::idintifier::identifier::ID;

/// Create a Struct that contains a Set, Hash and ID
/// it should stores a value with a special id
pub struct Hashset<T> {
    set: Set<T>,
    hash: Hash,
    id: ID,
}
/// Implements for the Hashset
impl<T> Hashset<T> {
    pub async fn new(value: T) -> Self {
        Self {
            set: Set::new(value).await,
            hash: Hash::new().await,
            id: ID::new(0).await,
        }
    }

    /// gets the hash from tze current Hashset
    pub async fn get_hash(&self) -> &Hash {
        &self.hash
    }

    /// get the set from the hashset
    pub async fn get_set(&self) -> &Set<T> {
        &self.set
    }

    /// get the id from the hashset
    pub async fn get_id(&self) -> &ID {
        &self.id
    }
}
