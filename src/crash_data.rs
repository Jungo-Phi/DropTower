use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
struct CrashData {
    author: String,
    comment: String,
    date: u64,
}

impl Serialize for CrashData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        todo!()
    }
}
impl<'de> Deserialize<'de> for CrashData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        todo!()
    }
}