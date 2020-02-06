#[doc = "Reader of register SIMOBUCK4"]
pub type R = crate::R<u32, super::SIMOBUCK4>;
#[doc = "Writer for register SIMOBUCK4"]
pub type W = crate::W<u32, super::SIMOBUCK4>;
#[doc = "Register SIMOBUCK4 `reset()`'s with value 0x3c8d_80aa"]
impl crate::ResetValue for super::SIMOBUCK4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3c8d_80aa
    }
}
#[doc = "Reader of field `SIMOBUCKCOMP2TIMEOUTEN`"]
pub type SIMOBUCKCOMP2TIMEOUTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIMOBUCKCOMP2TIMEOUTEN`"]
pub struct SIMOBUCKCOMP2TIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCOMP2TIMEOUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKCLKDIVSEL`"]
pub type SIMOBUCKCLKDIVSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKCLKDIVSEL`"]
pub struct SIMOBUCKCLKDIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKCLKDIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `SIMOBUCKMEMLPLOWTONTRIM`"]
pub type SIMOBUCKMEMLPLOWTONTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SIMOBUCKMEMLPLOWTONTRIM`"]
pub struct SIMOBUCKMEMLPLOWTONTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMOBUCKMEMLPLOWTONTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - simobuck_comp2_timeout_en"]
    #[inline(always)]
    pub fn simobuckcomp2timeouten(&self) -> SIMOBUCKCOMP2TIMEOUTEN_R {
        SIMOBUCKCOMP2TIMEOUTEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&self) -> SIMOBUCKCLKDIVSEL_R {
        SIMOBUCKCLKDIVSEL_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - simobuck_mem_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtontrim(&self) -> SIMOBUCKMEMLPLOWTONTRIM_R {
        SIMOBUCKMEMLPLOWTONTRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - simobuck_comp2_timeout_en"]
    #[inline(always)]
    pub fn simobuckcomp2timeouten(&mut self) -> SIMOBUCKCOMP2TIMEOUTEN_W {
        SIMOBUCKCOMP2TIMEOUTEN_W { w: self }
    }
    #[doc = "Bits 21:22 - simobuck_clkdiv_sel"]
    #[inline(always)]
    pub fn simobuckclkdivsel(&mut self) -> SIMOBUCKCLKDIVSEL_W {
        SIMOBUCKCLKDIVSEL_W { w: self }
    }
    #[doc = "Bits 0:3 - simobuck_mem_lp_low_ton_trim"]
    #[inline(always)]
    pub fn simobuckmemlplowtontrim(&mut self) -> SIMOBUCKMEMLPLOWTONTRIM_W {
        SIMOBUCKMEMLPLOWTONTRIM_W { w: self }
    }
}
