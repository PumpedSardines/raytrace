use crate::gpu::type_mapping;
use rand::{rngs::ThreadRng, *};

pub(crate) fn build_bvh_tree(
    triangles: &Vec<type_mapping::Triangle>,
) -> Vec<type_mapping::BVHNode> {
    let mut rand = rand::thread_rng();

    let mut bvh_list = Vec::new();

    let entry =
        build_bvh_tree_recursive(triangles, 0, triangles.len() - 1, &mut bvh_list, &mut rand);

    println!("BVH tree: {:?}", bvh_list);
    println!("Triangles: {:?}", triangles);
    println!("Entry: {}", entry);

    bvh_list
}

/// Returns the index of the root node
fn build_bvh_tree_recursive(
    triangles: &Vec<type_mapping::Triangle>,
    triangle_start: usize,
    triangle_end: usize,
    bvh_list: &mut Vec<type_mapping::BVHNode>,
    rand: &mut ThreadRng,
) -> usize {
    let len = triangle_end - triangle_start + 1;
    let axis = rand.gen_range(0..3);

    if len == 1 {
        let triangle = &triangles[0];
        let bbox = triangle.bbox;

        bvh_list.push(type_mapping::BVHNode {
            left: triangle_start as u32,
            right: triangle_end as u32,
            is_leaf: true,
            aabb: bbox,
        });

        return bvh_list.len() - 1;
    } else if len == 2 {
        let triangle_a = &triangles[0];
        let triangle_b = &triangles[1];

        let bbox = type_mapping::AABB::combine(triangle_a.bbox, triangle_b.bbox);

        bvh_list.push(type_mapping::BVHNode {
            left: triangle_start as u32,
            right: triangle_end as u32,
            is_leaf: true,
            aabb: bbox,
        });

        return bvh_list.len() - 1;
    } else {
        let mut triangles_new = triangles.clone();
        triangles_new.sort_by(|a, b| box_compare(&a.bbox, &b.bbox, axis));
        let mid = triangles_new.len() / 2;

        bvh_list.push(type_mapping::BVHNode {
            left: 0,
            right: 0,
            is_leaf: false,
            aabb: type_mapping::AABB::default(),
        });

        let index = bvh_list.len() - 1;

        let left = build_bvh_tree_recursive(
            &triangles_new[0..mid].to_vec(),
            triangle_start,
            triangle_start + mid - 1,
            bvh_list,
            rand,
        );
        let right = build_bvh_tree_recursive(
            &triangles_new[mid..].to_vec(),
            triangle_start + mid,
            triangle_end,
            bvh_list,
            rand,
        );

        let bbox = type_mapping::AABB::combine(bvh_list[left].aabb, bvh_list[right].aabb);
        bvh_list[index].left = left as u32;
        bvh_list[index].right = right as u32;
        bvh_list[index].aabb = bbox;

        return index;
    }
}

fn box_compare(a: &type_mapping::AABB, b: &type_mapping::AABB, axis: u32) -> std::cmp::Ordering {
    let a_box_min = a.min;
    let b_box_min = b.min;

    a_box_min[axis as usize]
        .partial_cmp(&b_box_min[axis as usize])
        .unwrap()
}

pub(crate) fn bbox_from_triangles(triangles: &Vec<type_mapping::Triangle>) -> type_mapping::AABB {
    let mut bbox = triangles[0].bbox;

    for triangle in triangles.iter().skip(1) {
        bbox = type_mapping::AABB::combine(bbox, triangle.bbox);
    }

    bbox
}
