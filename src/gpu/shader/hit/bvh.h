bool bvh_node_hit(
  const device Triangle* triangles,
  const device BVHNode* bvh_nodes,
  const device Uniforms* uniforms,
  Ray ray,
  float t_min,
  float t_max,
  volatile thread HitInfo &hit_info
) {
  // These need to be volatile for some f***ing reason
  volatile int stack[64];
  volatile int stack_pointer = 0;
  volatile bool has_hit = false;
  volatile uint closest = t_max;

  const device BVHNode* current_node = &bvh_nodes[0];

  volatile int i = 0;

  while (true) {
    if (current_node->is_leaf) {
      bool did_left_hit = triangle_hit(
        triangles[current_node->left],
        ray,
        t_min,
        t_max,
        hit_info
      );

      if (did_left_hit) {
        t_max = hit_info.t;
      }

      bool did_right_hit = triangle_hit(
        triangles[current_node->right],
        ray,
        t_min,
        t_max,
        hit_info
      );

      if (did_right_hit) {
        t_max = hit_info.t;
      }

      has_hit = has_hit || did_left_hit || did_right_hit;

      if (stack_pointer == 0) {
        break;
      }

      stack_pointer--;
      current_node = &bvh_nodes[stack[stack_pointer]];
      i = 1;

      /* if (i == 1) { */
        break;
      /* } */


      continue;
    }


    const device BVHNode* left_node = &bvh_nodes[current_node->left];
    const device BVHNode* right_node = &bvh_nodes[current_node->right];

    bool did_left_hit = aabb_hit(left_node->bbox, ray, t_min, closest);
    bool did_right_hit = aabb_hit(right_node->bbox, ray, t_min, closest);

    if (did_left_hit && did_right_hit) {
      stack[stack_pointer] = current_node->right;
      stack_pointer++;
      current_node = left_node;
    } else if (did_left_hit) {
      current_node = left_node;
    } else if (did_right_hit) {
      current_node = right_node;
    } else {
      if (stack_pointer == 0) {
        break;
      } else {
        stack_pointer--;
        current_node = &bvh_nodes[stack[stack_pointer]];
      }
    }
  }

  return has_hit;
}
