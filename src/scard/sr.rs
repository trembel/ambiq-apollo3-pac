#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO Half Full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FHF_A {
    #[doc = "1: FIFO is half full."]
    HALFFULL = 1,
}
impl From<FHF_A> for bool {
    #[inline(always)]
    fn from(variant: FHF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FHF`"]
pub type FHF_R = crate::R<bool, FHF_A>;
impl FHF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FHF_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FHF_A::HALFFULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HALFFULL`"]
    #[inline(always)]
    pub fn is_halffull(&self) -> bool {
        *self == FHF_A::HALFFULL
    }
}
#[doc = "Write proxy for field `FHF`"]
pub struct FHF_W<'a> {
    w: &'a mut W,
}
impl<'a> FHF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FHF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO is half full."]
    #[inline(always)]
    pub fn halffull(self) -> &'a mut W {
        self.variant(FHF_A::HALFFULL)
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
#[doc = "TX to RX finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FT2REND_A {
    #[doc = "1: TX to RX completed."]
    CMPL = 1,
    #[doc = "0: TX to RX not completed."]
    NOTCMPL = 0,
}
impl From<FT2REND_A> for bool {
    #[inline(always)]
    fn from(variant: FT2REND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FT2REND`"]
pub type FT2REND_R = crate::R<bool, FT2REND_A>;
impl FT2REND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FT2REND_A {
        match self.bits {
            true => FT2REND_A::CMPL,
            false => FT2REND_A::NOTCMPL,
        }
    }
    #[doc = "Checks if the value of the field is `CMPL`"]
    #[inline(always)]
    pub fn is_cmpl(&self) -> bool {
        *self == FT2REND_A::CMPL
    }
    #[doc = "Checks if the value of the field is `NOTCMPL`"]
    #[inline(always)]
    pub fn is_notcmpl(&self) -> bool {
        *self == FT2REND_A::NOTCMPL
    }
}
#[doc = "Write proxy for field `FT2REND`"]
pub struct FT2REND_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2REND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FT2REND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TX to RX completed."]
    #[inline(always)]
    pub fn cmpl(self) -> &'a mut W {
        self.variant(FT2REND_A::CMPL)
    }
    #[doc = "TX to RX not completed."]
    #[inline(always)]
    pub fn notcmpl(self) -> &'a mut W {
        self.variant(FT2REND_A::NOTCMPL)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Parity Error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "1: Parity error."]
    PEERR = 1,
    #[doc = "0: No parity error."]
    PENONE = 0,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            true => PE_A::PEERR,
            false => PE_A::PENONE,
        }
    }
    #[doc = "Checks if the value of the field is `PEERR`"]
    #[inline(always)]
    pub fn is_peerr(&self) -> bool {
        *self == PE_A::PEERR
    }
    #[doc = "Checks if the value of the field is `PENONE`"]
    #[inline(always)]
    pub fn is_penone(&self) -> bool {
        *self == PE_A::PENONE
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn peerr(self) -> &'a mut W {
        self.variant(PE_A::PEERR)
    }
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn penone(self) -> &'a mut W {
        self.variant(PE_A::PENONE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "RX FIFO overflow.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_A {
    #[doc = "1: RX FIFO overflow."]
    RXOVR = 1,
    #[doc = "0: RX FIFO no overflow."]
    RXOVRNONE = 0,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, OVR_A>;
impl OVR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            true => OVR_A::RXOVR,
            false => OVR_A::RXOVRNONE,
        }
    }
    #[doc = "Checks if the value of the field is `RXOVR`"]
    #[inline(always)]
    pub fn is_rxovr(&self) -> bool {
        *self == OVR_A::RXOVR
    }
    #[doc = "Checks if the value of the field is `RXOVRNONE`"]
    #[inline(always)]
    pub fn is_rxovrnone(&self) -> bool {
        *self == OVR_A::RXOVRNONE
    }
}
#[doc = "Write proxy for field `OVR`"]
pub struct OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX FIFO overflow."]
    #[inline(always)]
    pub fn rxovr(self) -> &'a mut W {
        self.variant(OVR_A::RXOVR)
    }
    #[doc = "RX FIFO no overflow."]
    #[inline(always)]
    pub fn rxovrnone(self) -> &'a mut W {
        self.variant(OVR_A::RXOVRNONE)
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
#[doc = "Framing error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER_A {
    #[doc = "1: Framing error."]
    FRAMINGERR = 1,
    #[doc = "0: No framing error detected."]
    NOFRAMINGERR = 0,
}
impl From<FER_A> for bool {
    #[inline(always)]
    fn from(variant: FER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FER`"]
pub type FER_R = crate::R<bool, FER_A>;
impl FER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FER_A {
        match self.bits {
            true => FER_A::FRAMINGERR,
            false => FER_A::NOFRAMINGERR,
        }
    }
    #[doc = "Checks if the value of the field is `FRAMINGERR`"]
    #[inline(always)]
    pub fn is_framingerr(&self) -> bool {
        *self == FER_A::FRAMINGERR
    }
    #[doc = "Checks if the value of the field is `NOFRAMINGERR`"]
    #[inline(always)]
    pub fn is_noframingerr(&self) -> bool {
        *self == FER_A::NOFRAMINGERR
    }
}
#[doc = "Write proxy for field `FER`"]
pub struct FER_W<'a> {
    w: &'a mut W,
}
impl<'a> FER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn framingerr(self) -> &'a mut W {
        self.variant(FER_A::FRAMINGERR)
    }
    #[doc = "No framing error detected."]
    #[inline(always)]
    pub fn noframingerr(self) -> &'a mut W {
        self.variant(FER_A::NOFRAMINGERR)
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
#[doc = "FIFO empty (transmit) or full (receive).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBERBF_A {
    #[doc = "1: Transmit: FIFO empty."]
    TXFIFOEMPTY = 1,
    #[doc = "0: Transmit: FIFO not empty."]
    TXFIFONOTEMPTY = 0,
}
impl From<TBERBF_A> for bool {
    #[inline(always)]
    fn from(variant: TBERBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TBERBF`"]
pub type TBERBF_R = crate::R<bool, TBERBF_A>;
impl TBERBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBERBF_A {
        match self.bits {
            true => TBERBF_A::TXFIFOEMPTY,
            false => TBERBF_A::TXFIFONOTEMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `TXFIFOEMPTY`"]
    #[inline(always)]
    pub fn is_txfifoempty(&self) -> bool {
        *self == TBERBF_A::TXFIFOEMPTY
    }
    #[doc = "Checks if the value of the field is `TXFIFONOTEMPTY`"]
    #[inline(always)]
    pub fn is_txfifonotempty(&self) -> bool {
        *self == TBERBF_A::TXFIFONOTEMPTY
    }
}
#[doc = "Write proxy for field `TBERBF`"]
pub struct TBERBF_W<'a> {
    w: &'a mut W,
}
impl<'a> TBERBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBERBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit: FIFO empty."]
    #[inline(always)]
    pub fn txfifoempty(self) -> &'a mut W {
        self.variant(TBERBF_A::TXFIFOEMPTY)
    }
    #[doc = "Transmit: FIFO not empty."]
    #[inline(always)]
    pub fn txfifonotempty(self) -> &'a mut W {
        self.variant(TBERBF_A::TXFIFONOTEMPTY)
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
#[doc = "RX FIFO not empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FNE_A {
    #[doc = "1: RX FIFO not empty."]
    NOTEMPTY = 1,
    #[doc = "0: RX FIFO empty."]
    EMPTY = 0,
}
impl From<FNE_A> for bool {
    #[inline(always)]
    fn from(variant: FNE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FNE`"]
pub type FNE_R = crate::R<bool, FNE_A>;
impl FNE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FNE_A {
        match self.bits {
            true => FNE_A::NOTEMPTY,
            false => FNE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTEMPTY`"]
    #[inline(always)]
    pub fn is_notempty(&self) -> bool {
        *self == FNE_A::NOTEMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FNE_A::EMPTY
    }
}
#[doc = "Write proxy for field `FNE`"]
pub struct FNE_W<'a> {
    w: &'a mut W,
}
impl<'a> FNE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FNE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RX FIFO not empty."]
    #[inline(always)]
    pub fn notempty(self) -> &'a mut W {
        self.variant(FNE_A::NOTEMPTY)
    }
    #[doc = "RX FIFO empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FNE_A::EMPTY)
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
    #[doc = "Bit 6 - FIFO Half Full."]
    #[inline(always)]
    pub fn fhf(&self) -> FHF_R {
        FHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX to RX finished."]
    #[inline(always)]
    pub fn ft2rend(&self) -> FT2REND_R {
        FT2REND_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Parity Error."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO overflow."]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Framing error."]
    #[inline(always)]
    pub fn fer(&self) -> FER_R {
        FER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive)."]
    #[inline(always)]
    pub fn tberbf(&self) -> TBERBF_R {
        TBERBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RX FIFO not empty."]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - FIFO Half Full."]
    #[inline(always)]
    pub fn fhf(&mut self) -> FHF_W {
        FHF_W { w: self }
    }
    #[doc = "Bit 5 - TX to RX finished."]
    #[inline(always)]
    pub fn ft2rend(&mut self) -> FT2REND_W {
        FT2REND_W { w: self }
    }
    #[doc = "Bit 4 - Parity Error."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO overflow."]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W {
        OVR_W { w: self }
    }
    #[doc = "Bit 2 - Framing error."]
    #[inline(always)]
    pub fn fer(&mut self) -> FER_W {
        FER_W { w: self }
    }
    #[doc = "Bit 1 - FIFO empty (transmit) or full (receive)."]
    #[inline(always)]
    pub fn tberbf(&mut self) -> TBERBF_W {
        TBERBF_W { w: self }
    }
    #[doc = "Bit 0 - RX FIFO not empty."]
    #[inline(always)]
    pub fn fne(&mut self) -> FNE_W {
        FNE_W { w: self }
    }
}
