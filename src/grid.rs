use crate::{CELLSIZE,verlet::VerletObject};

pub struct Grid {
    pub objects: Vec<Vec<usize>>,   
}

impl Grid {
    pub fn new(objects:& Vec<VerletObject>) -> Self {
        let mut grid = Grid {
            objects: vec![Vec::new();2000],
        };
        for (i,object) in objects.iter().enumerate() {
            grid.objects[object.get_grid_index()].push(i);
        }
        grid

    }
}