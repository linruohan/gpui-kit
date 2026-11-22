use std::sync::Arc;

use gpui::{HighlightStyle, Hsla, Pixels, Rems, StyleRefinement, px, rems};

use crate::highlighter::HighlightTheme;

/// TextViewStyle used to customize the style for [`super::TextView`].
///
/// This is the component-level style. It is folded onto the style the active
/// [`crate::Theme`] already derived, so a field left at its default keeps the
/// themed value rather than overriding it with a neutral one.
#[derive(Clone)]
pub struct TextViewStyle {
    /// Gap of each paragraphs, default is 0.75 rem.
    pub paragraph_gap: Rems,
    /// Base font size for headings, default is 14px.
    ///
    /// Left at the default together with [`Self::heading_font_size`], headings
    /// follow the text view's own scale, proportional to the body text.
    pub heading_base_font_size: Pixels,
    /// Function to calculate heading font size based on heading level (1-6).
    ///
    /// The first parameter is the heading level (1-6), the second parameter is
    /// the base font size.
    pub heading_font_size: Option<Arc<dyn Fn(u8, Pixels) -> Pixels + Send + Sync + 'static>>,
    /// Extra heading refinements (color, weight, border, padding) for levels
    /// 1–6. Folded onto the size the theme or [`Self::heading_font_size`] already
    /// chose. `None` leaves heading appearance to the theme.
    pub heading: Option<Arc<dyn Fn(u8) -> StyleRefinement + Send + Sync + 'static>>,
    /// Markdown link color. `None` keeps the theme link color.
    pub link: Option<Hsla>,
    /// Color for destinations marked unresolved. `None` fades the link color.
    pub unresolved_link: Option<Hsla>,
    /// Highlight theme for code blocks. Default: [`HighlightTheme::default_light()`]
    pub highlight_theme: Arc<HighlightTheme>,
    /// The style refinement for code blocks.
    ///
    /// Set `overflow.y` to `Overflow::Scroll` with a max height here to scroll
    /// long code inside the block, e.g.
    /// `TextViewStyle::default().code_block({ let mut s = StyleRefinement::default().max_h(px(320.)); s.overflow.y = Some(Overflow::Scroll); s })`.
    pub code_block: StyleRefinement,
    /// Style refinement applied to the table container (the bordered wrapper
    /// in wrap mode, the scroll viewport in horizontal-scroll mode).
    ///
    /// Set `overflow_x: scroll` here for adaptive table layout: columns fit
    /// their content when space allows, shrink (wrapping cell text) down to a
    /// per-column floor when the frame is narrower, and below that the table
    /// scrolls horizontally instead of squeezing further, e.g.
    /// `TextViewStyle::default().table({ let mut s = StyleRefinement::default(); s.overflow.x = Some(Overflow::Scroll); s })`.
    pub table: StyleRefinement,
    /// Style refinement applied to the header row (the first row) of a table,
    /// on top of the `table_head` background and foreground from the theme.
    pub table_head: StyleRefinement,
    /// Style refinement applied to each table cell.
    ///
    /// With the scroll layout, set `white_space: nowrap` here to keep cells
    /// on a single line — columns then never shrink and the table scrolls as
    /// soon as the content is wider than the frame.
    pub table_cell: StyleRefinement,
    /// The highlight style for inline code.
    ///
    /// Default is [`HighlightStyle::default()`], the `background_color` will
    /// fallback to `cx.theme().accent`, if it is `None`.
    pub inline_code: HighlightStyle,
    /// Box presentation for inline code (padding, radius, border, font).
    pub inline_code_box: StyleRefinement,
    /// Style refinement for bullet / numbered list markers.
    pub list: StyleRefinement,
    /// Style refinement for blockquote containers.
    pub blockquote: StyleRefinement,
    /// Horizontal-rule and table-frame border color. `None` keeps the theme.
    pub border: Option<Hsla>,
    /// Selected-text wash. `None` keeps the theme.
    pub selection: Option<Hsla>,
    /// Fenced code block fill. `None` keeps the theme.
    pub code_background: Option<Hsla>,
    /// Hovered table body row. `None` keeps the theme.
    pub table_row_hover: Option<Hsla>,
    /// Whether content-specific rendering should use dark-mode assets.
    pub is_dark: bool,
}

impl Default for TextViewStyle {
    fn default() -> Self {
        Self {
            paragraph_gap: rems(0.75),
            heading_base_font_size: px(14.),
            heading_font_size: None,
            heading: None,
            link: None,
            unresolved_link: None,
            highlight_theme: HighlightTheme::default_light(),
            code_block: StyleRefinement::default(),
            table: StyleRefinement::default(),
            table_head: StyleRefinement::default(),
            table_cell: StyleRefinement::default(),
            inline_code: HighlightStyle::default(),
            inline_code_box: StyleRefinement::default(),
            list: StyleRefinement::default(),
            blockquote: StyleRefinement::default(),
            border: None,
            selection: None,
            code_background: None,
            table_row_hover: None,
            is_dark: false,
        }
    }
}

