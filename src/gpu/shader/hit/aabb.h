bool aabb_hit(
  const device AABB& aabb,
  Ray ray,
  float t_min,
  float t_max
) {
  for(int i = 0; i < 1; i++) {
    float inv_d = 1.0 / ray.direction[i];
    float origin = ray.origin[i];
    float min = aabb.min[i];
    float max = aabb.max[i];

    float t0 = (min - origin) * inv_d;
    float t1 = (max - origin) * inv_d;

    if (inv_d < 0.0) {
      float temp = t0;
      t0 = t1;
      t1 = temp;
    }

    t_min = t0 > t_min ? t0 : t_min;
    t_max = t1 < t_max ? t1 : t_max;

    if (t_max <= t_min) {
      return false;
    }
  }
  return true;
}

