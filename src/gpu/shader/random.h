uint rand_xorshift(uint rng_state){    
  rng_state ^= (rng_state << 13);
  rng_state ^= (rng_state >> 17);    
  rng_state ^= (rng_state << 5);    
  return rng_state;
}

uint rand(thread float& output, uint rng_state) {
  uint rs = rand_xorshift(rng_state);
  output = (float) rs / (float) 0xffffffff;
  return rs;
}

uint rand_unit_float3(thread float3& output, uint rng_state) {
  uint rs = rand_xorshift(rng_state);
  /* while (true) { */
    float x;
    rs = rand(x, rs);
    float y;
    rs = rand(y, rs);
    float z;
    rs = rand(z, rs);

    float3 p = float3(
      2.0 * x - 1.0,
      2.0 * y - 1.0,
      2.0 * z - 1.0
    );

    /* if (dot(p, p) < 1.0) { */
      output = normalize(p);
      return rs;
  /*   } */
  /* } */
}

uint random_on_hemisphere(thread float3& output, thread float3& normal, uint rng_state) {
  float3 on_unit_sphere;
  uint rs = rand_unit_float3(on_unit_sphere, rng_state);
  if (dot(on_unit_sphere, normal) > 0.0) {
    output = on_unit_sphere;
  } else {
    output = -on_unit_sphere;
  }
  return rs;
}

