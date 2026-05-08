mat4 pixelMatrixToNDC(mat4 pixelMatrix, vec2 resolution) {
    mat4 ortho = mat4(
            2.0 / resolution.x, 0.0, 0.0, 0.0,
            0.0, 2.0 / resolution.y, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 1.0
        );

    return ortho * pixelMatrix;
}
