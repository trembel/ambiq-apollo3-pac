#[doc = "Reader of register STCFG"]
pub type R = crate::R<u32, super::STCFG>;
#[doc = "Writer for register STCFG"]
pub type W = crate::W<u32, super::STCFG>;
#[doc = "Register STCFG `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::STCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREEZE_A {
    #[doc = "0: Let the COUNTER register run on its input clock."]
    THAW = 0,
    #[doc = "1: Stop the COUNTER register for loading."]
    FREEZE = 1,
}
impl From<FREEZE_A> for bool {
    #[inline(always)]
    fn from(variant: FREEZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FREEZE`"]
pub type FREEZE_R = crate::R<bool, FREEZE_A>;
impl FREEZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREEZE_A {
        match self.bits {
            false => FREEZE_A::THAW,
            true => FREEZE_A::FREEZE,
        }
    }
    #[doc = "Checks if the value of the field is `THAW`"]
    #[inline(always)]
    pub fn is_thaw(&self) -> bool {
        *self == FREEZE_A::THAW
    }
    #[doc = "Checks if the value of the field is `FREEZE`"]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == FREEZE_A::FREEZE
    }
}
#[doc = "Write proxy for field `FREEZE`"]
pub struct FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREEZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREEZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn thaw(self) -> &'a mut W {
        self.variant(FREEZE_A::THAW)
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut W {
        self.variant(FREEZE_A::FREEZE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_A {
    #[doc = "0: Let the COUNTER register run on its input clock."]
    RUN = 0,
    #[doc = "1: Stop the COUNTER register for loading."]
    CLEAR = 1,
}
impl From<CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLEAR`"]
pub type CLEAR_R = crate::R<bool, CLEAR_A>;
impl CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLEAR_A {
        match self.bits {
            false => CLEAR_A::RUN,
            true => CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == CLEAR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLEAR_A::CLEAR
    }
}
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Let the COUNTER register run on its input clock."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CLEAR_A::RUN)
    }
    #[doc = "Stop the COUNTER register for loading."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_H_EN_A {
    #[doc = "0: Compare H disabled."]
    DISABLE = 0,
    #[doc = "1: Compare H enabled."]
    ENABLE = 1,
}
impl From<COMPARE_H_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_H_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_H_EN`"]
pub type COMPARE_H_EN_R = crate::R<bool, COMPARE_H_EN_A>;
impl COMPARE_H_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_H_EN_A {
        match self.bits {
            false => COMPARE_H_EN_A::DISABLE,
            true => COMPARE_H_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_H_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_H_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_H_EN`"]
pub struct COMPARE_H_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_H_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_H_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare H disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_H_EN_A::DISABLE)
    }
    #[doc = "Compare H enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_H_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_G_EN_A {
    #[doc = "0: Compare G disabled."]
    DISABLE = 0,
    #[doc = "1: Compare G enabled."]
    ENABLE = 1,
}
impl From<COMPARE_G_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_G_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_G_EN`"]
pub type COMPARE_G_EN_R = crate::R<bool, COMPARE_G_EN_A>;
impl COMPARE_G_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_G_EN_A {
        match self.bits {
            false => COMPARE_G_EN_A::DISABLE,
            true => COMPARE_G_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_G_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_G_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_G_EN`"]
pub struct COMPARE_G_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_G_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_G_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare G disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_G_EN_A::DISABLE)
    }
    #[doc = "Compare G enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_G_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_F_EN_A {
    #[doc = "0: Compare F disabled."]
    DISABLE = 0,
    #[doc = "1: Compare F enabled."]
    ENABLE = 1,
}
impl From<COMPARE_F_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_F_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_F_EN`"]
pub type COMPARE_F_EN_R = crate::R<bool, COMPARE_F_EN_A>;
impl COMPARE_F_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_F_EN_A {
        match self.bits {
            false => COMPARE_F_EN_A::DISABLE,
            true => COMPARE_F_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_F_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_F_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_F_EN`"]
pub struct COMPARE_F_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_F_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_F_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare F disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_F_EN_A::DISABLE)
    }
    #[doc = "Compare F enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_F_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_E_EN_A {
    #[doc = "0: Compare E disabled."]
    DISABLE = 0,
    #[doc = "1: Compare E enabled."]
    ENABLE = 1,
}
impl From<COMPARE_E_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_E_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_E_EN`"]
pub type COMPARE_E_EN_R = crate::R<bool, COMPARE_E_EN_A>;
impl COMPARE_E_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_E_EN_A {
        match self.bits {
            false => COMPARE_E_EN_A::DISABLE,
            true => COMPARE_E_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_E_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_E_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_E_EN`"]
