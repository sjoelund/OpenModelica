// Auto-generated from MetaModelica source
#![allow(warnings)]
#![allow(unreachable_patterns, unreachable_code, non_camel_case_types, non_snake_case, dead_code, unused_imports, unused_variables, non_upper_case_globals, unused_mut)]

use std::sync::Arc;
use anyhow::{Result, bail};
use loop_unwrap::unwrap_break_err;
use metamodelica::*; // Built-in types and functions
use const_str;
use arcstr::{ArcStr, literal, format};

use openmodelica_util::DiffAlgorithm;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::System;

/*
   Template for Lexer Code
   replace keywords:
   %LexerCode
   %time
   %Token
   %Lexer
   %LexTable
   %constant
   %nameSpan
   %functions
   %caseAction
  */
pub const debug: bool = false;

pub mod LexTable {
    use super::*;
    pub const yy_limit: i32 = 395;

    pub const yy_finish: i32 = 453;

    pub static yy_acclist: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![115, 114, 1, 114, 2, 114, 114, 101, 114, 114, 64, 114, 65, 114, 85, 114, 87, 114, 72, 114, 86, 114, 97, 114, 94, 114, 100, 114, 75, 114, 76, 114, 90, 114, 71, 114, 91, 114, 98, 114, 66, 114, 67, 114, 93, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 98, 114, 68, 114, 69, 114, 109, 114, 110, 114, 109, 114, 113, 114, 112, 114, 105, 114, 106, 114, 104, 105, 114, 105, 114, 1, 2, 82, 80, 81, 83, 5, 84, 107, 111, 3, 100, 74, 73, 88, 89, 70, 92, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 34, 98, 98, 36, 98, 98, 98, 98, 98, 46, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 108, 102, 103, 99, 3, 4, 98, 7, 98, 98, 98, 98, 98, 98, 98, 15, 98, 98, 98, 98, 98, 21, 98, 98, 98, 98, 98, 98, 98, 98, 32, 98, 98, 98, 98, 98, 98, 98, 98, 42, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 5, 3, 98, 98, 98, 98, 98, 98, 98, 98, 98, 17, 98, 18, 98, 98, 98, 98, 98, 98, 98, 98, 31, 98, 98, 98, 98, 98, 98, 98, 40, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 77, 98, 98, 98, 98, 98, 98, 98, 56, 98, 57, 98, 58, 98, 59, 98, 98, 98, 98, 98, 9, 98, 63, 98, 10, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 29, 98, 30, 98, 98, 98, 98, 98, 38, 98, 39, 98, 41, 98, 98, 98, 43, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 60, 98, 98, 98, 98, 98, 98, 98, 98, 98, 19, 98, 98, 98, 98, 98, 98, 98, 98, 98, 35, 98, 78, 98, 98, 98, 98, 47, 98, 98, 98, 98, 98, 98, 52, 98, 53, 98, 98, 98, 62, 98, 96, 98, 98, 61, 98, 98, 98, 11, 98, 98, 98, 98, 98, 98, 98, 98, 98, 98, 26, 98, 98, 98, 37, 98, 98, 98, 98, 48, 98, 98, 50, 98, 98, 98, 98, 98, 98, 98, 98, 13, 98, 98, 98, 14, 98, 20, 98, 98, 98, 23, 98, 98, 28, 98, 33, 98, 44, 98, 98, 45, 98, 98, 98, 98, 98, 98, 6, 98, 98, 12, 98, 98, 98, 98, 98, 98, 98, 49, 98, 51, 98, 54, 98, 98, 95, 98, 8, 98, 98, 16, 98, 98, 98, 25, 98, 98, 98, 98, 98, 22, 98, 98, 55, 98, 98, 24, 98, 79, 98, 27, 98].into_iter().cloned().collect()) });

    pub static yy_accept: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 5, 7, 8, 10, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 89, 91, 93, 95, 97, 100, 102, 103, 104, 104, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 141, 142, 144, 145, 146, 147, 148, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 167, 168, 168, 168, 169, 170, 172, 173, 174, 175, 176, 177, 178, 180, 181, 182, 183, 184, 186, 187, 188, 189, 190, 191, 192, 193, 195, 196, 197, 198, 199, 200, 201, 202, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 225, 226, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 238, 240, 241, 242, 243, 244, 245, 246, 247, 249, 250, 251, 252, 253, 254, 255, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 270, 271, 272, 273, 274, 275, 276, 278, 280, 282, 284, 285, 286, 287, 288, 290, 292, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 308, 310, 311, 312, 313, 314, 316, 318, 320, 321, 322, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 339, 340, 341, 342, 343, 344, 345, 346, 347, 349, 350, 351, 352, 353, 354, 355, 356, 357, 359, 361, 362, 363, 364, 366, 367, 368, 369, 370, 371, 373, 375, 376, 377, 379, 381, 382, 384, 385, 386, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 399, 400, 401, 403, 404, 405, 406, 408, 409, 411, 412, 413, 414, 415, 416, 417, 418, 420, 421, 422, 424, 426, 427, 428, 430, 431, 433, 435, 437, 438, 440, 441, 442, 443, 444, 445, 447, 448, 450, 451, 452, 453, 454, 455, 456, 458, 460, 462, 463, 465, 467, 468, 470, 471, 472, 474, 475, 476, 477, 478, 480, 481, 483, 484, 486, 488, 490, 490].into_iter().cloned().collect()) });

    pub static yy_ec: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 1, 1, 1, 1, 2, 3, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 5, 6, 7, 6, 6, 6, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 17, 17, 17, 17, 17, 17, 17, 17, 17, 18, 19, 20, 21, 22, 6, 6, 23, 23, 23, 23, 24, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 25, 26, 27, 28, 23, 1, 29, 30, 31, 32, 33, 34, 35, 36, 37, 23, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 6, 55, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1].into_iter().cloned().collect()) });

    pub static yy_meta: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 2, 2, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 5, 2, 4, 4, 2, 5, 2, 2, 6, 6, 4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 6, 4, 4, 4, 6, 4, 4, 4, 6, 4, 4, 6, 4, 2, 2].into_iter().cloned().collect()) });

    pub static yy_base: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![0, 0, 53, 54, 449, 448, 55, 56, 450, 453, 64, 453, 446, 453, 422, 453, 453, 453, 453, 453, 453, 56, 60, 62, 57, 453, 39, 426, 425, 0, 453, 453, 453, 44, 35, 48, 55, 60, 68, 62, 403, 402, 401, 69, 77, 409, 46, 79, 72, 453, 453, 453, 453, 425, 453, 453, 453, 453, 453, 93, 118, 453, 113, 0, 453, 453, 453, 453, 109, 453, 453, 453, 110, 113, 124, 453, 453, 453, 453, 453, 453, 0, 405, 103, 397, 405, 408, 395, 95, 389, 403, 387, 116, 384, 102, 392, 389, 387, 383, 386, 0, 383, 113, 383, 392, 376, 118, 0, 375, 388, 121, 378, 68, 126, 374, 388, 384, 368, 372, 122, 367, 453, 453, 146, 453, 162, 144, 166, 396, 395, 369, 0, 368, 378, 379, 361, 121, 369, 0, 374, 368, 370, 373, 0, 361, 371, 370, 365, 351, 367, 345, 0, 363, 133, 346, 359, 343, 347, 356, 0, 343, 350, 127, 341, 347, 142, 337, 344, 349, 339, 347, 340, 330, 344, 329, 334, 341, 340, 331, 332, 334, 352, 351, 350, 349, 320, 317, 325, 324, 315, 327, 312, 317, 312, 0, 145, 313, 322, 307, 312, 143, 319, 312, 0, 303, 304, 303, 310, 301, 298, 0, 305, 314, 302, 296, 292, 300, 309, 297, 299, 302, 297, 0, 288, 301, 302, 285, 300, 276, 0, 0, 0, 0, 294, 289, 288, 295, 0, 0, 0, 292, 156, 289, 288, 286, 283, 272, 272, 279, 283, 282, 272, 0, 0, 275, 264, 277, 280, 0, 0, 0, 261, 270, 0, 259, 263, 269, 270, 273, 270, 269, 267, 259, 266, 255, 255, 251, 0, 252, 245, 244, 243, 248, 259, 239, 239, 0, 252, 236, 254, 240, 252, 234, 250, 236, 0, 0, 238, 234, 222, 0, 245, 240, 225, 232, 223, 0, 0, 240, 235, 0, 0, 234, 0, 230, 228, 222, 216, 225, 220, 227, 218, 219, 210, 215, 225, 0, 215, 212, 0, 207, 222, 218, 0, 216, 0, 215, 202, 217, 203, 204, 201, 197, 0, 200, 203, 0, 0, 210, 201, 0, 198, 0, 0, 0, 189, 0, 190, 202, 200, 202, 195, 0, 185, 0, 188, 153, 152, 156, 164, 159, 0, 0, 0, 155, 0, 0, 161, 0, 159, 150, 0, 148, 154, 156, 131, 0, 91, 0, 39, 0, 0, 0, 453, 201, 207, 213, 218, 221, 225].into_iter().cloned().collect()) });

    pub static yy_def: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![394, 1, 395, 395, 396, 396, 397, 397, 394, 394, 394, 394, 394, 394, 398, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 399, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 398, 400, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 394, 394, 394, 394, 394, 394, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 394, 394, 394, 394, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 399, 0, 394, 394, 394, 394, 394, 394].into_iter().cloned().collect()) });

    pub static yy_nxt: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![10, 11, 12, 13, 11, 10, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 30, 31, 10, 32, 33, 34, 35, 36, 37, 38, 39, 30, 30, 40, 30, 41, 42, 43, 44, 45, 30, 46, 47, 48, 30, 30, 49, 30, 30, 30, 50, 51, 53, 53, 58, 58, 78, 79, 59, 59, 54, 54, 61, 65, 66, 61, 67, 71, 68, 69, 85, 76, 72, 73, 77, 74, 86, 60, 60, 83, 70, 84, 75, 87, 89, 91, 88, 393, 90, 115, 116, 75, 101, 96, 168, 92, 123, 93, 102, 103, 94, 97, 111, 98, 120, 121, 99, 95, 107, 169, 108, 117, 100, 109, 110, 124, 61, 125, 112, 61, 118, 113, 69, 127, 73, 138, 74, 119, 392, 126, 128, 132, 129, 75, 129, 64, 139, 130, 126, 128, 133, 147, 75, 143, 144, 148, 155, 161, 165, 123, 156, 179, 157, 170, 171, 180, 215, 127, 191, 391, 145, 162, 166, 192, 128, 172, 216, 219, 124, 173, 182, 206, 182, 128, 184, 183, 184, 207, 245, 185, 251, 283, 390, 389, 252, 220, 388, 387, 386, 385, 384, 246, 383, 382, 381, 380, 379, 284, 52, 52, 52, 52, 52, 52, 55, 55, 55, 55, 55, 55, 57, 57, 57, 57, 57, 57, 63, 378, 63, 63, 63, 82, 377, 82, 63, 376, 63, 63, 375, 374, 373, 372, 371, 370, 369, 368, 367, 366, 365, 364, 363, 362, 361, 360, 359, 358, 357, 356, 355, 354, 353, 352, 351, 350, 349, 348, 347, 346, 345, 344, 343, 342, 341, 340, 339, 338, 337, 336, 335, 334, 333, 332, 331, 330, 329, 328, 327, 326, 325, 324, 323, 322, 321, 320, 319, 318, 317, 316, 315, 314, 313, 312, 311, 310, 309, 308, 307, 306, 305, 304, 303, 302, 301, 300, 299, 298, 297, 296, 295, 294, 293, 292, 291, 290, 289, 288, 287, 286, 285, 282, 281, 280, 279, 278, 277, 276, 275, 274, 273, 272, 271, 270, 269, 268, 267, 266, 265, 264, 263, 262, 261, 260, 259, 258, 257, 256, 255, 254, 253, 250, 249, 248, 247, 244, 243, 242, 241, 240, 239, 238, 237, 236, 185, 185, 183, 183, 235, 234, 233, 232, 231, 230, 229, 228, 227, 226, 225, 224, 223, 222, 221, 218, 217, 214, 213, 212, 211, 210, 209, 208, 205, 204, 203, 202, 201, 200, 199, 198, 197, 196, 195, 194, 193, 190, 189, 188, 187, 186, 130, 130, 181, 178, 177, 176, 175, 174, 167, 164, 163, 160, 159, 158, 154, 153, 152, 151, 150, 149, 146, 142, 141, 140, 137, 136, 135, 134, 131, 122, 114, 106, 105, 104, 81, 80, 64, 62, 394, 56, 56, 9, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394].into_iter().cloned().collect()) });

    pub static yy_chk: std::sync::LazyLock<metamodelica::StaticArray<i32>> = std::sync::LazyLock::new(|| { metamodelica::StaticArray::new(list![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 7, 8, 27, 27, 7, 8, 3, 4, 11, 22, 22, 11, 22, 23, 22, 22, 35, 25, 23, 24, 25, 24, 35, 7, 8, 34, 22, 34, 24, 36, 37, 38, 36, 390, 37, 47, 47, 24, 40, 39, 113, 38, 60, 38, 40, 40, 38, 39, 45, 39, 49, 49, 39, 38, 44, 113, 44, 48, 39, 44, 44, 60, 61, 63, 45, 61, 48, 45, 69, 73, 74, 89, 74, 48, 388, 69, 73, 84, 75, 74, 75, 63, 89, 75, 69, 73, 84, 95, 74, 93, 93, 95, 103, 107, 111, 124, 103, 120, 103, 114, 114, 120, 163, 127, 137, 386, 93, 107, 111, 137, 127, 114, 163, 166, 124, 114, 126, 154, 126, 127, 128, 126, 128, 154, 196, 128, 201, 242, 385, 384, 201, 166, 383, 381, 380, 378, 375, 196, 371, 370, 369, 368, 367, 242, 395, 395, 395, 395, 395, 395, 396, 396, 396, 396, 396, 396, 397, 397, 397, 397, 397, 397, 398, 366, 398, 398, 398, 399, 364, 399, 400, 362, 400, 400, 361, 360, 359, 358, 356, 352, 350, 349, 346, 345, 343, 342, 341, 340, 339, 338, 337, 335, 333, 332, 331, 329, 328, 326, 325, 324, 323, 322, 321, 320, 319, 318, 317, 316, 315, 313, 310, 309, 306, 305, 304, 303, 302, 300, 299, 298, 295, 294, 293, 292, 291, 290, 289, 288, 286, 285, 284, 283, 282, 281, 280, 279, 277, 276, 275, 274, 273, 272, 271, 270, 269, 268, 267, 266, 265, 263, 262, 258, 257, 256, 255, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 241, 237, 236, 235, 234, 229, 228, 227, 226, 225, 224, 222, 221, 220, 219, 218, 217, 216, 215, 214, 213, 212, 210, 209, 208, 207, 206, 205, 203, 202, 200, 199, 198, 197, 194, 193, 192, 191, 190, 189, 188, 187, 186, 185, 184, 183, 182, 181, 180, 179, 178, 177, 176, 175, 174, 173, 172, 171, 170, 169, 168, 167, 165, 164, 162, 161, 159, 158, 157, 156, 155, 153, 151, 150, 149, 148, 147, 146, 145, 143, 142, 141, 140, 138, 136, 135, 134, 133, 131, 130, 129, 121, 119, 118, 117, 116, 115, 112, 110, 109, 106, 105, 104, 102, 100, 99, 98, 97, 96, 94, 92, 91, 90, 88, 87, 86, 85, 83, 54, 46, 43, 42, 41, 29, 28, 15, 13, 9, 6, 5, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394, 394].into_iter().cloned().collect()) });

}

