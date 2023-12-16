bool aabb_hit_x(
  const device AABB& aabb,
  Ray ray,
  float t_min,
  float t_max
) {
  float inv_d = 1.0 / ray.direction.x;
  float origin = ray.origin.x;
  float min = aabb.min.x;
  float max = aabb.max.x;

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

  return true;
}

bool aabb_hit_y(
  const device AABB& aabb,
  Ray ray,
  float t_min,
  float t_max
) {
  float inv_d = 1.0 / ray.direction.y;
  float origin = ray.origin.y;
  float min = aabb.min.y;
  float max = aabb.max.y;

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

  return true;
}

bool aabb_hit_z(
  const device AABB& aabb,
  Ray ray,
  float t_min,
  float t_max
) {
  float inv_d = 1.0 / ray.direction.y;
  float origin = ray.origin.y;
  float min = aabb.min.z;
  float max = aabb.max.z;

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

  return true;
}

// IM NOT EVEN JOKING I TRIED MAKING THIS WITH A FOR LOOP BUT THE COMPILER OPTIMIZED IT OUT
// IM GETTING SO TIRED OF DEALING WITH THIS STUPID AS SHIT COMPILER THAT REMOVES CODE THAT
// NEEDS TO BE THERE
//
// Anyway, that's why there are 3 functions instead of one :)
bool aabb_hit(
  const device AABB& aabb,
  Ray ray,
  float t_min,
  float t_max
) {
  return aabb_hit_x(aabb, ray, t_min, t_max);
         /* aabb_hit_y(aabb, ray, t_min, t_max) && */
         /* aabb_hit_z(aabb, ray, t_min, t_max); */
}
