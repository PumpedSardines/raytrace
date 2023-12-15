use const_format::{formatcp, str_replace};
macro_rules! import {
    ($file: expr) => {
        formatcp!("{}{}{}", "#include \"", $file, "\"")
    };
}

// HACK: Didn't get imports to work so this is my hacky solution

const MAIN_METAL: &str = include_str!("main.metal");

const RANDOM_H: &str = include_str!("random.h");
const RANDOM_H_IMPORT: &str = import!("random.h");

const INPUT_H: &str = include_str!("input.h");
const INPUT_H_IMPORT: &str = import!("input.h");

const RAY_H: &str = include_str!("ray.h");
const RAY_H_IMPORT: &str = import!("ray.h");

const UTILS_H: &str = include_str!("utils.h");
const UTILS_H_IMPORT: &str = import!("utils.h");

const HIT_H: &str = include_str!("hit.h");
const HIT_H_IMPORT: &str = import!("hit.h");

pub const fn shader_file() -> &'static str {
    const PASS_1: &'static str = str_replace!(MAIN_METAL, RANDOM_H_IMPORT, RANDOM_H);
    const PASS_2: &'static str = str_replace!(PASS_1, INPUT_H_IMPORT, INPUT_H);
    const PASS_3: &'static str = str_replace!(PASS_2, RAY_H_IMPORT, RAY_H);
    const PASS_4: &'static str = str_replace!(PASS_3, UTILS_H_IMPORT, UTILS_H);
    const PASS_5: &'static str = str_replace!(PASS_4, HIT_H_IMPORT, HIT_H);
    PASS_5
}
