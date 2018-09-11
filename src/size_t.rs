use crate::Word;
use std::{cmp, ops::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Size(Word);

impl From<usize> for Size {
    fn from(size: usize) -> Size {
        Size(size.into())
    }
}

impl From<Size> for usize {
    fn from(Size(size): Size) -> usize {
        size.into()
    }
}

impl cmp::PartialEq<usize> for Size {
    fn eq(&self, rhs: &usize) -> bool {
        let lhs: usize = (*self).into();
        lhs.eq(rhs)
    }
}

impl cmp::PartialOrd<usize> for Size {
    fn partial_cmp(&self, rhs: &usize) -> Option<cmp::Ordering> {
        let lhs: usize = (*self).into();
        lhs.partial_cmp(rhs)
    }
}

macro_rules! impl_arithmetic_ops_for_usize_and_size {
    (($func:ident, $op:tt)) => {
        impl $op<usize> for Size {
            type Output = Size;
            fn $func(self, rhs: usize) -> Size {
                let lhs: usize = self.into();
                lhs.$func(rhs).into()
            }
        }

        impl $op<Size> for Size {
            type Output = Size;
            fn $func(self, rhs: Size) -> Size {
                let lhs: usize = self.into();
                let rhs: usize = rhs.into();
                lhs.$func(rhs).into()
            }
        }
    };
    (($func_0:ident, $op_0:tt), $(($func:ident, $op:tt)),+) => {
        impl_arithmetic_ops_for_usize_and_size!(($func_0, $op_0));
        impl_arithmetic_ops_for_usize_and_size!($(($func, $op)),+);
    };
}

impl_arithmetic_ops_for_usize_and_size!(
    (add, Add),
    (sub, Sub),
    (mul, Mul),
    (div, Div),
    (bitand, BitAnd),
    (bitor, BitOr),
    (bitxor, BitXor),
    (rem, Rem),
    (shl, Shl),
    (shr, Shr)
);

macro_rules! impl_assignment_ops_for_usize_and_size {
    (($func:ident, $assign:ident, $op:tt)) => {
        impl $op<usize> for Size {
            fn $assign(&mut self, rhs: usize) {
                let lhs: usize = (*self).into();
                let res = lhs. $func (rhs);
                *self = res.into();
            }
        }
        impl $op<Size> for Size {
            fn $assign(&mut self, rhs: Size) {
                self. $assign (usize::from(rhs));
            }
        }
    };
    (($func_0:ident, $assign_0:ident, $op_0:tt), $(($func:ident, $assign:ident, $op:tt)),+) => {
        impl_assignment_ops_for_usize_and_size!(($func_0, $assign_0, $op_0));
        impl_assignment_ops_for_usize_and_size!($(($func, $assign, $op)),+);
    };
}

impl_assignment_ops_for_usize_and_size!(
    (add, add_assign, AddAssign),
    (sub, sub_assign, SubAssign),
    (mul, mul_assign, MulAssign),
    (div, div_assign, DivAssign),
    (bitand, bitand_assign, BitAndAssign),
    (bitor, bitor_assign, BitOrAssign),
    (bitxor, bitxor_assign, BitXorAssign),
    (rem, rem_assign, RemAssign),
    (shl, shl_assign, ShlAssign),
    (shr, shr_assign, ShrAssign)
);

macro_rules! impl_unary_ops {
    (($func:ident, $op:tt)) => {
        impl $op for Size {
            type Output = Self;
            fn $func(self) -> Self {
                let n: usize = self.into();
                n.$func().into()
            }
        }
    };
    (($func_0:ident, $op_0:tt), $(($func:ident, $op:tt)),+) => {
        impl_unary_ops!(($func_0, $op_0));
        impl_unary_ops!($(($func, $op)),+);
    }
}

impl_unary_ops!((not, Not));
