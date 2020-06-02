use font::Metrics;

use alacritty_terminal::index::{Column, Point};
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::color::Rgb;
use alacritty_terminal::term::SizeInfo;
use alacritty_terminal::text_run::TextRun;

#[derive(Debug, Copy, Clone)]
pub struct RenderRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Rgb,
    pub alpha: f32,
}

impl RenderRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Rgb, alpha: f32) -> Self {
        RenderRect { x, y, width, height, color, alpha }
    }

    pub fn from_text_run(
        text_run: &TextRun,
        (descent, position, thickness): (f32, f32, f32),
        size: &SizeInfo,
    ) -> Self {
        let start_point = text_run.start_point();
        let start_x = start_point.col.0 as f32 * size.cell_width;
        let end_x = (text_run.end_point().col.0 + 1) as f32 * size.cell_width;
        let width = end_x - start_x;

        let line_bottom = (start_point.line.0 + 1) as f32 * size.cell_height;
        let baseline = line_bottom + descent;

        // Make sure lines are always visible.
        let height = thickness.max(1.);

        let mut y = baseline - position - height / 2.;
        let max_y = line_bottom - height;
        y = y.min(max_y);

        Self {
            x: start_x + size.padding_x,
            y: y + size.padding_y,
            width,
            height,
            color: text_run.fg,
            alpha: 1.,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RenderLine {
    pub start: Point,
    pub end: Point,
    pub color: Rgb,
}

impl RenderLine {
    pub fn rects(&self, flag: Flags, metrics: &Metrics, size: &SizeInfo) -> Vec<RenderRect> {
        let mut rects = Vec::new();

        let mut start = self.start;
        while start.line < self.end.line {
            let mut end = start;
            end.col = size.cols() - 1;
            rects.push(Self::create_rect(metrics, size, flag, start, end, self.color));

            start.col = Column(0);
            start.line += 1;
        }

        rects.push(Self::create_rect(metrics, size, flag, start, self.end, self.color));

        rects
    }

    fn create_rect(
        metrics: &Metrics,
        size: &SizeInfo,
        flag: Flags,
        start: Point,
        end: Point,
        color: Rgb,
    ) -> RenderRect {
        let start_x = start.col.0 as f32 * size.cell_width;
        let end_x = (end.col.0 + 1) as f32 * size.cell_width;
        let width = end_x - start_x;

        let (position, mut height) = match flag {
            Flags::UNDERLINE => (metrics.underline_position, metrics.underline_thickness),
            Flags::STRIKEOUT => (metrics.strikeout_position, metrics.strikeout_thickness),
            _ => unimplemented!("Invalid flag for cell line drawing specified"),
        };

        // Make sure lines are always visible.
        height = height.max(1.);

        let line_bottom = (start.line.0 as f32 + 1.) * size.cell_height;
        let baseline = line_bottom + metrics.descent;

        let mut y = (baseline - position - height / 2.).ceil();
        let max_y = line_bottom - height;
        if y > max_y {
            y = max_y;
        }

        RenderRect::new(start_x + size.padding_x, y + size.padding_y, width, height, color, 1.)
    }
}
