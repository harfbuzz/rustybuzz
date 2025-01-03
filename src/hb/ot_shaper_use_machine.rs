#![allow(
    dead_code,
    non_upper_case_globals,
    unused_assignments,
    unused_parens,
    while_true,
    clippy::assign_op_pattern,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::double_parens,
    clippy::unnecessary_cast,
    clippy::single_match,
    clippy::never_loop
)]

use core::cell::Cell;

use super::buffer::{hb_buffer_t, HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE};
use super::hb_glyph_info_t;
use super::machine_cursor::MachineCursor;
use super::ot_layout::*;
use super::ot_shaper_use::category;

static _use_syllable_machine_trans_keys: [u8; 256] = [
    35, 37, 0, 42, 5, 42, 5, 42, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8, 32, 8, 32, 8, 8, 8, 34, 8,
    34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6, 39, 6, 39, 6, 39, 5, 42,
    1, 8, 1, 34, 8, 28, 8, 28, 5, 42, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8, 32, 8, 32, 8, 8, 8, 34,
    8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6, 39, 6, 39, 6, 39, 5,
    42, 1, 8, 1, 8, 1, 34, 8, 8, 7, 8, 3, 8, 5, 42, 5, 42, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8,
    32, 8, 32, 8, 8, 8, 34, 8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6,
    39, 6, 39, 6, 39, 5, 42, 1, 8, 1, 8, 1, 34, 8, 8, 5, 42, 1, 39, 8, 34, 8, 33, 8, 33, 8, 33, 8,
    32, 8, 32, 8, 8, 8, 34, 8, 34, 8, 34, 1, 8, 8, 34, 8, 39, 8, 39, 8, 39, 8, 39, 6, 39, 8, 39, 6,
    39, 6, 39, 6, 39, 5, 42, 1, 8, 1, 34, 3, 8, 7, 8, 1, 42, 8, 28, 8, 28, 1, 4, 8, 41, 8, 37, 8,
    38, 8, 40, 5, 42, 0, 0,
];
static _use_syllable_machine_char_class: [i8; 59] = [
    0, 1, 2, 2, 3, 4, 2, 2, 2, 2, 2, 5, 6, 7, 8, 2, 2, 2, 9, 2, 2, 2, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 2, 24, 25, 26, 2, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
    39, 40, 41, 42, 0, 0,
];
static _use_syllable_machine_index_offsets: [i16; 129] = [
    0, 3, 46, 84, 122, 161, 188, 214, 240, 266, 291, 316, 317, 344, 371, 398, 406, 433, 465, 497,
    529, 561, 595, 627, 661, 695, 729, 767, 775, 809, 830, 851, 889, 928, 955, 981, 1007, 1033,
    1058, 1083, 1084, 1111, 1138, 1165, 1173, 1200, 1232, 1264, 1296, 1328, 1362, 1394, 1428, 1462,
    1496, 1534, 1542, 1550, 1584, 1585, 1587, 1593, 1631, 1669, 1708, 1735, 1761, 1787, 1813, 1838,
    1863, 1864, 1891, 1918, 1945, 1953, 1980, 2012, 2044, 2076, 2108, 2142, 2174, 2208, 2242, 2276,
    2314, 2322, 2330, 2364, 2365, 2403, 2442, 2469, 2495, 2521, 2547, 2572, 2597, 2598, 2625, 2652,
    2679, 2687, 2714, 2746, 2778, 2810, 2842, 2876, 2908, 2942, 2976, 3010, 3048, 3056, 3090, 3096,
    3098, 3140, 3161, 3182, 3186, 3220, 3250, 3281, 3314, 0, 0,
];
static _use_syllable_machine_indices: [i16; 3354] = [
    1, 0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 9, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 33, 1, 6, 37, 6, 38, 6, 6, 36, 40, 41, 39, 42, 39,
    43, 44, 45, 46, 47, 48, 49, 50, 51, 40, 52, 53, 54, 55, 56, 57, 58, 59, 60, 39, 61, 62, 63, 64,
    61, 39, 39, 39, 39, 65, 39, 39, 64, 40, 41, 39, 42, 39, 43, 44, 45, 46, 47, 48, 49, 50, 51, 40,
    52, 53, 54, 55, 56, 57, 58, 39, 39, 39, 61, 62, 63, 64, 61, 39, 39, 39, 39, 65, 39, 39, 64, 40,
    39, 39, 39, 39, 39, 39, 42, 39, 39, 44, 45, 46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 56, 57,
    58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 42, 39, 39, 44, 45, 46, 47, 39, 39, 39,
    39, 39, 39, 39, 39, 39, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 42, 39, 39, 39, 45, 46, 47,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 62, 63, 64, 42, 39, 39, 39, 39,
    46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 62, 63, 64, 42, 39, 39,
    39, 39, 39, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 62, 63, 64, 42,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 62, 63,
    42, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    63, 42, 42, 39, 39, 39, 45, 46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 56, 57, 58, 39, 39, 39,
    39, 62, 63, 64, 66, 42, 39, 39, 39, 45, 46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 57, 58,
    39, 39, 39, 39, 62, 63, 64, 66, 42, 39, 39, 39, 45, 46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    39, 39, 58, 39, 39, 39, 39, 62, 63, 64, 66, 67, 39, 39, 39, 39, 39, 39, 42, 42, 39, 39, 39, 45,
    46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 62, 63, 64, 66, 42, 39,
    43, 44, 45, 46, 47, 39, 39, 39, 39, 39, 39, 53, 54, 55, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64,
    66, 39, 39, 39, 39, 44, 42, 39, 39, 44, 45, 46, 47, 39, 39, 39, 39, 39, 39, 53, 54, 55, 56, 57,
    58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 42, 39, 39, 44, 45, 46, 47, 39, 39, 39,
    39, 39, 39, 39, 54, 55, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 42, 39,
    39, 44, 45, 46, 47, 39, 39, 39, 39, 39, 39, 39, 39, 55, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64,
    66, 39, 39, 39, 39, 44, 68, 39, 42, 39, 43, 44, 45, 46, 47, 39, 49, 50, 39, 39, 39, 53, 54, 55,
    56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 42, 39, 39, 44, 45, 46, 47, 39,
    39, 39, 39, 39, 39, 39, 39, 39, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44,
    68, 39, 42, 39, 43, 44, 45, 46, 47, 39, 39, 50, 39, 39, 39, 53, 54, 55, 56, 57, 58, 39, 39, 39,
    39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 68, 39, 42, 39, 43, 44, 45, 46, 47, 39, 39, 39, 39, 39,
    39, 53, 54, 55, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 39, 39, 39, 39, 44, 68, 39, 42, 39,
    43, 44, 45, 46, 47, 48, 49, 50, 39, 39, 39, 53, 54, 55, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64,
    66, 39, 39, 39, 39, 44, 40, 41, 39, 42, 39, 43, 44, 45, 46, 47, 48, 49, 50, 51, 39, 52, 53, 54,
    55, 56, 57, 58, 39, 39, 39, 61, 62, 63, 64, 61, 39, 39, 39, 39, 65, 39, 39, 64, 40, 39, 39, 39,
    39, 39, 39, 42, 40, 39, 39, 39, 39, 39, 39, 42, 39, 39, 44, 45, 46, 47, 39, 39, 39, 39, 39, 39,
    39, 39, 39, 56, 57, 58, 39, 39, 39, 39, 62, 63, 64, 66, 42, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    39, 39, 39, 39, 39, 39, 39, 39, 39, 59, 60, 42, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    39, 39, 39, 39, 39, 39, 39, 60, 5, 70, 69, 71, 69, 72, 73, 74, 75, 76, 77, 78, 79, 80, 5, 81,
    82, 83, 84, 85, 86, 87, 69, 69, 69, 88, 89, 90, 91, 92, 69, 69, 69, 69, 93, 69, 69, 94, 5, 69,
    69, 69, 69, 69, 69, 71, 69, 69, 73, 74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 85, 86, 87,
    69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 71, 69, 69, 73, 74, 75, 76, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95, 71, 69, 69, 69, 74, 75, 76, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 89, 90, 91, 71, 69, 69, 69, 69, 75,
    76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 89, 90, 91, 71, 69, 69, 69,
    69, 69, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 89, 90, 91, 71, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 89, 90, 71,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 90,
    71, 71, 69, 69, 69, 74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 85, 86, 87, 69, 69, 69, 69,
    89, 90, 91, 95, 71, 69, 69, 69, 74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 86, 87, 69,
    69, 69, 69, 89, 90, 91, 95, 71, 69, 69, 69, 74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 87, 69, 69, 69, 69, 89, 90, 91, 95, 97, 96, 96, 96, 96, 96, 96, 98, 71, 69, 69, 69, 74, 75,
    76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 89, 90, 91, 95, 71, 69, 72,
    73, 74, 75, 76, 69, 69, 69, 69, 69, 69, 82, 83, 84, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95,
    69, 69, 69, 69, 73, 71, 69, 69, 73, 74, 75, 76, 69, 69, 69, 69, 69, 69, 82, 83, 84, 85, 86, 87,
    69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 71, 69, 69, 73, 74, 75, 76, 69, 69, 69, 69,
    69, 69, 69, 83, 84, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 71, 69, 69,
    73, 74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 84, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95,
    69, 69, 69, 69, 73, 99, 69, 71, 69, 72, 73, 74, 75, 76, 69, 78, 79, 69, 69, 69, 82, 83, 84, 85,
    86, 87, 69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 71, 69, 69, 73, 74, 75, 76, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 99,
    69, 71, 69, 72, 73, 74, 75, 76, 69, 69, 79, 69, 69, 69, 82, 83, 84, 85, 86, 87, 69, 69, 69, 69,
    89, 90, 91, 95, 69, 69, 69, 69, 73, 99, 69, 71, 69, 72, 73, 74, 75, 76, 69, 69, 69, 69, 69, 69,
    82, 83, 84, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95, 69, 69, 69, 69, 73, 99, 69, 71, 69, 72,
    73, 74, 75, 76, 77, 78, 79, 69, 69, 69, 82, 83, 84, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95,
    69, 69, 69, 69, 73, 5, 70, 69, 71, 69, 72, 73, 74, 75, 76, 77, 78, 79, 80, 69, 81, 82, 83, 84,
    85, 86, 87, 69, 69, 69, 88, 89, 90, 91, 92, 69, 69, 69, 69, 93, 69, 69, 94, 5, 100, 100, 100,
    100, 100, 100, 101, 5, 96, 96, 96, 96, 96, 96, 98, 5, 69, 69, 69, 69, 69, 69, 71, 69, 69, 73,
    74, 75, 76, 69, 69, 69, 69, 69, 69, 69, 69, 69, 85, 86, 87, 69, 69, 69, 69, 89, 90, 91, 95,
    101, 103, 104, 7, 105, 105, 105, 105, 106, 107, 108, 69, 71, 69, 109, 110, 111, 112, 113, 114,
    115, 116, 117, 107, 118, 119, 120, 121, 122, 123, 124, 59, 60, 69, 125, 126, 127, 128, 129, 69,
    69, 69, 69, 130, 69, 69, 131, 107, 108, 69, 71, 69, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 107, 118, 119, 120, 121, 122, 123, 124, 69, 69, 69, 125, 126, 127, 128, 129, 69, 69, 69,
    69, 130, 69, 69, 131, 107, 69, 69, 69, 69, 69, 69, 71, 69, 69, 110, 111, 112, 113, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110,
    71, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 122, 123, 124, 69, 69, 69,
    69, 126, 127, 128, 132, 71, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 126, 127, 128, 71, 69, 69, 69, 69, 112, 113, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 126, 127, 128, 71, 69, 69, 69, 69, 69, 113, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 126, 127, 128, 71, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 126, 127, 71, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 127, 71, 71, 69,
    69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 122, 123, 124, 69, 69, 69, 69, 126,
    127, 128, 132, 71, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 123, 124,
    69, 69, 69, 69, 126, 127, 128, 132, 71, 69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 69, 124, 69, 69, 69, 69, 126, 127, 128, 132, 133, 96, 96, 96, 96, 96, 96, 98, 71,
    69, 69, 69, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 126,
    127, 128, 132, 71, 69, 109, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 119, 120, 121, 122,
    123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110, 71, 69, 69, 110, 111, 112,
    113, 69, 69, 69, 69, 69, 69, 119, 120, 121, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132,
    69, 69, 69, 69, 110, 71, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 120, 121, 122,
    123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110, 71, 69, 69, 110, 111, 112,
    113, 69, 69, 69, 69, 69, 69, 69, 69, 121, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132,
    69, 69, 69, 69, 110, 134, 69, 71, 69, 109, 110, 111, 112, 113, 69, 115, 116, 69, 69, 69, 119,
    120, 121, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110, 71, 69, 69,
    110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69, 122, 123, 124, 69, 69, 69, 69, 126,
    127, 128, 132, 69, 69, 69, 69, 110, 134, 69, 71, 69, 109, 110, 111, 112, 113, 69, 69, 116, 69,
    69, 69, 119, 120, 121, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110,
    134, 69, 71, 69, 109, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 119, 120, 121, 122, 123, 124,
    69, 69, 69, 69, 126, 127, 128, 132, 69, 69, 69, 69, 110, 134, 69, 71, 69, 109, 110, 111, 112,
    113, 114, 115, 116, 69, 69, 69, 119, 120, 121, 122, 123, 124, 69, 69, 69, 69, 126, 127, 128,
    132, 69, 69, 69, 69, 110, 107, 108, 69, 71, 69, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    69, 118, 119, 120, 121, 122, 123, 124, 69, 69, 69, 125, 126, 127, 128, 129, 69, 69, 69, 69,
    130, 69, 69, 131, 107, 100, 100, 100, 100, 100, 100, 101, 107, 96, 96, 96, 96, 96, 96, 98, 107,
    69, 69, 69, 69, 69, 69, 71, 69, 69, 110, 111, 112, 113, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    122, 123, 124, 69, 69, 69, 69, 126, 127, 128, 132, 101, 9, 10, 135, 12, 135, 14, 15, 16, 17,
    18, 19, 20, 21, 22, 9, 23, 24, 25, 26, 27, 28, 29, 135, 135, 135, 33, 34, 35, 36, 33, 135, 135,
    135, 135, 38, 135, 135, 36, 9, 135, 135, 135, 135, 135, 135, 12, 135, 135, 15, 16, 17, 18, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135,
    135, 135, 135, 15, 12, 135, 135, 15, 16, 17, 18, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 12, 135, 135, 135, 16, 17, 18, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 34, 35, 36, 12, 135, 135, 135,
    135, 17, 18, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    34, 35, 36, 12, 135, 135, 135, 135, 135, 18, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 34, 35, 36, 12, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 34, 35, 12, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    35, 12, 12, 135, 135, 135, 16, 17, 18, 135, 135, 135, 135, 135, 135, 135, 135, 135, 27, 28, 29,
    135, 135, 135, 135, 34, 35, 36, 136, 12, 135, 135, 135, 16, 17, 18, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 12, 135, 135, 135, 16,
    17, 18, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 29, 135, 135, 135, 135, 34, 35,
    36, 136, 137, 135, 135, 135, 135, 135, 135, 12, 12, 135, 135, 135, 16, 17, 18, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 34, 35, 36, 136, 12, 135, 14,
    15, 16, 17, 18, 135, 135, 135, 135, 135, 135, 24, 25, 26, 27, 28, 29, 135, 135, 135, 135, 34,
    35, 36, 136, 135, 135, 135, 135, 15, 12, 135, 135, 15, 16, 17, 18, 135, 135, 135, 135, 135,
    135, 24, 25, 26, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135, 135, 135, 135, 15, 12,
    135, 135, 15, 16, 17, 18, 135, 135, 135, 135, 135, 135, 135, 25, 26, 27, 28, 29, 135, 135, 135,
    135, 34, 35, 36, 136, 135, 135, 135, 135, 15, 12, 135, 135, 15, 16, 17, 18, 135, 135, 135, 135,
    135, 135, 135, 135, 26, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135, 135, 135, 135,
    15, 138, 135, 12, 135, 14, 15, 16, 17, 18, 135, 20, 21, 135, 135, 135, 24, 25, 26, 27, 28, 29,
    135, 135, 135, 135, 34, 35, 36, 136, 135, 135, 135, 135, 15, 12, 135, 135, 15, 16, 17, 18, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135,
    135, 135, 135, 15, 138, 135, 12, 135, 14, 15, 16, 17, 18, 135, 135, 21, 135, 135, 135, 24, 25,
    26, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135, 135, 135, 135, 15, 138, 135, 12, 135,
    14, 15, 16, 17, 18, 135, 135, 135, 135, 135, 135, 24, 25, 26, 27, 28, 29, 135, 135, 135, 135,
    34, 35, 36, 136, 135, 135, 135, 135, 15, 138, 135, 12, 135, 14, 15, 16, 17, 18, 19, 20, 21,
    135, 135, 135, 24, 25, 26, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 135, 135, 135, 135,
    15, 9, 10, 135, 12, 135, 14, 15, 16, 17, 18, 19, 20, 21, 22, 135, 23, 24, 25, 26, 27, 28, 29,
    135, 135, 135, 33, 34, 35, 36, 33, 135, 135, 135, 135, 38, 135, 135, 36, 9, 135, 135, 135, 135,
    135, 135, 12, 9, 135, 135, 135, 135, 135, 135, 12, 135, 135, 15, 16, 17, 18, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 27, 28, 29, 135, 135, 135, 135, 34, 35, 36, 136, 139, 135, 135,
    135, 135, 12, 11, 12, 5, 135, 135, 5, 9, 10, 11, 12, 135, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    9, 23, 24, 25, 26, 27, 28, 29, 30, 31, 135, 33, 34, 35, 36, 33, 135, 135, 135, 135, 38, 135,
    135, 36, 12, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 30, 31, 12, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 31, 5, 140, 140, 5, 142, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 143,
    141, 144, 141, 144, 145, 142, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 1, 143, 143, 142, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 143, 141, 144, 142, 141, 141, 141, 141, 141, 141, 141, 141, 141,
    141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 141, 143,
    141, 144, 141, 144, 40, 41, 39, 42, 39, 43, 44, 45, 46, 47, 48, 49, 50, 51, 40, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 39, 61, 62, 63, 64, 61, 1, 39, 2, 39, 65, 39, 39, 64, 0, 0,
];
static _use_syllable_machine_index_defaults: [i16; 129] = [
    0, 6, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39,
    39, 39, 39, 39, 39, 39, 39, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 96, 69, 69, 69, 69,
    69, 69, 69, 69, 69, 69, 69, 100, 96, 69, 100, 102, 105, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69,
    69, 69, 69, 96, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 69, 100, 96, 69, 100, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135, 135,
    135, 135, 135, 135, 135, 135, 135, 135, 135, 140, 141, 141, 141, 141, 39, 0, 0,
];
static _use_syllable_machine_cond_targs: [i8; 148] = [
    1, 122, 0, 1, 2, 31, 1, 59, 61, 90, 91, 116, 1, 118, 104, 92, 93, 94, 95, 108, 110, 111, 112,
    113, 105, 106, 107, 99, 100, 101, 119, 120, 121, 114, 96, 97, 98, 126, 115, 1, 3, 4, 1, 17, 5,
    6, 7, 8, 21, 23, 24, 25, 26, 18, 19, 20, 12, 13, 14, 29, 30, 27, 9, 10, 11, 28, 15, 16, 22, 1,
    32, 1, 45, 33, 34, 35, 36, 49, 51, 52, 53, 54, 46, 47, 48, 40, 41, 42, 55, 37, 38, 39, 56, 57,
    58, 43, 1, 44, 1, 50, 1, 1, 1, 60, 1, 1, 1, 62, 63, 76, 64, 65, 66, 67, 80, 82, 83, 84, 85, 77,
    78, 79, 71, 72, 73, 86, 68, 69, 70, 87, 88, 89, 74, 75, 81, 1, 102, 103, 109, 117, 1, 1, 1,
    123, 124, 125, 0, 0,
];
static _use_syllable_machine_cond_actions: [i8; 148] = [
    1, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 9, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 11, 0, 12, 0, 13, 14, 15, 0, 16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 20, 21, 22, 0, 0, 0, 0, 0,
];
static _use_syllable_machine_to_state_actions: [i8; 129] = [
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];
static _use_syllable_machine_from_state_actions: [i8; 129] = [
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0,
];
static _use_syllable_machine_eof_trans: [i16; 129] = [
    1, 4, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40,
    40, 40, 40, 40, 40, 40, 40, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 97, 70, 70, 70, 70,
    70, 70, 70, 70, 70, 70, 70, 101, 97, 70, 101, 103, 106, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70,
    70, 70, 70, 97, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 70, 101, 97, 70, 101, 136, 136, 136,
    136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136, 136,
    136, 136, 136, 136, 136, 136, 136, 136, 136, 141, 142, 142, 142, 142, 40, 0, 0,
];
static use_syllable_machine_start: i32 = 1;
static use_syllable_machine_first_final: i32 = 1;
static use_syllable_machine_error: i32 = -1;
static use_syllable_machine_en_main: i32 = 1;
#[derive(Clone, Copy)]
pub enum SyllableType {
    IndependentCluster,
    ViramaTerminatedCluster,
    SakotTerminatedCluster,
    StandardCluster,
    NumberJoinerTerminatedCluster,
    NumeralCluster,
    SymbolCluster,
    HieroglyphCluster,
    BrokenCluster,
    NonCluster,
}

