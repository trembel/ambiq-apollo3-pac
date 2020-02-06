#[doc = "Reader of register BPRL"]
pub type R = crate::R<u32, super::BPRL>;
#[doc = "Writer for register BPRL"]
pub type W = crate::W<u32, super::BPRL>;
#[doc = "Register BPRL `reset()`'s with value 0x74"]
impl crate::ResetValue for super::BPRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x74
    }
}
#[doc = "Reader of field `BPRL`"]
pub type BPRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BPRL`"]
pub struct BPRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BPRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Baud rate low"]
    #[inline(always)]
    pub fn bprl(&self) -> BPRL_R {
        BPRL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Baud rate low"]
    #[inline(always)]
    pub fn bprl(&mut self) -> BPRL_W {
        BPRL_W { w: self }
    }
}
