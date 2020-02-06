#[doc = "Reader of register IER1"]
pub type R = crate::R<u32, super::IER1>;
#[doc = "Writer for register IER1"]
pub type W = crate::W<u32, super::IER1>;
#[doc = "Register IER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNCENDEN`"]
pub type SYNCENDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNCENDEN`"]
pub struct SYNCENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCENDEN_W<'a> {
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
#[doc = "Reader of field `PRLEN`"]
pub type PRLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRLEN`"]
pub struct PRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRLEN_W<'a> {
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
#[doc = "Reader of field `ECNTOVEREN`"]
pub type ECNTOVEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECNTOVEREN`"]
pub struct ECNTOVEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNTOVEREN_W<'a> {
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
    #[doc = "Bit 2 - Write complete synchronization interrupt enable."]
    #[inline(always)]
    pub fn syncenden(&self) -> SYNCENDEN_R {
        SYNCENDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Card insert/remove interrupt enable."]
    #[inline(always)]
    pub fn prlen(&self) -> PRLEN_R {
        PRLEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ETU counter overflow interrupt enable."]
    #[inline(always)]
    pub fn ecntoveren(&self) -> ECNTOVEREN_R {
        ECNTOVEREN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Write complete synchronization interrupt enable."]
    #[inline(always)]
    pub fn syncenden(&mut self) -> SYNCENDEN_W {
        SYNCENDEN_W { w: self }
    }
    #[doc = "Bit 1 - Card insert/remove interrupt enable."]
    #[inline(always)]
    pub fn prlen(&mut self) -> PRLEN_W {
        PRLEN_W { w: self }
    }
    #[doc = "Bit 0 - ETU counter overflow interrupt enable."]
    #[inline(always)]
    pub fn ecntoveren(&mut self) -> ECNTOVEREN_W {
        ECNTOVEREN_W { w: self }
    }
}
