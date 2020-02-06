#[doc = "Reader of register BPRH"]
pub type R = crate::R<u32, super::BPRH>;
#[doc = "Writer for register BPRH"]
pub type W = crate::W<u32, super::BPRH>;
#[doc = "Register BPRH `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BPRH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `BPRH`"]
pub type BPRH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BPRH`"]
pub struct BPRH_W<'a> {
    w: &'a mut W,
}
impl<'a> BPRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Baud rate high"]
    #[inline(always)]
    pub fn bprh(&self) -> BPRH_R {
        BPRH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Baud rate high"]
    #[inline(always)]
    pub fn bprh(&mut self) -> BPRH_W {
        BPRH_W { w: self }
    }
}