pub fn scan(mut fileName: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut contents: ArcStr = arcstr::literal!("");
    contents = (System::readFile((fileName.clone()).clone())?).clone();
    (tokens, errorTokens) = lex((fileName.clone()).clone(), (contents.clone()).clone())?;
    Ok((tokens, errorTokens))
}

pub fn scanString(mut fileSource: ArcStr, mut fileName: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    (tokens, errorTokens) = lex((fileName.clone()).clone(), (fileSource.clone()).clone())?;
    Ok((tokens, errorTokens))
}

pub fn action(mut act: i32, mut startSt: i32, mut mm_currSt: i32, mut mm_pos: i32, mut mm_sPos: i32, mut mm_ePos: i32, mut mm_linenr: i32, mut lineNrStart: i32, mut buffer: i32, mut fileNm: ArcStr, mut fileContents: ArcStr, mut inErrorTokens: Arc<metamodelica::List<Token>>) -> Result<(Token, i32, i32, Arc<metamodelica::List<Token>>)> {
    let mut token: Token = <Token as ::std::default::Default>::default();
    let mut mm_startSt: i32 = 0;
    let mut bufferRet: i32 = 0;
    let mut errorTokens: Arc<metamodelica::List<Token>> = inErrorTokens.clone();
    mm_startSt = startSt.clone();
    bufferRet = 0;
    token = (match act.clone() {
        1 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::WHITESPACE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        2 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::NEWLINE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        3 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::UNSIGNED_REAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        4 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::UNSIGNED_REAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        5 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::UNSIGNED_REAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        6 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ALGORITHM.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        7 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::AND.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        8 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ANNOTATION.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        9 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::BLOCK.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        10 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::CLASS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        11 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::CONNECT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        12 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::CONNECTOR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        13 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::CONSTANT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        14 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::DISCRETE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        15 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::DER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        16 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::DEFINEUNIT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        17 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EACH.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        18 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ELSE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        19 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ELSEIF.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        20 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ELSEWHEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        21 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::END.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        22 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ENUMERATION.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        23 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EQUATION.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        24 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ENCAPSULATED.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        25 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EXPANDABLE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        26 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EXTENDS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        27 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::CONSTRAINEDBY.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        28 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EXTERNAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        29 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FALSE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        30 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FINAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        31 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FLOW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        32 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FOR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        33 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::FUNCTION.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        34 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IF.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        35 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IMPORT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        36 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        37 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::INITIAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        38 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::INNER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        39 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::INPUT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        40 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LOOP.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        41 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::MODEL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        42 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::NOT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        43 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OUTER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        44 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OPERATOR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        45 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OVERLOAD.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        46 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        47 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OUTPUT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        48 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PACKAGE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        49 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PARAMETER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        50 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PARTIAL.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        51 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PROTECTED.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        52 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PUBLIC.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        53 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::RECORD.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        54 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::REDECLARE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        55 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::REPLACEABLE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        56 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::THEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        57 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::TRUE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        58 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::TYPE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        59 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::WHEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        60 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::WHILE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        61 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::WITHIN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        62 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::RETURN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        63 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::BREAK.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        64 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LPAR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        65 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::RPAR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        66 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LBRACK.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        67 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::RBRACK.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        68 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LBRACE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        69 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::RBRACE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        70 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EQEQ.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        71 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::EQUALS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        72 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::COMMA.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        73 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::ASSIGN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        74 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::COLONCOLON.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        75 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::COLON.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        76 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::SEMICOLON.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        77 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PURE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        78 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IMPURE.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        79 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::OPTIMIZATION.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        80 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PLUS_EW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        81 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::MINUS_EW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        82 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STAR_EW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        83 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::SLASH_EW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        84 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::POWER_EW.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        85 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STAR.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        86 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::MINUS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        87 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::PLUS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        88 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LESSEQ.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        89 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LESSGT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        90 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LESS.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        91 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::GREATER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        92 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::GREATEREQ.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        93 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::POWER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        94 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::SLASH.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        95 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::SUBTYPEOF.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        96 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STREAM.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        97 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::DOT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        98 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IDENT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        99 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::IDENT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        100 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::UNSIGNED_INTEGER.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        101 => {
            mm_startSt = 7;
            bufferRet = buffer.clone();
            noToken.clone()
        },
        102 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        103 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        104 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            mm_startSt = 1;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::STRING.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        105 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        106 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        107 => {
            mm_startSt = 3;
            bufferRet = buffer.clone();
            noToken.clone()
        },
        108 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            mm_startSt = 1;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::BLOCK_COMMENT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        109 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        110 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        111 => {
            mm_startSt = 5;
            bufferRet = buffer.clone();
            noToken.clone()
        },
        112 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            mm_startSt = 1;
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::LINE_COMMENT.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            tok.clone()
        },
        113 => {
            bufferRet = buffer.clone();
            noToken.clone()
        },
        114 => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            errorTokens = cons(tok.clone(), errorTokens.clone());
            noToken.clone()
        },
        _ => {
            let mut tok: Token = <Token as ::std::default::Default>::default();
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLexer unknown rule, action=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
            tok = Token { fileName: (fileNm.clone()).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (fileContents.clone()).clone(), byteOffset: mm_pos.clone() - buffer.clone(), length: buffer.clone(), lineNumberStart: lineNrStart.clone(), columnNumberStart: mm_ePos.clone() + 1, lineNumberEnd: mm_linenr.clone(), columnNumberEnd: mm_sPos.clone() + 1 };
            println!("{}", (printToken(tok.clone())?).clone());
            bail!("fail")
        },
    });
    Ok((token, mm_startSt, bufferRet, errorTokens))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum TokenId {
    _NO_TOKEN = 1,
    ALGORITHM = 2,
    AND = 3,
    ANNOTATION = 4,
    ASSIGN = 5,
    BLOCK = 6,
    BLOCK_COMMENT = 7,
    BREAK = 8,
    CLASS = 9,
    COLON = 10,
    COLONCOLON = 11,
    COMMA = 12,
    CONNECT = 13,
    CONNECTOR = 14,
    CONSTANT = 15,
    CONSTRAINEDBY = 16,
    DEFINEUNIT = 17,
    DER = 18,
    DISCRETE = 19,
    DOT = 20,
    EACH = 21,
    ELSE = 22,
    ELSEIF = 23,
    ELSEWHEN = 24,
    ENCAPSULATED = 25,
    END = 26,
    ENUMERATION = 27,
    EQEQ = 28,
    EQUALS = 29,
    EQUATION = 30,
    EXPANDABLE = 31,
    EXTENDS = 32,
    EXTERNAL = 33,
    FALSE = 34,
    FINAL = 35,
    FLOW = 36,
    FOR = 37,
    FUNCTION = 38,
    GREATER = 39,
    GREATEREQ = 40,
    IDENT = 41,
    IF = 42,
    IMPORT = 43,
    IMPURE = 44,
    IN = 45,
    INITIAL = 46,
    INNER = 47,
    INPUT = 48,
    LBRACE = 49,
    LBRACK = 50,
    LESS = 51,
    LESSEQ = 52,
    LESSGT = 53,
    LINE_COMMENT = 54,
    LOOP = 55,
    LPAR = 56,
    MINUS = 57,
    MINUS_EW = 58,
    MODEL = 59,
    MODELICA = 60,
    NEWLINE = 61,
    NOT = 62,
    OPERATOR = 63,
    OPTIMIZATION = 64,
    OR = 65,
    OUTER = 66,
    OUTPUT = 67,
    OVERLOAD = 68,
    PACKAGE = 69,
    PARAMETER = 70,
    PARTIAL = 71,
    PLUS = 72,
    PLUS_EW = 73,
    POWER = 74,
    POWER_EW = 75,
    PROTECTED = 76,
    PUBLIC = 77,
    PURE = 78,
    RBRACE = 79,
    RBRACK = 80,
    RECORD = 81,
    REDECLARE = 82,
    REPLACEABLE = 83,
    RETURN = 84,
    RPAR = 85,
    SEMICOLON = 86,
    SLASH = 87,
    SLASH_EW = 88,
    STAR = 89,
    STAR_EW = 90,
    STREAM = 91,
    STRING = 92,
    SUBTYPEOF = 93,
    THEN = 94,
    TRUE = 95,
    TYPE = 96,
    UNSIGNED_INTEGER = 97,
    UNSIGNED_REAL = 98,
    WHEN = 99,
    WHILE = 100,
    WHITESPACE = 101,
    WITHIN = 102,
}
impl PartialOrd for TokenId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for TokenId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { (*self as i32).cmp(&(*other as i32)) }
}
impl Default for TokenId {
    fn default() -> Self { Self::_NO_TOKEN }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    pub fileName: ArcStr,
    pub id: TokenId,
    pub fileContents: ArcStr,
    pub byteOffset: i32,
    pub length: i32,
    pub lineNumberStart: i32,
    pub columnNumberStart: i32,
    pub lineNumberEnd: i32,
    pub columnNumberEnd: i32,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            fileName: Default::default(),
            id: Default::default(),
            fileContents: Default::default(),
            byteOffset: Default::default(),
            length: Default::default(),
            lineNumberStart: Default::default(),
            columnNumberStart: Default::default(),
            lineNumberEnd: Default::default(),
            columnNumberEnd: Default::default(),
        }
    }
}

