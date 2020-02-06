#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `DMAMD`"]
pub type DMAMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMD`"]
pub struct DMAMD_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FIP`"]
pub type FIP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIP`"]
pub struct FIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FIP_W<'a> {
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
#[doc = "Reader of field `AUTOCONV`"]
pub type AUTOCONV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOCONV`"]
pub struct AUTOCONV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCONV_W<'a> {
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
#[doc = "Reader of field `PROT`"]
pub type PROT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PROT`"]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
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
#[doc = "Reader of field `TR`"]
pub type TR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR`"]
pub struct TR_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_W<'a> {
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
#[doc = "Reader of field `LCT`"]
pub type LCT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCT`"]
pub struct LCT_W<'a> {
    w: &'a mut W,
}
impl<'a> LCT_W<'a> {
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
#[doc = "Reader of field `SS`"]
pub type SS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SS`"]
pub struct SS_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_W<'a> {
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
#[doc = "Reader of field `CONV`"]
pub type CONV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONV`"]
pub struct CONV_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_W<'a> {
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
    #[doc = "Bit 7 - DMA direction."]
    #[inline(always)]
    pub fn dmamd(&self) -> DMAMD_R {
        DMAMD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Parity select."]
    #[inline(always)]
    pub fn fip(&self) -> FIP_R {
        FIP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Automatic conversion."]
    #[inline(always)]
    pub fn autoconv(&self) -> AUTOCONV_R {
        AUTOCONV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PROT control."]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit/receive mode."]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast TX to RX."]
    #[inline(always)]
    pub fn lct(&self) -> LCT_R {
        LCT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Use first byte to configure conversion."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Conversion inversion control."]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - DMA direction."]
    #[inline(always)]
    pub fn dmamd(&mut self) -> DMAMD_W {
        DMAMD_W { w: self }
    }
    #[doc = "Bit 6 - Parity select."]
    #[inline(always)]
    pub fn fip(&mut self) -> FIP_W {
        FIP_W { w: self }
    }
    #[doc = "Bit 5 - Automatic conversion."]
    #[inline(always)]
    pub fn autoconv(&mut self) -> AUTOCONV_W {
        AUTOCONV_W { w: self }
    }
    #[doc = "Bit 4 - PROT control."]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Bit 3 - Transmit/receive mode."]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W { w: self }
    }
    #[doc = "Bit 2 - Fast TX to RX."]
    #[inline(always)]
    pub fn lct(&mut self) -> LCT_W {
        LCT_W { w: self }
    }
    #[doc = "Bit 1 - Use first byte to configure conversion."]
    #[inline(always)]
    pub fn ss(&mut self) -> SS_W {
        SS_W { w: self }
    }
    #[doc = "Bit 0 - Conversion inversion control."]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W { w: self }
    }
}
