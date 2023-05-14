/// A struct that conatisn a Hash
///  this hash is stored in 3 Different ways
///  String
///  i64
///  [u8, 128]
pub struct Hash {
    st_hash: String,
    int_hash: i64,
    buf_hash: [u8; 128],
}

impl Hash {
    pub async fn new() -> Self {
        Self {
            st_hash: String::new(),
            int_hash: 0,
            buf_hash: [0; 128],
        }
    }

    /// get the Stringed Hash
    pub async fn get_st_hash(&self) -> &String {
        &self.st_hash
    }

    /// get the Hash as a i64
    pub async fn get_int_hash(&self) -> &i64 {
        &self.int_hash
    }

    /// get the Hash as a u8 buffer with 128 elemnts
    pub async fn get_buf_hash(&self) -> &[u8; 128] {
        &self.buf_hash
    }
}