pub type TOKEN = Token;


pub static noToken: std::sync::LazyLock<Token> = std::sync::LazyLock::new(|| { Token { fileName: (literal!("<NoFile>")).clone(), id: TokenId::_NO_TOKEN.clone(), fileContents: (literal!("")).clone(), byteOffset: 0, length: 0, lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0 } });

pub fn printToken(mut token: Token) -> Result<ArcStr> {
    let mut strTk: ArcStr = arcstr::literal!("");
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut contents: ArcStr = arcstr::literal!("");
    let mut byteOffset: i32 = 0;
    let mut length: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, id: __pa3, .. } = (token.clone()) else { bail!("pattern mismatch") };
    length = __pa0.clone();
    byteOffset = __pa1.clone();
    contents = __pa2.clone();
    id = __pa3.clone();
    contents = (if (length.clone() > 0) {substring((contents.clone()).clone(), byteOffset.clone(), byteOffset.clone() + length.clone() - 1)?} else {literal!("")}).clone();
    strTk = ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("[TOKEN:")); __mm_s.push_str(&*ArcStr::from(::std::format!("{:?}", id.clone()))); __mm_s.push_str(&*literal!(" '")); __mm_s.push_str(&*contents.clone()); __mm_s.push_str(&*literal!("' (")); __mm_s.push_str(&*intString(token.lineNumberStart.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(token.columnNumberStart.clone())); __mm_s.push_str(&*literal!("-")); __mm_s.push_str(&*intString(token.lineNumberEnd.clone())); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*intString(token.columnNumberEnd.clone())); __mm_s.push_str(&*literal!(")]")); ArcStr::from(__mm_s) }).clone();
    Ok(strTk)
}

