#[doc = "Reader of register SL5CFG"]
pub type R = crate::R<u32, super::SL5CFG>;
#[doc = "Writer for register SL5CFG"]
pub type W = crate::W<u32, super::SL5CFG>;
#[doc = "Register SL5CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::SL5CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL5_A {
    #[doc = "0: Average in 1 measurement in the accumulate divide module for this slot."]
    AVG_1_MSRMT = 0,
    #[doc = "1: Average in 2 measurements in the accumulate divide module for this slot."]
    AVG_2_MSRMTS = 1,
    #[doc = "2: Average in 4 measurements in the accumulate divide module for this slot."]
    AVG_4_MSRMTS = 2,
    #[doc = "3: Average in 8 measurements in the accumulate divide module for this slot."]
    AVG_8_MSRMT = 3,
    #[doc = "4: Average in 16 measurements in the accumulate divide module for this slot."]
    AVG_16_MSRMTS = 4,
    #[doc = "5: Average in 32 measurements in the accumulate divide module for this slot."]
    AVG_32_MSRMTS = 5,
    #[doc = "6: Average in 64 measurements in the accumulate divide module for this slot."]
    AVG_64_MSRMTS = 6,
    #[doc = "7: Average in 128 measurements in the accumulate divide module for this slot."]
    AVG_128_MSRMTS = 7,
}
impl From<ADSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADSEL5`"]
pub type ADSEL5_R = crate::R<u8, ADSEL5_A>;
impl ADSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL5_A {
        match self.bits {
            0 => ADSEL5_A::AVG_1_MSRMT,
            1 => ADSEL5_A::AVG_2_MSRMTS,
            2 => ADSEL5_A::AVG_4_MSRMTS,
            3 => ADSEL5_A::AVG_8_MSRMT,
            4 => ADSEL5_A::AVG_16_MSRMTS,
            5 => ADSEL5_A::AVG_32_MSRMTS,
            6 => ADSEL5_A::AVG_64_MSRMTS,
            7 => ADSEL5_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        *self == ADSEL5_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        *self == ADSEL5_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        *self == ADSEL5_A::AVG_128_MSRMTS
    }
}
#[doc = "Write proxy for field `ADSEL5`"]
pub struct ADSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL5_A::AVG_128_MSRMTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Set the Precision Mode For Slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMODE5_A {
    #[doc = "0: 14-bit precision mode"]
    P14B = 0,
    #[doc = "1: 12-bit precision mode"]
    P12B = 1,
    #[doc = "2: 10-bit precision mode"]
    P10B = 2,
    #[doc = "3: 8-bit precision mode"]
    P8B = 3,
}
impl From<PRMODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMODE5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRMODE5`"]
pub type PRMODE5_R = crate::R<u8, PRMODE5_A>;
impl PRMODE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRMODE5_A {
        match self.bits {
            0 => PRMODE5_A::P14B,
            1 => PRMODE5_A::P12B,
            2 => PRMODE5_A::P10B,
            3 => PRMODE5_A::P8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P14B`"]
    #[inline(always)]
    pub fn is_p14b(&self) -> bool {
        *self == PRMODE5_A::P14B
    }
    #[doc = "Checks if the value of the field is `P12B`"]
    #[inline(always)]
    pub fn is_p12b(&self) -> bool {
        *self == PRMODE5_A::P12B
    }
    #[doc = "Checks if the value of the field is `P10B`"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        *self == PRMODE5_A::P10B
    }
    #[doc = "Checks if the value of the field is `P8B`"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        *self == PRMODE5_A::P8B
    }
}
#[doc = "Write proxy for field `PRMODE5`"]
pub struct PRMODE5_W<'a> {
    w: &'a mut W,
}
impl<'a> PRMODE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRMODE5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "14-bit precision mode"]
    #[inline(always)]
    pub fn p14b(self) -> &'a mut W {
        self.variant(PRMODE5_A::P14B)
    }
    #[doc = "12-bit precision mode"]
    #[inline(always)]
    pub fn p12b(self) -> &'a mut W {
        self.variant(PRMODE5_A::P12B)
    }
    #[doc = "10-bit precision mode"]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut W {
        self.variant(PRMODE5_A::P10B)
    }
    #[doc = "8-bit precision mode"]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut W {
        self.variant(PRMODE5_A::P8B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL5_A {
    #[doc = "0: single ended external GPIO connection to pad16."]
    SE0 = 0,
    #[doc = "1: single ended external GPIO connection to pad29."]
    SE1 = 1,
    #[doc = "2: single ended external GPIO connection to pad11."]
    SE2 = 2,
    #[doc = "3: single ended external GPIO connection to pad31."]
    SE3 = 3,
    #[doc = "4: single ended external GPIO connection to pad32."]
    SE4 = 4,
    #[doc = "5: single ended external GPIO connection to pad33."]
    SE5 = 5,
    #[doc = "6: single ended external GPIO connection to pad34."]
    SE6 = 6,
    #[doc = "7: single ended external GPIO connection to pad35."]
    SE7 = 7,
    #[doc = "8: single ended external GPIO connection to pad13."]
    SE8 = 8,
    #[doc = "9: single ended external GPIO connection to pad12."]
    SE9 = 9,
    #[doc = "10: differential external GPIO connections to pad12(N) and pad13(P)."]
    DF0 = 10,
    #[doc = "11: differential external GPIO connections to pad15(N) and pad14(P)."]
    DF1 = 11,
    #[doc = "12: internal temperature sensor."]
    TEMP = 12,
    #[doc = "13: internal voltage divide-by-3 connection."]
    BATT = 13,
    #[doc = "14: Input VSS"]
    VSS = 14,
}
impl From<CHSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHSEL5`"]
pub type CHSEL5_R = crate::R<u8, CHSEL5_A>;
impl CHSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHSEL5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHSEL5_A::SE0),
            1 => Val(CHSEL5_A::SE1),
            2 => Val(CHSEL5_A::SE2),
            3 => Val(CHSEL5_A::SE3),
            4 => Val(CHSEL5_A::SE4),
            5 => Val(CHSEL5_A::SE5),
            6 => Val(CHSEL5_A::SE6),
            7 => Val(CHSEL5_A::SE7),
            8 => Val(CHSEL5_A::SE8),
            9 => Val(CHSEL5_A::SE9),
            10 => Val(CHSEL5_A::DF0),
            11 => Val(CHSEL5_A::DF1),
            12 => Val(CHSEL5_A::TEMP),
            13 => Val(CHSEL5_A::BATT),
            14 => Val(CHSEL5_A::VSS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        *self == CHSEL5_A::SE0
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        *self == CHSEL5_A::SE1
    }
    #[doc = "Checks if the value of the field is `SE2`"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        *self == CHSEL5_A::SE2
    }
    #[doc = "Checks if the value of the field is `SE3`"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        *self == CHSEL5_A::SE3
    }
    #[doc = "Checks if the value of the field is `SE4`"]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        *self == CHSEL5_A::SE4
    }
    #[doc = "Checks if the value of the field is `SE5`"]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        *self == CHSEL5_A::SE5
    }
    #[doc = "Checks if the value of the field is `SE6`"]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        *self == CHSEL5_A::SE6
    }
    #[doc = "Checks if the value of the field is `SE7`"]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        *self == CHSEL5_A::SE7
    }
    #[doc = "Checks if the value of the field is `SE8`"]
    #[inline(always)]
    pub fn is_se8(&self) -> bool {
        *self == CHSEL5_A::SE8
    }
    #[doc = "Checks if the value of the field is `SE9`"]
    #[inline(always)]
    pub fn is_se9(&self) -> bool {
        *self == CHSEL5_A::SE9
    }
    #[doc = "Checks if the value of the field is `DF0`"]
    #[inline(always)]
    pub fn is_df0(&self) -> bool {
        *self == CHSEL5_A::DF0
    }
    #[doc = "Checks if the value of the field is `DF1`"]
    #[inline(always)]
    pub fn is_df1(&self) -> bool {
        *self == CHSEL5_A::DF1
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        *self == CHSEL5_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BATT`"]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        *self == CHSEL5_A::BATT
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        *self == CHSEL5_A::VSS
    }
}
#[doc = "Write proxy for field `CHSEL5`"]
pub struct CHSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single ended external GPIO connection to pad16."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE0)
    }
    #[doc = "single ended external GPIO connection to pad29."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE1)
    }
    #[doc = "single ended external GPIO connection to pad11."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE2)
    }
    #[doc = "single ended external GPIO connection to pad31."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE3)
    }
    #[doc = "single ended external GPIO connection to pad32."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE4)
    }
    #[doc = "single ended external GPIO connection to pad33."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE5)
    }
    #[doc = "single ended external GPIO connection to pad34."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE6)
    }
    #[doc = "single ended external GPIO connection to pad35."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE7)
    }
    #[doc = "single ended external GPIO connection to pad13."]
    #[inline(always)]
    pub fn se8(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE8)
    }
    #[doc = "single ended external GPIO connection to pad12."]
    #[inline(always)]
    pub fn se9(self) -> &'a mut W {
        self.variant(CHSEL5_A::SE9)
    }
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P)."]
    #[inline(always)]
    pub fn df0(self) -> &'a mut W {
        self.variant(CHSEL5_A::DF0)
    }
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P)."]
    #[inline(always)]
    pub fn df1(self) -> &'a mut W {
        self.variant(CHSEL5_A::DF1)
    }
    #[doc = "internal temperature sensor."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL5_A::TEMP)
    }
    #[doc = "internal voltage divide-by-3 connection."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut W {
        self.variant(CHSEL5_A::BATT)
    }
    #[doc = "Input VSS"]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL5_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN5_A {
    #[doc = "1: Enable the window compare for slot 5."]
    WCEN = 1,
}
impl From<WCEN5_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WCEN5`"]
pub type WCEN5_R = crate::R<bool, WCEN5_A>;
impl WCEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, WCEN5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(WCEN5_A::WCEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        *self == WCEN5_A::WCEN
    }
}
#[doc = "Write proxy for field `WCEN5`"]
pub struct WCEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the window compare for slot 5."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN5_A::WCEN)
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
#[doc = "This bit enables slot 5 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN5_A {
    #[doc = "1: Enable slot 5 for ADC conversions."]
    SLEN = 1,
}
impl From<SLEN5_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEN5`"]
pub type SLEN5_R = crate::R<bool, SLEN5_A>;
impl SLEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, SLEN5_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(SLEN5_A::SLEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        *self == SLEN5_A::SLEN
    }
}
#[doc = "Write proxy for field `SLEN5`"]
pub struct SLEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN5_A::SLEN)
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
    #[doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel5(&self) -> ADSEL5_R {
        ADSEL5_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode5(&self) -> PRMODE5_R {
        PRMODE5_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel5(&self) -> CHSEL5_R {
        CHSEL5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub fn wcen5(&self) -> WCEN5_R {
        WCEN5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen5(&self) -> SLEN5_R {
        SLEN5_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel5(&mut self) -> ADSEL5_W {
        ADSEL5_W { w: self }
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode5(&mut self) -> PRMODE5_W {
        PRMODE5_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel5(&mut self) -> CHSEL5_W {
        CHSEL5_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub fn wcen5(&mut self) -> WCEN5_W {
        WCEN5_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub fn slen5(&mut self) -> SLEN5_W {
        SLEN5_W { w: self }
    }
}
