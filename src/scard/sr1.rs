#[doc = "Reader of register SR1"]
pub type R = crate::R<u32, super::SR1>;
#[doc = "Writer for register SR1"]
pub type W = crate::W<u32, super::SR1>;
#[doc = "Register SR1 `reset()`'s with value 0x08"]
impl crate::ResetValue for super::SR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "ISO7816 idle.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "1: ISO7816 idle."]
    IDLE = 1,
    #[doc = "0: ISO7816 active."]
    ACTIVE = 0,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            true => IDLE_A::IDLE,
            false => IDLE_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLE_A::ACTIVE
    }
}
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ISO7816 idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(IDLE_A::IDLE)
    }
    #[doc = "ISO7816 active."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLE_A::ACTIVE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write complete synchronization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCEND_A {
    #[doc = "1: Synchronization complete."]
    CMPL = 1,
    #[doc = "0: Incomplete."]
    INCMPL = 0,
}
impl From<SYNCEND_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYNCEND`"]
pub type SYNCEND_R = crate::R<bool, SYNCEND_A>;
impl SYNCEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEND_A {
        match self.bits {
            true => SYNCEND_A::CMPL,
            false => SYNCEND_A::INCMPL,
        }
    }
    #[doc = "Checks if the value of the field is `CMPL`"]
    #[inline(always)]
    pub fn is_cmpl(&self) -> bool {
        *self == SYNCEND_A::CMPL
    }
    #[doc = "Checks if the value of the field is `INCMPL`"]
    #[inline(always)]
    pub fn is_incmpl(&self) -> bool {
        *self == SYNCEND_A::INCMPL
    }
}
#[doc = "Write proxy for field `SYNCEND`"]
pub struct SYNCEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYNCEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Synchronization complete."]
    #[inline(always)]
    pub fn cmpl(self) -> &'a mut W {
        self.variant(SYNCEND_A::CMPL)
    }
    #[doc = "Incomplete."]
    #[inline(always)]
    pub fn incmpl(self) -> &'a mut W {
        self.variant(SYNCEND_A::INCMPL)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Card insert/remove.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRL_A {
    #[doc = "1: Card inserted/removed."]
    INSREM = 1,
}
impl From<PRL_A> for bool {
    #[inline(always)]
    fn from(variant: PRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PRL`"]
pub type PRL_R = crate::R<bool, PRL_A>;
impl PRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, PRL_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(PRL_A::INSREM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INSREM`"]
    #[inline(always)]
    pub fn is_insrem(&self) -> bool {
        *self == PRL_A::INSREM
    }
}
#[doc = "Write proxy for field `PRL`"]
pub struct PRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Card inserted/removed."]
    #[inline(always)]
    pub fn insrem(self) -> &'a mut W {
        self.variant(PRL_A::INSREM)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "ETU counter overflow.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECNTOVER_A {
    #[doc = "1: ETU overflow."]
    OVR = 1,
}
impl From<ECNTOVER_A> for bool {
    #[inline(always)]
    fn from(variant: ECNTOVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECNTOVER`"]
pub type ECNTOVER_R = crate::R<bool, ECNTOVER_A>;
impl ECNTOVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ECNTOVER_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(ECNTOVER_A::OVR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `OVR`"]
    #[inline(always)]
    pub fn is_ovr(&self) -> bool {
        *self == ECNTOVER_A::OVR
    }
}
#[doc = "Write proxy for field `ECNTOVER`"]
pub struct ECNTOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> ECNTOVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECNTOVER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "ETU overflow."]
    #[inline(always)]
    pub fn ovr(self) -> &'a mut W {
        self.variant(ECNTOVER_A::OVR)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - ISO7816 idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write complete synchronization."]
    #[inline(always)]
    pub fn syncend(&self) -> SYNCEND_R {
        SYNCEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Card insert/remove."]
    #[inline(always)]
    pub fn prl(&self) -> PRL_R {
        PRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ETU counter overflow."]
    #[inline(always)]
    pub fn ecntover(&self) -> ECNTOVER_R {
        ECNTOVER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ISO7816 idle."]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 2 - Write complete synchronization."]
    #[inline(always)]
    pub fn syncend(&mut self) -> SYNCEND_W {
        SYNCEND_W { w: self }
    }
    #[doc = "Bit 1 - Card insert/remove."]
    #[inline(always)]
    pub fn prl(&mut self) -> PRL_W {
        PRL_W { w: self }
    }
    #[doc = "Bit 0 - ETU counter overflow."]
    #[inline(always)]
    pub fn ecntover(&mut self) -> ECNTOVER_W {
        ECNTOVER_W { w: self }
    }
}