pub fn find_syllables(buffer: &mut hb_buffer_t) {
    let mut cs = 0;
    let infos = Cell::as_slice_of_cells(Cell::from_mut(&mut buffer.info));
    let p0 = MachineCursor::new(infos, included);
    let mut p = p0;
    let mut ts = p0;
    let mut te = p0;
    let pe = p.end();
    let eof = p.end();
    let mut syllable_serial = 1u8;

    // Please manually replace assignments of 0 to p, ts, and te
    // to use p0 instead

    macro_rules! found_syllable {
        ($kind:expr) => {{
            found_syllable(ts.index(), te.index(), &mut syllable_serial, $kind, infos);
        }};
    }

    {
        cs = (use_syllable_machine_start) as i32;
        ts = p0;
        te = p0;
    }

    {
        let mut _trans = 0;
        let mut _keys: i32 = 0;
        let mut _inds: i32 = 0;
        let mut _ic = 0;
        '_resume: while (p != pe || p == eof) {
            '_again: while (true) {
                match (_use_syllable_machine_from_state_actions[(cs) as usize]) {
                    3 => {
                        ts = p;
                    }

                    _ => {}
                }
                if (p == eof) {
                    {
                        if (_use_syllable_machine_eof_trans[(cs) as usize] > 0) {
                            {
                                _trans =
                                    (_use_syllable_machine_eof_trans[(cs) as usize]) as u32 - 1;
                            }
                        }
                    }
                } else {
                    {
                        _keys = (cs << 1) as i32;
                        _inds = (_use_syllable_machine_index_offsets[(cs) as usize]) as i32;
                        if ((infos[p.index()].get().use_category() as u8) <= 56) {
                            {
                                _ic = (_use_syllable_machine_char_class[((infos[p.index()]
                                    .get()
                                    .use_category()
                                    as u8)
                                    as i32
                                    - 0)
                                    as usize]) as i32;
                                if (_ic
                                    <= (_use_syllable_machine_trans_keys[(_keys + 1) as usize])
                                        as i32
                                    && _ic
                                        >= (_use_syllable_machine_trans_keys[(_keys) as usize])
                                            as i32)
                                {
                                    _trans = (_use_syllable_machine_indices[(_inds
                                        + (_ic
                                            - (_use_syllable_machine_trans_keys[(_keys) as usize])
                                                as i32)
                                            as i32)
                                        as usize])
                                        as u32;
                                } else {
                                    _trans = (_use_syllable_machine_index_defaults[(cs) as usize])
                                        as u32;
                                }
                            }
                        } else {
                            {
                                _trans =
                                    (_use_syllable_machine_index_defaults[(cs) as usize]) as u32;
                            }
                        }
                    }
                }
                cs = (_use_syllable_machine_cond_targs[(_trans) as usize]) as i32;
                if (_use_syllable_machine_cond_actions[(_trans) as usize] != 0) {
                    {
                        match (_use_syllable_machine_cond_actions[(_trans) as usize]) {
                            6 => {
                                te = p + 1;
                            }
                            14 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::ViramaTerminatedCluster);
                                }
                            }
                            12 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::SakotTerminatedCluster);
                                }
                            }
                            10 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            18 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NumberJoinerTerminatedCluster);
                                }
                            }
                            16 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NumeralCluster);
                                }
                            }
                            8 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }
                            22 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::HieroglyphCluster);
                                }
                            }
                            5 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                    buffer.scratch_flags |=
                                        HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                }
                            }
                            4 => {
                                te = p + 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }
                            13 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::ViramaTerminatedCluster);
                                }
                            }
                            11 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SakotTerminatedCluster);
                                }
                            }
                            9 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::StandardCluster);
                                }
                            }
                            17 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumberJoinerTerminatedCluster);
                                }
                            }
                            15 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NumeralCluster);
                                }
                            }
                            7 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }
                            21 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::HieroglyphCluster);
                                }
                            }
                            19 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::BrokenCluster);
                                    buffer.scratch_flags |=
                                        HB_BUFFER_SCRATCH_FLAG_HAS_BROKEN_SYLLABLE;
                                }
                            }
                            20 => {
                                te = p;
                                p = p - 1;
                                {
                                    found_syllable!(SyllableType::NonCluster);
                                }
                            }
                            1 => {
                                p = (te) - 1;
                                {
                                    found_syllable!(SyllableType::SymbolCluster);
                                }
                            }

                            _ => {}
                        }
                    }
                }
                break '_again;
            }
            if (p == eof) {
                {
                    if (cs >= 1) {
                        break '_resume;
                    }
                }
            } else {
                {
                    match (_use_syllable_machine_to_state_actions[(cs) as usize]) {
                        2 => {
                            ts = p0;
                        }

                        _ => {}
                    }
                    p += 1;
                    continue '_resume;
                }
            }
            break '_resume;
        }
    }
}

#[inline]
fn found_syllable(
    start: usize,
    end: usize,
    syllable_serial: &mut u8,
    kind: SyllableType,
    buffer: &[Cell<hb_glyph_info_t>],
) {
    for i in start..end {
        let mut glyph = buffer[i].get();
        glyph.set_syllable((*syllable_serial << 4) | kind as u8);
        buffer[i].set(glyph);
    }

    *syllable_serial += 1;

    if *syllable_serial == 16 {
        *syllable_serial = 1;
    }
}

fn not_ccs_default_ignorable(i: &hb_glyph_info_t) -> bool {
    i.use_category() != category::CGJ
}

fn included(infos: &[Cell<hb_glyph_info_t>], i: usize) -> bool {
    let glyph = infos[i].get();
    if !not_ccs_default_ignorable(&glyph) {
        return false;
    }
    if glyph.use_category() == category::ZWNJ {
        for glyph2 in &infos[i + 1..] {
            if not_ccs_default_ignorable(&glyph2.get()) {
                return !_hb_glyph_info_is_unicode_mark(&glyph2.get());
            }
        }
    }
    true
}
