#[doc = "Reader of register REGACCINTSTAT"]
pub type R = crate::R<u32, super::REGACCINTSTAT>;
#[doc = "Writer for register REGACCINTSTAT"]
pub type W = crate::W<u32, super::REGACCINTSTAT>;
#[doc = "Register REGACCINTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::REGACCINTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGACC`"]
pub type REGACC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGACC`"]
pub struct REGACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REGACC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    pub fn regacc(&self) -> REGACC_R {
        REGACC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register access interrupts."]
    #[inline(always)]
    pub fn regacc(&mut self) -> REGACC_W {
        REGACC_W { w: self }
    }
}
