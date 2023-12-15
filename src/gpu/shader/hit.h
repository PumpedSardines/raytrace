struct HitInfo {
  float t;
  float3 point;
  float3 normal;
};

bool sphere_hit(const device Sphere& sphere, Ray ray, float t_min, float t_max, thread HitInfo &hit_info) {
  float3 oc = ray.origin - sphere.center;
  float a = dot(ray.direction, ray.direction);
  float b = 2.0 * dot(oc, ray.direction);
  float c = dot(oc, oc) - sphere.radius * sphere.radius;
  float discriminant = b * b - 4.0 * a * c;

  if (discriminant < 0) {
    return false;
  }

  float t = (-b - sqrt(discriminant)) / (2.0 * a);

  if(t < t_min || t > t_max) {
    return false;
  }

  float3 outwards_normal = (ray_point_at(ray, t) - sphere.center);
  bool front_face = dot(ray.direction, outwards_normal) < 0.0;

  hit_info.t = t;
  hit_info.point = ray_point_at(ray, t);
  
  if (front_face) {
    hit_info.normal = normalize(outwards_normal);
  } else {
    hit_info.normal = -normalize(outwards_normal);
  }

  return true;
}

bool plane_hit(const device Plane& plane, Ray ray, float t_min, float t_max, thread HitInfo &hit_info) {
  float a = dot(ray.direction, plane.normal);

  if (abs(a) < 0.0001) {
    return false;
  }

  float b = plane.distance - dot(ray.origin, plane.normal);

  float t = b / a;

  if (t < t_min || t > t_max) {
    return false;
  }

  bool front_face = dot(ray.direction, plane.normal) < 0.0;

  hit_info.t = t;
  hit_info.point = ray_point_at(ray, t);

  if (front_face) {
    hit_info.normal = normalize(plane.normal);
  } else {
    hit_info.normal = -normalize(plane.normal);
  }

  return true;
}

bool calc_hit(
  const device Sphere* spheres,
  const device Plane* planes,
  const device Triangle* triangles,
  const device Uniforms* uniforms,
  Ray ray,
  thread const device Material ** material,
  volatile thread HitInfo &hit_info_res
) {
  HitInfo hit_info;
  bool hit = false;
  float closest = 10000.0;
  
  for(uint i = 0; i < uniforms->plane_count; i++) {
    HitInfo temp_hit_info;
    if (plane_hit(planes[i], ray, 0.001, closest, temp_hit_info)) {
      hit = true;
      hit_info = temp_hit_info;
      *material = &planes[i].material;
      closest = temp_hit_info.t;
    }
  }
  
  for(uint i = 0; i < uniforms->sphere_count; i++) {
    HitInfo temp_hit_info;
    if (sphere_hit(spheres[i], ray, 0.001, closest, temp_hit_info)) {
      hit = true;
      hit_info = temp_hit_info;
      *material = &spheres[i].material;
      closest = temp_hit_info.t;
    }
  }

  for(uint i = 0; i < uniforms->triangle_count; i++) {
    HitInfo temp_hit_info;
    if (triangle_hit(triangles[i], ray, 0.001, closest, temp_hit_info)) {
      hit = true;
      hit_info = temp_hit_info;
      *material = &triangles[i].material;
      closest = temp_hit_info.t;
    }
  }

  if (hit) {
    hit_info_res.t = hit_info.t;
    hit_info_res.point = hit_info.point;
    hit_info_res.normal = hit_info.normal;
    return true;
  }
  return false;
}
