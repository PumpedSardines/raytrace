use vec3::{Point3, Vec3};

use crate::object::aabb;
use crate::object::material::Material;
use crate::ray::Ray;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Point3<f64>,
    pub normal: Vec3<f64>,
    pub front_face: bool,
    pub material: Material,
}

pub trait Hittable: Sync + Send + HittableClone {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> &Option<aabb::AABB> {
        &None
    }
}
// https://stackoverflow.com/a/30353928
pub trait HittableClone {
    fn clone_box(&self) -> Box<dyn Hittable>;
}
impl<T> HittableClone for T
where
    T: 'static + Hittable + Clone,
{
    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

pub struct HittableList {
    bbox: Option<aabb::AABB>,
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> HittableList {
        let mut bbox = None;

        for object in &objects {
            if let Some(object_bbox) = object.bounding_box() {
                bbox = Some(match bbox {
                    Some(bbox) => aabb::AABB::combine(&bbox, object_bbox),
                    None => object_bbox.clone(),
                });
            }
        }

        HittableList { bbox, objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        if let Some(object_bbox) = object.bounding_box() {
            self.bbox = Some(match &self.bbox {
                Some(bbox) => aabb::AABB::combine(&bbox, object_bbox),
                None => object_bbox.clone(),
            });
        }
        self.objects.push(object);
    }
}

#[derive(Clone)]
pub struct BVHNode {
    bbox: Option<aabb::AABB>,
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
}

impl BVHNode {
    pub fn new(list: &HittableList) -> BVHNode {
        let mut rand = random::Random::from_time();
        let axis = rand.next_u32_range(0, 3);

        let comp = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!("Invalid axis"),
        };

        match list.objects.len() {
            1 => {
                let left = list.objects[0].clone();
                let right = list.objects[0].clone();

                Self {
                    bbox: list.bbox.clone(),
                    left,
                    right,
                }
            }
            2 => {
                let left = list.objects[0].clone();
                let right = list.objects[1].clone();

                Self {
                    bbox: Some(aabb::AABB::combine(
                        left.bounding_box().as_ref().unwrap(),
                        right.bounding_box().as_ref().unwrap(),
                    )),
                    left,
                    right,
                }
            }
            _ => {
                let mut objects = list.objects.clone();

                objects.sort_by(comp);

                let mid = objects.len() / 2;

                let left = BVHNode::new(&HittableList::new(objects[..mid].to_vec()));
                let right = BVHNode::new(&HittableList::new(objects[mid..].to_vec()));

                Self {
                    bbox: Some(aabb::AABB::combine(
                        left.bounding_box().as_ref().unwrap(),
                        right.bounding_box().as_ref().unwrap(),
                    )),
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
        }
    }

    fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: u32) -> std::cmp::Ordering {
        let a_box_min: [f64; 3] = a.bounding_box().as_ref().unwrap().get_min().into();
        let b_box_min: [f64; 3] = b.bounding_box().as_ref().unwrap().get_max().into();

        a_box_min[axis as usize]
            .partial_cmp(&b_box_min[axis as usize])
            .unwrap()
    }

    fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> &Option<aabb::AABB> {
        &self.bbox
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self
            .bbox
            .as_ref()
            .expect("Should never be set to None")
            .hit(r, t_min, t_max)
        {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);

        if hit_left.is_some() && hit_right.is_some() {
            let hit_left = hit_left.unwrap();
            let hit_right = hit_right.unwrap();

            if hit_left.t < hit_right.t {
                return Some(hit_left);
            } else {
                return Some(hit_right);
            }
        }

        hit_left.or(hit_right)
    }
}
