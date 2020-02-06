#[doc = "Reader of register ECNTH"]
pub type R = crate::R<u32, super::ECNTH>;
#[doc = "Writer for register ECNTH"]
pub type W = crate::W<u32, super::ECNTH>;
#[doc = "Register ECNTH `reset()`'s with value 0"]
impl crate::ResetValue for super::ECNTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECNTH`"]
pub type ECNTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECNTH`"]
pub struct ECNTH_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ETU counter high register."]
    #[inline(always)]
    pub fn ecnth(&self) -> ECNTH_R {
        ECNTH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ETU counter high register."]
    #[inline(always)]
    pub fn ecnth(&mut self) -> ECNTH_W {
        ECNTH_W { w: self }
    }
}
