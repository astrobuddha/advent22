pub struct Day3 {
    file_path: String,
}

impl Day3 {
    pub fn new(file_path: &str) -> Day3 {
        Day3 {
            file_path: String::from(file_path),
        }
    }

    pub fn get_first(&self) -> u32 {
        0
    }
}
