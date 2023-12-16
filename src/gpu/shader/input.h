struct Camera {
  float3 origin;
  float3 viewport_upper_left;
  float3 pixel_delta_u;
  float3 pixel_delta_v;
  uint image_width;
  uint image_height;
};

struct AABB {
  float3 min;
  float3 max;
};

struct Uniforms {
  uint seed;
  uint sphere_count;
  uint plane_count;
  uint triangle_count;
  uint bvh_nodes_count;
  uint samples;
  uint max_bounces;
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

struct Plane {
  float3 normal;
  float distance;
  Material material;
};

struct Triangle {
  float3 a;
  float3 b;
  float3 c;
  float3 normal;
  float distance;
  Material material;
  AABB bbox;
};

struct BVHNode {
  uint left;
  uint right;
  bool is_leaf;
  AABB bbox;
};
