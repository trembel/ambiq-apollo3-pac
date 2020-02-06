#[doc = "Reader of register OUTCFG2"]
pub type R = crate::R<u32, super::OUTCFG2>;
#[doc = "Writer for register OUTCFG2"]
pub type W = crate::W<u32, super::OUTCFG2>;
#[doc = "Register OUTCFG2 `reset()`'s with value 0x2492_2292"]
impl crate::ResetValue for super::OUTCFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2492_2292
    }
}
#[doc = "Pad output 29 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG29_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is A3OUT2."]
    A3OUT2 = 5,
    #[doc = "4: Output is A7OUT."]
    A7OUT = 4,
    #[doc = "3: Output is A1OUT."]
    A1OUT = 3,
    #[doc = "2: Output is B5OUT2"]
    B5OUT2 = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG29_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG29_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG29`"]
pub type CFG29_R = crate::R<u8, CFG29_A>;
impl CFG29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG29_A {
        match self.bits {
            7 => CFG29_A::A7OUT2,
            6 => CFG29_A::A6OUT2,
            5 => CFG29_A::A3OUT2,
            4 => CFG29_A::A7OUT,
            3 => CFG29_A::A1OUT,
            2 => CFG29_A::B5OUT2,
            1 => CFG29_A::ONE,
            0 => CFG29_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG29_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG29_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == CFG29_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        *self == CFG29_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG29_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG29_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG29_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG29_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG29`"]
pub struct CFG29_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG29_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG29_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG29_A::A6OUT2)
    }
    #[doc = "Output is A3OUT2."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(CFG29_A::A3OUT2)
    }
    #[doc = "Output is A7OUT."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG29_A::A7OUT)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG29_A::A1OUT)
    }
    #[doc = "Output is B5OUT2"]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG29_A::B5OUT2)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG29_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG29_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Pad output 28 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG28_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is B0OUT2."]
    B0OUT2 = 5,
    #[doc = "4: Output is A5OUT2."]
    A5OUT2 = 4,
    #[doc = "3: Output is A3OUT."]
    A3OUT = 3,
    #[doc = "2: Output is A7OUT"]
    A7OUT = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG28_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG28_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG28`"]
pub type CFG28_R = crate::R<u8, CFG28_A>;
impl CFG28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG28_A {
        match self.bits {
            7 => CFG28_A::A7OUT2,
            6 => CFG28_A::A6OUT2,
            5 => CFG28_A::B0OUT2,
            4 => CFG28_A::A5OUT2,
            3 => CFG28_A::A3OUT,
            2 => CFG28_A::A7OUT,
            1 => CFG28_A::ONE,
            0 => CFG28_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG28_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG28_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG28_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG28_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == CFG28_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        *self == CFG28_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG28_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG28_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG28`"]