pub struct COMPARE_E_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_E_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_E_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare E disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_E_EN_A::DISABLE)
    }
    #[doc = "Compare E enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_E_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_D_EN_A {
    #[doc = "0: Compare D disabled."]
    DISABLE = 0,
    #[doc = "1: Compare D enabled."]
    ENABLE = 1,
}
impl From<COMPARE_D_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_D_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_D_EN`"]
pub type COMPARE_D_EN_R = crate::R<bool, COMPARE_D_EN_A>;
impl COMPARE_D_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_D_EN_A {
        match self.bits {
            false => COMPARE_D_EN_A::DISABLE,
            true => COMPARE_D_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_D_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_D_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_D_EN`"]
pub struct COMPARE_D_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_D_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_D_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare D disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_D_EN_A::DISABLE)
    }
    #[doc = "Compare D enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_D_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_C_EN_A {
    #[doc = "0: Compare C disabled."]
    DISABLE = 0,
    #[doc = "1: Compare C enabled."]
    ENABLE = 1,
}
impl From<COMPARE_C_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_C_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_C_EN`"]
pub type COMPARE_C_EN_R = crate::R<bool, COMPARE_C_EN_A>;
impl COMPARE_C_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_C_EN_A {
        match self.bits {
            false => COMPARE_C_EN_A::DISABLE,
            true => COMPARE_C_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_C_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_C_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_C_EN`"]
pub struct COMPARE_C_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_C_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_C_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare C disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_C_EN_A::DISABLE)
    }
    #[doc = "Compare C enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_C_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_B_EN_A {
    #[doc = "0: Compare B disabled."]
    DISABLE = 0,
    #[doc = "1: Compare B enabled."]
    ENABLE = 1,
}
impl From<COMPARE_B_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_B_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_B_EN`"]
pub type COMPARE_B_EN_R = crate::R<bool, COMPARE_B_EN_A>;
impl COMPARE_B_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_B_EN_A {
        match self.bits {
            false => COMPARE_B_EN_A::DISABLE,
            true => COMPARE_B_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_B_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_B_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_B_EN`"]
