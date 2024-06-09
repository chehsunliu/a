fn symmetric_point(p: [i32; 2], q: [i32; 2]) -> [i32; 2] {
  [q[0] + q[0] - p[0], q[1] + q[1] - p[1]]
}