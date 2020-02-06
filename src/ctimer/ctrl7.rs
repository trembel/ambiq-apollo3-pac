#[doc = "Reader of register CTRL7"]
pub type R = crate::R<u32, super::CTRL7>;
#[doc = "Writer for register CTRL7"]
pub type W = crate::W<u32, super::CTRL7>;
#[doc = "Register CTRL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter/Timer A7/B7 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK7_A {
    #[doc = "0: Use A7/B7 timers as two independent 16-bit timers (default)."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A7/B7 timers into a single 32-bit timer."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK7_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTLINK7`"]
pub type CTLINK7_R = crate::R<bool, CTLINK7_A>;
impl CTLINK7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK7_A {
        match self.bits {
            false => CTLINK7_A::TWO_16BIT_TIMERS,
            true => CTLINK7_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        *self == CTLINK7_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        *self == CTLINK7_A::_32BIT_TIMER
    }
}
#[doc = "Write proxy for field `CTLINK7`"]
pub struct CTLINK7_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Use A7/B7 timers as two independent 16-bit timers (default)."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK7_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A7/B7 timers into a single 32-bit timer."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK7_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B7 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7POL_A {
    #[doc = "0: The polarity of the TMRPINB7 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB7 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRB7POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB7POL`"]
pub type TMRB7POL_R = crate::R<bool, TMRB7POL_A>;
impl TMRB7POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7POL_A {
        match self.bits {
            false => TMRB7POL_A::NORMAL,
            true => TMRB7POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRB7POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRB7POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRB7POL`"]
pub struct TMRB7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINB7 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB7POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB7 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB7POL_A::INVERTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Counter/Timer B7 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7CLR_A {
    #[doc = "0: Allow counter/timer B7 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer B7 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRB7CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB7CLR`"]
pub type TMRB7CLR_R = crate::R<bool, TMRB7CLR_A>;
impl TMRB7CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7CLR_A {
        match self.bits {
            false => TMRB7CLR_A::RUN,
            true => TMRB7CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRB7CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRB7CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRB7CLR`"]
pub struct TMRB7CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer B7 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB7CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B7 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB7CLR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7IE1_A {
    #[doc = "0: Disable counter/timer B7 from generating an interrupt based on COMPR1."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B7 to generate an interrupt based on COMPR1."]
    EN = 1,
}
impl From<TMRB7IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB7IE1`"]
pub type TMRB7IE1_R = crate::R<bool, TMRB7IE1_A>;
impl TMRB7IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7IE1_A {
        match self.bits {
            false => TMRB7IE1_A::DIS,
            true => TMRB7IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB7IE1_A::EN
    }
}
#[doc = "Write proxy for field `TMRB7IE1`"]
pub struct TMRB7IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7IE1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Counter/Timer B7 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7IE0_A {
    #[doc = "0: Disable counter/timer B7 from generating an interrupt based on COMPR0."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B7 to generate an interrupt based on COMPR0"]
    EN = 1,
}
impl From<TMRB7IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB7IE0`"]
pub type TMRB7IE0_R = crate::R<bool, TMRB7IE0_A>;
impl TMRB7IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7IE0_A {
        match self.bits {
            false => TMRB7IE0_A::DIS,
            true => TMRB7IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB7IE0_A::EN
    }
}
#[doc = "Write proxy for field `TMRB7IE0`"]
pub struct TMRB7IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer B7 from generating an interrupt based on COMPR0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B7 to generate an interrupt based on COMPR0"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7IE0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Counter/Timer B7 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB7FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B7, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B7, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM"]
    ALTPWN = 7,
}
impl From<TMRB7FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB7FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB7FN`"]
pub type TMRB7FN_R = crate::R<u8, TMRB7FN_A>;
impl TMRB7FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7FN_A {
        match self.bits {
            0 => TMRB7FN_A::SINGLECOUNT,
            1 => TMRB7FN_A::REPEATEDCOUNT,
            2 => TMRB7FN_A::PULSE_ONCE,
            3 => TMRB7FN_A::PULSE_CONT,
            4 => TMRB7FN_A::SINGLEPATTERN,
            5 => TMRB7FN_A::REPEATPATTERN,
            6 => TMRB7FN_A::CONTINUOUS,
            7 => TMRB7FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRB7FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRB7FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRB7FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRB7FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRB7FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRB7FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRB7FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRB7FN_A::ALTPWN
    }
}
#[doc = "Write proxy for field `TMRB7FN`"]
pub struct TMRB7FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7FN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B7, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB7FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B7, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB7FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B7, assert, count to CMPR1B7, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB7FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B7, assert, count to CMPR1B7, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB7FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB7FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB7FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB7FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM"]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB7FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B7 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB7CLK_A {
    #[doc = "0: Clock source is TMRPINB."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4"]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16"]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256"]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024"]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096"]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated)."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2"]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16"]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128"]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2"]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32"]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024"]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC"]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode)"]
    HCLK_DIV4 = 15,
    #[doc = "16: Clock source is XT / 4"]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8"]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32"]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERA7 OUT."]
    CTMRA7 = 20,
    #[doc = "21: Clock source is CTIMERA2 OUT."]
    CTMRA2 = 21,
    #[doc = "22: Clock source is CTIMERB2 OUT."]
    CTMRB2 = 22,
    #[doc = "23: Clock source is CTIMERA0 OUT."]
    CTMRA0 = 23,
    #[doc = "24: Clock source is CTIMERB0 OUT."]
    CTMRB0 = 24,
    #[doc = "25: Clock source is CTIMERB1 OUT."]
    CTMRB1 = 25,
    #[doc = "26: Clock source is CTIMERB3 OUT."]
    CTMRB3 = 26,
    #[doc = "27: Clock source is CTIMERB4 OUT."]
    CTMRB4 = 27,
    #[doc = "28: Clock source is CTIMERB5 OUT."]
    CTMRB5 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses."]
    BUCKA = 31,
}
impl From<TMRB7CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB7CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRB7CLK`"]
pub type TMRB7CLK_R = crate::R<u8, TMRB7CLK_A>;
impl TMRB7CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRB7CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRB7CLK_A::TMRPIN),
            1 => Val(TMRB7CLK_A::HFRC_DIV4),
            2 => Val(TMRB7CLK_A::HFRC_DIV16),
            3 => Val(TMRB7CLK_A::HFRC_DIV256),
            4 => Val(TMRB7CLK_A::HFRC_DIV1024),
            5 => Val(TMRB7CLK_A::HFRC_DIV4K),
            6 => Val(TMRB7CLK_A::XT),
            7 => Val(TMRB7CLK_A::XT_DIV2),
            8 => Val(TMRB7CLK_A::XT_DIV16),
            9 => Val(TMRB7CLK_A::XT_DIV128),
            10 => Val(TMRB7CLK_A::LFRC_DIV2),
            11 => Val(TMRB7CLK_A::LFRC_DIV32),
            12 => Val(TMRB7CLK_A::LFRC_DIV1K),
            13 => Val(TMRB7CLK_A::LFRC),
            14 => Val(TMRB7CLK_A::RTC_100HZ),
            15 => Val(TMRB7CLK_A::HCLK_DIV4),
            16 => Val(TMRB7CLK_A::XT_DIV4),
            17 => Val(TMRB7CLK_A::XT_DIV8),
            18 => Val(TMRB7CLK_A::XT_DIV32),
            20 => Val(TMRB7CLK_A::CTMRA7),
            21 => Val(TMRB7CLK_A::CTMRA2),
            22 => Val(TMRB7CLK_A::CTMRB2),
            23 => Val(TMRB7CLK_A::CTMRA0),
            24 => Val(TMRB7CLK_A::CTMRB0),
            25 => Val(TMRB7CLK_A::CTMRB1),
            26 => Val(TMRB7CLK_A::CTMRB3),
            27 => Val(TMRB7CLK_A::CTMRB4),
            28 => Val(TMRB7CLK_A::CTMRB5),
            29 => Val(TMRB7CLK_A::BUCKBLE),
            30 => Val(TMRB7CLK_A::BUCKB),
            31 => Val(TMRB7CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRB7CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRB7CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRB7CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRB7CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRB7CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRB7CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRB7CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRB7CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRB7CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRB7CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRB7CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRB7CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRB7CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRB7CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline(always)]
    pub fn is_ctmra7(&self) -> bool {
        *self == TMRB7CLK_A::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRB7CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline(always)]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRB7CLK_A::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRB7CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        *self == TMRB7CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRB7CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRB7CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRB7CLK`"]
pub struct TMRB7CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode)"]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA7 OUT."]
    #[inline(always)]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRA7)
    }
    #[doc = "Clock source is CTIMERA2 OUT."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA0 OUT."]
    #[inline(always)]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::CTMRB5)
    }
    #[doc = "Clock source is BLE buck converter TON pulses."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB7CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B7 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7EN_A {
    #[doc = "0: Counter/Timer B7 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer B7 Enable."]
    EN = 1,
}
impl From<TMRB7EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRB7EN`"]
pub type TMRB7EN_R = crate::R<bool, TMRB7EN_A>;
impl TMRB7EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7EN_A {
        match self.bits {
            false => TMRB7EN_A::DIS,
            true => TMRB7EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRB7EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRB7EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRB7EN`"]
