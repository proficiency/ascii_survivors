use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AsciiCell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for AsciiCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct AsciiGrid {
    pub cell_size: Vec2,
    pub grid_size: UVec2,
}

impl AsciiGrid {
    pub fn world_size(&self) -> Vec2 {
        Vec2::new(
            self.grid_size.x as f32 * self.cell_size.x,
            self.grid_size.y as f32 * self.cell_size.y,
        )
    }

    pub fn world_center(&self) -> Vec2 {
        self.world_size() * 0.5
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AsciiFrame {
    pub size: UVec2,
    pub cells: Vec<AsciiCell>,
}

impl AsciiFrame {
    pub fn new(size: UVec2) -> Self {
        let cell_count = (size.x * size.y) as usize;
        Self {
            size,
            cells: vec![AsciiCell::default(); cell_count],
        }
    }

    pub fn resize(&mut self, size: UVec2) {
        if self.size == size {
            return;
        }
        self.size = size;
        let cell_count = (size.x * size.y) as usize;
        self.cells.resize(cell_count, AsciiCell::default());
        self.clear();
    }

    pub fn clear(&mut self) {
        self.cells.fill(AsciiCell::default());
    }

    pub fn contains(&self, pos: IVec2) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.size.x as i32
            && pos.y < self.size.y as i32
    }

    pub fn cell_at(&self, x: u32, y: u32) -> Option<AsciiCell> {
        let index = self.index_of(x, y)?;
        self.cells.get(index).copied()
    }

    pub fn put_cell(&mut self, x: i32, y: i32, cell: AsciiCell) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as u32, y as u32);
        if let Some(index) = self.index_of(x, y) {
            if let Some(target) = self.cells.get_mut(index) {
                *target = cell;
            }
        }
    }

    pub fn put_char(&mut self, pos: IVec2, ch: char, fg: Color, bg: Color) {
        if self.contains(pos) {
            let mut resolved_bg = bg;
            if resolved_bg == Color::NONE {
                if let Some(existing) = self.cell_at(pos.x as u32, pos.y as u32) {
                    resolved_bg = existing.bg;
                } else {
                    resolved_bg = Color::BLACK;
                }
            }
            self.put_cell(
                pos.x,
                pos.y,
                AsciiCell {
                    ch,
                    fg,
                    bg: resolved_bg,
                },
            );
        }
    }

    pub fn put_string(&mut self, pos: IVec2, text: &str, fg: Color, bg: Color) {
        if text.is_empty() {
            return;
        }

        let mut x = pos.x;
        for ch in text.chars() {
            let cursor = IVec2::new(x, pos.y);
            if !self.contains(cursor) {
                break;
            }
            self.put_char(cursor, ch, fg, bg);
            x += 1;
        }
    }

    fn index_of(&self, x: u32, y: u32) -> Option<usize> {
        if x >= self.size.x || y >= self.size.y {
            return None;
        }
        let index = (y * self.size.x + x) as usize;
        Some(index)
    }
}
