#[doc = "Reader of register AUX1"]
pub type R = crate::R<u32, super::AUX1>;
#[doc = "Writer for register AUX1"]
pub type W = crate::W<u32, super::AUX1>;
#[doc = "Register AUX1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AUX1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer B1 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1EN23_A {
    #[doc = "1: Disable enhanced functions."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions."]
    EN = 0,
}
impl From<TMRB1EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1EN23`"]
pub type TMRB1EN23_R = crate::R<bool, TMRB1EN23_A>;
impl TMRB1EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1EN23_A {
        match self.bits {
            true => TMRB1EN23_A::DIS,
            false => TMRB1EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB1EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRB1EN23`"]
pub struct TMRB1EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1EN23_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1POL23_A {
    #[doc = "0: Upper output normal polarity"]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity."]
    INV = 1,
}
impl From<TMRB1POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1POL23`"]
pub type TMRB1POL23_R = crate::R<bool, TMRB1POL23_A>;
impl TMRB1POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1POL23_A {
        match self.bits {
            false => TMRB1POL23_A::NORM,
            true => TMRB1POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == TMRB1POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRB1POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRB1POL23`"]
pub struct TMRB1POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity"]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB1POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB1POL23_A::INV)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Counter/Timer B1 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1TINV_A {
    #[doc = "0: Disable invert on trigger"]
    DIS = 0,
    #[doc = "1: Enable invert on trigger"]
    EN = 1,
}
impl From<TMRB1TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1TINV`"]
pub type TMRB1TINV_R = crate::R<bool, TMRB1TINV_A>;
impl TMRB1TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1TINV_A {
        match self.bits {
            false => TMRB1TINV_A::DIS,
            true => TMRB1TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB1TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRB1TINV`"]
pub struct TMRB1TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB1TINV_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB1NOSYNC_A {
    #[doc = "0: Synchronization on source clock"]
    DIS = 0,
    #[doc = "1: No synchronization on source clock"]
    NOSYNC = 1,
}
impl From<TMRB1NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB1NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB1NOSYNC`"]
pub type TMRB1NOSYNC_R = crate::R<bool, TMRB1NOSYNC_A>;
impl TMRB1NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1NOSYNC_A {
        match self.bits {
            false => TMRB1NOSYNC_A::DIS,
            true => TMRB1NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRB1NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRB1NOSYNC`"]
pub struct TMRB1NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock"]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB1NOSYNC_A::NOSYNC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Counter/Timer B1 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB1TRIG_A {
    #[doc = "0: Trigger source is disabled."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA1 OUT."]
    A1OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA6 OUT."]
    A6OUT = 4,
    #[doc = "5: Trigger source is CTIMERB6 OUT."]
    B6OUT = 5,
    #[doc = "6: Trigger source is CTIMERA0 OUT."]
    A0OUT = 6,
    #[doc = "7: Trigger source is CTIMERB0 OUT."]
    B0OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA4 OUT2."]
    A4OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB4 OUT2."]
    B4OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB5 OUT2, dual edge."]
    B5OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA5 OUT2, dual edge."]
    A5OUT2DUAL = 15,
}
impl From<TMRB1TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB1TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB1TRIG`"]
pub type TMRB1TRIG_R = crate::R<u8, TMRB1TRIG_A>;
impl TMRB1TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB1TRIG_A {
        match self.bits {
            0 => TMRB1TRIG_A::DIS,
            1 => TMRB1TRIG_A::A1OUT,
            2 => TMRB1TRIG_A::B3OUT,
            3 => TMRB1TRIG_A::A3OUT,
            4 => TMRB1TRIG_A::A6OUT,
            5 => TMRB1TRIG_A::B6OUT,
            6 => TMRB1TRIG_A::A0OUT,
            7 => TMRB1TRIG_A::B0OUT,
            8 => TMRB1TRIG_A::B3OUT2,
            9 => TMRB1TRIG_A::A3OUT2,
            10 => TMRB1TRIG_A::A4OUT2,
            11 => TMRB1TRIG_A::B4OUT2,
            12 => TMRB1TRIG_A::A6OUT2DUAL,
            13 => TMRB1TRIG_A::A7OUT2DUAL,
            14 => TMRB1TRIG_A::B5OUT2DUAL,
            15 => TMRB1TRIG_A::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB1TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        *self == TMRB1TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRB1TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRB1TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        *self == TMRB1TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        *self == TMRB1TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        *self == TMRB1TRIG_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        *self == TMRB1TRIG_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRB1TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRB1TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        *self == TMRB1TRIG_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        *self == TMRB1TRIG_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRB1TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRB1TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRB1TRIG_A::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRB1TRIG_A::A5OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRB1TRIG`"]