pub fn tokenContent(mut token: Token) -> Result<ArcStr> {
    let mut contents: ArcStr = arcstr::literal!("");
    let mut byteOffset: i32 = 0;
    let mut length: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, .. } = (token.clone()) else { bail!("pattern mismatch") };
    length = __pa0.clone();
    byteOffset = __pa1.clone();
    contents = __pa2.clone();
    contents = (if (length.clone() > 0) {substring((contents.clone()).clone(), byteOffset.clone(), byteOffset.clone() + length.clone() - 1)?} else {literal!("")}).clone();
    Ok(contents)
}

pub fn tokenContentEq(mut token1: Token, mut token2: Token) -> Result<bool> {
    let mut b: bool = false;
    let mut contents1: ArcStr = arcstr::literal!("");
    let mut contents2: ArcStr = arcstr::literal!("");
    let mut offset1: i32 = 0;
    let mut length1: i32 = 0;
    let mut offset2: i32 = 0;
    let mut length2: i32 = 0;
    let Token { length: __pa0, byteOffset: __pa1, fileContents: __pa2, .. } = (token1.clone()) else { bail!("pattern mismatch") };
    length1 = __pa0.clone();
    offset1 = __pa1.clone();
    contents1 = __pa2.clone();
    let Token { length: __pa3, byteOffset: __pa4, fileContents: __pa5, .. } = (token2.clone()) else { bail!("pattern mismatch") };
    length2 = __pa3.clone();
    offset2 = __pa4.clone();
    contents2 = __pa5.clone();
    b = if (length1.clone() != length2.clone()) {false} else {0 == System::strcmp_offset((contents1.clone()).clone(), offset1.clone(), length1.clone(), (contents2.clone()).clone(), offset2.clone(), length2.clone())};
    Ok(b)
}

pub fn tokenSourceInfo(mut token: Token) -> Result<SourceInfo> {
    let mut info: SourceInfo = <SourceInfo as ::std::default::Default>::default();
    info = { let mut t = token.clone(); (match t.clone() {
        Token { .. } => SourceInfo { fileName: (t.fileName.clone()).clone(), isReadOnly: false, lineNumberStart: t.lineNumberStart.clone(), columnNumberStart: t.columnNumberStart.clone(), lineNumberEnd: t.lineNumberEnd.clone(), columnNumberEnd: t.columnNumberEnd.clone(), lastModification: metamodelica::OrderedFloat(0.0_f64) },
        _ => bail!("match: no arm matched"),
    }) };
    Ok(info)
}

