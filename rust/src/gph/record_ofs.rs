use super::Record;

impl Record {
    /// `true` if this record draws as an XOR overlay (`cmp dl,1` at `0x2FBE7`).
    #[must_use]
    pub fn is_xor(&self) -> bool {
        self.mode_flag == 1
    }
}
