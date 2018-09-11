#![recursion_limit = "1024"]
#![feature(try_from, specialization)]

use saturn_tagging::*;

mod word;
pub use crate::word::Word;

mod size_t;
pub use crate::size_t::Size;

pub unsafe trait SaturnType {
    fn type_id() -> TypeId;
    fn try_thin_type_id() -> Option<ThinTypeId> {
        use std::convert::TryInto;

        Self::type_id().try_into().ok()
    }
    /// Panics if the type of `Self` cannot fit into a `ThinTypeId`
    fn thin_type_id() -> ThinTypeId {
        Self::try_thin_type_id().unwrap()
    }
    /// The size of this object in `Word`s. A default impl exists for
    /// objects which are `Sized`, but it *must be overridden* for
    /// values which are `Sized` but are dynamically extended past
    /// their struct boundary.
    unsafe fn size(&self) -> Size;
    /// A straight-up bitwise cast to `&[Word]`. The returned slice
    /// will have a length equal to `self.size()`.
    unsafe fn words(&self) -> &[Word] {
        let ptr = self as *const Self as *const Word;
        let len = self.size();
        std::slice::from_raw_parts(ptr, len.into())
    }
    unsafe fn words_mut(&mut self) -> &mut [Word] {
        let ptr = self as *mut Self as *mut Word;
        let len = self.size();
        std::slice::from_raw_parts_mut(ptr, len.into())
    }
}

default unsafe impl<T> SaturnType for T
where
    T: Sized,
{
    unsafe fn size(&self) -> Size {
        ((std::mem::size_of::<Self>() + 7) / 8).into()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
