use crate::MyVertex;

pub enum Shapes {
    Square([f32; 2]),
    Circle,
}

pub fn get_vertex_from_shapes(shape: Shapes) -> Vec<MyVertex> {
    match shape {
        Shapes::Square(size) => {
            vec![
                MyVertex {
                    position: [-size[0], -size[0]],
                },
                MyVertex {
                    position: [-size[1], size[1]],
                },
                MyVertex {
                    position: [size[1], -size[1]],
                },
                MyVertex {
                    position: [size[0], size[0]],
                },
            ]
        }
        Shapes::Circle => todo!(),
    }
}
