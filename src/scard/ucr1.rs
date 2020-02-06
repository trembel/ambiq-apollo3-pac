#[doc = "Reader of register UCR1"]
pub type R = crate::R<u32, super::UCR1>;
#[doc = "Writer for register UCR1"]
pub type W = crate::W<u32, super::UCR1>;
#[doc = "Register UCR1 `reset()`'s with value 0x30"]
impl crate::ResetValue for super::UCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x30
    }
}
#[doc = "Reader of field `ENLASTB`"]
pub type ENLASTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENLASTB`"]
pub struct ENLASTB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENLASTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `CLKIOV`"]
pub type CLKIOV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKIOV`"]
pub struct CLKIOV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIOV_W<'a> {
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
#[doc = "Reader of field `T1PAREN`"]
pub type T1PAREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `T1PAREN`"]
pub struct T1PAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> T1PAREN_W<'a> {
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
#[doc = "Reader of field `STSP`"]
pub type STSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STSP`"]
pub struct STSP_W<'a> {
    w: &'a mut W,
}
impl<'a> STSP_W<'a> {
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
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR`"]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
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
    #[doc = "Bit 5 - Enable last byte function."]
    #[inline(always)]
    pub fn enlastb(&self) -> ENLASTB_R {
        ENLASTB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output clock level."]
    #[inline(always)]
    pub fn clkiov(&self) -> CLKIOV_R {
        CLKIOV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Parity check control."]
    #[inline(always)]
    pub fn t1paren(&self) -> T1PAREN_R {
        T1PAREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ETU counter control. This bit is write-only."]
    #[inline(always)]
    pub fn stsp(&self) -> STSP_R {
        STSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Query Card Detect."]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Enable last byte function."]
    #[inline(always)]
    pub fn enlastb(&mut self) -> ENLASTB_W {
        ENLASTB_W { w: self }
    }
    #[doc = "Bit 4 - Output clock level."]
    #[inline(always)]
    pub fn clkiov(&mut self) -> CLKIOV_W {
        CLKIOV_W { w: self }
    }
    #[doc = "Bit 3 - Parity check control."]
    #[inline(always)]
    pub fn t1paren(&mut self) -> T1PAREN_W {
        T1PAREN_W { w: self }
    }
    #[doc = "Bit 2 - ETU counter control. This bit is write-only."]
    #[inline(always)]
    pub fn stsp(&mut self) -> STSP_W {
        STSP_W { w: self }
    }
    #[doc = "Bit 0 - Query Card Detect."]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
}
