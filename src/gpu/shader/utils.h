__attribute__((always_inline))
float3 to_gamma(float3 color) {
  return float3(sqrt(color.x), sqrt(color.y), sqrt(color.z));
}

// ============ OUTPUT HELPERS ============
__attribute__((always_inline))
void set_output(
  device float *output,
  uint index,
  float3 color
) {
  uint i = index * 3;

  output[i + 0] = color.x;
  output[i + 1] = color.y;
  output[i + 2] = color.z;
}

__attribute__((always_inline))
void add_output(
  device float *output,
  uint index,
  float3 color
) {
  uint i = index * 3;

  output[i + 0] += color.x;
  output[i + 1] += color.y;
  output[i + 2] += color.z;
}


