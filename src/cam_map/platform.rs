#![allow(non_snake_case, unused)]

cfg_if::cfg_if!(
    if #[cfg(target_os = "macos")] {
        // https://stackoverflow.com/a/16125341 reference
        pub mod Scancode { // ScanCode is pubic in bevy::Key...

            // Number keys       SHIFT layout =   EN  GER
            pub const NUM1: u32 = 0x12; // 18      !   !
            pub const NUM2: u32 = 0x13; // 19      @   "
            pub const NUM3: u32 = 0x14; // 20      #   §
            pub const NUM4: u32 = 0x15; // 21      $   $
            pub const NUM5: u32 = 0x17; // 23      %   %
            pub const NUM6: u32 = 0x16; // 22      ˆ   &
            pub const NUM7: u32 = 0x1A; // 26      &   /
            pub const NUM8: u32 = 0x1C; // 28      *   (
            pub const NUM9: u32 = 0x19; // 25      (   )
            pub const NUM0: u32 = 0x1D; // 29      )   =

            pub const A: u32 = 0x00;
            pub const B: u32 = 0x0b;
            pub const C: u32 = 0x08;
            pub const D: u32 = 0x02;
            pub const E: u32 = 0x0E;
            pub const F: u32 = 0x03;
            pub const G: u32 = 0x05;
            pub const H: u32 = 0x04; //22 26 28 25 2e 2d 1f
            pub const P: u32 = 0x23;
            pub const Q: u32 = 0x0C;
            pub const R: u32 = 0x0F;
            pub const S: u32 = 0x01;
            pub const T: u32 = 0x11; // 20 09 
            pub const W: u32 = 0x0D; // 07 06
            pub const Y: u32 = 0x99; // German layout: Z
            pub const Z: u32 = 0x06; // German layout: Y

            // German layout:
            pub const U_UML: u32 = 0x21;
            pub const A_UML: u32 = 0x27;
            pub const O_UML: u32 = 0x29;

            // GERMAN or ENGLISH layout ???
            pub const SEMICOLON: u32 = 0x29;
            pub const QUOTE: u32 = 0x27;
            pub const COMMA: u32 = 0x2B;
            pub const PERIOD: u32 = 0x2F;
            pub const SPACE: u32 = 0x31;
            pub const SQUARE_BARCKET: u32 = 0x1E;  // ]  german layout: PLUS  +
            pub const SLASH:          u32 = 0x2C;  // /  german layout: MINUS -
            pub const BACKSLASH:      u32 = 0x2A;  // \  GERMAN layout: SHARP #

            pub const ESCAPE: u32 = 0x35; // 53
            pub const DEL:    u32 = 0x33;

            pub const LEFT_ARROW:  u32 = 0x7B;
            pub const RIGHT_ARROW: u32 = 0x7C;
            pub const DOWN_ARROW:  u32 = 0x7D;
            pub const UP_ARROW:    u32 = 0x7E;

            pub const SHIFT:   u32 = 0x38; // left!,         right: 0x3C
            pub const COMMAND: u32 = 0x37; //                right: 0x36  
            pub const LALT:    u32 = 0x3A; // macOS: OPTION  right: 0x3D
            pub const CONTROL: u32 = 0x3B; // macOS only, left only
                                           // macOS "fn" is not a key, going into the software
        }

    }


    else


    if #[cfg(target_arch = "wasm32")]
    {
        // https://www.codetable.net/asciikeycodes
        pub mod Scancode {
            pub const W: u32 = 0x57;
            pub const A: u32 = 0x41;
            pub const S: u32 = 0x53;
            pub const D: u32 = 0x44;
            pub const E: u32 = 0x45;
            pub const F: u32 = 0x46;
            pub const G: u32 = 0x47;
            pub const R: u32 = 0x52;
            pub const T: u32 = 0x54;
            pub const Q: u32 = 0x51;
            pub const Z: u32 = 0x5a;
            pub const P: u32 = 0x50;
            pub const SEMICOLON: u32 = 0xba;
            pub const QUOTE: u32 = 0xde;
            pub const COMMA: u32 = 0xbc;
            pub const PERIOD: u32 = 0xbe;
            pub const ESCAPE: u32 = 0x1b;
            pub const LALT: u32 = 0x12;

            pub const LEFT_ARROW: u32 = 0x00;
            pub const RIGHT_ARROW: u32 = 0x00;
            pub const DOWN_ARROW: u32 = 0x00;
            pub const UP_ARROW: u32 = 0x00;
            pub const PLUS: u32 = 0x00;  // Backslash
            pub const SHARP: u32 = 0x00; // ???
            pub const U_UML: u32 = 0x00;
            pub const A_UML: u32 = 0x00;
            pub const DEL:   u32 = 0x00; // erase?
        }
    } 



    else 



    {
        // Windows? Linux? DIRECT X KEY CODES: https://www.dougdoug.com/twitchplays-keycodes-py-3-x
        pub mod Scancodes {
            pub const W: u32 = 0x11;
            pub const A: u32 = 0x1E;
            pub const S: u32 = 0x1F;
            pub const D: u32 = 0x20;
            pub const E: u32 = 0x12;
            pub const F: u32 = 0x21;
            pub const G: u32 = 0x22;
            pub const R: u32 = 0x13;
            pub const T: u32 = 0x14;
            pub const Q: u32 = 0x10;
            pub const Z: u32 = 0x2C;
            pub const P: u32 = 0x19;
            pub const SEMICOLON: u32 = 0x27;
            pub const QUOTE: u32 = 0x28;
            pub const COMMA: u32 = 0x33;
            pub const PERIOD: u32 = 0x34;
            pub const SHIFT: u32 = 0x2A;
            pub const ESCAPE: u32 = 0x01;
            pub const LALT: u32 = 0x38;

            pub const LEFT_ARROW: u32 = 0x00;
            pub const RIGHT_ARROW: u32 = 0x00;
            pub const DOWN_ARROW: u32 = 0x00;
            pub const UP_ARROW: u32 = 0x00;
            pub const PLUS: u32 = 0x00;  // Backslash
            pub const SHARP: u32 = 0x00; // ???
            pub const U_UML: u32 = 0x00;
            pub const A_UML: u32 = 0x00;
            pub const DEL:   u32 = 0x00; // erase?
        }
    }
);
