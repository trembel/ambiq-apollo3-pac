#[doc = "Reader of register SIMOBUCK2"]
pub type R = crate::R<u32, super::SIMOBUCK2>;
#[doc = "Writer for register SIMOBUCK2"]
pub type W = crate::W<u32, super::SIMOBUCK2>;
#[doc = "Register SIMOBUCK2 `reset()`'s with value 0x00aa_0000"]
impl crate::ResetValue for super::SIMOBUCK2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00aa_0000
    }
}
#[doc = "Reader of field `RESERVED_RW_24`"]
pub type RESERVED_RW_24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED_RW_24`"]
pub struct RESERVED_RW_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCORELPLOWTONTRIM`"]
pub type SIMOBUCKCORELPLOWTONTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCORELPLOWTONTRIM`"]
pub struct SIMOBUCKCORELPLOWTONTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCORELPLOWTONTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCORELPHIGHTONTRIM`"]
pub type SIMOBUCKCORELPHIGHTONTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCORELPHIGHTONTRIM`"]
pub struct SIMOBUCKCORELPHIGHTONTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCORELPHIGHTONTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED_RW_0`"]
pub type RESERVED_RW_0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED_RW_0`"]
pub struct RESERVED_RW_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_24(&self) -> RESERVED_RW_24_R {
        RESERVED_RW_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - simobuck_core_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckcorelplowtontrim(&self) -> SIMOBUCKCORELPLOWTONTRIM_R {
        SIMOBUCKCORELPLOWTONTRIM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - simobuck_core_lp_high_ton_trim"]
    #[inline(always)]
    pub fn simobuckcorelphightontrim(&self) -> SIMOBUCKCORELPHIGHTONTRIM_R {
        SIMOBUCKCORELPHIGHTONTRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&self) -> RESERVED_RW_0_R {
        RESERVED_RW_0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_24(&mut self) -> RESERVED_RW_24_W {
        RESERVED_RW_24_W { w: self }
    }
    #[doc = "Bits 20:23 - simobuck_core_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckcorelplowtontrim(&mut self) -> SIMOBUCKCORELPLOWTONTRIM_W {
        SIMOBUCKCORELPLOWTONTRIM_W { w: self }
    }
    #[doc = "Bits 16:19 - simobuck_core_lp_high_ton_trim"]
    #[inline(always)]
    pub fn simobuckcorelphightontrim(&mut self) -> SIMOBUCKCORELPHIGHTONTRIM_W {
        SIMOBUCKCORELPHIGHTONTRIM_W { w: self }
    }
    #[doc = "Bits 0:15 - Reserved bits, always leave unchanged. The SIMOBUCK2 register must be modified via atomic RMW, leaving this bit field completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&mut self) -> RESERVED_RW_0_W {
        RESERVED_RW_0_W { w: self }
    }
}
