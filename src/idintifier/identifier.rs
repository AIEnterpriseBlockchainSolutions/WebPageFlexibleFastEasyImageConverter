/// a struct stored a ID
/// teh ID is stored as a i64
///  and it stores a counter how often
///  the id got read
pub struct ID {
    id: i64,
    read: i64,
}

impl ID {
    pub async fn new(id: i64) -> Self {
        Self { id, read: 0 }
    }
    pub async fn get_id(&mut self) -> i64 {
        self.read += 1;
        self.id
    }

    pub async fn read(&self) -> i64 {
        self.read
    }
}
