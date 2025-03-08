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
fn bugs_003() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/NotoSansMalayalam.subset1.ttf",
            "\u{0D38}\u{0D4D}\u{0D25}",
            "",
        ),
        "gid7=0+1891"
    );
}

#[test]
fn colr_001() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F0100}\u{F0101}\u{F0102}\u{F0103}",
            "--show-extents",
        ),
        "linear_repeat_0_1=0+1000<100,950,800,-700>|\
         linear_repeat_0.2_0.8=1+1000<100,950,800,-700>|\
         linear_repeat_0_1.5=2+1000<100,950,800,-700>|\
         linear_repeat_0.5_1.5=3+1000<100,950,800,-700>"
    );
}

#[test]
fn colr_002() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F0200}\u{F0201}\u{F0202}\u{F0203}\u{F0204}\u{F0205}\u{F0206}\u{F0207}\u{F0208}\u{F0209}\
             \u{F020A}\u{F020B}\u{F020C}\u{F020D}\u{F020E}\u{F020F}\u{F0210}\u{F0211}\u{F0212}\u{F0213}\
             \u{F0214}\u{F0215}\u{F0216}\u{F0217}\u{F0218}\u{F0219}\u{F021A}\u{F021B}\u{F021C}\u{F021D}\
             \u{F021E}\u{F021F}\u{F0220}\u{F0221}\u{F0222}\u{F0223}\u{F0224}\u{F0225}\u{F0226}\u{F0227}\
             \u{F0228}\u{F0229}\u{F022A}\u{F022B}\u{F022C}\u{F022D}\u{F022E}\u{F022F}\u{F0230}\u{F0231}\
             \u{F0232}\u{F0233}\u{F0234}\u{F0235}\u{F0236}\u{F0237}\u{F0238}\u{F0239}\u{F023A}\u{F023B}\
             \u{F023C}\u{F023D}\u{F023E}\u{F023F}\u{F0240}\u{F0241}\u{F0242}\u{F0243}\u{F0244}\u{F0245}\
             \u{F0246}\u{F0247}",
            "--show-extents",
        ),
        "sweep_0_360_pad_narrow=0+1000<150,950,700,-700>|\
         sweep_60_300_pad_narrow=1+1000<150,950,700,-700>|\
         sweep_0_90_pad_narrow=2+1000<150,950,700,-700>|\
         sweep_90_0_pad_narrow=3+1000<150,950,700,-700>|\
         sweep_45_90_pad_narrow=4+1000<150,950,700,-700>|\
         sweep_90_45_pad_narrow=5+1000<150,950,700,-700>|\
         sweep_247.5_292.5_pad_narrow=6+1000<150,950,700,-700>|\
         sweep_-45_45_pad_narrow=7+1000<150,950,700,-700>|\
         sweep_45_-45_pad_narrow=8+1000<150,950,700,-700>|\
         sweep_270_440_pad_narrow=9+1000<150,950,700,-700>|\
         sweep_440_270_pad_narrow=10+1000<150,950,700,-700>|\
         sweep_-180_540_pad_narrow=11+1000<150,950,700,-700>|\
         sweep_0_360_reflect_narrow=12+1000<150,950,700,-700>|\
         sweep_60_300_reflect_narrow=13+1000<150,950,700,-700>|\
         sweep_0_90_reflect_narrow=14+1000<150,950,700,-700>|\
         sweep_90_0_reflect_narrow=15+1000<150,950,700,-700>|\
         sweep_45_90_reflect_narrow=16+1000<150,950,700,-700>|\
         sweep_90_45_reflect_narrow=17+1000<150,950,700,-700>|\
         sweep_247.5_292.5_reflect_narrow=18+1000<150,950,700,-700>|\
         sweep_-45_45_reflect_narrow=19+1000<150,950,700,-700>|\
         sweep_45_-45_reflect_narrow=20+1000<150,950,700,-700>|\
         sweep_270_440_reflect_narrow=21+1000<150,950,700,-700>|\
         sweep_440_270_reflect_narrow=22+1000<150,950,700,-700>|\
         sweep_-180_540_reflect_narrow=23+1000<150,950,700,-700>|\
         sweep_0_360_repeat_narrow=24+1000<150,950,700,-700>|\
         sweep_60_300_repeat_narrow=25+1000<150,950,700,-700>|\
         sweep_0_90_repeat_narrow=26+1000<150,950,700,-700>|\
         sweep_90_0_repeat_narrow=27+1000<150,950,700,-700>|\
         sweep_45_90_repeat_narrow=28+1000<150,950,700,-700>|\
         sweep_90_45_repeat_narrow=29+1000<150,950,700,-700>|\
         sweep_247.5_292.5_repeat_narrow=30+1000<150,950,700,-700>|\
         sweep_-45_45_repeat_narrow=31+1000<150,950,700,-700>|\
         sweep_45_-45_repeat_narrow=32+1000<150,950,700,-700>|\
         sweep_270_440_repeat_narrow=33+1000<150,950,700,-700>|\
         sweep_440_270_repeat_narrow=34+1000<150,950,700,-700>|\
         sweep_-180_540_repeat_narrow=35+1000<150,950,700,-700>|\
         sweep_0_360_pad_wide=36+1000<150,950,700,-700>|\
         sweep_60_300_pad_wide=37+1000<150,950,700,-700>|\
         sweep_0_90_pad_wide=38+1000<150,950,700,-700>|\
         sweep_90_0_pad_wide=39+1000<150,950,700,-700>|\
         sweep_45_90_pad_wide=40+1000<150,950,700,-700>|\
         sweep_90_45_pad_wide=41+1000<150,950,700,-700>|\
         sweep_247.5_292.5_pad_wide=42+1000<150,950,700,-700>|\
         sweep_-45_45_pad_wide=43+1000<150,950,700,-700>|\
         sweep_45_-45_pad_wide=44+1000<150,950,700,-700>|\
         sweep_270_440_pad_wide=45+1000<150,950,700,-700>|\
         sweep_440_270_pad_wide=46+1000<150,950,700,-700>|\
         sweep_-180_540_pad_wide=47+1000<150,950,700,-700>|\
         sweep_0_360_reflect_wide=48+1000<150,950,700,-700>|\
         sweep_60_300_reflect_wide=49+1000<150,950,700,-700>|\
         sweep_0_90_reflect_wide=50+1000<150,950,700,-700>|\
         sweep_90_0_reflect_wide=51+1000<150,950,700,-700>|\
         sweep_45_90_reflect_wide=52+1000<150,950,700,-700>|\
         sweep_90_45_reflect_wide=53+1000<150,950,700,-700>|\
         sweep_247.5_292.5_reflect_wide=54+1000<150,950,700,-700>|\
         sweep_-45_45_reflect_wide=55+1000<150,950,700,-700>|\
         sweep_45_-45_reflect_wide=56+1000<150,950,700,-700>|\
         sweep_270_440_reflect_wide=57+1000<150,950,700,-700>|\
         sweep_440_270_reflect_wide=58+1000<150,950,700,-700>|\
         sweep_-180_540_reflect_wide=59+1000<150,950,700,-700>|\
         sweep_0_360_repeat_wide=60+1000<150,950,700,-700>|\
         sweep_60_300_repeat_wide=61+1000<150,950,700,-700>|\
         sweep_0_90_repeat_wide=62+1000<150,950,700,-700>|\
         sweep_90_0_repeat_wide=63+1000<150,950,700,-700>|\
         sweep_45_90_repeat_wide=64+1000<150,950,700,-700>|\
         sweep_90_45_repeat_wide=65+1000<150,950,700,-700>|\
         sweep_247.5_292.5_repeat_wide=66+1000<150,950,700,-700>|\
         sweep_-45_45_repeat_wide=67+1000<150,950,700,-700>|\
         sweep_45_-45_repeat_wide=68+1000<150,950,700,-700>|\
         sweep_270_440_repeat_wide=69+1000<150,950,700,-700>|\
         sweep_440_270_repeat_wide=70+1000<150,950,700,-700>|\
         sweep_-180_540_repeat_wide=71+1000<150,950,700,-700>"
    );
}

