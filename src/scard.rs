#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISO7816 interrupt status"]
    pub sr: SR,
    #[doc = "0x04 - ISO7816 interrupt enable"]
    pub ier: IER,
    #[doc = "0x08 - ISO7816 transmit control"]
    pub tcr: TCR,
    #[doc = "0x0c - ISO7816 user control"]
    pub ucr: UCR,
    #[doc = "0x10 - ISO7816 data"]
    pub dr: DR,
    #[doc = "0x14 - ISO7816 baud rate low"]
    pub bprl: BPRL,
    #[doc = "0x18 - ISO7816 baud rate high"]
    pub bprh: BPRH,
    #[doc = "0x1c - ISO7816 user control 1"]
    pub ucr1: UCR1,
    #[doc = "0x20 - ISO7816 interrupt status 1"]
    pub sr1: SR1,
    #[doc = "0x24 - ISO7816 interrupt enable 1"]
    pub ier1: IER1,
    #[doc = "0x28 - ETU counter low"]
    pub ecntl: ECNTL,
    #[doc = "0x2c - ETU counter high"]
    pub ecnth: ECNTH,
    #[doc = "0x30 - ISO7816 guard time configuration"]
    pub gtr: GTR,
    #[doc = "0x34 - ISO7816 resend count"]
    pub retxcnt: RETXCNT,
    #[doc = "0x38 - ISO7816 resent count inquiry"]
    pub retxcntrmi: RETXCNTRMI,
    _reserved15: [u8; 196usize],
    #[doc = "0x100 - Clock Control"]
    pub clkctrl: CLKCTRL,
}
#[doc = "ISO7816 interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "ISO7816 interrupt status"]
pub mod sr;
#[doc = "ISO7816 interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "ISO7816 interrupt enable"]
pub mod ier;
#[doc = "ISO7816 transmit control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "ISO7816 transmit control"]
pub mod tcr;
#[doc = "ISO7816 user control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucr](ucr) module"]
pub type UCR = crate::Reg<u32, _UCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCR;
#[doc = "`read()` method returns [ucr::R](ucr::R) reader structure"]
impl crate::Readable for UCR {}
#[doc = "`write(|w| ..)` method takes [ucr::W](ucr::W) writer structure"]
impl crate::Writable for UCR {}
#[doc = "ISO7816 user control"]
pub mod ucr;
#[doc = "ISO7816 data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "ISO7816 data"]
pub mod dr;
#[doc = "ISO7816 baud rate low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bprl](bprl) module"]
pub type BPRL = crate::Reg<u32, _BPRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPRL;
#[doc = "`read()` method returns [bprl::R](bprl::R) reader structure"]
impl crate::Readable for BPRL {}
#[doc = "`write(|w| ..)` method takes [bprl::W](bprl::W) writer structure"]
impl crate::Writable for BPRL {}
#[doc = "ISO7816 baud rate low"]
pub mod bprl;
#[doc = "ISO7816 baud rate high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bprh](bprh) module"]
pub type BPRH = crate::Reg<u32, _BPRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BPRH;
#[doc = "`read()` method returns [bprh::R](bprh::R) reader structure"]
impl crate::Readable for BPRH {}
#[doc = "`write(|w| ..)` method takes [bprh::W](bprh::W) writer structure"]
impl crate::Writable for BPRH {}
#[doc = "ISO7816 baud rate high"]
pub mod bprh;
#[doc = "ISO7816 user control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucr1](ucr1) module"]
pub type UCR1 = crate::Reg<u32, _UCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UCR1;
#[doc = "`read()` method returns [ucr1::R](ucr1::R) reader structure"]
impl crate::Readable for UCR1 {}
#[doc = "`write(|w| ..)` method takes [ucr1::W](ucr1::W) writer structure"]
impl crate::Writable for UCR1 {}
#[doc = "ISO7816 user control 1"]
pub mod ucr1;
#[doc = "ISO7816 interrupt status 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](sr1) module"]
pub type SR1 = crate::Reg<u32, _SR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR1;
#[doc = "`read()` method returns [sr1::R](sr1::R) reader structure"]
impl crate::Readable for SR1 {}
#[doc = "`write(|w| ..)` method takes [sr1::W](sr1::W) writer structure"]
impl crate::Writable for SR1 {}
#[doc = "ISO7816 interrupt status 1"]
pub mod sr1;
#[doc = "ISO7816 interrupt enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier1](ier1) module"]
pub type IER1 = crate::Reg<u32, _IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER1;
#[doc = "`read()` method returns [ier1::R](ier1::R) reader structure"]
impl crate::Readable for IER1 {}
#[doc = "`write(|w| ..)` method takes [ier1::W](ier1::W) writer structure"]
impl crate::Writable for IER1 {}
#[doc = "ISO7816 interrupt enable 1"]
pub mod ier1;
#[doc = "ETU counter low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecntl](ecntl) module"]
pub type ECNTL = crate::Reg<u32, _ECNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECNTL;
#[doc = "`read()` method returns [ecntl::R](ecntl::R) reader structure"]
impl crate::Readable for ECNTL {}
#[doc = "`write(|w| ..)` method takes [ecntl::W](ecntl::W) writer structure"]
impl crate::Writable for ECNTL {}
#[doc = "ETU counter low"]
pub mod ecntl;
#[doc = "ETU counter high\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecnth](ecnth) module"]
pub type ECNTH = crate::Reg<u32, _ECNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECNTH;
#[doc = "`read()` method returns [ecnth::R](ecnth::R) reader structure"]
impl crate::Readable for ECNTH {}
#[doc = "`write(|w| ..)` method takes [ecnth::W](ecnth::W) writer structure"]
impl crate::Writable for ECNTH {}
#[doc = "ETU counter high"]
pub mod ecnth;
#[doc = "ISO7816 guard time configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtr](gtr) module"]
pub type GTR = crate::Reg<u32, _GTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GTR;
#[doc = "`read()` method returns [gtr::R](gtr::R) reader structure"]
impl crate::Readable for GTR {}
#[doc = "`write(|w| ..)` method takes [gtr::W](gtr::W) writer structure"]
impl crate::Writable for GTR {}
#[doc = "ISO7816 guard time configuration"]
pub mod gtr;
#[doc = "ISO7816 resend count\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retxcnt](retxcnt) module"]
pub type RETXCNT = crate::Reg<u32, _RETXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETXCNT;
#[doc = "`read()` method returns [retxcnt::R](retxcnt::R) reader structure"]
impl crate::Readable for RETXCNT {}
#[doc = "`write(|w| ..)` method takes [retxcnt::W](retxcnt::W) writer structure"]
impl crate::Writable for RETXCNT {}
#[doc = "ISO7816 resend count"]
pub mod retxcnt;
#[doc = "ISO7816 resent count inquiry\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retxcntrmi](retxcntrmi) module"]
pub type RETXCNTRMI = crate::Reg<u32, _RETXCNTRMI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RETXCNTRMI;
#[doc = "`read()` method returns [retxcntrmi::R](retxcntrmi::R) reader structure"]
impl crate::Readable for RETXCNTRMI {}
#[doc = "`write(|w| ..)` method takes [retxcntrmi::W](retxcntrmi::W) writer structure"]
impl crate::Writable for RETXCNTRMI {}
#[doc = "ISO7816 resent count inquiry"]
pub mod retxcntrmi;
#[doc = "Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](clkctrl) module"]
pub type CLKCTRL = crate::Reg<u32, _CLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKCTRL;
#[doc = "`read()` method returns [clkctrl::R](clkctrl::R) reader structure"]
impl crate::Readable for CLKCTRL {}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](clkctrl::W) writer structure"]
impl crate::Writable for CLKCTRL {}
#[doc = "Clock Control"]
pub mod clkctrl;
