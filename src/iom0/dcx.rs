#[doc = "Reader of register DCX"]
pub type R = crate::R<u32, super::DCX>;
#[doc = "Writer for register DCX"]
pub type W = crate::W<u32, super::DCX>;
#[doc = "Register DCX `reset()`'s with value 0"]
impl crate::ResetValue for super::DCX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCXEN_A {
    #[doc = "1: Enable DCX."]
    EN = 1,
    #[doc = "0: Disable DCX."]
    DIS = 0,
}
impl From<DCXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCXEN`"]
pub type DCXEN_R = crate::R<bool, DCXEN_A>;
impl DCXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCXEN_A {
        match self.bits {
            true => DCXEN_A::EN,
            false => DCXEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == DCXEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == DCXEN_A::DIS
    }
}
#[doc = "Write proxy for field `DCXEN`"]
pub struct DCXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable DCX."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(DCXEN_A::EN)
    }
    #[doc = "Disable DCX."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(DCXEN_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CE3OUT`"]
pub type CE3OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE3OUT`"]
pub struct CE3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE3OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CE2OUT`"]
pub type CE2OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE2OUT`"]
pub struct CE2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE2OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `CE1OUT`"]
pub type CE1OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE1OUT`"]
pub struct CE1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE1OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CE0OUT`"]
pub type CE0OUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CE0OUT`"]
pub struct CE0OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CE0OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline(always)]
    pub fn dcxen(&self) -> DCXEN_R {
        DCXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline(always)]
    pub fn ce3out(&self) -> CE3OUT_R {
        CE3OUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline(always)]
    pub fn ce2out(&self) -> CE2OUT_R {
        CE2OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline(always)]
    pub fn ce1out(&self) -> CE1OUT_R {
        CE1OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline(always)]
    pub fn ce0out(&self) -> CE0OUT_R {
        CE0OUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline(always)]
    pub fn dcxen(&mut self) -> DCXEN_W {
        DCXEN_W { w: self }
    }
    #[doc = "Bit 3 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline(always)]
    pub fn ce3out(&mut self) -> CE3OUT_W {
        CE3OUT_W { w: self }
    }
    #[doc = "Bit 2 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline(always)]
    pub fn ce2out(&mut self) -> CE2OUT_W {
        CE2OUT_W { w: self }
    }
    #[doc = "Bit 1 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline(always)]
    pub fn ce1out(&mut self) -> CE1OUT_W {
        CE1OUT_W { w: self }
    }
    #[doc = "Bit 0 - Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline(always)]
    pub fn ce0out(&mut self) -> CE0OUT_W {
        CE0OUT_W { w: self }
    }
}
