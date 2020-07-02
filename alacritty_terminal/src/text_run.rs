use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};

use crate::ansi::Color;
use crate::config::Config;
use crate::gl::types::*;
use crate::grid::Indexed;
use crate::index::{Column, Line, Point};
use crate::term::cell::{Cell, Flags, MAX_ZEROWIDTH_CHARS};
use crate::term::color::{self, Rgb};
use crate::term::{CursorKey, RenderableCell, RenderableCellContent};

#[derive(Copy, Debug, Clone, Default)]
pub struct Glyph {
    pub tex_id: GLuint,
    pub colored: bool,
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
    pub uv_bot: f32,
    pub uv_left: f32,
    pub uv_width: f32,
    pub uv_height: f32,
}

#[derive(Debug)]
pub struct RunStart {
    pub line: Line,
    pub column: Column,
    pub fg: Color,
    pub bg: Color,
    pub selected: bool,
    pub flags: Flags,
}

impl RunStart {
    /// Compare cell and check if it belongs to the same run.
    #[inline]
    pub fn belongs_to_text_run(&self, cell: &Indexed<Cell>, selected: bool) -> bool {
        self.line == cell.line
            && self.fg == cell.fg
            && self.bg == cell.bg
            && self.flags == cell.flags
            && self.selected == selected
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum TextRunContent {
    Cursor(CursorKey),
    CharRun(String, Vec<[char; MAX_ZEROWIDTH_CHARS]>),
}

/// Represents a set of renderable cells that all share the same rendering properties.
/// The assumption is that if two cells are in the same TextRun they can be sent off together to
/// be shaped. This allows for ligatures to be rendered but not when something breaks up a ligature
/// (e.g. selection highlight) which is desired behavior.
#[derive(Debug, Clone)]
pub struct TextRun {
    /// A run never spans multiple lines.
    pub line: Line,
    /// Span of columns the text run covers.
    pub span: (Column, Column),
    /// Cursor of sequence of characters.
    pub content: TextRunContent,
    /// Foreground color of text run content.
    pub fg: Rgb,
    /// Background color of text run content.
    pub bg: Rgb,
    /// Background color opacity of the text run.
    pub bg_alpha: f32,
    /// Attributes of this text run.
    pub flags: Flags,
    /// cached glyph and cell for rendering.
    pub data: Option<Vec<(RenderableCell, Glyph)>>,
}

impl Hash for TextRun {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.span.1 - self.span.0).hash(state);
        self.content.hash(state);
        self.flags.hash(state);
    }
}

impl PartialEq for TextRun {
    fn eq(&self, other: &Self) -> bool {
        (self.span.1 - self.span.0) == (other.span.1 - other.span.0)
            && self.content == other.content
            && self.flags == other.flags
    }
}

impl Eq for TextRun {}

impl TextRun {
    pub fn from_cursor_key<C>(
        config: &Config<C>,
        colors: &color::List,
        start: RunStart,
        cursor: CursorKey,
    ) -> Self {
        let (fg, bg, bg_alpha) = RenderableCell::color_to_rgb(config, colors, &start);
        TextRun {
            line: start.line,
            span: (start.column, start.column),
            content: TextRunContent::Cursor(cursor),
            fg,
            bg,
            bg_alpha,
            flags: start.flags,
            data: None,
        }
    }

    #[inline]
    pub fn update_from_data(&mut self, other: &Self) {
        if let Some(data) = &other.data {
            let mut data = data.clone();
            if other.line != self.line {
                for (cell, _) in &mut data {
                    cell.line = self.line;
                }
            }
            let start = other.span.0;
            if start != self.span.0 {
                for (cell, _) in &mut data {
                    cell.column = self.span.0 + cell.column - start;
                }
            }
            if other.fg != self.fg {
                for (cell, _) in &mut data {
                    cell.fg = self.fg;
                }
            }
            if other.bg != self.bg {
                for (cell, _) in &mut data {
                    cell.bg = self.bg;
                }
            }
            if other.bg_alpha.to_bits() != self.bg_alpha.to_bits() {
                for (cell, _) in &mut data {
                    cell.bg_alpha = self.bg_alpha;
                }
            }
            self.data = Some(data);
        }
    }

    /// Returns dummy RenderableCell containing no content with positioning and color information
    /// from this TextRun.
    #[inline]
    fn dummy_cell_at(&self, col: Column) -> RenderableCell {
        RenderableCell {
            line: self.line,
            column: col,
            inner: RenderableCellContent::Chars([' '; crate::term::cell::MAX_ZEROWIDTH_CHARS + 1]),
            fg: self.fg,
            bg: self.bg,
            bg_alpha: self.bg_alpha,
            flags: self.flags,
        }
    }

    /// First cell in the TextRun
    #[inline]
    pub fn start_cell(&self) -> RenderableCell {
        self.dummy_cell_at(self.span.0)
    }

    /// First point covered by this TextRun
    #[inline]
    pub fn start_point(&self) -> Point {
        Point { line: self.line, col: self.span.0 }
    }

    /// End point covered by this TextRun
    #[inline]
    pub fn end_point(&self) -> Point {
        Point { line: self.line, col: self.span.1 }
    }
}