pub struct CFG28_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG28_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG28_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG28_A::A6OUT2)
    }
    #[doc = "Output is B0OUT2."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG28_A::B0OUT2)
    }
    #[doc = "Output is A5OUT2."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG28_A::A5OUT2)
    }
    #[doc = "Output is A3OUT."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(CFG28_A::A3OUT)
    }
    #[doc = "Output is A7OUT"]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG28_A::A7OUT)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG28_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG28_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Pad output 27 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG27_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is B2OUT2."]
    B2OUT2 = 5,
    #[doc = "4: Output is B6OUT."]
    B6OUT = 4,
    #[doc = "3: Output is A1OUT."]
    A1OUT = 3,
    #[doc = "2: Output is B6OUT2"]
    B6OUT2 = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG27_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG27_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG27`"]
pub type CFG27_R = crate::R<u8, CFG27_A>;
impl CFG27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG27_A {
        match self.bits {
            7 => CFG27_A::A7OUT2,
            6 => CFG27_A::A6OUT2,
            5 => CFG27_A::B2OUT2,
            4 => CFG27_A::B6OUT,
            3 => CFG27_A::A1OUT,
            2 => CFG27_A::B6OUT2,
            1 => CFG27_A::ONE,
            0 => CFG27_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG27_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG27_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG27_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        *self == CFG27_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG27_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT2`"]
    #[inline(always)]
    pub fn is_b6out2(&self) -> bool {
        *self == CFG27_A::B6OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG27_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG27_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG27`"]
pub struct CFG27_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG27_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG27_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG27_A::A6OUT2)
    }
    #[doc = "Output is B2OUT2."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG27_A::B2OUT2)
    }
    #[doc = "Output is B6OUT."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG27_A::B6OUT)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG27_A::A1OUT)
    }
    #[doc = "Output is B6OUT2"]
    #[inline(always)]
    pub fn b6out2(self) -> &'a mut W {
        self.variant(CFG27_A::B6OUT2)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG27_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG27_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Pad output 26 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG26_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is A1OUT2."]
    A1OUT2 = 5,
    #[doc = "4: Output is A5OUT."]
    A5OUT = 4,
    #[doc = "3: Output is B2OUT."]
    B2OUT = 3,
    #[doc = "2: Output is B6OUT"]
    B6OUT = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG26_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG26_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG26`"]
pub type CFG26_R = crate::R<u8, CFG26_A>;
impl CFG26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG26_A {
        match self.bits {
            7 => CFG26_A::A7OUT2,
            6 => CFG26_A::A6OUT2,
            5 => CFG26_A::A1OUT2,
            4 => CFG26_A::A5OUT,
            3 => CFG26_A::B2OUT,
            2 => CFG26_A::B6OUT,
            1 => CFG26_A::ONE,
            0 => CFG26_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG26_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG26_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG26_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == CFG26_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == CFG26_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        *self == CFG26_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG26_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG26_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG26`"]
pub struct CFG26_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG26_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG26_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG26_A::A6OUT2)
    }
    #[doc = "Output is A1OUT2."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG26_A::A1OUT2)
    }
    #[doc = "Output is A5OUT."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG26_A::A5OUT)
    }
    #[doc = "Output is B2OUT."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG26_A::B2OUT)
    }
    #[doc = "Output is B6OUT"]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(CFG26_A::B6OUT)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG26_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG26_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Pad output 25 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG25_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is A2OUT2."]
    A2OUT2 = 5,
    #[doc = "4: Output is A6OUT."]
    A6OUT = 4,
    #[doc = "3: Output is B2OUT."]
    B2OUT = 3,
    #[doc = "2: Output is B4OUT2"]
    B4OUT2 = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG25_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG25_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG25`"]
pub type CFG25_R = crate::R<u8, CFG25_A>;
impl CFG25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG25_A {
        match self.bits {
            7 => CFG25_A::A7OUT2,
            6 => CFG25_A::A6OUT2,
            5 => CFG25_A::A2OUT2,
            4 => CFG25_A::A6OUT,
            3 => CFG25_A::B2OUT,
            2 => CFG25_A::B4OUT2,
            1 => CFG25_A::ONE,
            0 => CFG25_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG25_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG25_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG25_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == CFG25_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        *self == CFG25_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        *self == CFG25_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG25_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG25_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG25`"]
pub struct CFG25_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG25_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG25_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG25_A::A6OUT2)
    }
    #[doc = "Output is A2OUT2."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG25_A::A2OUT2)
    }
    #[doc = "Output is A6OUT."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG25_A::A6OUT)
    }
    #[doc = "Output is B2OUT."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(CFG25_A::B2OUT)
    }
    #[doc = "Output is B4OUT2"]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(CFG25_A::B4OUT2)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG25_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG25_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Pad output 24 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG24_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is B1OUT2."]
    B1OUT2 = 5,
    #[doc = "4: Output is A1OUT."]
    A1OUT = 4,
    #[doc = "3: Output is A2OUT."]
    A2OUT = 3,
    #[doc = "2: Output is A6OUT"]
    A6OUT = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG24_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG24_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG24`"]
pub type CFG24_R = crate::R<u8, CFG24_A>;
impl CFG24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG24_A {
        match self.bits {
            7 => CFG24_A::A7OUT2,
            6 => CFG24_A::A6OUT2,
            5 => CFG24_A::B1OUT2,
            4 => CFG24_A::A1OUT,
            3 => CFG24_A::A2OUT,
            2 => CFG24_A::A6OUT,
            1 => CFG24_A::ONE,
            0 => CFG24_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG24_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG24_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        *self == CFG24_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG24_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        *self == CFG24_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == CFG24_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG24_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG24_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG24`"]
pub struct CFG24_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG24_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG24_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG24_A::A6OUT2)
    }
    #[doc = "Output is B1OUT2."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(CFG24_A::B1OUT2)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG24_A::A1OUT)
    }
    #[doc = "Output is A2OUT."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(CFG24_A::A2OUT)
    }
    #[doc = "Output is A6OUT"]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG24_A::A6OUT)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG24_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG24_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Pad output 23 configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG23_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is B0OUT2."]
    B0OUT2 = 5,
    #[doc = "4: Output is A5OUT."]
    A5OUT = 4,
    #[doc = "3: Output is A7OUT."]
    A7OUT = 3,
    #[doc = "2: Output is B5OUT2"]
    B5OUT2 = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG23_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG23_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG23`"]
pub type CFG23_R = crate::R<u8, CFG23_A>;
impl CFG23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG23_A {
        match self.bits {
            7 => CFG23_A::A7OUT2,
            6 => CFG23_A::A6OUT2,
            5 => CFG23_A::B0OUT2,
            4 => CFG23_A::A5OUT,
            3 => CFG23_A::A7OUT,
            2 => CFG23_A::B5OUT2,
            1 => CFG23_A::ONE,
            0 => CFG23_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG23_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG23_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B0OUT2`"]
    #[inline(always)]
    pub fn is_b0out2(&self) -> bool {
        *self == CFG23_A::B0OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == CFG23_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        *self == CFG23_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        *self == CFG23_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG23_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG23_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG23`"]
pub struct CFG23_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG23_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG23_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG23_A::A6OUT2)
    }
    #[doc = "Output is B0OUT2."]
    #[inline(always)]
    pub fn b0out2(self) -> &'a mut W {
        self.variant(CFG23_A::B0OUT2)
    }
    #[doc = "Output is A5OUT."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG23_A::A5OUT)
    }
    #[doc = "Output is A7OUT."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(CFG23_A::A7OUT)
    }
    #[doc = "Output is B5OUT2"]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(CFG23_A::B5OUT2)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG23_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG23_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Pad output 22 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG22_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is A2OUT2."]
    A2OUT2 = 5,
    #[doc = "4: Output is A1OUT."]
    A1OUT = 4,
    #[doc = "3: Output is A6OUT."]
    A6OUT = 3,
    #[doc = "2: Output is B5OUT"]
    B5OUT = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG22_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG22_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG22`"]
pub type CFG22_R = crate::R<u8, CFG22_A>;
impl CFG22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG22_A {
        match self.bits {
            7 => CFG22_A::A7OUT2,
            6 => CFG22_A::A6OUT2,
            5 => CFG22_A::A2OUT2,
            4 => CFG22_A::A1OUT,
            3 => CFG22_A::A6OUT,
            2 => CFG22_A::B5OUT,
            1 => CFG22_A::ONE,
            0 => CFG22_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG22_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG22_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        *self == CFG22_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG22_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == CFG22_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == CFG22_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG22_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG22_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG22`"]
pub struct CFG22_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG22_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG22_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG22_A::A6OUT2)
    }
    #[doc = "Output is A2OUT2."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(CFG22_A::A2OUT2)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG22_A::A1OUT)
    }
    #[doc = "Output is A6OUT."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(CFG22_A::A6OUT)
    }
    #[doc = "Output is B5OUT"]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG22_A::B5OUT)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG22_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG22_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Pad output 21 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG21_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is A0OUT2."]
    A0OUT2 = 5,
    #[doc = "4: Output is B5OUT."]
    B5OUT = 4,
    #[doc = "3: Output is A1OUT."]
    A1OUT = 3,
    #[doc = "2: Output is A5OUT2"]
    A5OUT2 = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG21_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG21_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG21`"]
pub type CFG21_R = crate::R<u8, CFG21_A>;
impl CFG21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG21_A {
        match self.bits {
            7 => CFG21_A::A7OUT2,
            6 => CFG21_A::A6OUT2,
            5 => CFG21_A::A0OUT2,
            4 => CFG21_A::B5OUT,
            3 => CFG21_A::A1OUT,
            2 => CFG21_A::A5OUT2,
            1 => CFG21_A::ONE,
            0 => CFG21_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG21_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG21_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `A0OUT2`"]
    #[inline(always)]
    pub fn is_a0out2(&self) -> bool {
        *self == CFG21_A::A0OUT2
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == CFG21_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG21_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        *self == CFG21_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG21_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG21_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG21`"]
pub struct CFG21_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG21_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG21_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG21_A::A6OUT2)
    }
    #[doc = "Output is A0OUT2."]
    #[inline(always)]
    pub fn a0out2(self) -> &'a mut W {
        self.variant(CFG21_A::A0OUT2)
    }
    #[doc = "Output is B5OUT."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(CFG21_A::B5OUT)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG21_A::A1OUT)
    }
    #[doc = "Output is A5OUT2"]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(CFG21_A::A5OUT2)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG21_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG21_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Pad output 20 configuration\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG20_A {
    #[doc = "7: Output is A7OUT2."]
    A7OUT2 = 7,
    #[doc = "6: Output is A6OUT2."]
    A6OUT2 = 6,
    #[doc = "5: Output is B2OUT2."]
    B2OUT2 = 5,
    #[doc = "4: Output is A1OUT2."]
    A1OUT2 = 4,
    #[doc = "3: Output is A1OUT."]
    A1OUT = 3,
    #[doc = "2: Output is A5OUT"]
    A5OUT = 2,
    #[doc = "1: Force output to 1."]
    ONE = 1,
    #[doc = "0: Force output to 0"]
    ZERO = 0,
}
impl From<CFG20_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG20_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CFG20`"]
pub type CFG20_R = crate::R<u8, CFG20_A>;
impl CFG20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG20_A {
        match self.bits {
            7 => CFG20_A::A7OUT2,
            6 => CFG20_A::A6OUT2,
            5 => CFG20_A::B2OUT2,
            4 => CFG20_A::A1OUT2,
            3 => CFG20_A::A1OUT,
            2 => CFG20_A::A5OUT,
            1 => CFG20_A::ONE,
            0 => CFG20_A::ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A7OUT2`"]
    #[inline(always)]
    pub fn is_a7out2(&self) -> bool {
        *self == CFG20_A::A7OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2`"]
    #[inline(always)]
    pub fn is_a6out2(&self) -> bool {
        *self == CFG20_A::A6OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        *self == CFG20_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        *self == CFG20_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == CFG20_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == CFG20_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CFG20_A::ONE
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CFG20_A::ZERO
    }
}
#[doc = "Write proxy for field `CFG20`"]
pub struct CFG20_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFG20_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is A7OUT2."]
    #[inline(always)]
    pub fn a7out2(self) -> &'a mut W {
        self.variant(CFG20_A::A7OUT2)
    }
    #[doc = "Output is A6OUT2."]
    #[inline(always)]
    pub fn a6out2(self) -> &'a mut W {
        self.variant(CFG20_A::A6OUT2)
    }
    #[doc = "Output is B2OUT2."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(CFG20_A::B2OUT2)
    }
    #[doc = "Output is A1OUT2."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(CFG20_A::A1OUT2)
    }
    #[doc = "Output is A1OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(CFG20_A::A1OUT)
    }
    #[doc = "Output is A5OUT"]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(CFG20_A::A5OUT)
    }
    #[doc = "Force output to 1."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CFG20_A::ONE)
    }
    #[doc = "Force output to 0"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CFG20_A::ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - Pad output 29 configuration"]
    #[inline(always)]
    pub fn cfg29(&self) -> CFG29_R {
        CFG29_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27 - Pad output 28 configuration"]
    #[inline(always)]
    pub fn cfg28(&self) -> CFG28_R {
        CFG28_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24 - Pad output 27 configuration"]
    #[inline(always)]
    pub fn cfg27(&self) -> CFG27_R {
        CFG27_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Pad output 26 configuration"]
    #[inline(always)]
    pub fn cfg26(&self) -> CFG26_R {
        CFG26_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Pad output 25 configuration"]
    #[inline(always)]
    pub fn cfg25(&self) -> CFG25_R {
        CFG25_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Pad output 24 configuration"]
    #[inline(always)]
    pub fn cfg24(&self) -> CFG24_R {
        CFG24_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - Pad output 23 configuration"]
    #[inline(always)]
    pub fn cfg23(&self) -> CFG23_R {
        CFG23_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - Pad output 22 configuration"]
    #[inline(always)]
    pub fn cfg22(&self) -> CFG22_R {
        CFG22_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Pad output 21 configuration"]
    #[inline(always)]
    pub fn cfg21(&self) -> CFG21_R {
        CFG21_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Pad output 20 configuration"]
    #[inline(always)]
    pub fn cfg20(&self) -> CFG20_R {
        CFG20_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - Pad output 29 configuration"]
    #[inline(always)]
    pub fn cfg29(&mut self) -> CFG29_W {
        CFG29_W { w: self }
    }
    #[doc = "Bits 25:27 - Pad output 28 configuration"]
    #[inline(always)]
    pub fn cfg28(&mut self) -> CFG28_W {
        CFG28_W { w: self }
    }
    #[doc = "Bits 22:24 - Pad output 27 configuration"]
    #[inline(always)]
    pub fn cfg27(&mut self) -> CFG27_W {
        CFG27_W { w: self }
    }
    #[doc = "Bits 19:21 - Pad output 26 configuration"]
    #[inline(always)]
    pub fn cfg26(&mut self) -> CFG26_W {
        CFG26_W { w: self }
    }
    #[doc = "Bits 16:18 - Pad output 25 configuration"]
    #[inline(always)]
    pub fn cfg25(&mut self) -> CFG25_W {
        CFG25_W { w: self }
    }
    #[doc = "Bits 12:14 - Pad output 24 configuration"]
    #[inline(always)]
    pub fn cfg24(&mut self) -> CFG24_W {
        CFG24_W { w: self }
    }
    #[doc = "Bits 9:11 - Pad output 23 configuration"]
    #[inline(always)]
    pub fn cfg23(&mut self) -> CFG23_W {
        CFG23_W { w: self }
    }
    #[doc = "Bits 6:8 - Pad output 22 configuration"]
    #[inline(always)]
    pub fn cfg22(&mut self) -> CFG22_W {
        CFG22_W { w: self }
    }
    #[doc = "Bits 3:5 - Pad output 21 configuration"]
    #[inline(always)]
    pub fn cfg21(&mut self) -> CFG21_W {
        CFG21_W { w: self }
    }
    #[doc = "Bits 0:2 - Pad output 20 configuration"]
    #[inline(always)]
    pub fn cfg20(&mut self) -> CFG20_W {
        CFG20_W { w: self }
    }
}
