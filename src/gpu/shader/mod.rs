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

const HIT_SPHERE_H: &str = include_str!("hit/sphere.h");
const HIT_SPHERE_H_IMPORT: &str = import!("hit/sphere.h");

const HIT_PLANE_H: &str = include_str!("hit/plane.h");
const HIT_PLANE_H_IMPORT: &str = import!("hit/plane.h");

const HIT_TRIANGLE_H: &str = include_str!("hit/triangle.h");
const HIT_TRIANGLE_H_IMPORT: &str = import!("hit/triangle.h");

const HIT_BVH_H: &str = include_str!("hit/bvh.h");
const HIT_BVH_H_IMPORT: &str = import!("hit/bvh.h");

pub const fn shader_file() -> &'static str {
    const PASS_1: &'static str = str_replace!(MAIN_METAL, RANDOM_H_IMPORT, RANDOM_H);
    const PASS_2: &'static str = str_replace!(PASS_1, INPUT_H_IMPORT, INPUT_H);
    const PASS_3: &'static str = str_replace!(PASS_2, RAY_H_IMPORT, RAY_H);
    const PASS_4: &'static str = str_replace!(PASS_3, UTILS_H_IMPORT, UTILS_H);
    const PASS_5: &'static str = str_replace!(PASS_4, HIT_H_IMPORT, HIT_H);
    const PASS_6: &'static str = str_replace!(PASS_5, HIT_SPHERE_H_IMPORT, HIT_SPHERE_H);
    const PASS_7: &'static str = str_replace!(PASS_6, HIT_PLANE_H_IMPORT, HIT_PLANE_H);
    const PASS_8: &'static str = str_replace!(PASS_7, HIT_TRIANGLE_H_IMPORT, HIT_TRIANGLE_H);
    const PASS_9: &'static str = str_replace!(PASS_8, HIT_BVH_H_IMPORT, HIT_BVH_H);

    PASS_9
}
