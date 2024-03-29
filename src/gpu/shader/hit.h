struct HitInfo {
  float t;
  float3 point;
  float3 normal;
};

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

  return true;
}

bool triangle_hit(
  const device Triangle& triangle,
  Ray ray,
  float t_min,
  float t_max,
  volatile thread HitInfo &hit_info
) {
  float normal_dot_direction = dot(triangle.normal, ray.direction);

  if (abs(normal_dot_direction) < 0.0001) {
    return false;
  }

  float t = -(dot(triangle.normal, ray.origin) + triangle.distance) / normal_dot_direction;

  if (t < t_min || t > t_max) {
    return false;
  }

  float3 point = ray_point_at(ray, t);

  float3 edge_a = triangle.b - triangle.a;
  float3 edge_b = triangle.c - triangle.b;
  float3 edge_c = triangle.a - triangle.c;
  
  float3 c_a = point - triangle.a;
  float3 c_b = point - triangle.b;
  float3 c_c = point - triangle.c;

  float3 test_a = cross(edge_a, c_a);
  float3 test_b = cross(edge_b, c_b);
  float3 test_c = cross(edge_c, c_c);

  bool res_a = dot(triangle.normal, test_a) > 0.0;
  bool res_b = dot(triangle.normal, test_b) > 0.0;
  bool res_c = dot(triangle.normal, test_c) > 0.0;

  if (res_a && res_b && res_c) {
    bool front_face = dot(ray.direction, triangle.normal) < 0.0;

    hit_info.t = t;
    hit_info.point = point;

    if (front_face) {
      hit_info.normal = normalize(triangle.normal);
    } else {
      hit_info.normal = -normalize(triangle.normal);
    }

    return true;
  }

  return false;
}

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
