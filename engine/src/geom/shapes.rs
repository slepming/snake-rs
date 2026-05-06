use strum::IntoStaticStr;

use crate::MyVertex;

#[derive(IntoStaticStr, Clone, Copy)]
pub enum Shapes {
    Square([f32; 2]),
    Circle([f32; 2]),
}

pub fn get_vertex_from_shapes(shape: Shapes) -> Vec<MyVertex> {
    match shape {
        Shapes::Square(size) => {
            vec![
                MyVertex {
                    position: [-size[0], -size[1]], // x0 -> y0
                },
                MyVertex {
                    position: [size[0], -size[1]], // x1 -> y0
                },
                MyVertex {
                    position: [size[0], size[1]], // x1 -> y1
                },
                MyVertex {
                    position: [-size[0], size[1]], // x0 -> y1
                },
            ]
        }
        Shapes::Circle(size) => {
            vec![
                MyVertex {
                    position: [-size[0], -size[1]], // x0 -> y0
                },
                MyVertex {
                    position: [size[0], -size[1]], // x1 -> y0
                },
                MyVertex {
                    position: [size[0], size[1]], // x1 -> y1
                },
                MyVertex {
                    position: [-size[0], size[1]], // x0 -> y1
                },
            ]
        }
    }
}
