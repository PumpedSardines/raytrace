struct Ray {
  float3 origin;
  float3 direction;
};

float3 ray_point_at(Ray ray, float t) {
  return ray.origin + t * ray.direction;
}

uint random_ray_within_pixel(
  const device Camera* camera,
  uint2 gid,
  thread Ray& ray,
  uint rng_state
) {
  ray.origin = camera->origin;

  // Anti-aliasing
  // Randomly offset the ray direction within the pixel to create a smoother image
  float px;
  rng_state = rand(px, rng_state);

  float py;
  rng_state = rand(py, rng_state);

  float x = (float) gid.x + px;
  float y = (float) gid.y + py;
  float3 pixel_center = camera->viewport_upper_left +
    x * camera->pixel_delta_u +
    y * camera->pixel_delta_v +
    (camera->pixel_delta_u + camera->pixel_delta_v) * 0.5;

  ray.direction = normalize(pixel_center - camera->origin);

  return rng_state;
}