pub struct COMPARE_B_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_B_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_B_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare B disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_B_EN_A::DISABLE)
    }
    #[doc = "Compare B enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_B_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE_A_EN_A {
    #[doc = "0: Compare A disabled."]
    DISABLE = 0,
    #[doc = "1: Compare A enabled."]
    ENABLE = 1,
}
impl From<COMPARE_A_EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE_A_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE_A_EN`"]
pub type COMPARE_A_EN_R = crate::R<bool, COMPARE_A_EN_A>;
impl COMPARE_A_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE_A_EN_A {
        match self.bits {
            false => COMPARE_A_EN_A::DISABLE,
            true => COMPARE_A_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COMPARE_A_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COMPARE_A_EN_A::ENABLE
    }
}
#[doc = "Write proxy for field `COMPARE_A_EN`"]
pub struct COMPARE_A_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_A_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE_A_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare A disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COMPARE_A_EN_A::DISABLE)
    }
    #[doc = "Compare A enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COMPARE_A_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Selects an appropriate clock source and divider to use for the System Timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: No clock enabled."]
    NOCLK = 0,
    #[doc = "1: 3MHz from the HFRC clock divider."]
    HFRC_DIV16 = 1,
    #[doc = "2: 187.5KHz from the HFRC clock divider."]
    HFRC_DIV256 = 2,
    #[doc = "3: 32768Hz from the crystal oscillator."]
    XTAL_DIV1 = 3,
    #[doc = "4: 16384Hz from the crystal oscillator."]
    XTAL_DIV2 = 4,
    #[doc = "5: 1024Hz from the crystal oscillator."]
    XTAL_DIV32 = 5,
    #[doc = "6: Approximately 1KHz from the LFRC oscillator (uncalibrated)."]
    LFRC_DIV1 = 6,
    #[doc = "7: Use CTIMER 0 section A as a prescaler for the clock source."]
    CTIMER0A = 7,
    #[doc = "8: Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source."]
    CTIMER0B = 8,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKSEL_A::NOCLK),
            1 => Val(CLKSEL_A::HFRC_DIV16),
            2 => Val(CLKSEL_A::HFRC_DIV256),
            3 => Val(CLKSEL_A::XTAL_DIV1),
            4 => Val(CLKSEL_A::XTAL_DIV2),
            5 => Val(CLKSEL_A::XTAL_DIV32),
            6 => Val(CLKSEL_A::LFRC_DIV1),
            7 => Val(CLKSEL_A::CTIMER0A),
            8 => Val(CLKSEL_A::CTIMER0B),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCLK`"]
    #[inline(always)]
    pub fn is_noclk(&self) -> bool {
        *self == CLKSEL_A::NOCLK
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == CLKSEL_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == CLKSEL_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV1`"]
    #[inline(always)]
    pub fn is_xtal_div1(&self) -> bool {
        *self == CLKSEL_A::XTAL_DIV1
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV2`"]
    #[inline(always)]
    pub fn is_xtal_div2(&self) -> bool {
        *self == CLKSEL_A::XTAL_DIV2
    }
    #[doc = "Checks if the value of the field is `XTAL_DIV32`"]
    #[inline(always)]
    pub fn is_xtal_div32(&self) -> bool {
        *self == CLKSEL_A::XTAL_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1`"]
    #[inline(always)]
    pub fn is_lfrc_div1(&self) -> bool {
        *self == CLKSEL_A::LFRC_DIV1
    }
    #[doc = "Checks if the value of the field is `CTIMER0A`"]
    #[inline(always)]
    pub fn is_ctimer0a(&self) -> bool {
        *self == CLKSEL_A::CTIMER0A
    }
    #[doc = "Checks if the value of the field is `CTIMER0B`"]
    #[inline(always)]
    pub fn is_ctimer0b(&self) -> bool {
        *self == CLKSEL_A::CTIMER0B
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock enabled."]
    #[inline(always)]
    pub fn noclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::NOCLK)
    }
    #[doc = "3MHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC_DIV16)
    }
    #[doc = "187.5KHz from the HFRC clock divider."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(CLKSEL_A::HFRC_DIV256)
    }
    #[doc = "32768Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_div1(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV1)
    }
    #[doc = "16384Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_div2(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV2)
    }
    #[doc = "1024Hz from the crystal oscillator."]
    #[inline(always)]
    pub fn xtal_div32(self) -> &'a mut W {
        self.variant(CLKSEL_A::XTAL_DIV32)
    }
    #[doc = "Approximately 1KHz from the LFRC oscillator (uncalibrated)."]
    #[inline(always)]
    pub fn lfrc_div1(self) -> &'a mut W {
        self.variant(CLKSEL_A::LFRC_DIV1)
    }
    #[doc = "Use CTIMER 0 section A as a prescaler for the clock source."]
    #[inline(always)]
    pub fn ctimer0a(self) -> &'a mut W {
        self.variant(CLKSEL_A::CTIMER0A)
    }
    #[doc = "Use CTIMER 0 section B (or A and B linked together) as a prescaler for the clock source."]
    #[inline(always)]
    pub fn ctimer0b(self) -> &'a mut W {
        self.variant(CLKSEL_A::CTIMER0B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_h_en(&self) -> COMPARE_H_EN_R {
        COMPARE_H_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_g_en(&self) -> COMPARE_G_EN_R {
        COMPARE_G_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_f_en(&self) -> COMPARE_F_EN_R {
        COMPARE_F_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_e_en(&self) -> COMPARE_E_EN_R {
        COMPARE_E_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_d_en(&self) -> COMPARE_D_EN_R {
        COMPARE_D_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_c_en(&self) -> COMPARE_C_EN_R {
        COMPARE_C_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_b_en(&self) -> COMPARE_B_EN_R {
        COMPARE_B_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_a_en(&self) -> COMPARE_A_EN_R {
        COMPARE_A_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to one to freeze the clock input to the COUNTER register. Once frozen, the value can be safely written from the MCU. Unfreeze to resume."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W { w: self }
    }
    #[doc = "Bit 30 - Set this bit to one to clear the System Timer register. If this bit is set to '1', the system timer register will stay cleared. It needs to be set to '0' for the system timer to start running."]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 15 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_h_en(&mut self) -> COMPARE_H_EN_W {
        COMPARE_H_EN_W { w: self }
    }
    #[doc = "Bit 14 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_g_en(&mut self) -> COMPARE_G_EN_W {
        COMPARE_G_EN_W { w: self }
    }
    #[doc = "Bit 13 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_f_en(&mut self) -> COMPARE_F_EN_W {
        COMPARE_F_EN_W { w: self }
    }
    #[doc = "Bit 12 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_e_en(&mut self) -> COMPARE_E_EN_W {
        COMPARE_E_EN_W { w: self }
    }
    #[doc = "Bit 11 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_d_en(&mut self) -> COMPARE_D_EN_W {
        COMPARE_D_EN_W { w: self }
    }
    #[doc = "Bit 10 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_c_en(&mut self) -> COMPARE_C_EN_W {
        COMPARE_C_EN_W { w: self }
    }
    #[doc = "Bit 9 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_b_en(&mut self) -> COMPARE_B_EN_W {
        COMPARE_B_EN_W { w: self }
    }
    #[doc = "Bit 8 - Selects whether compare is enabled for the corresponding SCMPR register. If compare is enabled, the interrupt status is set once the comparision is met."]
    #[inline(always)]
    pub fn compare_a_en(&mut self) -> COMPARE_A_EN_W {
        COMPARE_A_EN_W { w: self }
    }
    #[doc = "Bits 0:3 - Selects an appropriate clock source and divider to use for the System Timer clock."]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
}
