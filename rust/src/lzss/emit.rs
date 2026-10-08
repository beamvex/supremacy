/// Byte sink for decoded output.
///
/// The asm emits each byte with `stosb` (opaque) or `xor [es:di],al` /
/// `inc di` (overlay), wrapping to the next row every `w` bytes. `Emit`
/// abstracts that so the same decoder drives flat buffers, strided frames,
/// and XOR overlays.
pub trait Emit {
    /// Consume one decoded byte.
    fn emit(&mut self, b: u8);
}
