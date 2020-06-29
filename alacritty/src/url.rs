use std::cmp::min;
use std::mem;

use glutin::event::{ElementState, ModifiersState};
use urlocator::{UrlLocation, UrlLocator};

use font::Metrics;

use alacritty_terminal::index::{Column, Point};
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::color::Rgb;
use alacritty_terminal::term::{RenderableCell, SizeInfo};
use alacritty_terminal::text_run::{TextRun, TextRunContent};

use crate::config::Config;
use crate::event::Mouse;
use crate::renderer::rects::{RenderLine, RenderRect};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url {
    lines: Vec<RenderLine>,
    end_offset: u16,
    num_cols: Column,
}

impl Url {
    pub fn rects(&self, metrics: &Metrics, size: &SizeInfo) -> Vec<RenderRect> {
        let end = self.end();
        self.lines
            .iter()
            .filter(|line| line.start <= end)
            .map(|line| {
                let mut rect_line = *line;
                rect_line.end = min(line.end, end);
                rect_line.rects(Flags::UNDERLINE, metrics, size)
            })
            .flatten()
            .collect()
    }

    pub fn start(&self) -> Point {
        self.lines[0].start
    }

    pub fn end(&self) -> Point {
        self.lines[self.lines.len() - 1].end.sub(self.num_cols, self.end_offset as usize)
    }
}

pub struct Urls {
    locator: UrlLocator,
    urls: Vec<Url>,
    scheme_buffer: Vec<RenderableCell>,
    last_point: Option<Point>,
    state: UrlLocation,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            locator: UrlLocator::new(),
            scheme_buffer: Vec::new(),
            urls: Vec::new(),
            state: UrlLocation::Reset,
            last_point: None,
        }
    }
}

impl Urls {
    pub fn new() -> Self {
        Self::default()
    }

    // Update tracked URLs.
    pub fn update(&mut self, num_cols: Column, text_run: &TextRun) {
        // Convert cell to character.
        let has_wide_char_spacer = text_run.flags.contains(Flags::WIDE_CHAR_SPACER);
        let has_wrapline = text_run.flags.contains(Flags::WRAPLINE);
        if let TextRunContent::CharRun(run, _) = &text_run.content {
            let step = if text_run.flags.contains(Flags::WIDE_CHAR) { 2 } else { 1 };
            let mut cell = text_run.start_cell();
            for c in run.chars() {
                let point: Point = cell.into();
                let end = point;
                // Reset URL when empty cells have been skipped.
                if point != Point::default() && Some(point.sub(num_cols, 1)) != self.last_point {
                    self.reset();
                }

                self.last_point = Some(end);

                // Extend current state if a wide char spacer is encountered.
                if has_wide_char_spacer {
                    if let UrlLocation::Url(_, mut end_offset) = self.state {
                        if end_offset != 0 {
                            end_offset += 1;
                        }

                        self.extend_url(point, end, cell.fg, end_offset);
                    }

                    continue;
                }

                // Advance parser.
                let last_state = mem::replace(&mut self.state, self.locator.advance(c));
                match (self.state, last_state) {
                    (UrlLocation::Url(_length, end_offset), UrlLocation::Scheme) => {
                        // Create empty URL.
                        self.urls.push(Url { lines: Vec::new(), end_offset, num_cols });

                        // Push schemes into URL.
                        for scheme_cell in self.scheme_buffer.split_off(0) {
                            let point = scheme_cell.into();
                            self.extend_url(point, point, scheme_cell.fg, end_offset);
                        }

                        // Push the new cell into URL.
                        self.extend_url(point, end, cell.fg, end_offset);
                    },
                    (UrlLocation::Url(_length, end_offset), UrlLocation::Url(..)) => {
                        self.extend_url(point, end, cell.fg, end_offset);
                    },
                    (UrlLocation::Scheme, _) => self.scheme_buffer.push(cell),
                    (UrlLocation::Reset, _) => self.reset(),
                    _ => (),
                }

                // Reset at un-wrapped linebreak.
                if cell.column + 1 == num_cols && !has_wrapline {
                    self.reset();
                }
                cell.column += step;
            }
        }
    }

