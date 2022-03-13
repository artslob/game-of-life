use macroquad::prelude::*;

pub struct GameplayParams {
    pub cell_update_frequency: f64,
    pub grid_line_thickness: f32,
    pub cell_shape: CellShape,
    pub field_borders: FieldBorders,
    pub map_generation: MapGeneration,
    pub background_color: BackgroundColor,
    pub cell_color: CellColor,
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

impl FieldBorders {
    pub fn subtract_index(&self, index: usize, max_index: usize) -> Option<usize> {
        match index.checked_sub(1) {
            None => match self {
                FieldBorders::Connected => Some(max_index - 1),
                FieldBorders::Limited => None,
            },
            Some(index) => Some(index),
        }
    }

    pub fn add_index(&self, index: usize, max_index: usize) -> Option<usize> {
        if index + 1 >= max_index {
            match self {
                FieldBorders::Connected => Some(0),
                FieldBorders::Limited => None,
            }
        } else {
            Some(index + 1)
        }
    }
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum MapGeneration {
    Random,
    Glider,
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum BackgroundColor {
    Black,
    Lightgray,
    Gray,
    Darkgray,
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
    White,
}

impl BackgroundColor {
    pub const fn color(&self) -> Color {
        match self {
            BackgroundColor::Lightgray => LIGHTGRAY,
            BackgroundColor::Gray => GRAY,
            BackgroundColor::Darkgray => DARKGRAY,
            BackgroundColor::Yellow => YELLOW,
            BackgroundColor::Red => RED,
            BackgroundColor::Green => GREEN,
            BackgroundColor::Blue => BLUE,
            BackgroundColor::Purple => PURPLE,
            BackgroundColor::White => WHITE,
            BackgroundColor::Black => BLACK,
        }
    }
}

#[derive(Debug, Copy, Clone, strum::EnumVariantNames, strum::FromRepr)]
pub enum CellColor {
    White,
    Black,
    Lightgray,
    Gray,
    Darkgray,
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
}

impl CellColor {
    pub const fn color(&self) -> Color {
        match self {
            Self::Lightgray => LIGHTGRAY,
            Self::Gray => GRAY,
            Self::Darkgray => DARKGRAY,
            Self::Yellow => YELLOW,
            Self::Red => RED,
            Self::Green => GREEN,
            Self::Blue => BLUE,
            Self::Purple => PURPLE,
            Self::White => WHITE,
            Self::Black => BLACK,
        }
    }
}
