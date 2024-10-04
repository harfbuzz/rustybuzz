// WARNING: this file was generated by ../scripts/gen-shaping-tests.py

use crate::shape;

#[test]
fn bugs_001() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/NotoSansCJK.subset1.otf",
            "\u{4F60}\u{597D}\u{FF0C}",
            "--direction rtl",
        ),
        "gid6=2+1000|\
         gid3=1+1000|\
         gid1=0+1000"
    );
}

#[test]
fn bugs_002() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/OpenSans.subset1.ttf",
            "\u{0065}",
            "--variations=wght=500,wdth=80",
        ),
        "gid0=0+1218"
    );
}

#[test]
fn fuzzer_001() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/PT_Sans-Caption-Web-Regular.ttf",
            "\u{1EA4}\u{006E}",
            "",
        ),
        "Acircumflex=0+645|\
         uniF401=0+0|\
         n=1+641"
    );
}

#[test]
fn fuzzer_002() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/AdobeBlank-Regular.ttf",
            "\u{0F42}\u{0FB7}",
            "--no-glyph-names",
        ),
        "1859=0+0|\
         1976=0+0"
    );
}

#[test]
fn fuzzer_003() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/Rasa.subset1.otf",
            "\u{0A93}\u{0ABC}",
            "",
        ),
        "gid5=0+982|\
         gid22=0@-1,0+0|\
         gid21=0+0"
    );
}

#[test]
fn fuzzer_004() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/AdobeBlank-Regular.ttf",
            "\u{104A}\u{102F}",
            "",
        ),
        "cid00075=0+0|\
         cid00048=0+0"
    );
}

#[test]
fn fuzzer_005() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/NotoSansMyanmarUI-Regular.subset1.otf",
            "\u{1004}\u{103A}\u{1039}\u{1002}\u{101C}",
            "",
        ),
        "gid1=0+668|\
         gid5=0@-4,0+0|\
         gid3=4+1126"
    );
}

#[test]
fn fuzzer_006() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/NotoSansSinhala.subset1.otf",
            "\u{0DC1}\u{200D}\u{0DCA}\u{200D}\u{0DBB}\u{0DD3}",
            "",
        ),
        "gid2=0+917|\
         gid7=0+0|\
         gid4=0+0"
    );
}

#[test]
fn fuzzer_007() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/LaBelleAurore.ttf",
            "\u{006B}\u{0065}\u{031D}",
            "",
        ),
        "k=0+479|\
         e=1+343|\
         .notdef=1@-172,-59+0"
    );
}

#[test]
fn fuzzer_008() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/Linefont.ttf",
            "\u{0054}\u{021F}",
            "--no-glyph-names",
        ),
        "93=0+100|\
         233=1@0,900+100|\
         2=1@0,140+0|\
         1=1+0"
    );
}

#[test]
fn fuzzer_009() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/Linefont.ttf",
            "\u{021F}\u{0061}",
            "--no-glyph-names",
        ),
        "17=0+100|\
         1=0+0|\
         4=1+100"
    );
}

#[test]
fn glyph_flags_001() {
    assert_eq!(
        shape(
            "tests/fonts/aots/gpos_chaining1_boundary_f1.otf",
            "\u{0000}\u{0014}\u{0015}\u{0016}\u{0017}\u{0000}",
            "--show-flags --features=\"test\"",
        ),
        ".notdef=0+1500|\
         g20=1+1500|\
         g21=2+1500#1|\
         g22=3+1500#1|\
         g23=4+1500#1|\
         .notdef=5+1500"
    );
}

#[test]
fn glyph_flags_003() {
    assert_eq!(
        shape(
            "tests/fonts/text-rendering-tests/TestMORXThirtyfive.ttf",
            "\u{0058}\u{0041}",
            "--show-flags --ned --remove-default-ignorables",
        ),
        "X|\
         A@586,0#1|\
         B@1225,0#1|\
         C@1851,0#1|\
         E@2447,0#1"
    );
}