fn lex(mut fileName: ArcStr, mut contents: ArcStr) -> Result<(Arc<metamodelica::List<Token>>, Arc<metamodelica::List<Token>>)> {
    let mut tokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut startSt: i32 = 0;
    let mut i: i32 = 0;
    let mut cTok: i32 = 0;
    let mut currSt: i32 = 0;
    let mut pos: i32 = 0;
    let mut sPos: i32 = 0;
    let mut ePos: i32 = 0;
    let mut linenr: i32 = 0;
    let mut contentLen: i32 = 0;
    let mut numBacktrack: i32 = 0;
    let mut buffer: i32 = 0;
    let mut lineNrStart: i32 = 0;
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    startSt = 1;
    currSt = 1;
    pos = 1;
    sPos = 0;
    ePos = 0;
    linenr = 1;
    lineNrStart = 1;
    buffer = 0;
    states = metamodelica::nil();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nLexer analyzer LexerCode...")); __mm_s.push_str(&*fileName.clone()); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
    }
    tokens = metamodelica::nil();
    if debug.clone() {
        println!("{}", (literal!("\n TOTAL Chars:")).clone());
        println!("{}", (intString(((contents.clone()).clone().len() as i32))).clone());
    }
    contentLen = ((contents.clone()).clone().len() as i32);
    i = 1;
    while i.clone() <= contentLen.clone() {
        cTok = stringGet((contents.clone()).clone(),i.clone())?;
        (tokens, numBacktrack, startSt, currSt, pos, sPos, ePos, linenr, lineNrStart, buffer, states, errorTokens) = consume(cTok.clone(), tokens.clone(), (contents.clone()).clone(), startSt.clone(), currSt.clone(), pos.clone(), sPos.clone(), ePos.clone(), linenr.clone(), lineNrStart.clone(), buffer.clone(), states.clone(), (fileName.clone()).clone(), errorTokens.clone())?;
        i = i.clone() - numBacktrack.clone() + 1;
    }
    tokens = tokens.clone().reverse();
    errorTokens = errorTokens.clone().reverse();
    Ok((tokens, errorTokens))
}

fn consume(mut cp: i32, mut tokens: Arc<metamodelica::List<Token>>, mut fileContents: ArcStr, mut startSt: i32, mut currSt: i32, mut pos: i32, mut sPos: i32, mut ePos: i32, mut linenr: i32, mut inLineNrStart: i32, mut inBuffer: i32, mut inStates: Arc<metamodelica::List<i32>>, mut fileName: ArcStr, mut inErrorTokens: Arc<metamodelica::List<Token>>) -> Result<(Arc<metamodelica::List<Token>>, i32, i32, i32, i32, i32, i32, i32, i32, i32, Arc<metamodelica::List<i32>>, Arc<metamodelica::List<Token>>)> {
    let mut resToken: Arc<metamodelica::List<Token>> = metamodelica::nil();
    let mut bkBuffer: i32 = 0;
    let mut mm_startSt: i32 = 0;
    let mut mm_currSt: i32 = 0;
    let mut mm_pos: i32 = 0;
    let mut mm_sPos: i32 = 0;
    let mut mm_ePos: i32 = 0;
    let mut mm_linenr: i32 = 0;
    let mut lineNrStart: i32 = 0;
    let mut buffer: i32 = 0;
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut errorTokens: Arc<metamodelica::List<Token>> = inErrorTokens.clone();
    let mut tok: Token = <Token as ::std::default::Default>::default();
    let mut act: i32 = 0;
    let mut buffer2: i32 = 0;
    let mut c: i32 = 0;
    let mut baseCond: i32 = 0;
    mm_startSt = startSt.clone();
    mm_currSt = currSt.clone();
    mm_pos = pos.clone();
    mm_sPos = sPos.clone();
    mm_ePos = ePos.clone();
    mm_linenr = linenr.clone();
    lineNrStart = inLineNrStart.clone();
    buffer = inBuffer.clone();
    states = inStates.clone();
    baseCond = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nPROGRAM:{")); __mm_s.push_str(&*intString(cp.clone())); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nBUFFER:{")); __mm_s.push_str(&*intString(buffer.clone())); __mm_s.push_str(&*literal!("} ")); ArcStr::from(__mm_s) }).clone());
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("base:")); __mm_s.push_str(&*intString(baseCond.clone())); __mm_s.push_str(&*literal!(" st:")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!(" ")); ArcStr::from(__mm_s) }).clone());
    }
    buffer = buffer.clone() + 1;
    mm_pos = mm_pos.clone() + 1;
    if cp.clone() == 10 {
        mm_linenr = mm_linenr.clone() + 1;
        mm_sPos = 0;
    } else {
        mm_sPos = mm_sPos.clone() + 1;
    }
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[Reading:'")); __mm_s.push_str(&*intStringChar(cp.clone())); __mm_s.push_str(&*literal!("' at p:")); __mm_s.push_str(&*intString(mm_pos.clone() - 1)); __mm_s.push_str(&*literal!(" line:")); __mm_s.push_str(&*intString(mm_linenr.clone())); __mm_s.push_str(&*literal!(" rPos:")); __mm_s.push_str(&*intString(mm_sPos.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    c = LexTable::yy_ec.borrow()[(cp.clone()-1) as usize].clone();
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" evalState Before[c")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(",s")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    (mm_currSt, c) = evalState(mm_currSt.clone(), c.clone());
    if debug.clone() == true {
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!(" After[c")); __mm_s.push_str(&*intString(c.clone())); __mm_s.push_str(&*literal!(",s")); __mm_s.push_str(&*intString(mm_currSt.clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
    }
    if mm_currSt.clone() > 0 {
        mm_currSt = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
        mm_currSt = LexTable::yy_nxt.borrow()[(mm_currSt.clone() + c.clone()-1) as usize].clone();
    } else {
        mm_currSt = LexTable::yy_nxt.borrow()[(c.clone()-1) as usize].clone();
    }
    states = cons(mm_currSt.clone(), states.clone());
    baseCond = LexTable::yy_base.borrow()[(mm_currSt.clone()-1) as usize].clone();
    if baseCond.clone() == LexTable::yy_finish.clone() {
        if debug.clone() == true {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[RESTORE=")); __mm_s.push_str(&*intString(LexTable::yy_accept.borrow()[(mm_currSt.clone()-1) as usize].clone())); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) }).clone());
        }
        (act, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule((fileContents.clone()).clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), buffer.clone(), bkBuffer.clone(), states.clone())?;
        if debug.clone() == true {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\nFound rule: ")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", act.clone()))); ArcStr::from(__mm_s) }).clone());
        }
        (tok, mm_startSt, buffer2, errorTokens) = action(act.clone(), mm_startSt.clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), lineNrStart.clone(), buffer.clone(), (fileName.clone()).clone(), (fileContents.clone()).clone(), errorTokens.clone())?;
        if debug.clone() == true {
            println!("{}", (literal!("\nDid action")).clone());
        }
        mm_currSt = mm_startSt.clone();
        states = metamodelica::nil();
        if buffer.clone() != buffer2.clone() {
            mm_ePos = mm_sPos.clone();
            lineNrStart = linenr.clone();
        }
        buffer = buffer2.clone();
        resToken = (match tok.clone() {
        Token { id: TokenId::_NO_TOKEN, .. } => tokens.clone(),
        _ => cons(tok.clone(), tokens.clone()),
    });
        if debug.clone() {
            println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n CountTokens:")); __mm_s.push_str(&*intString((resToken.clone().len() as i32))); ArcStr::from(__mm_s) }).clone());
        }
    } else {
        bkBuffer = 0;
        resToken = tokens.clone();
    }
    Ok((resToken, bkBuffer, mm_startSt, mm_currSt, mm_pos, mm_sPos, mm_ePos, mm_linenr, lineNrStart, buffer, states, errorTokens))
}

