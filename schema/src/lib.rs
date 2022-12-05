#[derive(Debug, Serialize, Deserialize, Collection)]
#[collection(name = "messages")]
struct Message {
    pub timestamp: SystemTime,
    pub contents: String,
}