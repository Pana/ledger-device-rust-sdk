#![no_std]
#![no_main]

// Force boot section to be embedded in
use ledger_device_sdk as _;

use const_zero::const_zero;
use ledger_device_sdk::io::*;
use ledger_device_sdk::nbgl::{Field, NbglHome, NbglReview};
use ledger_device_sdk::testing::debug_print;
use ledger_secure_sdk_sys::*;

pub enum Instruction {
    GetVersion,
    GetAppName,
}

impl TryFrom<ApduHeader> for Instruction {
    type Error = StatusWords;

    fn try_from(value: ApduHeader) -> Result<Self, Self::Error> {
        match value.ins {
            3 => Ok(Instruction::GetVersion),
            4 => Ok(Instruction::GetAppName),
            _ => Err(StatusWords::NothingReceived),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    unsafe {
        nbgl_refreshReset();
    }

    let mut comm = Comm::new();

    loop {
        match NbglHome::new(&mut comm)
            .app_name("Stax Sample\0")
            .info_contents(env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"))
            .icon(&BTC_BMP)
            .show_home::<Instruction>()
        {
            Event::Command(_) => {
                let fields = [
                    Field {
                        name: "Field 1\0",
                        value: "Value 1\0",
                    },
                    Field {
                        name: "Field 2\0",
                        value: "Value 2\0",
                    },
                    Field {
                        name: "Field 3\0",
                        value: "Value 3\0",
                    },
                ];
                if NbglReview::new(&mut comm).review_transaction(&fields) {
                    debug_print("Validation result: true\n");
                } else {
                    debug_print("Validation result: false\n");
                }
            }
            _ => (),
        };
    }

    // let fields = [
    //     Field {
    //         name: "Field 1\0",
    //         value: "Value 1\0",
    //     },
    //     Field {
    //         name: "Field 2\0",
    //         value: "Value 2\0",
    //     },
    //     Field {
    //         name: "Field 3\0",
    //         value: "Value 3\0",
    //     },
    // ];

    // if nbgl_ui.verify_address("1BvBMSEYstWetqTFn5Au4m4GFg7xJaNVN2\0") {
    //     debug_print("Address verified\n");
    // } else {
    //     debug_print("Address not verified\n");
    // }

    // if nbgl_ui.review_transaction(&fields) {
    //     debug_print("Validation result: true\n");
    // } else {
    //     debug_print("Validation result: false\n");
    // }

    // exit_app(0);
}

const BTC_BMP: [u8; 573] = [
    0x40, 0x00, 0x40, 0x00, 0x21, 0x35, 0x02, 0x00, 0x33, 0x02, 0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x02, 0xff, 0xbd, 0x55, 0xb9, 0x4e, 0x23, 0x41, 0x10, 0x6d, 0x7b, 0x3c, 0x5c, 0x46,
    0x5e, 0x2f, 0x96, 0x20, 0xc3, 0x10, 0x5b, 0x82, 0xdd, 0x6c, 0x1d, 0x2d, 0x64, 0x10, 0x81, 0x88,
    0x10, 0x01, 0x87, 0x03, 0x02, 0x84, 0x38, 0x02, 0x62, 0x90, 0x58, 0x44, 0xb0, 0x01, 0xbb, 0xf0,
    0x01, 0x26, 0xd9, 0x68, 0x03, 0x10, 0x1f, 0x00, 0xa4, 0x44, 0xfc, 0x02, 0x26, 0x34, 0x88, 0xe1,
    0x32, 0x87, 0x8d, 0xa6, 0xe8, 0xee, 0xa9, 0x3e, 0xa6, 0xa7, 0x6d, 0x13, 0x51, 0xc1, 0xa8, 0x67,
    0x5e, 0x4f, 0xd7, 0xab, 0xaa, 0x57, 0xd5, 0x00, 0x9f, 0x67, 0x57, 0x5b, 0xfd, 0x84, 0x64, 0x46,
    0x4e, 0xec, 0xa8, 0xbf, 0x99, 0x26, 0x81, 0xe5, 0x3d, 0x0b, 0x5c, 0x9b, 0x23, 0xd2, 0xdc, 0xe8,
    0x06, 0x7f, 0x88, 0x90, 0x46, 0x1b, 0x56, 0x49, 0xc8, 0x5a, 0x0d, 0xf8, 0x92, 0x18, 0xd6, 0x1b,
    0x82, 0x5f, 0xd3, 0x26, 0x1e, 0x2b, 0xea, 0xf8, 0x0a, 0x89, 0x98, 0xee, 0xe1, 0x99, 0x58, 0x6c,
    0xbd, 0xe1, 0xef, 0xfa, 0x01, 0x2f, 0xc4, 0x6a, 0x92, 0x41, 0xc9, 0x8e, 0xcb, 0x10, 0xd2, 0x76,
    0x3c, 0xde, 0x88, 0x9d, 0xc6, 0xf0, 0xa2, 0x1e, 0x8e, 0x0e, 0xc6, 0xeb, 0xe1, 0x89, 0xa0, 0x32,
    0xf2, 0x3d, 0x7f, 0xfd, 0x8f, 0x3e, 0xbf, 0x2a, 0x3a, 0x5c, 0x09, 0x15, 0x3d, 0xe0, 0x12, 0xc9,
    0x81, 0xff, 0x57, 0x7c, 0x59, 0x0e, 0xb9, 0x67, 0x74, 0x6a, 0x09, 0x3d, 0x5f, 0x9c, 0xc0, 0x1f,
    0x51, 0x11, 0xee, 0x6d, 0x5a, 0x8f, 0xa8, 0x8d, 0xbd, 0x7c, 0xc3, 0x97, 0x76, 0xad, 0x20, 0xf8,
    0xcd, 0x61, 0xeb, 0xc0, 0xf7, 0x04, 0xc9, 0x6a, 0xf8, 0x06, 0x1e, 0x49, 0x65, 0x54, 0xe3, 0xab,
    0x14, 0x94, 0x8b, 0xe0, 0xcf, 0x9f, 0x43, 0x38, 0x25, 0x27, 0xa2, 0x38, 0x3f, 0x83, 0x3a, 0x74,
    0xd0, 0x70, 0xd7, 0x68, 0x5d, 0x6e, 0x11, 0xdf, 0x01, 0x78, 0xe4, 0x8b, 0x9c, 0xc7, 0xeb, 0xe0,
    0xb0, 0x4d, 0xae, 0xc2, 0x67, 0x00, 0x1e, 0x90, 0xca, 0x22, 0xf7, 0x04, 0x40, 0x75, 0xec, 0x49,
    0x7c, 0x10, 0xe0, 0x4e, 0xc6, 0xc2, 0x12, 0xc5, 0x37, 0x15, 0xa5, 0xff, 0x2c, 0x1e, 0xe5, 0x9c,
    0x0d, 0x77, 0xc2, 0x3d, 0x2b, 0x69, 0x85, 0xe3, 0x87, 0x88, 0x7f, 0x41, 0xbc, 0x05, 0xe0, 0x9c,
    0xfd, 0x94, 0xe0, 0xc7, 0x1d, 0xc0, 0x91, 0x81, 0x27, 0x19, 0xfd, 0x6a, 0x81, 0x3a, 0x39, 0x0d,
    0xa2, 0x2a, 0x17, 0x42, 0xb8, 0xf3, 0x9b, 0x87, 0x5d, 0x5e, 0xe2, 0x99, 0x61, 0xb1, 0x54, 0x05,
    0x8e, 0xfc, 0x32, 0xd8, 0x71, 0xbb, 0x8c, 0x84, 0xef, 0xc1, 0x93, 0xe0, 0x77, 0x8f, 0xae, 0x44,
    0x47, 0x96, 0x0b, 0xad, 0x50, 0x8d, 0x2f, 0x5c, 0x8a, 0xb4, 0x55, 0xc2, 0x62, 0xa4, 0x76, 0xa3,
    0x24, 0x31, 0x26, 0xc5, 0xef, 0xea, 0xed, 0x56, 0x1d, 0x56, 0xf9, 0x0d, 0xea, 0x43, 0x33, 0xff,
    0x10, 0x5f, 0x50, 0x5d, 0x7f, 0xd6, 0x27, 0xea, 0xe3, 0xa7, 0xb1, 0x80, 0xfb, 0xd4, 0xc9, 0x94,
    0xdc, 0x11, 0xe0, 0x4a, 0x0b, 0x59, 0x98, 0xe5, 0x34, 0xf8, 0x19, 0xff, 0x51, 0xb4, 0x8e, 0xd2,
    0x42, 0x6c, 0x14, 0x65, 0x1b, 0x5f, 0xa2, 0x85, 0xcc, 0x6c, 0x2b, 0x49, 0x95, 0x22, 0xaa, 0x7f,
    0x13, 0xcb, 0x14, 0xc3, 0x9f, 0x0c, 0xbc, 0x45, 0xf5, 0xf3, 0x0c, 0x1f, 0x6c, 0x06, 0x9e, 0x44,
    0xc9, 0xc8, 0x9c, 0x0d, 0x19, 0x73, 0xc1, 0x13, 0x92, 0x70, 0xed, 0xed, 0xef, 0xee, 0xeb, 0xee,
    0x9b, 0xf7, 0x37, 0xe6, 0x22, 0x62, 0xb1, 0x8f, 0xce, 0x97, 0x57, 0x3b, 0xbe, 0xf3, 0xd1, 0xf9,
    0xd6, 0x74, 0x3e, 0x36, 0x9b, 0xaf, 0xf0, 0xd2, 0x64, 0x3e, 0x47, 0x43, 0x18, 0x30, 0x2e, 0x00,
    0xc3, 0x43, 0x4f, 0xe4, 0x7e, 0x09, 0x4d, 0xb9, 0x84, 0xe5, 0x02, 0xd2, 0x36, 0x74, 0xdb, 0xae,
    0x38, 0x7f, 0x4f, 0x50, 0xfb, 0xe1, 0xd9, 0x6f, 0xc0, 0xda, 0xaf, 0xef, 0x84, 0x74, 0x4d, 0x1e,
    0x7f, 0xe2, 0x95, 0x0c, 0xef, 0xe7, 0x7f, 0x81, 0xae, 0x00, 0x08, 0x00, 0x00,
];