    /// Extend the last URL.
    fn extend_url(&mut self, start: Point, end: Point, color: Rgb, end_offset: u16) {
        let url = self.urls.last_mut().unwrap();

        // If color changed, we need to insert a new line.
        if url.lines.last().map(|last| last.color) == Some(color) {
            url.lines.last_mut().unwrap().end = end;
        } else {
            url.lines.push(RenderLine { color, start, end });
        }

        // Update excluded cells at the end of the URL.
        url.end_offset = end_offset;
    }

    /// Find URL below the mouse cursor.
    pub fn highlighted(
        &self,
        config: &Config,
        mouse: &Mouse,
        mods: ModifiersState,
        mouse_mode: bool,
        selection: bool,
    ) -> Option<Url> {
        // Require additional shift in mouse mode.
        let mut required_mods = config.ui_config.mouse.url.mods();
        if mouse_mode {
            required_mods |= ModifiersState::SHIFT;
        }

        // Make sure all prerequisites for highlighting are met.
        if selection
            || !mouse.inside_grid
            || config.ui_config.mouse.url.launcher.is_none()
            || required_mods != mods
            || mouse.left_button_state == ElementState::Pressed
        {
            return None;
        }

        self.find_at(Point::new(mouse.line, mouse.column))
    }

    /// Find URL at location.
    pub fn find_at(&self, point: Point) -> Option<Url> {
        for url in &self.urls {
            if (url.start()..=url.end()).contains(&point) {
                return Some(url.clone());
            }
        }
        None
    }

    fn reset(&mut self) {
        self.locator = UrlLocator::new();
        self.state = UrlLocation::Reset;
        self.scheme_buffer.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alacritty_terminal::index::{Column, Line};
    use alacritty_terminal::term::cell::MAX_ZEROWIDTH_CHARS;

    fn text_to_text_run(start: usize, text: &str) -> TextRun {
        let chars: String = text.chars().collect();
        let extras = vec![[char::default(); MAX_ZEROWIDTH_CHARS]; chars.len()];
        TextRun {
            line: Line(0),
            span: (Column(start), Column(chars.len() - 1 + start)),
            content: TextRunContent::CharRun(chars, extras),
            fg: Default::default(),
            bg: Default::default(),
            bg_alpha: 0.,
            flags: Flags::empty(),
            data: None,
        }
    }

    fn text_to_text_runs(texts: &[&str]) -> (Vec<TextRun>, usize) {
        let mut out = vec![];
        let mut index = 0;
        for text in texts {
            let text_run = text_to_text_run(index, text);
            out.push(text_run);
            index += text.len();
        }
        (out, index)
    }

    #[test]
    fn multi_color_url() {
        let (mut input, num_cols) = text_to_text_runs(&["test http", "s", "://example.org ing"]);
        input[1].fg = Rgb { r: 0xff, g: 0x00, b: 0xff };

        let mut urls = Urls::new();

        for text_run in input.iter() {
            urls.update(Column(num_cols), text_run);
        }

        let url = urls.urls.first().unwrap();
        assert_eq!(url.start().col, Column(5));
        assert_eq!(url.end().col, Column(23));
    }

    #[test]
    fn multiple_urls() {
        let (input, num_cols) = text_to_text_runs(&["test git:a git:b git:c ing"]);

        let mut urls = Urls::new();

        for text_run in &input {
            urls.update(Column(num_cols), text_run);
        }

        assert_eq!(urls.urls.len(), 3);

        assert_eq!(urls.urls[0].start().col, Column(5));
        assert_eq!(urls.urls[0].end().col, Column(9));

        assert_eq!(urls.urls[1].start().col, Column(11));
        assert_eq!(urls.urls[1].end().col, Column(15));

        assert_eq!(urls.urls[2].start().col, Column(17));
        assert_eq!(urls.urls[2].end().col, Column(21));
    }
}
