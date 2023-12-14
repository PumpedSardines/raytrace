#include <metal_stdlib>

using namespace metal;

// WARNING: These imports are not handled by the Metal compiler
// but instead by my own script. Didn't get the imports to work ¯\_(ツ)_/¯
#include "utils.h"
#include "input.h"
#include "random.h"
#include "ray.h"

struct HitInfo {
  float t;
  float3 point;
  float3 normal;
};

inline bool sphere_hit(const device Sphere& sphere, Ray ray, float t_min, float t_max, thread HitInfo &hit_info) {
  float3 oc = ray.origin - sphere.center;
  float a = dot(ray.direction, ray.direction);
  float b = 2.0 * dot(oc, ray.direction);
  float c = dot(oc, oc) - sphere.radius * sphere.radius;
  float discriminant = b * b - 4.0 * a * c;

  if (discriminant < 0) {
    return false;
  }

  float t = (-b - sqrt(discriminant)) / (2.0 * a);

  if(t < t_min || t > t_max) {
    return false;
  }

  float3 outwards_normal = (ray_point_at(ray, t) - sphere.center);
  bool front_face = dot(ray.direction, outwards_normal) < 0.0;

  hit_info.t = t;
  hit_info.point = ray_point_at(ray, t);
  
  if (front_face) {
    hit_info.normal = normalize(outwards_normal);
  } else {
    hit_info.normal = -normalize(outwards_normal);
  }

  return true;
}

void test(float3 t) {
  t = float3(1.0, 1.0, 1.0);
}

kernel void ray_trace(
  uint2  gid [[ thread_position_in_grid ]],
  device float3 *output [[ buffer(0) ]],

  device const Uniforms *uniforms [[ buffer(1) ]],
  device const Camera *camera [[ buffer(2) ]],
  device const Sphere *spheres [[ buffer(3) ]]

) {

  uint width = camera->image_width;
  uint index = gid.y * width + gid.x;

  uint rng_state = rand_xorshift(index + width * uniforms->seed);
  
  Camera cam = *camera;
  
  for(uint sample = 0; sample < uniforms->samples; sample++) {
    Ray ray;
    ray.origin = cam.origin;
  
    float px;
    rng_state = rand(px, rng_state);
  
    float py;
    rng_state = rand(py, rng_state);
  
    ray.direction = cam.viewport_upper_left +
      gid.x * cam.pixel_delta_u +
      gid.y * cam.pixel_delta_v + 
      cam.pixel_delta_u * (-0.5 + px) +
      cam.pixel_delta_v * (-0.5 + py);
  
  
    float3 current_color = float3(-1.0, -1.0, -1.0);
  
    for(uint depth = 0; depth < 10; depth++) {
      HitInfo hit_info;
      Material material;
      bool hit = false;
      float closest = 10000.0;
      
      for(uint i = 0; i < uniforms->sphere_count; i++) {
        HitInfo temp_hit_info;
        if (sphere_hit(spheres[i], ray, 0.001, closest, temp_hit_info)) {
          hit = true;
          hit_info = temp_hit_info;
          material = spheres[i].material;
          closest = temp_hit_info.t;
        }
      }
       
      if (hit) {
        float3 rand_direction_first_pass;
        rng_state = rand_unit_float3(rand_direction_first_pass, rng_state);
         
        float3 rand_direction = rand_direction_first_pass + hit_info.normal;
        float3 reflect_direction = reflect(ray.direction, hit_info.normal);

        float3 scatter_direction = lerp(
          reflect_direction,
          rand_direction,
          material.roughness
        );

        ray.origin = hit_info.point;
        ray.direction = scatter_direction;
         
        float light_strength = abs(dot(hit_info.normal, ray.direction));
        float3 color = material.albedo;
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
  
        output[index] += current_color * sky_color;
        break;
      }
    }
  }
  
  output[index] = to_gamma(output[index] / (float)uniforms->samples);
}
