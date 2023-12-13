#include <metal_stdlib>

using namespace metal;

kernel void sum(
  device const float *data [[ buffer(0) ]],
  device float *output [[ buffer(1) ]],
  uint gid [[ thread_position_in_grid ]]
) {
  output[gid] = data[gid] * data[gid];
}
