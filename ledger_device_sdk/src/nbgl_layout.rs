use alloc::format;
use const_zero::const_zero;
use core::ffi::c_char;
use ledger_secure_sdk_sys::*;

extern crate alloc;
use alloc::ffi::CString;

use crate::testing;

#[no_mangle]
pub static mut G_ux_params: bolos_ux_params_t = unsafe { const_zero!(bolos_ux_params_t) };

pub struct NbglGlyph<'a> {
    pub width: u16,
    pub height: u16,
    pub bpp: u8,
    pub is_file: bool,
    pub bitmap: &'a [u8],
}

impl<'a> NbglGlyph<'a> {
    pub const fn new(
        bitmap: &'a [u8],
        width: u16,
        height: u16,
        bpp: u8,
        is_file: bool,
    ) -> NbglGlyph<'a> {
        NbglGlyph {
            width,
            height,
            bpp,
            is_file,
            bitmap,
        }
    }
    pub const fn from_include(packed: (&'a [u8], u16, u16, u8, bool)) -> NbglGlyph<'a> {
        NbglGlyph {
            width: packed.1,
            height: packed.2,
            bpp: packed.3,
            is_file: packed.4,
            bitmap: packed.0,
        }
    }
}

impl<'a> Into<nbgl_icon_details_t> for &NbglGlyph<'a> {
    fn into(self) -> nbgl_icon_details_t {
        let bpp = match self.bpp {
            1 => NBGL_BPP_1,
            2 => NBGL_BPP_2,
            4 => NBGL_BPP_4,
            _ => panic!("Invalid bpp"),
        };
        nbgl_icon_details_t {
            width: self.width,
            height: self.height,
            bpp,
            isFile: self.is_file,
            bitmap: self.bitmap.as_ptr() as *const u8,
        }
    }
}

unsafe extern "C" fn on_action_callback(token: ::core::ffi::c_int, index: u8) {
    testing::debug_print("Tap !!! \n");
    testing::debug_print(format!("token: {}\n", token).as_str());
}

pub fn display<'a>(glyph: &'a NbglGlyph) {
    let tap_action_text = CString::new("Tap to continue").unwrap();

    let appname = CString::new("Starknet").unwrap();
    let desc =
        CString::new("This app enables signing transactions on the Starknet network").unwrap();

    unsafe {
        let mut description: nbgl_layoutDescription_t = nbgl_layoutDescription_t::default();
        description.modal = false;
        description.tapActionText = tap_action_text.as_ptr() as *const c_char;
        description.tapActionToken = 1;
        description.tapTuneId = TUNE_NEUTRAL;
        description.onActionCallback = Some(on_action_callback);

        let layout = nbgl_layoutGet(&description);

        let icon = glyph.into();

        let centered_info: nbgl_contentCenteredInfo_t = nbgl_contentCenteredInfo_t {
            icon: &icon,
            text1: appname.as_ptr() as *const c_char,
            text2: desc.as_ptr() as *const c_char,
            text3: core::ptr::null(),
            onTop: false,
            style: LARGE_CASE_INFO,
            offsetY: 0,
        };

        nbgl_layoutAddCenteredInfo(layout, &centered_info);

        //let reject = CString::new("Reject Transaction").unwrap();
        //nbgl_layoutAddFooter(layout, reject.as_ptr() as *const c_char, 2, TUNE_TAP_CASUAL);

        let ret = nbgl_layoutDraw(layout);
        nbgl_refresh();
        if ret != 0 {
            testing::debug_print("display error\n");
        }
    }
}

enum TuneIndex {
    Reserved,
    Boot,
    Charging,
    LedgerMoment,
    Error,
    Neutral,
    Lock,
    Success,
    LookAtMe,
    TapCasual,
    TapNext,
}

impl TryFrom<u8> for TuneIndex {
    type Error = ();
    fn try_from(index: u8) -> Result<TuneIndex, ()> {
        Ok(match index {
            TUNE_RESERVED => TuneIndex::Reserved,
            TUNE_BOOT => TuneIndex::Boot,
            TUNE_CHARGING => TuneIndex::Charging,
            TUNE_LEDGER_MOMENT => TuneIndex::LedgerMoment,
            TUNE_ERROR => TuneIndex::Error,
            TUNE_NEUTRAL => TuneIndex::Neutral,
            TUNE_LOCK => TuneIndex::Lock,
            TUNE_SUCCESS => TuneIndex::Success,
            TUNE_LOOK_AT_ME => TuneIndex::LookAtMe,
            TUNE_TAP_CASUAL => TuneIndex::TapCasual,
            TUNE_TAP_NEXT => TuneIndex::TapNext,
            _ => return Err(()),
        })
    }
}

// this is a mock that does nothing yet, but should become a direct translation
// of the C original. This was done to avoid compiling `os_io_seproxyhal.c` which
// includes many other things
#[no_mangle]
extern "C" fn io_seproxyhal_play_tune(tune_index: u8) {
    let index = TuneIndex::try_from(tune_index);
    if index.is_err() {
        return;
    }
}