fn findRule(mut fileContents: ArcStr, mut currSt: i32, mut pos: i32, mut sPos: i32, mut mm_ePos: i32, mut linenr: i32, mut inBuffer: i32, mut inBkBuffer: i32, mut inStates: Arc<metamodelica::List<i32>>) -> Result<(i32, i32, i32, i32, i32, i32, i32, Arc<metamodelica::List<i32>>)> {
    let mut action: i32 = 0;
    let mut mm_currSt: i32 = 0;
    let mut mm_pos: i32 = 0;
    let mut mm_sPos: i32 = 0;
    let mut mm_linenr: i32 = 0;
    let mut buffer: i32 = 0;
    let mut bkBuffer: i32 = 0;
    let mut states: Arc<metamodelica::List<i32>> = metamodelica::nil();
    let mut lp: i32 = 0;
    let mut lp1: i32 = 0;
    let mut stCmp: i32 = 0;
    let mut cp: i32 = 0;
    let mut st: bool = false;
    mm_currSt = currSt.clone();
    mm_pos = pos.clone();
    mm_sPos = sPos.clone();
    mm_linenr = linenr.clone();
    buffer = inBuffer.clone();
    bkBuffer = inBkBuffer.clone();
    states = inStates.clone();
    stCmp = (states.clone()).get(1)?;
    lp = LexTable::yy_accept.borrow()[(stCmp.clone()-1) as usize].clone();
    lp1 = LexTable::yy_accept.borrow()[(stCmp.clone() + 1-1) as usize].clone();
    st = intGt(lp.clone(), 0) && intLt(lp.clone(), lp1.clone());
    if st.clone() {
        if debug.clone() {
            checkArrayModelica(LexTable::yy_accept.clone(), stCmp.clone(), metamodelica::sourceInfo!())?;
            checkArrayModelica(LexTable::yy_acclist.clone(), lp.clone(), metamodelica::sourceInfo!())?;
        }
        lp = LexTable::yy_accept.borrow()[(stCmp.clone()-1) as usize].clone();
        action = LexTable::yy_acclist.borrow()[(lp.clone()-1) as usize].clone();
    } else {
        cp = stringGet((fileContents.clone()).clone(),mm_pos.clone() - 1)?;
        buffer = buffer.clone() - 1;
        bkBuffer = bkBuffer.clone() + 1;
        mm_pos = mm_pos.clone() - 1;
        mm_sPos = mm_sPos.clone() - 1;
        if cp.clone() == 10 {
            mm_sPos = mm_ePos.clone();
            mm_linenr = mm_linenr.clone() - 1;
        }
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(states.clone()) {
            Deref @ metamodelica::List::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => bail!("pattern mismatch"),
        } };
        mm_currSt = __pa0.clone();
        states = __pa1.clone();
        (action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states) = findRule((fileContents.clone()).clone(), mm_currSt.clone(), mm_pos.clone(), mm_sPos.clone(), mm_ePos.clone(), mm_linenr.clone(), buffer.clone(), bkBuffer.clone(), states.clone())?;
    }
    Ok((action, mm_currSt, mm_pos, mm_sPos, mm_linenr, buffer, bkBuffer, states))
}

fn evalState(mut cState: i32, mut c: i32) -> (i32, i32) {
    let mut new_state: i32 = 0;
    let mut new_c: i32 = 0;
    let mut cState1: i32 = cState.clone();
    let mut c1: i32 = c.clone();
    let mut val: i32 = 0;
    let mut val2: i32 = 0;
    let mut chk: i32 = 0;
    chk = LexTable::yy_base.borrow()[(cState1.clone()-1) as usize].clone();
    chk = chk.clone() + c1.clone();
    val = LexTable::yy_chk.borrow()[(chk.clone()-1) as usize].clone();
    val2 = LexTable::yy_base.borrow()[(cState1.clone()-1) as usize].clone() + c1.clone();
    if cState1.clone() != val.clone() {
        cState1 = LexTable::yy_def.borrow()[(cState1.clone()-1) as usize].clone();
        if cState1.clone() >= LexTable::yy_limit.clone() {
            c1 = LexTable::yy_meta.borrow()[(c1.clone()-1) as usize].clone();
        }
        if cState1.clone() > 0 {
            (cState1, c1) = evalState(cState1.clone(), c1.clone());
        }
    }
    new_state = cState1.clone();
    new_c = c1.clone();
    (new_state, new_c)
}

fn checkArray<T: Clone + 'static>(mut arr: metamodelica::Array<T>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr = arcstr::literal!("");
    let mut lineStart: i32 = 0;
    if index.clone() < 1 || index.clone() > (arr.clone().borrow().len() as i32) {
        let SourceInfo { lineNumberStart: __pa0, fileName: __pa1, .. } = (info.clone()) else { bail!("pattern mismatch") };
        lineStart = __pa0.clone();
        filename = __pa1.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart.clone()))); __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (arr.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" index=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        bail!("fail");
    }
    Ok(())
}

fn checkArrayModelica(mut arr: metamodelica::Array<i32>, mut index: i32, mut info: SourceInfo) -> Result<()> {
    let mut filename: ArcStr = arcstr::literal!("");
    let mut lineStart: i32 = 0;
    if index.clone() < 1 || index.clone() > (arr.clone().borrow().len() as i32) {
        let SourceInfo { lineNumberStart: __pa0, fileName: __pa1, .. } = (info.clone()) else { bail!("pattern mismatch") };
        lineStart = __pa0.clone();
        filename = __pa1.clone();
        println!("{}", ({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n[")); __mm_s.push_str(&*filename.clone()); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", lineStart.clone()))); __mm_s.push_str(&*literal!("]: checkArray failed: arrayLength=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", (arr.clone().borrow().len() as i32)))); __mm_s.push_str(&*literal!(" index=")); __mm_s.push_str(&*ArcStr::from(::std::format!("{}", index.clone()))); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }).clone());
        bail!("fail");
    }
    Ok(())
}

pub fn modelicaDiffTokenEq(mut ta: Token, mut tb: Token) -> Result<bool> {
    let mut b: bool = false;
    let mut ida: TokenId = TokenId::_NO_TOKEN;
    let mut idb: TokenId = TokenId::_NO_TOKEN;
    let TOKEN { id: __pa0, .. } = (ta.clone()) else { bail!("pattern mismatch") };
    ida = __pa0.clone();
    let TOKEN { id: __pa1, .. } = (tb.clone()) else { bail!("pattern mismatch") };
    idb = __pa1.clone();
    if ida.clone() != idb.clone() {
        b = false;
        return Ok(b.clone());
    }
    b = (match ida.clone() {
        TokenId::IDENT { .. } => tokenContentEq(ta.clone(), tb.clone())?,
        TokenId::UNSIGNED_INTEGER => tokenContentEq(ta.clone(), tb.clone())?,
        TokenId::UNSIGNED_REAL => stringReal((tokenContent(ta.clone())?).clone())? == stringReal((tokenContent(tb.clone())?).clone())?,
        TokenId::BLOCK_COMMENT => blockCommentCanonical(ta.clone())? == blockCommentCanonical(tb.clone())?,
        TokenId::LINE_COMMENT => tokenContentEq(ta.clone(), tb.clone())?,
        TokenId::STRING { .. } => {
            b = tokenContentEq(ta.clone(), tb.clone())?;
            if !(b.clone()) {
                b = if (0 != StringUtil::findChar((tokenContent(ta.clone())?).clone(), stringCharInt((literal!("\n")).clone())?, 1, 0)) {blockCommentCanonical(ta.clone())? == blockCommentCanonical(tb.clone())?} else {false};
            }
            b.clone()
        },
        TokenId::WHITESPACE => true,
        _ => true,
    });
    Ok(b)
}

pub fn modelicaDiffTokenWhitespace(mut t: Token) -> Result<bool> {
    let mut b: bool = false;
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let TOKEN { id: __pa0, .. } = (t.clone()) else { bail!("pattern mismatch") };
    id = __pa0.clone();
    b = id.clone() == TokenId::BLOCK_COMMENT.clone() || id.clone() == TokenId::LINE_COMMENT.clone() || id.clone() == TokenId::WHITESPACE.clone() || id.clone() == TokenId::NEWLINE.clone();
    Ok(b)
}