pub struct TMRB1TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB1TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA1 OUT."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B0OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A4OUT2)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2."]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B4OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge."]
    #[inline(always)]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB1TRIG_A::A5OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `TMRB1LMT`"]
pub type TMRB1LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRB1LMT`"]
pub struct TMRB1LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB1LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A1 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1EN23_A {
    #[doc = "1: Disable enhanced functions."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions."]
    EN = 0,
}
impl From<TMRA1EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1EN23`"]
pub type TMRA1EN23_R = crate::R<bool, TMRA1EN23_A>;
impl TMRA1EN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1EN23_A {
        match self.bits {
            true => TMRA1EN23_A::DIS,
            false => TMRA1EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA1EN23_A::EN
    }
}
#[doc = "Write proxy for field `TMRA1EN23`"]
pub struct TMRA1EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1EN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable enhanced functions."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1EN23_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Counter/Timer A1 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1POL23_A {
    #[doc = "0: Upper output normal polarity"]
    NORMAL = 0,
    #[doc = "1: Upper output inverted polarity."]
    INV = 1,
}
impl From<TMRA1POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1POL23`"]
pub type TMRA1POL23_R = crate::R<bool, TMRA1POL23_A>;
impl TMRA1POL23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1POL23_A {
        match self.bits {
            false => TMRA1POL23_A::NORMAL,
            true => TMRA1POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA1POL23_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == TMRA1POL23_A::INV
    }
}
#[doc = "Write proxy for field `TMRA1POL23`"]
pub struct TMRA1POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1POL23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Upper output normal polarity"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA1POL23_A::NORMAL)
    }
    #[doc = "Upper output inverted polarity."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA1POL23_A::INV)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Counter/Timer A1 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1TINV_A {
    #[doc = "0: Disable invert on trigger"]
    DIS = 0,
    #[doc = "1: Enable invert on trigger"]
    EN = 1,
}
impl From<TMRA1TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1TINV`"]
pub type TMRA1TINV_R = crate::R<bool, TMRA1TINV_A>;
impl TMRA1TINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1TINV_A {
        match self.bits {
            false => TMRA1TINV_A::DIS,
            true => TMRA1TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA1TINV_A::EN
    }
}
#[doc = "Write proxy for field `TMRA1TINV`"]
pub struct TMRA1TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1TINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable invert on trigger"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA1TINV_A::EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA1NOSYNC_A {
    #[doc = "0: Synchronization on source clock"]
    DIS = 0,
    #[doc = "1: No synchronization on source clock"]
    NOSYNC = 1,
}
impl From<TMRA1NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA1NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA1NOSYNC`"]
pub type TMRA1NOSYNC_R = crate::R<bool, TMRA1NOSYNC_A>;
impl TMRA1NOSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1NOSYNC_A {
        match self.bits {
            false => TMRA1NOSYNC_A::DIS,
            true => TMRA1NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        *self == TMRA1NOSYNC_A::NOSYNC
    }
}
#[doc = "Write proxy for field `TMRA1NOSYNC`"]
pub struct TMRA1NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1NOSYNC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization on source clock"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock"]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA1NOSYNC_A::NOSYNC)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Counter/Timer A1 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA1TRIG_A {
    #[doc = "0: Trigger source is disabled."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB1 OUT."]
    B1OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA0 OUT."]
    A0OUT = 4,
    #[doc = "5: Trigger source is CTIMERB0 OUT."]
    B0OUT = 5,
    #[doc = "6: Trigger source is CTIMERA5 OUT."]
    A5OUT = 6,
    #[doc = "7: Trigger source is CTIMERB5 OUT."]
    B5OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA4 OUT2."]
    A4OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB4 OUT2."]
    B4OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB5 OUT2, dual edge."]
    B5OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA5 OUT2, dual edge."]
    A5OUT2DUAL = 15,
}
impl From<TMRA1TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA1TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA1TRIG`"]
pub type TMRA1TRIG_R = crate::R<u8, TMRA1TRIG_A>;
impl TMRA1TRIG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA1TRIG_A {
        match self.bits {
            0 => TMRA1TRIG_A::DIS,
            1 => TMRA1TRIG_A::B1OUT,
            2 => TMRA1TRIG_A::B3OUT,
            3 => TMRA1TRIG_A::A3OUT,
            4 => TMRA1TRIG_A::A0OUT,
            5 => TMRA1TRIG_A::B0OUT,
            6 => TMRA1TRIG_A::A5OUT,
            7 => TMRA1TRIG_A::B5OUT,
            8 => TMRA1TRIG_A::B3OUT2,
            9 => TMRA1TRIG_A::A3OUT2,
            10 => TMRA1TRIG_A::A4OUT2,
            11 => TMRA1TRIG_A::B4OUT2,
            12 => TMRA1TRIG_A::A6OUT2DUAL,
            13 => TMRA1TRIG_A::A7OUT2DUAL,
            14 => TMRA1TRIG_A::B5OUT2DUAL,
            15 => TMRA1TRIG_A::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA1TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        *self == TMRA1TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        *self == TMRA1TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        *self == TMRA1TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A0OUT`"]
    #[inline(always)]
    pub fn is_a0out(&self) -> bool {
        *self == TMRA1TRIG_A::A0OUT
    }
    #[doc = "Checks if the value of the field is `B0OUT`"]
    #[inline(always)]
    pub fn is_b0out(&self) -> bool {
        *self == TMRA1TRIG_A::B0OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        *self == TMRA1TRIG_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        *self == TMRA1TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        *self == TMRA1TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        *self == TMRA1TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A4OUT2`"]
    #[inline(always)]
    pub fn is_a4out2(&self) -> bool {
        *self == TMRA1TRIG_A::A4OUT2
    }
    #[doc = "Checks if the value of the field is `B4OUT2`"]
    #[inline(always)]
    pub fn is_b4out2(&self) -> bool {
        *self == TMRA1TRIG_A::B4OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        *self == TMRA1TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        *self == TMRA1TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b5out2dual(&self) -> bool {
        *self == TMRA1TRIG_A::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        *self == TMRA1TRIG_A::A5OUT2DUAL
    }
}
#[doc = "Write proxy for field `TMRA1TRIG`"]
pub struct TMRA1TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA1TRIG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger source is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB1 OUT."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA0 OUT."]
    #[inline(always)]
    pub fn a0out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A0OUT)
    }
    #[doc = "Trigger source is CTIMERB0 OUT."]
    #[inline(always)]
    pub fn b0out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B0OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2."]
    #[inline(always)]
    pub fn a4out2(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A4OUT2)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2."]
    #[inline(always)]
    pub fn b4out2(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B4OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge."]
    #[inline(always)]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA1TRIG_A::A5OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `TMRA1LMT`"]
pub type TMRA1LMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRA1LMT`"]
pub struct TMRA1LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA1LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B1 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb1en23(&self) -> TMRB1EN23_R {
        TMRB1EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb1pol23(&self) -> TMRB1POL23_R {
        TMRB1POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B1 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb1tinv(&self) -> TMRB1TINV_R {
        TMRB1TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb1nosync(&self) -> TMRB1NOSYNC_R {
        TMRB1NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B1 Trigger Select."]
    #[inline(always)]
    pub fn tmrb1trig(&self) -> TMRB1TRIG_R {
        TMRB1TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B1 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb1lmt(&self) -> TMRB1LMT_R {
        TMRB1LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A1 Upper compare enable."]
    #[inline(always)]
    pub fn tmra1en23(&self) -> TMRA1EN23_R {
        TMRA1EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A1 Upper output polarity"]
    #[inline(always)]
    pub fn tmra1pol23(&self) -> TMRA1POL23_R {
        TMRA1POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A1 Invert on trigger."]
    #[inline(always)]
    pub fn tmra1tinv(&self) -> TMRA1TINV_R {
        TMRA1TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra1nosync(&self) -> TMRA1NOSYNC_R {
        TMRA1NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A1 Trigger Select."]
    #[inline(always)]
    pub fn tmra1trig(&self) -> TMRA1TRIG_R {
        TMRA1TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A1 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra1lmt(&self) -> TMRA1LMT_R {
        TMRA1LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B1 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb1en23(&mut self) -> TMRB1EN23_W {
        TMRB1EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb1pol23(&mut self) -> TMRB1POL23_W {
        TMRB1POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B1 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb1tinv(&mut self) -> TMRB1TINV_W {
        TMRB1TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb1nosync(&mut self) -> TMRB1NOSYNC_W {
        TMRB1NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B1 Trigger Select."]
    #[inline(always)]
    pub fn tmrb1trig(&mut self) -> TMRB1TRIG_W {
        TMRB1TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B1 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb1lmt(&mut self) -> TMRB1LMT_W {
        TMRB1LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A1 Upper compare enable."]
    #[inline(always)]
    pub fn tmra1en23(&mut self) -> TMRA1EN23_W {
        TMRA1EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A1 Upper output polarity"]
    #[inline(always)]
    pub fn tmra1pol23(&mut self) -> TMRA1POL23_W {
        TMRA1POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A1 Invert on trigger."]
    #[inline(always)]
    pub fn tmra1tinv(&mut self) -> TMRA1TINV_W {
        TMRA1TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra1nosync(&mut self) -> TMRA1NOSYNC_W {
        TMRA1NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A1 Trigger Select."]
    #[inline(always)]
    pub fn tmra1trig(&mut self) -> TMRA1TRIG_W {
        TMRA1TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A1 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra1lmt(&mut self) -> TMRA1LMT_W {
        TMRA1LMT_W { w: self }
    }
}
