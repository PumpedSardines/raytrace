bool plane_hit(
  const device Plane& plane,
  Ray ray,
  float t_min,
  float t_max,
  volatile thread HitInfo &hit_info
) {
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

  hit_info.material = &plane.material;

  return true;
}

