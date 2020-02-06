#[doc = "Reader of register CLOCKEN3STAT"]
pub type R = crate::R<u32, super::CLOCKEN3STAT>;
#[doc = "Writer for register CLOCKEN3STAT"]
pub type W = crate::W<u32, super::CLOCKEN3STAT>;
#[doc = "Register CLOCKEN3STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCKEN3STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock enable status 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN3STAT_A {
    #[doc = "131072: DAP clock is enabled \\[17\\]"]
    DAP_ENABLED = 131072,
    #[doc = "262144: VCOMP powerdown indicator \\[18\\]"]
    VCOMP_ENABLED = 262144,
    #[doc = "16777216: XTAL is enabled \\[24\\]"]
    XTAL_ENABLED = 16777216,
    #[doc = "33554432: HFRC is enabled \\[25\\]"]
    HFRC_ENABLED = 33554432,
    #[doc = "67108864: HFRC Adjust enabled \\[26\\]"]
    HFADJEN = 67108864,
    #[doc = "134217728: HFRC Enabled out \\[27\\]"]
    HFRC_EN_OUT = 134217728,
    #[doc = "268435456: RTC use XT \\[28\\]"]
    RTC_XT = 268435456,
    #[doc = "536870912: XTAL clkout enabled \\[29\\]"]
    CLKOUT_XTAL_EN = 536870912,
    #[doc = "1073741824: HFRC clkout enabled \\[30\\]"]
    CLKOUT_HFRC_EN = 1073741824,
    #[doc = "2147483648: Flash clk is enabled \\[31\\]"]
    FLASHCLK_EN = 2147483648,
}
impl From<CLOCKEN3STAT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN3STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLOCKEN3STAT`"]
pub type CLOCKEN3STAT_R = crate::R<u32, CLOCKEN3STAT_A>;
impl CLOCKEN3STAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CLOCKEN3STAT_A> {
        use crate::Variant::*;
        match self.bits {
            131072 => Val(CLOCKEN3STAT_A::DAP_ENABLED),
            262144 => Val(CLOCKEN3STAT_A::VCOMP_ENABLED),
            16777216 => Val(CLOCKEN3STAT_A::XTAL_ENABLED),
            33554432 => Val(CLOCKEN3STAT_A::HFRC_ENABLED),
            67108864 => Val(CLOCKEN3STAT_A::HFADJEN),
            134217728 => Val(CLOCKEN3STAT_A::HFRC_EN_OUT),
            268435456 => Val(CLOCKEN3STAT_A::RTC_XT),
            536870912 => Val(CLOCKEN3STAT_A::CLKOUT_XTAL_EN),
            1073741824 => Val(CLOCKEN3STAT_A::CLKOUT_HFRC_EN),
            2147483648 => Val(CLOCKEN3STAT_A::FLASHCLK_EN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DAP_ENABLED`"]
    #[inline(always)]
    pub fn is_dap_enabled(&self) -> bool {
        *self == CLOCKEN3STAT_A::DAP_ENABLED
    }
    #[doc = "Checks if the value of the field is `VCOMP_ENABLED`"]
    #[inline(always)]
    pub fn is_vcomp_enabled(&self) -> bool {
        *self == CLOCKEN3STAT_A::VCOMP_ENABLED
    }
    #[doc = "Checks if the value of the field is `XTAL_ENABLED`"]
    #[inline(always)]
    pub fn is_xtal_enabled(&self) -> bool {
        *self == CLOCKEN3STAT_A::XTAL_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFRC_ENABLED`"]
    #[inline(always)]
    pub fn is_hfrc_enabled(&self) -> bool {
        *self == CLOCKEN3STAT_A::HFRC_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFADJEN`"]
    #[inline(always)]
    pub fn is_hfadjen(&self) -> bool {
        *self == CLOCKEN3STAT_A::HFADJEN
    }
    #[doc = "Checks if the value of the field is `HFRC_EN_OUT`"]
    #[inline(always)]
    pub fn is_hfrc_en_out(&self) -> bool {
        *self == CLOCKEN3STAT_A::HFRC_EN_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline(always)]
    pub fn is_rtc_xt(&self) -> bool {
        *self == CLOCKEN3STAT_A::RTC_XT
    }
    #[doc = "Checks if the value of the field is `CLKOUT_XTAL_EN`"]
    #[inline(always)]
    pub fn is_clkout_xtal_en(&self) -> bool {
        *self == CLOCKEN3STAT_A::CLKOUT_XTAL_EN
    }
    #[doc = "Checks if the value of the field is `CLKOUT_HFRC_EN`"]
    #[inline(always)]
    pub fn is_clkout_hfrc_en(&self) -> bool {
        *self == CLOCKEN3STAT_A::CLKOUT_HFRC_EN
    }
    #[doc = "Checks if the value of the field is `FLASHCLK_EN`"]
    #[inline(always)]
    pub fn is_flashclk_en(&self) -> bool {
        *self == CLOCKEN3STAT_A::FLASHCLK_EN
    }
}
#[doc = "Write proxy for field `CLOCKEN3STAT`"]
pub struct CLOCKEN3STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN3STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN3STAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DAP clock is enabled \\[17\\]"]
    #[inline(always)]
    pub fn dap_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::DAP_ENABLED)
    }
    #[doc = "VCOMP powerdown indicator \\[18\\]"]
    #[inline(always)]
    pub fn vcomp_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::VCOMP_ENABLED)
    }
    #[doc = "XTAL is enabled \\[24\\]"]
    #[inline(always)]
    pub fn xtal_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::XTAL_ENABLED)
    }
    #[doc = "HFRC is enabled \\[25\\]"]
    #[inline(always)]
    pub fn hfrc_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFRC_ENABLED)
    }
    #[doc = "HFRC Adjust enabled \\[26\\]"]
    #[inline(always)]
    pub fn hfadjen(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFADJEN)
    }
    #[doc = "HFRC Enabled out \\[27\\]"]
    #[inline(always)]
    pub fn hfrc_en_out(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFRC_EN_OUT)
    }
    #[doc = "RTC use XT \\[28\\]"]
    #[inline(always)]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::RTC_XT)
    }
    #[doc = "XTAL clkout enabled \\[29\\]"]
    #[inline(always)]
    pub fn clkout_xtal_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::CLKOUT_XTAL_EN)
    }
    #[doc = "HFRC clkout enabled \\[30\\]"]
    #[inline(always)]
    pub fn clkout_hfrc_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::CLKOUT_HFRC_EN)
    }
    #[doc = "Flash clk is enabled \\[31\\]"]
    #[inline(always)]
    pub fn flashclk_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::FLASHCLK_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3stat(&self) -> CLOCKEN3STAT_R {
        CLOCKEN3STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3stat(&mut self) -> CLOCKEN3STAT_W {
        CLOCKEN3STAT_W { w: self }
    }
}
