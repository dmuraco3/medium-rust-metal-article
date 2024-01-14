kernel void add_vectors(
    device const float* a [[         buffer(0)      ]],
    device const float* b [[         buffer(1)      ]],
    device       float* c [[         buffer(2)      ]],
    uint         pos      [[ thread_position_in_grid]]
) {
    c[pos] = a[pos] + b[pos];
}