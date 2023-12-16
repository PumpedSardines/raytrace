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
  uint2  gid [[ thread_position_in_grid ]],
  device float3 *output [[ buffer(0) ]],

  device const Uniforms *uniforms [[ buffer(1) ]],
  device const Camera *camera [[ buffer(2) ]],

  device const Sphere *spheres [[ buffer(3) ]],
  device const Plane *planes [[ buffer(4) ]]
) {

  uint width = camera->image_width;
  uint index = gid.y * width + gid.x;

  uint rng_state = rand_xorshift(index);
  rng_state = rand_xorshift(rng_state + rand_xorshift(uniforms->seed));
  
  Camera cam = *camera;
  
  float3 total_color = float3(0.0, 0.0, 0.0);

  for(uint sample = 0; sample < uniforms->samples; sample++) {
    Ray ray;
    ray.origin = cam.origin;
  
    float px;
    rng_state = rand(px, rng_state);
  
    float py;
    rng_state = rand(py, rng_state);
  
    float x = (float) gid.x + px;
    float y = (float) gid.y + py;
    float3 pixel_center = cam.viewport_upper_left +
      x * cam.pixel_delta_u +
      y * cam.pixel_delta_v +
      (cam.pixel_delta_u + cam.pixel_delta_v) * 0.5;

    ray.direction = normalize(pixel_center - cam.origin);
  
  
    float3 current_color = float3(-1.0, -1.0, -1.0);
  
    for(uint depth = 0; depth < 10; depth++) {
      /* device Material &material = materials[index]; */
      HitInfo hit_info;
      const device Material* material;

      bool hit = calc_hit(
        spheres,
        planes,
        uniforms,
        ray,
        &material,
        hit_info
      );
        
      if (hit) {
        float3 rand_direction_first_pass;
        rng_state = rand_unit_float3(rand_direction_first_pass, rng_state);
         
        float3 rand_direction = rand_direction_first_pass + hit_info.normal;
        float3 reflect_direction = reflect(ray.direction, hit_info.normal);

        float3 scatter_direction = lerp(
          reflect_direction,
          rand_direction,
          material->roughness
        );

        ray.origin = hit_info.point;
        ray.direction = scatter_direction;
         
        float light_strength = abs(dot(hit_info.normal, ray.direction));
        float3 color = material->albedo;
        float3 multiply_color = color * 0.5 * light_strength;
  
        if (current_color.x < 0.0) {
          current_color = multiply_color;
        } else {
          current_color = current_color * multiply_color;
        }

      } else {
        float t = 0.5 * (ray.direction.y + 1.0);
        float3 sky_color = lerp(float3(1.0, 1.0, 1.0), float3(0.5, 0.7, 1.0), t);
  
        if (current_color.x < 0.0) {
          current_color = sky_color;
        }
  
        total_color += current_color * sky_color;
        break;
      }
    }
  }
  
  output[index] += to_gamma(total_color / (float)uniforms->samples);

}
