use std::mem::size_of;
use byteorder::*;
use paste::paste;

#[macro_export]
    /// generates read<T> functions.
    macro_rules! read_type {
    ($ty:ty) => {
        paste! {
            #[doc = "Reads a `" $ty "` type from the stream and advances the cursor by the size of the type" ]
            pub fn [<read_ $ty>](&mut self) -> std::io::Result<$ty> {
                let mut data = vec![0u8; size_of::<$ty>()];
                self.cursor.read(&mut data[..])?;
                let mut data = data.as_slice();
                match self.endianness {
                    LittleEndian => data.[<read_ $ty>]::<LittleEndian>(),
                    NativeEndian => data.[<read_ $ty>]::<NativeEndian>(),
                    _ => data.[<read_ $ty>]::<BigEndian>(),
                }
            }

        }
    };
}


