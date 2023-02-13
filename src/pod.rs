//Ty to Tremwil for providing this code. https://github.com/tremwil

use std::io::{Read, Write, Result as IoResult};
use std::mem::{MaybeUninit, size_of};

/// #Safety
/// Implementor must be a primitive type or plain-old-data struct
pub unsafe trait Pod: Sized {
    fn read(reader: &mut impl Read) -> IoResult<Self> {
        let mut s: MaybeUninit<Self> = MaybeUninit::uninit();

        let as_slice = unsafe {
            std::slice::from_raw_parts_mut((&mut s).as_ptr() as *mut u8, size_of::<Self>())
        };
        reader.read_exact(as_slice)?;
        Ok(unsafe { s.assume_init() })
    }

    fn write(&self, writer: &mut impl Write) -> IoResult<()> {
        let as_slice = unsafe {
            std::slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>())
        };
        writer.write_all(as_slice)?;
        Ok(())
    }
}

// Implement Pod for all constant sized slices of Pod
unsafe impl<T: Pod, const SIZE: usize> Pod for [T; SIZE] {}

// Implement Pod for u128 (may want to do this for all primitive types via a macro)
unsafe impl Pod for u128 {}