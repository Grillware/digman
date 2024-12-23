use ratatui::style::{Color, palette::tailwind};

pub struct TableColors {
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_row_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
}

impl TableColors {
    pub const fn new() -> Self {
        Self {
            header_bg: tailwind::SLATE.c200,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: tailwind::SLATE.c200,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
        }
    }
}
