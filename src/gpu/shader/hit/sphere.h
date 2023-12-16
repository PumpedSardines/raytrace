bool sphere_hit(
  const device Sphere& sphere,
  Ray ray,
  float t_min,
  float t_max,
  volatile thread HitInfo &hit_info
) {
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