pub fn filterModelicaDiff(mut diffs: Arc<metamodelica::List<(DiffAlgorithm::Diff, Arc<metamodelica::List<Token>>)>>, mut removeWhitespace: bool) -> Result<Arc<metamodelica::List<(DiffAlgorithm::Diff, Arc<metamodelica::List<Token>>)>>> {
    use openmodelica_util::DiffAlgorithm::Diff;
    let mut odiffs: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Token>>)>> = metamodelica::nil();
    let mut addedLineComments: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut removedLineComments: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    let mut addedBlockComments: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut removedBlockComments: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
    let mut simpleDiff: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
    let mut tmp: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
    let mut rest: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
    let mut lastIsNewline: bool = false;
    let mut depth: i32 = 0;
    let () = (::match_deref::match_deref! { match &(diffs.clone()) {
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Nil } => {
            odiffs = diffs.clone();
            return Ok(odiffs.clone());
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    odiffs = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Token>>)>> = metamodelica::nil();
        for mut e in (diffs.clone()).into_iter().cloned() {
            if !((::match_deref::match_deref! { match &(e.clone()) {
        (DiffAlgorithm::Diff::Add, Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, .. }, tail: Deref @ metamodelica::List::Nil }) => !(removeWhitespace.clone()),
        (DiffAlgorithm::Diff::Add, Deref @ metamodelica::List::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil }) => !(removeWhitespace.clone()),
        (_, Deref @ metamodelica::List::Nil) => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })) { continue; }
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        (DiffAlgorithm::Diff::Delete, ts @ Deref @ metamodelica::List::Cons { head: Token { id: TokenId::WHITESPACE, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            (Diff::Equal.clone(), ts.clone())
        },
        (DiffAlgorithm::Diff::Delete, ts @ Deref @ metamodelica::List::Cons { head: Token { id: TokenId::NEWLINE, .. }, tail: Deref @ metamodelica::List::Nil }) => {
            (Diff::Equal.clone(), ts.clone())
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc
    });
    simpleDiff = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
        for mut e in (odiffs.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(e.clone()) {
        (DiffAlgorithm::Diff::Add, ts) => {
            ({
        let mut __acc: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
        for mut t in (ts.clone()).into_iter().cloned() {
            let __x = (Diff::Add.clone(), t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (DiffAlgorithm::Diff::Equal, ts) => {
            ({
        let mut __acc: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
        for mut t in (ts.clone()).into_iter().cloned() {
            let __x = (Diff::Equal.clone(), t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (DiffAlgorithm::Diff::Delete, ts) => {
            ({
        let mut __acc: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
        for mut t in (ts.clone()).into_iter().cloned() {
            let __x = (Diff::Delete.clone(), t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        _ => bail!("match: no arm matched"),
    } });
            __acc = __x.append(&__acc);
        }
        __acc
    });
    tmp = metamodelica::nil();
    lastIsNewline = false;
    depth = 2;
    while !(simpleDiff.clone().is_empty()) {
        (lastIsNewline, simpleDiff, tmp) = (::match_deref::match_deref! { match &(simpleDiff.clone()) {
        Deref @ metamodelica::List::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, t1 @ Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, t2 @ Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::List::Cons { head: e2 @ (DiffAlgorithm::Diff::Equal, _), tail: rest } } } } => {
            (false, cons(e1.clone(), cons((Diff::Equal.clone(), t1.clone()), cons((Diff::Equal.clone(), t2.clone()), cons(e2.clone(), rest.clone())))), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, _), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Delete, t @ Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::List::Cons { head: e2 @ (DiffAlgorithm::Diff::Equal, _), tail: rest } } } => {
            (false, cons(e1.clone(), cons((Diff::Equal.clone(), t.clone()), cons(e2.clone(), rest.clone()))), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: e1 @ (DiffAlgorithm::Diff::Equal, Token { id: t3, .. }), tail: rest } if (t3.clone() != TokenId::WHITESPACE.clone() && t3.clone() != TokenId::NEWLINE.clone() && (deleteWhitespaceFollowedByEqualNonWhitespace(rest.clone())?).0) => {
            let mut rest = (*rest).clone();
            (_, rest) = deleteWhitespaceFollowedByEqualNonWhitespace(rest.clone())?;
            (false, cons(e1.clone(), rest.clone()), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t4, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t5, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone()) && (t5.clone() == TokenId::NEWLINE.clone() || t5.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t4, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), rest.clone()), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (d4, tk4 @ Token { id: t4, .. }), tail: Deref @ metamodelica::List::Cons { head: (d5, tk5 @ Token { id: t5, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (d4.clone() == Diff::Equal.clone() || d4.clone() == Diff::Delete.clone()) && (d5.clone() == Diff::Equal.clone() || d5.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone()) && (t5.clone() == TokenId::NEWLINE.clone() || t5.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), cons((Diff::Equal.clone(), tk3.clone()), cons((Diff::Equal.clone(), tk4.clone()), cons((Diff::Equal.clone(), tk5.clone()), rest.clone())))), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (d4, tk4 @ Token { id: t4, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (d4.clone() == Diff::Equal.clone() || d4.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone()) && (t4.clone() == TokenId::NEWLINE.clone() || t4.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), cons((Diff::Equal.clone(), tk3.clone()), cons((Diff::Equal.clone(), tk4.clone()), rest.clone()))), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (d1, t1), tail: Deref @ metamodelica::List::Cons { head: (d3, tk3 @ Token { id: t3, .. }), tail: Deref @ metamodelica::List::Cons { head: (d2, t2), tail: rest } } } if ((d1.clone() == Diff::Add.clone() && d2.clone() == Diff::Delete.clone() || d2.clone() == Diff::Add.clone() && d1.clone() == Diff::Delete.clone()) && modelicaDiffTokenEq(t1.clone(), t2.clone())? && (d3.clone() == Diff::Equal.clone() || d3.clone() == Diff::Delete.clone()) && (t3.clone() == TokenId::NEWLINE.clone() || t3.clone() == TokenId::WHITESPACE.clone())) => {
            (false, cons((Diff::Equal.clone(), t1.clone()), cons((Diff::Equal.clone(), tk3.clone()), rest.clone())), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: rest @ Deref @ metamodelica::List::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } } => {
            (false, rest.clone(), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: rest @ Deref @ metamodelica::List::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } => {
            (false, rest.clone(), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: e @ (_, Token { id: TokenId::NEWLINE, .. }), tail: Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::NEWLINE, .. }), tail: rest } } => {
            (false, cons(e.clone(), rest.clone()), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: e @ (_, Token { id: TokenId::NEWLINE, .. }), tail: rest } => {
            (true, rest.clone(), cons(e.clone(), tmp.clone()))
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: Deref @ metamodelica::List::Cons { head: e @ (DiffAlgorithm::Diff::Add, _), tail: rest } } if (lastIsNewline.clone()) => {
            (false, rest.clone(), cons(e.clone(), cons((Diff::Add.clone(), Token { fileName: (literal!("WHITESPACE")).clone(), id: TokenId::WHITESPACE.clone(), fileContents: (({
        let mut __acc = String::new();
        for mut i in (1..=depth.clone()).into_iter() {
            let __x = literal!(" ");
            __acc.push_str(&__x);
        }
        ArcStr::from(__acc)
    })).clone(), byteOffset: 1, length: depth.clone(), lineNumberStart: 0, columnNumberStart: 0, lineNumberEnd: 0, columnNumberEnd: 0 }), tmp.clone())))
        },
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Add, Token { id: TokenId::WHITESPACE, .. }), tail: rest @ Deref @ metamodelica::List::Cons { head: (_, Token { id: TokenId::NEWLINE, .. }), tail: _ } } if (lastIsNewline.clone()) => {
            (true, rest.clone(), tmp.clone())
        },
        Deref @ metamodelica::List::Cons { head: e @ (_, t @ Token { id: TokenId::WHITESPACE, .. }), tail: rest } if (lastIsNewline.clone()) => {
            let Token { length: __pa0, .. } = (t.clone()) else { bail!("pattern mismatch") };
            depth = __pa0.clone();
            (false, rest.clone(), cons(e.clone(), tmp.clone()))
        },
        Deref @ metamodelica::List::Cons { head: e, tail: rest } => {
            (false, rest.clone(), cons(e.clone(), tmp.clone()))
        },
        _ => bail!("match: no arm matched"),
    } });
    }
    simpleDiff = tmp.clone().reverse();
    addedLineComments = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Add.clone() == tuple21(e.clone()) && isLineComment(tuple22(e.clone()))) { continue; }
            let __x = tokenContent(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    removedLineComments = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Delete.clone() == tuple21(e.clone()) && isLineComment(tuple22(e.clone()))) { continue; }
            let __x = tokenContent(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    addedBlockComments = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Add.clone() == tuple21(e.clone()) && isBlockComment(tuple22(e.clone()))) { continue; }
            let __x = blockCommentCanonical(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    removedBlockComments = ({
        let mut __acc: Arc<metamodelica::List<Arc<metamodelica::List<ArcStr>>>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !(Diff::Delete.clone() == tuple21(e.clone()) && isBlockComment(tuple22(e.clone()))) { continue; }
            let __x = blockCommentCanonical(tuple22(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    simpleDiff = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            if !((match e.clone() {
        (DiffAlgorithm::Diff::Add, mut t @ Token { id: TokenId::LINE_COMMENT, .. }) => {
            !(listMember((tokenContent(t.clone())?).clone(), removedLineComments.clone()))
        },
        (DiffAlgorithm::Diff::Add, mut t @ Token { id: TokenId::BLOCK_COMMENT, .. }) => {
            !(listMember(blockCommentCanonical(t.clone())?, removedBlockComments.clone()))
        },
        _ => {
            true
        },
    })) { continue; }
            let __x = (match e.clone() {
        (DiffAlgorithm::Diff::Delete, mut t @ Token { id: TokenId::LINE_COMMENT, .. }) => {
            if (listMember((tokenContent(t.clone())?).clone(), addedLineComments.clone())) {(Diff::Equal.clone(), t.clone())} else {e.clone()}
        },
        (DiffAlgorithm::Diff::Delete, mut t @ Token { id: TokenId::BLOCK_COMMENT, .. }) => {
            if (listMember(blockCommentCanonical(t.clone())?, addedBlockComments.clone())) {(Diff::Equal.clone(), t.clone())} else {e.clone()}
        },
        _ => {
            e.clone()
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    odiffs = ({
        let mut __acc: Arc<metamodelica::List<(Diff, Arc<metamodelica::List<Token>>)>> = metamodelica::nil();
        for mut e in (simpleDiff.clone()).into_iter().cloned() {
            let __x = (match e.clone() {
        (mut d, mut t) => {
            (d.clone(), list![t.clone()])
        },
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(odiffs)
}

pub fn isBlockComment(mut t: Token) -> bool {
    let mut b: bool = false;
    b = (match t.clone() {
        Token { id: TokenId::BLOCK_COMMENT, .. } => true,
        _ => false,
    });
    b
}

pub fn isLineComment(mut t: Token) -> bool {
    let mut b: bool = false;
    b = (match t.clone() {
        Token { id: TokenId::LINE_COMMENT, .. } => true,
        _ => false,
    });
    b
}

pub fn tuple21<A: Clone + 'static, B: Clone + 'static>(mut t: (A, B)) -> A {
    let mut a: A;
    (a, _) = t.clone();
    a
}

pub fn tuple22<A: Clone + 'static, B: Clone + 'static>(mut t: (A, B)) -> B {
    let mut b: B;
    (_, b) = t.clone();
    b
}

pub fn blockCommentCanonical(mut t: Token) -> Result<Arc<metamodelica::List<ArcStr>>> {
    let mut lines: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
    lines = ({
        let mut __acc: Arc<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut s in (System::strtok((tokenContent(t.clone())?).clone(), (literal!("\n")).clone())).into_iter().cloned() {
            let __x = System::trim((s.clone()).clone(), (literal!(" \u{c}\n\r\t\u{b}")).clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(lines)
}

pub fn deleteWhitespaceFollowedByEqualNonWhitespace(mut inRest: Arc<metamodelica::List<(DiffAlgorithm::Diff, Token)>>) -> Result<(bool, Arc<metamodelica::List<(DiffAlgorithm::Diff, Token)>>)> {
    use openmodelica_util::DiffAlgorithm::Diff;
    let mut b: bool = false;
    let mut result: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
    let mut head: (Diff, Token);
    let mut diff: Diff = Diff::Add;
    let mut t: Token = <Token as ::std::default::Default>::default();
    let mut id: TokenId = TokenId::_NO_TOKEN;
    let mut rest: Arc<metamodelica::List<(Diff, Token)>> = metamodelica::nil();
    let mut foundWS: bool = false;
    let mut foundNL: bool = false;
    rest = inRest.clone();
    result = metamodelica::nil();
    while !(rest.clone().is_empty()) {
        let ref __pa3 @ (ref __pa0, ref __pa2 @ Token { id: ref __pa1, .. }) = (listHead(rest.clone())?) else { bail!("pattern mismatch") };
        diff = __pa0.clone();
        id = __pa1.clone();
        t = __pa2.clone();
        head = __pa3.clone();
        if diff.clone() != Diff::Delete.clone() {
            break;
        }
        rest = listRest(rest.clone())?;
        if id.clone() == TokenId::WHITESPACE.clone() && !(foundWS.clone()) {
            foundWS = true;
            result = cons((Diff::Equal.clone(), t.clone()), result.clone());
        } else if id.clone() == TokenId::NEWLINE.clone() {
            foundNL = true;
            break;
        } else {
            result = cons(head.clone(), result.clone());
        }
    }
    if !(foundWS.clone()) || foundNL.clone() {
        b = false;
        result = metamodelica::nil();
        return Ok((b.clone(), result.clone()));
    }
    let () = (::match_deref::match_deref! { match &(rest.clone()) {
        Deref @ metamodelica::List::Cons { head: (DiffAlgorithm::Diff::Equal, t), tail: _ } => (),
        _ => {
            b = false;
            result = metamodelica::nil();
            return Ok((b.clone(), result.clone()));
            bail!("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b = true;
    for mut i in &*result.clone() {
        let mut i = i.clone();
        rest = cons(i.clone(), rest.clone());
    }
    result = rest.clone();
    Ok((b, result))
}

pub fn reportErrors(mut tokens: Arc<metamodelica::List<Token>>) -> Result<()> {
    let mut i: i32 = 0;
    let mut content: ArcStr = arcstr::literal!("");
    for mut t in &*tokens.clone() {
        let mut t = t.clone();
        i = i.clone() + 1;
        if i.clone() > 10 {
            Error::addMessage(Error::SCANNER_ERROR_LIMIT.clone(), metamodelica::nil())?;
        }
        content = (tokenContent(t.clone())?).clone();
        Error::addSourceMessage(Error::SCANNER_ERROR.clone(), list![(StringUtil::convertCharNonAsciiToHex((content.clone()).clone())?).clone()], tokenSourceInfo(t.clone())?)?;
    }
    if !(tokens.clone().is_empty()) {
        bail!("fail");
    }
    Ok(())
}