#[test]
fn colr_004() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F0500}\u{F0501}\u{F0502}",
            "--show-extents",
        ),
        "linear_gradient_extend_mode_pad=0+1000<0,1000,1000,-1000>|\
         linear_gradient_extend_mode_repeat=1+1000<0,1000,1000,-1000>|\
         linear_gradient_extend_mode_reflect=2+1000<0,1000,1000,-1000>"
    );
}

#[test]
fn colr_005() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F0503}\u{F0504}\u{F0505}\u{F0506}\u{F0507}\u{F0508}",
            "--show-extents",
        ),
        "radial_contained_gradient_extend_mode_pad=0+1000<0,1000,1000,-1000>|\
         radial_contained_gradient_extend_mode_repeat=1+1000<0,1000,1000,-1000>|\
         radial_contained_gradient_extend_mode_reflect=2+1000<0,1000,1000,-1000>|\
         radial_horizontal_gradient_extend_mode_pad=3+1000<0,1000,1000,-1000>|\
         radial_horizontal_gradient_extend_mode_repeat=4+1000<0,1000,1000,-1000>|\
         radial_horizontal_gradient_extend_mode_reflect=5+1000<0,1000,1000,-1000>"
    );
}

#[test]
fn colr_013() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F0D00}",
            "--show-extents",
        ),
        "gradient_p2_skewed=0+1250<100,950,1100,-700>"
    );
}

