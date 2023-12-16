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
    hit_info.material = &triangle.material;

    if (front_face) {
      hit_info.normal = normalize(triangle.normal);
    } else {
      hit_info.normal = -normalize(triangle.normal);
    }

    return true;
  }

  return false;
}

