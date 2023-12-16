struct HitInfo {
  float t;
  float3 point;
  float3 normal;
};

#include "hit/sphere.h"
#include "hit/plane.h"
#include "hit/triangle.h"

bool calc_hit(
  const device Sphere* spheres,
  const device Plane* planes,
  const device Triangle* triangles,
  const device Uniforms* uniforms,
  Ray ray,
  volatile thread const device Material ** material,
  volatile thread HitInfo &hit_info
) {
  bool hit = false;
  float closest = 10000.0;

  for(uint i = 0; i < uniforms->plane_count; i++) {
    if (plane_hit(planes[i], ray, 0.001, closest, hit_info)) {
      hit = true;
      *material = &planes[i].material;
      closest = hit_info.t;
    }
  }

  for(uint i = 0; i < uniforms->sphere_count; i++) {
    if (sphere_hit(spheres[i], ray, 0.001, closest, hit_info)) {
      hit = true;
      *material = &spheres[i].material;
      closest = hit_info.t;
    }
  }

  for(uint i = 0; i < uniforms->triangle_count; i++) {
    if (triangle_hit(triangles[i], ray, 0.001, closest, hit_info)) {
      hit = true;
      *material = &triangles[i].material;
      closest = hit_info.t;
    }
  }

  return hit;
}
