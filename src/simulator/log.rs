pub struct Log {
    pub heading: String,
    pub info: String,
    pub owner: String
}

impl Log {
    pub fn new(heading: String, info: String, owner: String) -> Self {
        Self {
            heading,
            info,
            owner
        }
    }
}