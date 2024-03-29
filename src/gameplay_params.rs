use macroquad::prelude::*;

pub struct GameplayParams {
    pub updates_per_sec: f64,
    pub grid_line_thickness: f32,
    pub cell_shape: CellShape,
    pub field_borders: FieldBorders,
    pub map_generation: MapGeneration,
    pub background_color: Color,
    pub cell_color: Color,
    pub grid_line_color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumIter, strum::IntoStaticStr)]
pub enum CellShape {
    Square,
    Circle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumIter, strum::IntoStaticStr)]
pub enum FieldBorders {
    Connected,
    Limited,
}

impl FieldBorders {
    pub fn create_index_iter(
        &self,
        index: usize,
        max_index: usize,
    ) -> impl Iterator<Item = usize> + Clone {
        let prev = self.decrease_index(index, max_index);
        let next = self.increase_index(index, max_index);

        use std::iter::once;

        prev.into_iter().chain(once(index)).chain(next.into_iter())
    }

    fn decrease_index(&self, index: usize, max_index: usize) -> Option<usize> {
        index
            .checked_sub(1)
            .or_else(|| self.when_connected(max_index - 1))
    }

    fn increase_index(&self, index: usize, max_index: usize) -> Option<usize> {
        let next_index = index + 1;
        if next_index < max_index {
            Some(next_index)
        } else {
            self.when_connected(0)
        }
    }

    #[inline]
    fn when_connected(&self, num: usize) -> Option<usize> {
        match self {
            FieldBorders::Connected => Some(num),
            FieldBorders::Limited => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumIter, strum::IntoStaticStr)]
pub enum MapGeneration {
    Random,
    Glider,
}
