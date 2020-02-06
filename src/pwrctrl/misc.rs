#[doc = "Reader of register MISC"]
pub type R = crate::R<u32, super::MISC>;
#[doc = "Writer for register MISC"]
pub type W = crate::W<u32, super::MISC>;
#[doc = "Register MISC `reset()`'s with value 0"]
impl crate::ResetValue for super::MISC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMVRLPBLE_A {
    #[doc = "1: Mem VR can go to lp mode even when BLE is powered on."]
    EN = 1,
    #[doc = "0: Mem VR will stay in active mode when BLE is powered on."]
    DIS = 0,
}
impl From<MEMVRLPBLE_A> for bool {
    #[inline(always)]
    fn from(variant: MEMVRLPBLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MEMVRLPBLE`"]
pub type MEMVRLPBLE_R = crate::R<bool, MEMVRLPBLE_A>;
impl MEMVRLPBLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEMVRLPBLE_A {
        match self.bits {
            true => MEMVRLPBLE_A::EN,
            false => MEMVRLPBLE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == MEMVRLPBLE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == MEMVRLPBLE_A::DIS
    }
}
#[doc = "Write proxy for field `MEMVRLPBLE`"]
pub struct MEMVRLPBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMVRLPBLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMVRLPBLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mem VR can go to lp mode even when BLE is powered on."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::EN)
    }
    #[doc = "Mem VR will stay in active mode when BLE is powered on."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MEMVRLPBLE_A::DIS)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FORCEMEMVRLPTIMERS`"]
pub type FORCEMEMVRLPTIMERS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEMEMVRLPTIMERS`"]
pub struct FORCEMEMVRLPTIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEMEMVRLPTIMERS_W<'a> {
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
impl R {
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&self) -> MEMVRLPBLE_R {
        MEMVRLPBLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&self) -> FORCEMEMVRLPTIMERS_R {
        FORCEMEMVRLPTIMERS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Control Bit to let Mem VR go to lp mode in deep sleep even when BLEL or BLEH is powered on given none of the other domains require it."]
    #[inline(always)]
    pub fn memvrlpble(&mut self) -> MEMVRLPBLE_W {
        MEMVRLPBLE_W { w: self }
    }
    #[doc = "Bit 3 - Control Bit to force Mem VR to LP mode in deep sleep even when hfrc based ctimer or stimer is running."]
    #[inline(always)]
    pub fn forcememvrlptimers(&mut self) -> FORCEMEMVRLPTIMERS_W {
        FORCEMEMVRLPTIMERS_W { w: self }
    }
}
