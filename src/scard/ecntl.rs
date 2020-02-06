#[doc = "Reader of register ECNTL"]
pub type R = crate::R<u32, super::ECNTL>;
#[doc = "Writer for register ECNTL"]
pub type W = crate::W<u32, super::ECNTL>;
#[doc = "Register ECNTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ECNTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ECNTL`"]
pub type ECNTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ECNTL`"]
pub struct ECNTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ETU counter low register."]
    #[inline(always)]
    pub fn ecntl(&self) -> ECNTL_R {
        ECNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ETU counter low register."]
    #[inline(always)]
    pub fn ecntl(&mut self) -> ECNTL_W {
        ECNTL_W { w: self }
    }
}
