#[doc = "Reader of register GTR"]
pub type R = crate::R<u32, super::GTR>;
#[doc = "Writer for register GTR"]
pub type W = crate::W<u32, super::GTR>;
#[doc = "Register GTR `reset()`'s with value 0xff"]
impl crate::ResetValue for super::GTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff
    }
}
#[doc = "Reader of field `GTR`"]
pub type GTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTR`"]
pub struct GTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Guard time configuration register."]
    #[inline(always)]
    pub fn gtr(&self) -> GTR_R {
        GTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Guard time configuration register."]
    #[inline(always)]
    pub fn gtr(&mut self) -> GTR_W {
        GTR_W { w: self }
    }
}
