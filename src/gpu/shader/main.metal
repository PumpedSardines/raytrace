#include <metal_stdlib>

using namespace metal;

// WARNING: These imports are not handled by the Metal compiler
// but instead by my own script. Didn't get the imports to work ¯\_(ツ)_/¯
#include "utils.h"
#include "input.h"
#include "random.h"
#include "ray.h"
#include "hit.h"

kernel void ray_trace(
  // Current pixel position, provided by the gpu
  uint2  gid [[ thread_position_in_grid ]],
  // Output buffer
  device float3 *output [[ buffer(0) ]],

  // Constant variables
  device const Uniforms *uniforms [[ buffer(1) ]],
  // The camera
  device const Camera *camera [[ buffer(2) ]],

  // Scene objects
  device const Sphere *spheres [[ buffer(3) ]],
  device const Plane *planes [[ buffer(4) ]],
  device const Triangle *triangles [[ buffer(5) ]]
) {
  
  uint width = camera->image_width;
  // The index of the current pixel in the output buffer
  uint index = gid.y * width + gid.x;

  // Set up random state
  // NOTE: uniforms->seed is a seed that's sent by the cpu to generate variation in 
  // the noise when layering multiple renders on top of each other
  uint rng_state = rand_xorshift(index);
  rng_state = rand_xorshift(rng_state + rand_xorshift(uniforms->seed));

  // The sum of all samples for the current pixel
  // NOTE: This color will be averaged later
  float3 sum_of_colors = float3(0.0, 0.0, 0.0);

  // Loop through all samples
  for(uint sample = 0; sample < uniforms->samples; sample++) {
    // Create a ray from the camera to the current pixel
    Ray ray;
    rng_state = random_ray_within_pixel(
      camera,
      gid,
      ray,
      rng_state
    );

    // The combined color of all bounces in the current sample
    float3 current_color = float3(0.0, 0.0, 0.0);

    for(uint depth = 0; depth < uniforms->max_bounces; depth++) {
      // Check if the ray hits anything
      volatile HitInfo hit_info;
      const device Material* material; // material is a pointer to the material of the hit object
      bool hit = calc_hit(
        spheres,
        planes,
        triangles,
        uniforms,
        ray,
        &material,
        hit_info
      );

      if (hit) {
        // Calculate the new ray direction
        float3 rand_direction;
        rng_state = rand_unit_float3(rand_direction, rng_state);
        float3 diffuse_direction = rand_direction + hit_info.normal;
        float3 reflect_direction = reflect(ray.direction, hit_info.normal);
        float3 scatter_direction = lerp(reflect_direction, diffuse_direction, material->roughness);

        // Set ray to to go in a new direction
        ray.origin = hit_info.point;
        ray.direction = scatter_direction;

        // Light strength calculates how much light hits the surface
        // The greater the angle between the normal and the ray direction, the less light hits the surface
        float light_strength = abs(dot(hit_info.normal, ray.direction));
        float3 hit_color = material->albedo * 0.5 * light_strength;

        if (depth == 0) {
          // If this is the first bounce no color has been calculated yet,
          // therefore we set the color to the hit color instead of multiply
          current_color = hit_color;
        } else {
          current_color *= hit_color;
        }
      } else {
        // Calculate the sky color
        float t = 0.5 * (ray.direction.y + 1.0);
        float3 sky_color = lerp(float3(1.0, 1.0, 1.0), float3(0.5, 0.7, 1.0), t);

        if (depth == 0) {
          current_color = sky_color;
        } else {
          current_color *= sky_color;
        }

        break;
      }
    }

    sum_of_colors += current_color;
  }
  
  output[index] += to_gamma(sum_of_colors / (float)uniforms->samples);
}
