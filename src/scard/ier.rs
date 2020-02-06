#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FHFEN`"]
pub type FHFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FHFEN`"]
pub struct FHFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FHFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FT2RENDEN`"]
pub type FT2RENDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT2RENDEN`"]
pub struct FT2RENDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2RENDEN_W<'a> {
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
#[doc = "Reader of field `PEEN`"]
pub type PEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEEN`"]
pub struct PEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEEN_W<'a> {
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
#[doc = "Reader of field `OVREN`"]
pub type OVREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVREN`"]
pub struct OVREN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVREN_W<'a> {
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
#[doc = "Reader of field `FEREN`"]
pub type FEREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEREN`"]
pub struct FEREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEREN_W<'a> {
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
#[doc = "Reader of field `TBERBFEN`"]
pub type TBERBFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBERBFEN`"]
pub struct TBERBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBERBFEN_W<'a> {
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
#[doc = "Reader of field `FNEEN`"]
pub type FNEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FNEEN`"]
pub struct FNEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FNEEN_W<'a> {
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
    #[doc = "Bit 6 - FIFO Half Full interrupt enable."]
    #[inline(always)]
    pub fn fhfen(&self) -> FHFEN_R {
        FHFEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX to RX finished interrupt enable."]
    #[inline(always)]
    pub fn ft2renden(&self) -> FT2RENDEN_R {
        FT2RENDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error interrupt enable."]
    #[inline(always)]
    pub fn peen(&self) -> PEEN_R {
        PEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFOI overflow interrupt enable."]
    #[inline(always)]
    pub fn ovren(&self) -> OVREN_R {
        OVREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Framing error interrupt enable."]
    #[inline(always)]
    pub fn feren(&self) -> FEREN_R {
        FEREN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive) interrupt enable."]
    #[inline(always)]
    pub fn tberbfen(&self) -> TBERBFEN_R {
        TBERBFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RX FIFO not empty interrupt enable."]
    #[inline(always)]
    pub fn fneen(&self) -> FNEEN_R {
        FNEEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - FIFO Half Full interrupt enable."]
    #[inline(always)]
    pub fn fhfen(&mut self) -> FHFEN_W {
        FHFEN_W { w: self }
    }
    #[doc = "Bit 5 - TX to RX finished interrupt enable."]
    #[inline(always)]
    pub fn ft2renden(&mut self) -> FT2RENDEN_W {
        FT2RENDEN_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error interrupt enable."]
    #[inline(always)]
    pub fn peen(&mut self) -> PEEN_W {
        PEEN_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFOI overflow interrupt enable."]
    #[inline(always)]
    pub fn ovren(&mut self) -> OVREN_W {
        OVREN_W { w: self }
    }
    #[doc = "Bit 2 - Framing error interrupt enable."]
    #[inline(always)]
    pub fn feren(&mut self) -> FEREN_W {
        FEREN_W { w: self }
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive) interrupt enable."]
    #[inline(always)]
    pub fn tberbfen(&mut self) -> TBERBFEN_W {
        TBERBFEN_W { w: self }
    }
    #[doc = "Bit 0 - RX FIFO not empty interrupt enable."]
    #[inline(always)]
    pub fn fneen(&mut self) -> FNEEN_W {
        FNEEN_W { w: self }
    }
}