impl PartialEq for TextViewStyle {
    fn eq(&self, other: &Self) -> bool {
        self.paragraph_gap == other.paragraph_gap
            && self.heading_base_font_size == other.heading_base_font_size
            && match (&self.heading_font_size, &other.heading_font_size) {
                (Some(left), Some(right)) => (1..=6).all(|level| {
                    left(level, self.heading_base_font_size)
                        == right(level, other.heading_base_font_size)
                }),
                (None, None) => true,
                _ => false,
            }
            && match (&self.heading, &other.heading) {
                (Some(left), Some(right)) => (1..=6).all(|level| left(level) == right(level)),
                (None, None) => true,
                _ => false,
            }
            && self.link == other.link
            && self.unresolved_link == other.unresolved_link
            && self.highlight_theme == other.highlight_theme
            && self.code_block == other.code_block
            && self.table == other.table
            && self.table_head == other.table_head
            && self.table_cell == other.table_cell
            && self.inline_code == other.inline_code
            && self.inline_code_box == other.inline_code_box
            && self.list == other.list
            && self.blockquote == other.blockquote
            && self.border == other.border
            && self.selection == other.selection
            && self.code_background == other.code_background
            && self.table_row_hover == other.table_row_hover
            && self.is_dark == other.is_dark
    }
}

impl TextViewStyle {
    /// Set paragraph gap, default is 0.75 rem.
    pub fn paragraph_gap(mut self, gap: Rems) -> Self {
        self.paragraph_gap = gap;
        self
    }
    /// Set the function that resolves a heading's font size from its level
    /// (1-6) and [`Self::heading_base_font_size`].
    pub fn heading_font_size<F>(mut self, f: F) -> Self
    where
        F: Fn(u8, Pixels) -> Pixels + Send + Sync + 'static,
    {
        self.heading_font_size = Some(Arc::new(f));
        self
    }
    /// Set per-level heading refinements such as color, weight, and a left bar.
    pub fn heading<F>(mut self, f: F) -> Self
    where
        F: Fn(u8) -> StyleRefinement + Send + Sync + 'static,
    {
        self.heading = Some(Arc::new(f));
        self
    }
    /// Set the Markdown link color.
    pub fn link(mut self, color: Hsla) -> Self {
        self.link = Some(color);
        self
    }
    /// Set the color of destinations marked unresolved.
    pub fn unresolved_link(mut self, color: Hsla) -> Self {
        self.unresolved_link = Some(color);
        self
    }
    /// Set style for code blocks.
    ///
    /// Set `overflow.y` to `Overflow::Scroll` with a max height on the
    /// refinement to scroll long code inside the block instead of growing it.
    pub fn code_block(mut self, style: StyleRefinement) -> Self {
        self.code_block = style;
        self
    }
    /// Set style for inline code spans.
    pub fn inline_code(mut self, style: HighlightStyle) -> Self {
        self.inline_code = style;
        self
    }
    /// Set extra style for the table container.
    ///
    /// Set `overflow_x: scroll` on the refinement for adaptive layout: cells
    /// wrap as the frame narrows, and once columns reach their minimum width
    /// the table scrolls horizontally instead of shrinking further.
    pub fn table(mut self, style: StyleRefinement) -> Self {
        self.table = style;
        self
    }
    /// Set extra style for the table header row.
    pub fn table_head(mut self, style: StyleRefinement) -> Self {
        self.table_head = style;
        self
    }
    /// Set extra style for each table cell.
    ///
    /// With the scroll table layout, `white_space: nowrap` here keeps cells
    /// on a single line and the table scrolls whenever the content is wider
    /// than the frame.
    pub fn table_cell(mut self, style: StyleRefinement) -> Self {
        self.table_cell = style;
        self
    }
    /// Set inline code padding, radius, border, and font.
    pub fn inline_code_box(mut self, style: StyleRefinement) -> Self {
        self.inline_code_box = style;
        self
    }
    /// Set list marker color, weight, and typography.
    pub fn list(mut self, style: StyleRefinement) -> Self {
        self.list = style;
        self
    }
    /// Set blockquote bar, padding, and background.
    pub fn blockquote(mut self, style: StyleRefinement) -> Self {
        self.blockquote = style;
        self
    }
    /// Set the color of horizontal rules and table borders.
    pub fn border(mut self, color: Hsla) -> Self {
        self.border = Some(color);
        self
    }
    /// Set the selected-text background.
    pub fn selection(mut self, color: Hsla) -> Self {
        self.selection = Some(color);
        self
    }
    /// Set the fenced code block fill.
    pub fn code_background(mut self, color: Hsla) -> Self {
        self.code_background = Some(color);
        self
    }
    /// Set the hovered table body row fill.
    pub fn table_row_hover(mut self, color: Hsla) -> Self {
        self.table_row_hover = Some(color);
        self
    }
}
