#[doc = "Reader of register RETXCNT"]
pub type R = crate::R<u32, super::RETXCNT>;
#[doc = "Writer for register RETXCNT"]
pub type W = crate::W<u32, super::RETXCNT>;
#[doc = "Register RETXCNT `reset()`'s with value 0x04"]
impl crate::ResetValue for super::RETXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reader of field `RETXCNT`"]
pub type RETXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETXCNT`"]
pub struct RETXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RETXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Resend count register."]
    #[inline(always)]
    pub fn retxcnt(&self) -> RETXCNT_R {
        RETXCNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resend count register."]
    #[inline(always)]
    pub fn retxcnt(&mut self) -> RETXCNT_W {
        RETXCNT_W { w: self }
    }
}
