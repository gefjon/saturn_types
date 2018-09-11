#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
/// The basis of all values in Saturn. A machine word. It might be
/// meaningful to call this a `byte`, as it's the smallest
/// denomination of memory, but in modern usage `byte == u8`.
///
/// In any case, the stack is a `Vec<Word>` (or similar data
/// structure; it may be fixed-size); allocations are `&[Word]`,
/// arithmetic operations have signatures like `Fn(Word, Word) ->
/// Word` or `Fn(Word) -> Word`, etc.
///
/// Expect to see a lot of these, particularly at a low level.
pub struct Word(u64);

macro_rules! impl_convert_word_from_int {
    ($int:ty) => {
        impl From<$int> for Word {
            #[allow(cast_lossless)]
            fn from(n: $int) -> Word {
                Word(n as u64)
            }
        }
    };
    ($($int:ty),+) => {
        $(impl_convert_word_from_int!($int);)+
    };
}

macro_rules! impl_convert_int_from_word {
    ($int:ty) => {
        impl From<Word> for $int {
            fn from(Word(n): Word) -> $int {
                n as $int
            }
        }
    };
    ($int_0:ty, $($int:ty),+) => {
        impl_convert_int_from_word!($int_0);
        impl_convert_int_from_word!($($int),+);
    };
}

impl_convert_word_from_int!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl_convert_int_from_word!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