#[test]
fn colr_017() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F1100}\u{F1101}",
            "--show-extents",
        ),
        "paintcolrglyph_cycle_first=0+1000<0,0,0,0>|\
         paintcolrglyph_cycle_second=1+1000<0,0,0,0>"
    );
}

#[test]
fn colr_019() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F1300}\u{F1301}\u{F1302}\u{F1303}\u{F1304}\u{F1305}\u{F1306}\u{F1307}\u{F1308}\u{F1309}\
             \u{F130A}\u{F130B}\u{F130C}\u{F130D}\u{F130E}\u{F130F}\u{F1310}\u{F1311}\u{F1312}\u{F1313}\
             \u{F1314}\u{F1315}\u{F1316}\u{F1317}",
            "--show-extents",
        ),
        "sweep_coincident_angles_forward_blue_red_pad=0+1000<150,950,700,-700>|\
         sweep_coincident_angles_forward_blue_red_reflect=1+1000<150,950,700,-700>|\
         sweep_coincident_angles_forward_blue_red_repeat=2+1000<150,950,700,-700>|\
         sweep_coincident_angles_forward_linen_gray_pad=3+1000<150,950,700,-700>|\
         sweep_coincident_angles_forward_linen_gray_reflect=4+1000<150,950,700,-700>|\
         sweep_coincident_angles_forward_linen_gray_repeat=5+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_blue_red_pad=6+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_blue_red_reflect=7+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_blue_red_repeat=8+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_linen_gray_pad=9+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_linen_gray_reflect=10+1000<150,950,700,-700>|\
         sweep_coincident_angles_reverse_linen_gray_repeat=11+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_blue_red_pad=12+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_blue_red_reflect=13+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_blue_red_repeat=14+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_linen_gray_pad=15+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_linen_gray_reflect=16+1000<150,950,700,-700>|\
         sweep_coincident_stops_forward_linen_gray_repeat=17+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_blue_red_pad=18+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_blue_red_reflect=19+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_blue_red_repeat=20+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_linen_gray_pad=21+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_linen_gray_reflect=22+1000<150,950,700,-700>|\
         sweep_coincident_stops_reverse_linen_gray_repeat=23+1000<150,950,700,-700>"
    );
}

#[test]
fn colr_020() {
    assert_eq!(
        shape(
            "tests/fonts/rb_custom/test_glyphs-glyf_colr_1_no_cliplist.ttf",
            "\u{F1400}\u{F1401}\u{F1402}\u{F1403}\u{F1404}\u{F1405}\u{F1406}\u{F1407}\u{F1408}\u{F1409}\
             \u{F140A}\u{F140B}\u{F140C}\u{F140D}\u{F140E}\u{F140F}",
            "--show-extents",
        ),
        "paint_glyph_nested_identity_identity=0+1000<200,770,600,-520>|\
         paint_glyph_nested_identity_translate=1+1000<200,770,600,-520>|\
         paint_glyph_nested_identity_rotate_origin=2+1000<200,770,600,-520>|\
         paint_glyph_nested_identity_rotate_center=3+1000<200,770,600,-520>|\
         paint_glyph_nested_translate_identity=4+1000<320,890,600,-520>|\
         paint_glyph_nested_translate_translate=5+1000<320,890,600,-520>|\
         paint_glyph_nested_translate_rotate_origin=6+1000<320,890,600,-520>|\
         paint_glyph_nested_translate_rotate_center=7+1000<320,890,600,-520>|\
         paint_glyph_nested_rotate_origin_identity=8+1000<63,897,681,-616>|\
         paint_glyph_nested_rotate_origin_translate=9+1000<63,897,681,-616>|\
         paint_glyph_nested_rotate_origin_rotate_origin=10+1000<63,897,681,-616>|\
         paint_glyph_nested_rotate_origin_rotate_center=11+1000<63,897,681,-616>|\
         paint_glyph_nested_rotate_center_identity=12+1000<124,899,750,-779>|\
         paint_glyph_nested_rotate_center_translate=13+1000<124,899,750,-779>|\
         paint_glyph_nested_rotate_center_rotate_origin=14+1000<124,899,750,-779>|\
         paint_glyph_nested_rotate_center_rotate_center=15+1000<124,899,750,-779>"
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
