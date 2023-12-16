struct HitInfo {
  float t;
  float3 point;
  float3 normal;
  const device Material * material;
};

#include "hit/sphere.h"
#include "hit/plane.h"
#include "hit/triangle.h"
#include "hit/aabb.h"
#include "hit/bvh.h"

inline bool calc_hit(
  const device Sphere* spheres,
  const device Plane* planes,
  const device Triangle* triangles,
  const device BVHNode* bvh_nodes,
  const device Uniforms* uniforms,
  Ray ray,
  volatile thread HitInfo &hit_info
) {
  bool hit = false;
  float closest = 10000.0;

  for(uint i = 0; i < uniforms->plane_count; i++) {
    if (plane_hit(planes[i], ray, 0.001, closest, hit_info)) {
      hit = true;
      closest = hit_info.t;
    }
  }

  for(uint i = 0; i < uniforms->sphere_count; i++) {
    if (sphere_hit(spheres[i], ray, 0.001, closest, hit_info)) {
      hit = true;
      closest = hit_info.t;
    }
  }

  if (uniforms->bvh_nodes_count > 0) {
    if (bvh_node_hit(triangles, bvh_nodes, uniforms, ray, 0.001, closest, hit_info)) {
      hit = true;
      closest = hit_info.t;
    }
  }

  return hit;
}