pub struct TMRB7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer B7 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7EN_A::DIS)
    }
    #[doc = "Counter/Timer B7 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A7 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7POL_A {
    #[doc = "0: The polarity of the TMRPINA7 pin is the same as the timer output."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA7 pin is the inverse of the timer output."]
    INVERTED = 1,
}
impl From<TMRA7POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA7POL`"]
pub type TMRA7POL_R = crate::R<bool, TMRA7POL_A>;
impl TMRA7POL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7POL_A {
        match self.bits {
            false => TMRA7POL_A::NORMAL,
            true => TMRA7POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TMRA7POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TMRA7POL_A::INVERTED
    }
}
#[doc = "Write proxy for field `TMRA7POL`"]
pub struct TMRA7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7POL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The polarity of the TMRPINA7 pin is the same as the timer output."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA7POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA7 pin is the inverse of the timer output."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA7POL_A::INVERTED)
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
#[doc = "Counter/Timer A7 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7CLR_A {
    #[doc = "0: Allow counter/timer A7 to run"]
    RUN = 0,
    #[doc = "1: Holds counter/timer A7 at 0x0000."]
    CLEAR = 1,
}
impl From<TMRA7CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA7CLR`"]
pub type TMRA7CLR_R = crate::R<bool, TMRA7CLR_A>;
impl TMRA7CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7CLR_A {
        match self.bits {
            false => TMRA7CLR_A::RUN,
            true => TMRA7CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == TMRA7CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == TMRA7CLR_A::CLEAR
    }
}
#[doc = "Write proxy for field `TMRA7CLR`"]
pub struct TMRA7CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Allow counter/timer A7 to run"]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA7CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A7 at 0x0000."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA7CLR_A::CLEAR)
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
#[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7IE1_A {
    #[doc = "0: Disable counter/timer A7 from generating an interrupt based on COMPR1."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A7 to generate an interrupt based on COMPR1."]
    EN = 1,
}
impl From<TMRA7IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA7IE1`"]
pub type TMRA7IE1_R = crate::R<bool, TMRA7IE1_A>;
impl TMRA7IE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7IE1_A {
        match self.bits {
            false => TMRA7IE1_A::DIS,
            true => TMRA7IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA7IE1_A::EN
    }
}
#[doc = "Write proxy for field `TMRA7IE1`"]
pub struct TMRA7IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7IE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR1."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR1."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7IE1_A::EN)
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
#[doc = "Counter/Timer A7 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7IE0_A {
    #[doc = "0: Disable counter/timer A7 from generating an interrupt based on COMPR0."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A7 to generate an interrupt based on COMPR0."]
    EN = 1,
}
impl From<TMRA7IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA7IE0`"]
pub type TMRA7IE0_R = crate::R<bool, TMRA7IE0_A>;
impl TMRA7IE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7IE0_A {
        match self.bits {
            false => TMRA7IE0_A::DIS,
            true => TMRA7IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA7IE0_A::EN
    }
}
#[doc = "Write proxy for field `TMRA7IE0`"]
pub struct TMRA7IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7IE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable counter/timer A7 from generating an interrupt based on COMPR0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A7 to generate an interrupt based on COMPR0."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7IE0_A::EN)
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
#[doc = "Counter/Timer A7 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA7FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A7, stop."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A7, restart."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM"]
    ALTPWN = 7,
}
impl From<TMRA7FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA7FN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA7FN`"]
pub type TMRA7FN_R = crate::R<u8, TMRA7FN_A>;
impl TMRA7FN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7FN_A {
        match self.bits {
            0 => TMRA7FN_A::SINGLECOUNT,
            1 => TMRA7FN_A::REPEATEDCOUNT,
            2 => TMRA7FN_A::PULSE_ONCE,
            3 => TMRA7FN_A::PULSE_CONT,
            4 => TMRA7FN_A::SINGLEPATTERN,
            5 => TMRA7FN_A::REPEATPATTERN,
            6 => TMRA7FN_A::CONTINUOUS,
            7 => TMRA7FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        *self == TMRA7FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        *self == TMRA7FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        *self == TMRA7FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        *self == TMRA7FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        *self == TMRA7FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        *self == TMRA7FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TMRA7FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        *self == TMRA7FN_A::ALTPWN
    }
}
#[doc = "Write proxy for field `TMRA7FN`"]
pub struct TMRA7FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7FN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A7, stop."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA7FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A7, restart."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA7FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A7, assert, count to CMPR1A7, deassert, stop."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA7FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A7, assert, count to CMPR1A7, deassert, restart."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA7FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA7FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA7FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA7FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM"]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA7FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A7 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA7CLK_A {
    #[doc = "0: Clock source is TMRPINA."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4"]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16"]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256"]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024"]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096"]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated)."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2"]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16"]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128"]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2"]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32"]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024"]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC"]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode)"]
    HCLK_DIV4 = 15,
    #[doc = "16: Clock source is XT / 4"]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8"]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32"]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERB7 OUT."]
    CTMRB7 = 20,
    #[doc = "21: Clock source is CTIMERA2 OUT."]
    CTMRA2 = 21,
    #[doc = "22: Clock source is CTIMERB2 OUT."]
    CTMRB2 = 22,
    #[doc = "23: Clock source is CTIMERA0 OUT."]
    CTMRA0 = 23,
    #[doc = "24: Clock source is CTIMERB0 OUT."]
    CTMRB0 = 24,
    #[doc = "25: Clock source is CTIMERB1 OUT."]
    CTMRB1 = 25,
    #[doc = "26: Clock source is CTIMERB3 OUT."]
    CTMRB3 = 26,
    #[doc = "27: Clock source is CTIMERB4 OUT."]
    CTMRB4 = 27,
    #[doc = "28: Clock source is CTIMERB5 OUT."]
    CTMRB5 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses."]
    BUCKA = 31,
}
impl From<TMRA7CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA7CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TMRA7CLK`"]
pub type TMRA7CLK_R = crate::R<u8, TMRA7CLK_A>;
impl TMRA7CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TMRA7CLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TMRA7CLK_A::TMRPIN),
            1 => Val(TMRA7CLK_A::HFRC_DIV4),
            2 => Val(TMRA7CLK_A::HFRC_DIV16),
            3 => Val(TMRA7CLK_A::HFRC_DIV256),
            4 => Val(TMRA7CLK_A::HFRC_DIV1024),
            5 => Val(TMRA7CLK_A::HFRC_DIV4K),
            6 => Val(TMRA7CLK_A::XT),
            7 => Val(TMRA7CLK_A::XT_DIV2),
            8 => Val(TMRA7CLK_A::XT_DIV16),
            9 => Val(TMRA7CLK_A::XT_DIV128),
            10 => Val(TMRA7CLK_A::LFRC_DIV2),
            11 => Val(TMRA7CLK_A::LFRC_DIV32),
            12 => Val(TMRA7CLK_A::LFRC_DIV1K),
            13 => Val(TMRA7CLK_A::LFRC),
            14 => Val(TMRA7CLK_A::RTC_100HZ),
            15 => Val(TMRA7CLK_A::HCLK_DIV4),
            16 => Val(TMRA7CLK_A::XT_DIV4),
            17 => Val(TMRA7CLK_A::XT_DIV8),
            18 => Val(TMRA7CLK_A::XT_DIV32),
            20 => Val(TMRA7CLK_A::CTMRB7),
            21 => Val(TMRA7CLK_A::CTMRA2),
            22 => Val(TMRA7CLK_A::CTMRB2),
            23 => Val(TMRA7CLK_A::CTMRA0),
            24 => Val(TMRA7CLK_A::CTMRB0),
            25 => Val(TMRA7CLK_A::CTMRB1),
            26 => Val(TMRA7CLK_A::CTMRB3),
            27 => Val(TMRA7CLK_A::CTMRB4),
            28 => Val(TMRA7CLK_A::CTMRB5),
            29 => Val(TMRA7CLK_A::BUCKBLE),
            30 => Val(TMRA7CLK_A::BUCKB),
            31 => Val(TMRA7CLK_A::BUCKA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        *self == TMRA7CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        *self == TMRA7CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        *self == TMRA7CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        *self == TMRA7CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        *self == TMRA7CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        *self == TMRA7CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        *self == TMRA7CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        *self == TMRA7CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        *self == TMRA7CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        *self == TMRA7CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == TMRA7CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        *self == TMRA7CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        *self == TMRA7CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        *self == TMRA7CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline(always)]
    pub fn is_ctmrb7(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        *self == TMRA7CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline(always)]
    pub fn is_ctmra0(&self) -> bool {
        *self == TMRA7CLK_A::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        *self == TMRA7CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        *self == TMRA7CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        *self == TMRA7CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        *self == TMRA7CLK_A::BUCKA
    }
}
#[doc = "Write proxy for field `TMRA7CLK`"]
pub struct TMRA7CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4"]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16"]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256"]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024"]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096"]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated)."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2"]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16"]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128"]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2"]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32"]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024"]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode)"]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4"]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8"]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32"]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB7 OUT."]
    #[inline(always)]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB7)
    }
    #[doc = "Clock source is CTIMERA2 OUT."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA0 OUT."]
    #[inline(always)]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB0 OUT."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB3 OUT."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::CTMRB5)
    }
    #[doc = "Clock source is BLE buck converter TON pulses."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA7CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A7 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7EN_A {
    #[doc = "0: Counter/Timer A7 Disable."]
    DIS = 0,
    #[doc = "1: Counter/Timer A7 Enable."]
    EN = 1,
}
impl From<TMRA7EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TMRA7EN`"]
pub type TMRA7EN_R = crate::R<bool, TMRA7EN_A>;
impl TMRA7EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7EN_A {
        match self.bits {
            false => TMRA7EN_A::DIS,
            true => TMRA7EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TMRA7EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TMRA7EN_A::EN
    }
}
#[doc = "Write proxy for field `TMRA7EN`"]
pub struct TMRA7EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter/Timer A7 Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7EN_A::DIS)
    }
    #[doc = "Counter/Timer A7 Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A7/B7 Link bit."]
    #[inline(always)]
    pub fn ctlink7(&self) -> CTLINK7_R {
        CTLINK7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B7 output polarity."]
    #[inline(always)]
    pub fn tmrb7pol(&self) -> TMRB7POL_R {
        TMRB7POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B7 Clear bit."]
    #[inline(always)]
    pub fn tmrb7clr(&self) -> TMRB7CLR_R {
        TMRB7CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb7ie1(&self) -> TMRB7IE1_R {
        TMRB7IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb7ie0(&self) -> TMRB7IE0_R {
        TMRB7IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B7 Function Select."]
    #[inline(always)]
    pub fn tmrb7fn(&self) -> TMRB7FN_R {
        TMRB7FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B7 Clock Select."]
    #[inline(always)]
    pub fn tmrb7clk(&self) -> TMRB7CLK_R {
        TMRB7CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B7 Enable bit."]
    #[inline(always)]
    pub fn tmrb7en(&self) -> TMRB7EN_R {
        TMRB7EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A7 output polarity."]
    #[inline(always)]
    pub fn tmra7pol(&self) -> TMRA7POL_R {
        TMRA7POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A7 Clear bit."]
    #[inline(always)]
    pub fn tmra7clr(&self) -> TMRA7CLR_R {
        TMRA7CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra7ie1(&self) -> TMRA7IE1_R {
        TMRA7IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra7ie0(&self) -> TMRA7IE0_R {
        TMRA7IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A7 Function Select."]
    #[inline(always)]
    pub fn tmra7fn(&self) -> TMRA7FN_R {
        TMRA7FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A7 Clock Select."]
    #[inline(always)]
    pub fn tmra7clk(&self) -> TMRA7CLK_R {
        TMRA7CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A7 Enable bit."]
    #[inline(always)]
    pub fn tmra7en(&self) -> TMRA7EN_R {
        TMRA7EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A7/B7 Link bit."]
    #[inline(always)]
    pub fn ctlink7(&mut self) -> CTLINK7_W {
        CTLINK7_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B7 output polarity."]
    #[inline(always)]
    pub fn tmrb7pol(&mut self) -> TMRB7POL_W {
        TMRB7POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B7 Clear bit."]
    #[inline(always)]
    pub fn tmrb7clr(&mut self) -> TMRB7CLR_W {
        TMRB7CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B7 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb7ie1(&mut self) -> TMRB7IE1_W {
        TMRB7IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B7 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb7ie0(&mut self) -> TMRB7IE0_W {
        TMRB7IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B7 Function Select."]
    #[inline(always)]
    pub fn tmrb7fn(&mut self) -> TMRB7FN_W {
        TMRB7FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B7 Clock Select."]
    #[inline(always)]
    pub fn tmrb7clk(&mut self) -> TMRB7CLK_W {
        TMRB7CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B7 Enable bit."]
    #[inline(always)]
    pub fn tmrb7en(&mut self) -> TMRB7EN_W {
        TMRB7EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A7 output polarity."]
    #[inline(always)]
    pub fn tmra7pol(&mut self) -> TMRA7POL_W {
        TMRA7POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A7 Clear bit."]
    #[inline(always)]
    pub fn tmra7clr(&mut self) -> TMRA7CLR_W {
        TMRA7CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A7 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra7ie1(&mut self) -> TMRA7IE1_W {
        TMRA7IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A7 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra7ie0(&mut self) -> TMRA7IE0_W {
        TMRA7IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A7 Function Select."]
    #[inline(always)]
    pub fn tmra7fn(&mut self) -> TMRA7FN_W {
        TMRA7FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A7 Clock Select."]
    #[inline(always)]
    pub fn tmra7clk(&mut self) -> TMRA7CLK_W {
        TMRA7CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A7 Enable bit."]
    #[inline(always)]
    pub fn tmra7en(&mut self) -> TMRA7EN_W {
        TMRA7EN_W { w: self }
    }
}
