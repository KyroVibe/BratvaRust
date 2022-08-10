pub struct Transform {
    pub mat: [[f32; 4]; 4]
}

impl Transform {
    pub fn new(mat: Option<[[f32; 4]; 4]>) -> Self {
        if let Some(matrix) = mat {
            return Transform {
                mat: matrix
            };
        } else {
            return Transform {
                mat: [
                    [ 1.0, 0.0, 0.0, 0.0 ],
                    [ 0.0, 1.0, 0.0, 0.0 ],
                    [ 0.0, 0.0, 1.0, 0.0 ],
                    [ 0.0, 0.0, 0.0, 1.0 ]
                ]
            };
        }
    }

    pub fn rotation(theta: f32) -> Self {
        Transform {
            mat: [
                [ theta.cos(), theta.sin(), 0.0, 0.0 ],
                [-theta.sin(), theta.cos(), 0.0, 0.0 ],
                [          0.0,        0.0, 1.0, 0.0 ],
                [          0.0,        0.0, 0.0, 1.0 ]
            ]
        }
    }

    pub fn translation(x: f32, y: f32) -> Self {
        Transform {
            mat: [
                [ 1.0, 0.0, 0.0,   x ],
                [ 0.0, 1.0, 0.0,   y ],
                [ 0.0, 0.0, 1.0, 0.0 ],
                [ 0.0, 0.0, 0.0, 1.0 ]
            ]
        }
    }

    pub fn multi(a: &Transform, b: &Transform) -> Transform {
        Transform {
            mat: [
                [
                    b.mat[0][0] * a.mat[0][0] + b.mat[0][1] * a.mat[1][0] + b.mat[0][2] * a.mat[2][0] + b.mat[0][3] * a.mat[3][0],
                    b.mat[1][0] * a.mat[0][0] + b.mat[1][1] * a.mat[1][0] + b.mat[1][2] * a.mat[2][0] + b.mat[1][3] * a.mat[3][0],
                    b.mat[2][0] * a.mat[0][0] + b.mat[2][1] * a.mat[1][0] + b.mat[2][2] * a.mat[2][0] + b.mat[2][3] * a.mat[3][0],
                    b.mat[3][0] * a.mat[0][0] + b.mat[3][1] * a.mat[1][0] + b.mat[3][2] * a.mat[2][0] + b.mat[3][3] * a.mat[3][0],
                ],
                [
                    b.mat[0][0] * a.mat[0][1] + b.mat[0][1] * a.mat[1][1] + b.mat[0][2] * a.mat[2][1] + b.mat[0][3] * a.mat[3][1],
                    b.mat[1][0] * a.mat[0][1] + b.mat[1][1] * a.mat[1][1] + b.mat[1][2] * a.mat[2][1] + b.mat[1][3] * a.mat[3][1],
                    b.mat[2][0] * a.mat[0][1] + b.mat[2][1] * a.mat[1][1] + b.mat[2][2] * a.mat[2][1] + b.mat[2][3] * a.mat[3][1],
                    b.mat[3][0] * a.mat[0][1] + b.mat[3][1] * a.mat[1][1] + b.mat[3][2] * a.mat[2][1] + b.mat[3][3] * a.mat[3][1],
                ],
                [
                    b.mat[0][0] * a.mat[0][2] + b.mat[0][1] * a.mat[1][2] + b.mat[0][2] * a.mat[2][2] + b.mat[0][3] * a.mat[3][2],
                    b.mat[1][0] * a.mat[0][2] + b.mat[1][1] * a.mat[1][2] + b.mat[1][2] * a.mat[2][2] + b.mat[1][3] * a.mat[3][2],
                    b.mat[2][0] * a.mat[0][2] + b.mat[2][1] * a.mat[1][2] + b.mat[2][2] * a.mat[2][2] + b.mat[2][3] * a.mat[3][2],
                    b.mat[3][0] * a.mat[0][2] + b.mat[3][1] * a.mat[1][2] + b.mat[3][2] * a.mat[2][2] + b.mat[3][3] * a.mat[3][2],
                ],
                [
                    b.mat[0][0] * a.mat[0][3] + b.mat[0][1] * a.mat[1][3] + b.mat[0][2] * a.mat[2][3] + b.mat[0][3] * a.mat[3][3],
                    b.mat[1][0] * a.mat[0][3] + b.mat[1][1] * a.mat[1][3] + b.mat[1][2] * a.mat[2][3] + b.mat[1][3] * a.mat[3][3],
                    b.mat[2][0] * a.mat[0][3] + b.mat[2][1] * a.mat[1][3] + b.mat[2][2] * a.mat[2][3] + b.mat[2][3] * a.mat[3][3],
                    b.mat[3][0] * a.mat[0][3] + b.mat[3][1] * a.mat[1][3] + b.mat[3][2] * a.mat[2][3] + b.mat[3][3] * a.mat[3][3],
                ]
            ]
        }
    }

    pub fn apply(&self, t: &Transform) -> Transform {
        Transform::multi(self, t)
    }
}