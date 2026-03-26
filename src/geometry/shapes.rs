use crate::MyVertex;

pub enum Shapes {
    Cube,
    Circle,
}

pub fn get_vertex_from_shapes(shape: Shapes) -> Vec<MyVertex> {
    match shape {
        Shapes::Cube => {
            vec![
                MyVertex {
                    position: [-0.1, -0.1],
                },
                MyVertex {
                    position: [-0.1, 0.1],
                },
                MyVertex {
                    position: [0.1, -0.1],
                },
                MyVertex {
                    position: [0.1, 0.1],
                },
            ]
        }
        Shapes::Circle => todo!(),
    }
}
