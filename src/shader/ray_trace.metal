#include <metal_stdlib>

using namespace metal;

struct Camera {
  float3 origin;
  float3 viewport_upper_left;
  float3 pixel_delta_u;
  float3 pixel_delta_v;
  uint image_width;
  uint image_height;
};

struct Uniforms {
  uint seed;
  uint sphere_count;
  uint samples;
};

struct Sphere {
  float3 origin;
  float radius;
};

struct Ray {
  float3 origin;
  float3 direction;
};

struct HitInfo {
  float t;
  float3 point;
  float3 normal;
};

float3 ray_point_at(Ray ray, float t) {
  return ray.origin + t * ray.direction;
}

float3 to_gamma(float3 color) {
  return float3(sqrt(color.x), sqrt(color.y), sqrt(color.z));
}

bool sphere_hit(Sphere sphere, Ray ray, float t_min, float t_max, thread HitInfo *hit_info) {
  float3 oc = ray.origin - sphere.origin;
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

  float3 outwards_normal = (ray_point_at(ray, t) - sphere.origin);
  bool front_face = dot(ray.direction, outwards_normal) < 0.0;

  hit_info->t = t;
  hit_info->point = ray_point_at(ray, t);
  
  if (front_face) {
    hit_info->normal = normalize(outwards_normal);
  } else {
    hit_info->normal = -normalize(outwards_normal);
  }

  return true;
}

float3 lerp(float3 a, float3 b, float t) {
  return (1.0 - t) * a + t * b;
}

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
  while (true) {
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

    if (dot(p, p) < 1.0) {
      output = normalize(p);
      return rs;
    }
  }
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

kernel void ray_trace(
  uint2  gid [[ thread_position_in_grid ]],

  device const Uniforms *uniforms [[ buffer(1) ]],
  device const Camera *camera [[ buffer(2) ]],
  device const Sphere *spheres [[ buffer(3) ]],

  device float3 *output [[ buffer(0) ]]
) {
  uint width = camera->image_width;
  uint index = gid.x + gid.y * width;
  uint rng_state = rand_xorshift(uniforms->seed + index);

  Camera cam = *camera;

  float3 total_color = float3(0.0, 0.0, 0.0);

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

    for(uint depth = 0; depth < 50; depth++) {
      HitInfo hit_info;
      bool hit = false;
      float closest = 10000.0;
      
      for(uint i = 0; i < uniforms->sphere_count; i++) {
        HitInfo temp_hit_info;
        if (sphere_hit(spheres[i], ray, 0.001, closest, &temp_hit_info)) {
          hit = true;
          hit_info = temp_hit_info;
          closest = temp_hit_info.t;
        }
      }

      if (hit) {
        float3 rand_direction_first_pass;
        rng_state = rand_unit_float3(rand_direction_first_pass, rng_state);
        float3 rand_direction = rand_direction_first_pass + hit_info.normal;
        float3 scatter_direction = reflect(ray.direction, hit_info.normal);
        // float light_strength = abs(dot(hit_info.normal, ray.direction));

        float3 color = float3(1.0, 1.0, 1.0);

        float3 multiply_color = color * 0.5; //* light_strength;

        if (current_color.x < 0.0) {
          current_color = multiply_color;
        } else {
          current_color = current_color * multiply_color;
        }

        ray.origin = hit_info.point;
        ray.direction = rand_direction;

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
