struct Ray {
  float3 origin;
  float3 direction;
};

float3 ray_point_at(Ray ray, float t) {
  return ray.origin + t * ray.direction;
}
