use super::Emit;

impl Emit for Vec<u8> {
    fn emit(&mut self, b: u8) {
        self.push(b);
    }
}
