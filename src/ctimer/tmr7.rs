#[doc = "Reader of register TMR7"]
pub type R = crate::R<u32, super::TMR7>;
#[doc = "Writer for register TMR7"]
pub type W = crate::W<u32, super::TMR7>;
#[doc = "Register TMR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTTMRB7`"]
pub type CTTMRB7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRB7`"]
pub struct CTTMRB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRB7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTTMRA7`"]
pub type CTTMRA7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CTTMRA7`"]
pub struct CTTMRA7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTTMRA7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Counter/Timer B7."]
    #[inline(always)]
    pub fn cttmrb7(&self) -> CTTMRB7_R {
        CTTMRB7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Counter/Timer A7."]
    #[inline(always)]
    pub fn cttmra7(&self) -> CTTMRA7_R {
        CTTMRA7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Counter/Timer B7."]
    #[inline(always)]
    pub fn cttmrb7(&mut self) -> CTTMRB7_W {
        CTTMRB7_W { w: self }
    }
    #[doc = "Bits 0:15 - Counter/Timer A7."]
    #[inline(always)]
    pub fn cttmra7(&mut self) -> CTTMRA7_W {
        CTTMRA7_W { w: self }
    }
}
