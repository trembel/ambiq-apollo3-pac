#[doc = "Reader of register UCR"]
pub type R = crate::R<u32, super::UCR>;
#[doc = "Writer for register UCR"]
pub type W = crate::W<u32, super::UCR>;
#[doc = "Register UCR `reset()`'s with value 0x08"]
impl crate::ResetValue for super::UCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `RETXEN`"]
pub type RETXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETXEN`"]
pub struct RETXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXEN_W<'a> {
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
#[doc = "Reader of field `RSTIN`"]
pub type RSTIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTIN`"]
pub struct RSTIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIN_W<'a> {
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
#[doc = "Reader of field `RIU`"]
pub type RIU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIU`"]
pub struct RIU_W<'a> {
    w: &'a mut W,
}
impl<'a> RIU_W<'a> {
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
#[doc = "Reader of field `CST`"]
pub type CST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CST`"]
pub struct CST_W<'a> {
    w: &'a mut W,
}
impl<'a> CST_W<'a> {
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
    #[doc = "Bit 3 - Enable TX/RX time configuration."]
    #[inline(always)]
    pub fn retxen(&self) -> RETXEN_R {
        RETXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset polarity."]
    #[inline(always)]
    pub fn rstin(&self) -> RSTIN_R {
        RSTIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ISO7816 reset. This bit is write-only."]
    #[inline(always)]
    pub fn riu(&self) -> RIU_R {
        RIU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clock control."]
    #[inline(always)]
    pub fn cst(&self) -> CST_R {
        CST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Enable TX/RX time configuration."]
    #[inline(always)]
    pub fn retxen(&mut self) -> RETXEN_W {
        RETXEN_W { w: self }
    }
    #[doc = "Bit 2 - Reset polarity."]
    #[inline(always)]
    pub fn rstin(&mut self) -> RSTIN_W {
        RSTIN_W { w: self }
    }
    #[doc = "Bit 1 - ISO7816 reset. This bit is write-only."]
    #[inline(always)]
    pub fn riu(&mut self) -> RIU_W {
        RIU_W { w: self }
    }
    #[doc = "Bit 0 - Clock control."]
    #[inline(always)]
    pub fn cst(&mut self) -> CST_W {
        CST_W { w: self }
    }
}
