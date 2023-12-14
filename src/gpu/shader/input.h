struct Camera {
  float3 origin;
  float3 viewport_upper_left;
  float3 pixel_delta_u;
  float3 pixel_delta_v;
  uint image_width;
  uint image_height;
};

struct Uniforms {
  uint sphere_count;
  uint samples;
};

struct Material {
  float3 albedo;
  float roughness;
};

struct Sphere {
  float3 center;
  float radius;
  Material material;
};
