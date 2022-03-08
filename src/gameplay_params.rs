pub struct GameplayParams {
    pub cell_update_frequency: f64,
    pub grid_line_thickness: f32,
    pub cell_shape: CellShape,
    pub field_borders: FieldBorders,
    pub map_generation: MapGeneration,
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum CellShape {
    Square,
    Circle,
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum FieldBorders {
    Connected,
    Limited,
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum MapGeneration {
    Random,
    Glider,
}
