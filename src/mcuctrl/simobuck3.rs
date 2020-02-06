#[doc = "Reader of register SIMOBUCK3"]
pub type R = crate::R<u32, super::SIMOBUCK3>;
#[doc = "Writer for register SIMOBUCK3"]
pub type W = crate::W<u32, super::SIMOBUCK3>;
#[doc = "Register SIMOBUCK3 `reset()`'s with value 0x5000_aaaa"]
impl crate::ResetValue for super::SIMOBUCK3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x5000_aaaa
    }
}
#[doc = "Reader of field `RESERVED_RW_31`"]
pub type RESERVED_RW_31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED_RW_31`"]
pub struct RESERVED_RW_31_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPHIGHTONTRIM`"]
pub type SIMOBUCKMEMLPHIGHTONTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPHIGHTONTRIM`"]
pub struct SIMOBUCKMEMLPHIGHTONTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPHIGHTONTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 27)) | (((value as u32) & 0x0f) << 27);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_RW_16`"]
pub type RESERVED_RW_16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED_RW_16`"]
pub struct RESERVED_RW_16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPLOWTOFFTRIM`"]
pub type SIMOBUCKMEMLPLOWTOFFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPLOWTOFFTRIM`"]
pub struct SIMOBUCKMEMLPLOWTOFFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPLOWTOFFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPHIGHTOFFTRIM`"]
pub type SIMOBUCKMEMLPHIGHTOFFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPHIGHTOFFTRIM`"]
pub struct SIMOBUCKMEMLPHIGHTOFFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPHIGHTOFFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCORELPLOWTOFFTRIM`"]
pub type SIMOBUCKCORELPLOWTOFFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCORELPLOWTOFFTRIM`"]
pub struct SIMOBUCKCORELPLOWTOFFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCORELPLOWTOFFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCORELPHIGHTOFFTRIM`"]
pub type SIMOBUCKCORELPHIGHTOFFTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCORELPHIGHTOFFTRIM`"]
pub struct SIMOBUCKCORELPHIGHTOFFTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCORELPHIGHTOFFTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_31(&self) -> RESERVED_RW_31_R {
        RESERVED_RW_31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 27:30 - simobuck_mem_lp_high_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlphightontrim(&self) -> SIMOBUCKMEMLPHIGHTONTRIM_R {
        SIMOBUCKMEMLPHIGHTONTRIM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bits 16:26 - Reserved bits, always leave unchanged. The SIMOBUCK3 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_16(&self) -> RESERVED_RW_16_R {
        RESERVED_RW_16_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 12:15 - simobuck_mem_lp_low_toff_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtofftrim(&self) -> SIMOBUCKMEMLPLOWTOFFTRIM_R {
        SIMOBUCKMEMLPLOWTOFFTRIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - simobuck_mem_lp_high_toff_trim"]
    #[inline(always)]
    pub fn simobuckmemlphightofftrim(&self) -> SIMOBUCKMEMLPHIGHTOFFTRIM_R {
        SIMOBUCKMEMLPHIGHTOFFTRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - simobuck_core_lp_low_toff_trim"]
    #[inline(always)]
    pub fn simobuckcorelplowtofftrim(&self) -> SIMOBUCKCORELPLOWTOFFTRIM_R {
        SIMOBUCKCORELPLOWTOFFTRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - simobuck_core_lp_high_toff_trim"]
    #[inline(always)]
    pub fn simobuckcorelphightofftrim(&self) -> SIMOBUCKCORELPHIGHTOFFTRIM_R {
        SIMOBUCKCORELPHIGHTOFFTRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_31(&mut self) -> RESERVED_RW_31_W {
        RESERVED_RW_31_W { w: self }
    }
    #[doc = "Bits 27:30 - simobuck_mem_lp_high_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlphightontrim(&mut self) -> SIMOBUCKMEMLPHIGHTONTRIM_W {
        SIMOBUCKMEMLPHIGHTONTRIM_W { w: self }
    }
    #[doc = "Bits 16:26 - Reserved bits, always leave unchanged. The SIMOBUCK3 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_16(&mut self) -> RESERVED_RW_16_W {
        RESERVED_RW_16_W { w: self }
    }
    #[doc = "Bits 12:15 - simobuck_mem_lp_low_toff_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtofftrim(&mut self) -> SIMOBUCKMEMLPLOWTOFFTRIM_W {
        SIMOBUCKMEMLPLOWTOFFTRIM_W { w: self }
    }
    #[doc = "Bits 8:11 - simobuck_mem_lp_high_toff_trim"]
    #[inline(always)]
    pub fn simobuckmemlphightofftrim(&mut self) -> SIMOBUCKMEMLPHIGHTOFFTRIM_W {
        SIMOBUCKMEMLPHIGHTOFFTRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - simobuck_core_lp_low_toff_trim"]
    #[inline(always)]
    pub fn simobuckcorelplowtofftrim(&mut self) -> SIMOBUCKCORELPLOWTOFFTRIM_W {
        SIMOBUCKCORELPLOWTOFFTRIM_W { w: self }
    }
    #[doc = "Bits 0:3 - simobuck_core_lp_high_toff_trim"]
    #[inline(always)]
    pub fn simobuckcorelphightofftrim(&mut self) -> SIMOBUCKCORELPHIGHTOFFTRIM_W {
        SIMOBUCKCORELPHIGHTOFFTRIM_W { w: self }
    }
}
