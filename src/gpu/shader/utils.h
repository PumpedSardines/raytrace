__attribute__((always_inline))
float3 to_gamma(float3 color) {
  return float3(sqrt(color.x), sqrt(color.y), sqrt(color.z));
}

__attribute__((always_inline))
float3 lerp(float3 a, float3 b, float t) {
  return (1.0 - t) * a + t * b;
}


