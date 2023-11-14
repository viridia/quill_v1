use bevy::{
    log::error,
    prelude::{Color, Handle, Image},
    ui,
    utils::HashSet,
};

use super::{computed::ComputedStyle, selector::Selector, style_expr::StyleExpr};

/// Controls behavior of bevy_mod_picking
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerEvents {
    /// No pointer events for this entity, or its children
    None,
    /// Pointer events from children only
    Children,
    /// Pointer events from attached entity but not child elements
    SelfOnly,
    /// Pointer events from both self and children
    All,
}

#[derive(Debug, Clone)]
pub enum StyleProp {
    BackgroundImage(Option<Handle<Image>>),
    BackgroundColor(StyleExpr<Option<Color>>),
    BorderColor(StyleExpr<Option<Color>>),
    Color(StyleExpr<Option<Color>>),

    ZIndex(StyleExpr<Option<i32>>),

    Display(StyleExpr<ui::Display>),
    Position(StyleExpr<ui::PositionType>),
    Overflow(StyleExpr<ui::OverflowAxis>),
    OverflowX(StyleExpr<ui::OverflowAxis>),
    OverflowY(StyleExpr<ui::OverflowAxis>),
    Direction(StyleExpr<ui::Direction>),

    Left(StyleExpr<ui::Val>),
    Right(StyleExpr<ui::Val>),
    Top(StyleExpr<ui::Val>),
    Bottom(StyleExpr<ui::Val>),

    Width(StyleExpr<ui::Val>),
    Height(StyleExpr<ui::Val>),
    MinWidth(StyleExpr<ui::Val>),
    MinHeight(StyleExpr<ui::Val>),
    MaxWidth(StyleExpr<ui::Val>),
    MaxHeight(StyleExpr<ui::Val>),
    // // pub aspect_ratio: StyleProp<f32>,

    // Allow margin sides to be set individually
    Margin(StyleExpr<ui::UiRect>),
    MarginLeft(StyleExpr<ui::Val>),
    MarginRight(StyleExpr<ui::Val>),
    MarginTop(StyleExpr<ui::Val>),
    MarginBottom(StyleExpr<ui::Val>),

    Padding(StyleExpr<ui::UiRect>),
    PaddingLeft(StyleExpr<ui::Val>),
    PaddingRight(StyleExpr<ui::Val>),
    PaddingTop(StyleExpr<ui::Val>),
    PaddingBottom(StyleExpr<ui::Val>),

    Border(StyleExpr<ui::UiRect>),
    BorderLeft(StyleExpr<ui::Val>),
    BorderRight(StyleExpr<ui::Val>),
    BorderTop(StyleExpr<ui::Val>),
    BorderBottom(StyleExpr<ui::Val>),

    FlexDirection(StyleExpr<ui::FlexDirection>),
    FlexWrap(StyleExpr<ui::FlexWrap>),
    // Flex(ExprList),
    FlexGrow(StyleExpr<f32>),
    FlexShrink(StyleExpr<f32>),
    FlexBasis(StyleExpr<ui::Val>),
    RowGap(StyleExpr<ui::Val>),
    ColumnGap(StyleExpr<ui::Val>),
    Gap(StyleExpr<ui::Val>),

    AlignItems(StyleExpr<ui::AlignItems>),
    AlignSelf(StyleExpr<ui::AlignSelf>),
    AlignContent(StyleExpr<ui::AlignContent>),
    JustifyItems(StyleExpr<ui::JustifyItems>),
    JustifySelf(StyleExpr<ui::JustifySelf>),
    JustifyContent(StyleExpr<ui::JustifyContent>),
    // TODO:
    // GridAutoFlow(bevy::ui::GridAutoFlow),
    // // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_auto_rows: Option<Vec<GridTrack>>,
    // // pub grid_auto_columns: Option<Vec<GridTrack>>,
    // GridRow(bevy::ui::GridPlacement),
    // GridRowStart(StyleExpr<i16>),
    // GridRowSpan(StyleExpr<u16>),
    // GridRowEnd(i16),
    // GridColumn(bevy::ui::GridPlacement),
    // GridColumnStart(i16),
    // GridColumnSpan(u16),
    // GridColumnEnd(i16),

    // LineBreak(BreakLineOn),
    PointerEvents(StyleExpr<PointerEvents>),
}

type SelectorList = Vec<(Box<Selector>, Vec<StyleProp>)>;

/// A collection of style attributes which can be merged to create a `ComputedStyle`.
#[derive(Debug, Default, Clone)]
pub struct StyleSet {
    /// List of style attributes.
    /// Rather than storing the attributes in a struct full of optional fields, we store a flat
    /// vector of enums, each of which stores a single style attribute. This "sparse" representation
    /// allows for fast merging of styles, particularly for styles which have few or no attributes.
    props: Vec<StyleProp>,
    // /// List of style variables to define when this style is invoked.
    // #[reflect(ignore)]
    // vars: VarsMap,
    /// List of conditional styles
    pub(crate) selectors: SelectorList,

    /// How many entity ancestor levels need to be checked when styles change.
    ancestor_depth: usize,
}

impl StyleSet {
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            // vars: VarsMap::new(),
            selectors: Vec::new(),
            ancestor_depth: 0,
        }
    }

    /// Build a StyleSet using a builder callback.
    pub fn build(builder_fn: impl FnOnce(&mut StyleSetBuilder) -> &mut StyleSetBuilder) -> Self {
        let mut builder = StyleSetBuilder::new();
        builder_fn(&mut builder);
        let depth = builder
            .selectors
            .iter()
            .map(|s| s.0.depth())
            .max()
            .unwrap_or(0);
        Self {
            props: builder.props,
            selectors: builder.selectors,
            ancestor_depth: depth,
        }
    }

    /// Return the number of UiNode levels reference by selectors.
    pub fn depth(&self) -> usize {
        self.ancestor_depth
    }

    /// Merge the style properties into a computed `Style` object.
    pub fn apply_to(&self, computed: &mut ComputedStyle, classes: &[HashSet<String>]) {
        // Apply unconditional styles
        self.apply_attrs_to(&self.props, computed);

        // Apply conditional styles
        for (selector, props) in self.selectors.iter() {
            if selector.match_classes(classes) {
                self.apply_attrs_to(&props, computed);
            }
        }
    }

    fn apply_attrs_to(&self, attrs: &Vec<StyleProp>, computed: &mut ComputedStyle) {
        for attr in attrs.iter() {
            match attr {
                StyleProp::BackgroundImage(image) => {
                    computed.image = image.clone();
                }
                StyleProp::BackgroundColor(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.background_color = color;
                    }
                }
                StyleProp::BorderColor(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.border_color = color;
                    }
                }
                StyleProp::Color(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.color = color;
                    }
                }
                StyleProp::ZIndex(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.z_index = val;
                    }
                }
                StyleProp::Display(expr) => {
                    if let Ok(disp) = expr.eval() {
                        computed.style.display = disp;
                    }
                }
                StyleProp::Position(expr) => {
                    if let Ok(pos) = expr.eval() {
                        computed.style.position_type = pos;
                    }
                }
                StyleProp::OverflowX(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.x = ov;
                    }
                }
                StyleProp::OverflowY(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.y = ov;
                    }
                }
                StyleProp::Overflow(expr) => {
                    if let Ok(ov) = expr.eval() {
                        computed.style.overflow.x = ov;
                        computed.style.overflow.y = ov;
                    }
                }
                StyleProp::Direction(expr) => {
                    if let Ok(dir) = expr.eval() {
                        computed.style.direction = dir;
                    }
                }
                StyleProp::Left(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.left = length;
                    }
                }
                StyleProp::Right(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.right = length;
                    }
                }
                StyleProp::Top(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.top = length;
                    }
                }
                StyleProp::Bottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.bottom = length;
                    }
                }
                StyleProp::Width(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.width = length;
                    }
                }
                StyleProp::Height(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.height = length;
                    }
                }
                StyleProp::MinWidth(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.min_width = length;
                    }
                }
                StyleProp::MinHeight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.min_height = length;
                    }
                }
                StyleProp::MaxWidth(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.max_width = length;
                    }
                }
                StyleProp::MaxHeight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.max_height = length;
                    }
                }
                StyleProp::Margin(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.margin = rect;
                    }
                }
                StyleProp::MarginLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.left = length;
                    }
                }
                StyleProp::MarginRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.right = length;
                    }
                }
                StyleProp::MarginTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.top = length;
                    }
                }
                StyleProp::MarginBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.margin.bottom = length;
                    }
                }
                StyleProp::Padding(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.padding = rect;
                    }
                }
                StyleProp::PaddingLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.left = length;
                    }
                }
                StyleProp::PaddingRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.right = length;
                    }
                }
                StyleProp::PaddingTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.top = length;
                    }
                }
                StyleProp::PaddingBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.padding.bottom = length;
                    }
                }
                StyleProp::Border(expr) => {
                    if let Ok(rect) = expr.eval() {
                        computed.style.border = rect;
                    }
                }
                StyleProp::BorderLeft(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.left = length;
                    }
                }
                StyleProp::BorderRight(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.right = length;
                    }
                }
                StyleProp::BorderTop(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.top = length;
                    }
                }
                StyleProp::BorderBottom(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.border.bottom = length;
                    }
                }
                StyleProp::FlexDirection(expr) => {
                    if let Ok(dir) = expr.eval() {
                        computed.style.flex_direction = dir;
                    }
                }
                StyleProp::FlexWrap(expr) => {
                    if let Ok(wrap) = expr.eval() {
                        computed.style.flex_wrap = wrap;
                    }
                }
                StyleProp::FlexGrow(expr) => {
                    if let Ok(amt) = expr.eval() {
                        computed.style.flex_grow = amt;
                    }
                }
                StyleProp::FlexShrink(expr) => {
                    if let Ok(amt) = expr.eval() {
                        computed.style.flex_shrink = amt;
                    }
                }
                StyleProp::FlexBasis(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.flex_basis = length;
                    }
                }
                StyleProp::ColumnGap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.column_gap = length;
                    }
                }
                StyleProp::RowGap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.row_gap = length;
                    }
                }
                StyleProp::Gap(expr) => {
                    if let Ok(length) = expr.eval() {
                        computed.style.column_gap = length;
                        computed.style.row_gap = length;
                    }
                }

                StyleProp::AlignItems(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_items = align;
                    }
                }
                StyleProp::AlignSelf(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_self = align;
                    }
                }
                StyleProp::AlignContent(expr) => {
                    if let Ok(align) = expr.eval() {
                        computed.style.align_content = align;
                    }
                }
                StyleProp::JustifyItems(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_items = justify;
                    }
                }
                StyleProp::JustifySelf(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_self = justify;
                    }
                }
                StyleProp::JustifyContent(expr) => {
                    if let Ok(justify) = expr.eval() {
                        computed.style.justify_content = justify;
                    }
                }

                StyleProp::PointerEvents(expr) => {
                    if let Ok(pickable) = expr.eval() {
                        computed.pickable = Some(pickable);
                    }
                }
            }
        }
    }
}

/// Trait that represents a CSS "length"
pub trait LengthParam {
    fn as_val(self) -> ui::Val;
}

impl LengthParam for ui::Val {
    fn as_val(self) -> ui::Val {
        self
    }
}

impl LengthParam for f32 {
    fn as_val(self) -> ui::Val {
        ui::Val::Px(self)
    }
}

impl LengthParam for i32 {
    fn as_val(self) -> ui::Val {
        ui::Val::Px(self as f32)
    }
}

/// Trait that represents CSS edge widths (margin, padding, etc.)
pub trait UiRectParam {
    fn as_uirect(self) -> ui::UiRect;
}

impl UiRectParam for ui::UiRect {
    fn as_uirect(self) -> ui::UiRect {
        self
    }
}

impl UiRectParam for ui::Val {
    fn as_uirect(self) -> ui::UiRect {
        ui::UiRect::all(self)
    }
}

impl UiRectParam for f32 {
    fn as_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self))
    }
}

impl UiRectParam for i32 {
    fn as_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self as f32))
    }
}

pub struct StyleSetBuilder {
    props: Vec<StyleProp>,
    selectors: SelectorList,
}

impl StyleSetBuilder {
    fn new() -> Self {
        Self {
            props: Vec::new(),
            selectors: Vec::new(),
        }
    }

    pub fn background_image(&mut self, img: Option<Handle<Image>>) -> &mut Self {
        self.props.push(StyleProp::BackgroundImage(img));
        self
    }

    pub fn background_color(&mut self, color: Option<Color>) -> &mut Self {
        self.props
            .push(StyleProp::BackgroundColor(StyleExpr::Constant(color)));
        self
    }

    pub fn border_color(&mut self, color: Option<Color>) -> &mut Self {
        self.props
            .push(StyleProp::BorderColor(StyleExpr::Constant(color)));
        self
    }

    pub fn color(&mut self, color: Option<Color>) -> &mut Self {
        self.props
            .push(StyleProp::Color(StyleExpr::Constant(color)));
        self
    }

    pub fn z_index(&mut self, index: Option<i32>) -> &mut Self {
        self.props
            .push(StyleProp::ZIndex(StyleExpr::Constant(index)));
        self
    }

    pub fn display(&mut self, disp: ui::Display) -> &mut Self {
        self.props
            .push(StyleProp::Display(StyleExpr::Constant(disp)));
        self
    }

    pub fn position(&mut self, pos: ui::PositionType) -> &mut Self {
        self.props
            .push(StyleProp::Position(StyleExpr::Constant(pos)));
        self
    }

    pub fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props
            .push(StyleProp::Overflow(StyleExpr::Constant(ov)));
        self
    }

    pub fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props
            .push(StyleProp::OverflowX(StyleExpr::Constant(ov)));
        self
    }

    pub fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props
            .push(StyleProp::OverflowY(StyleExpr::Constant(ov)));
        self
    }

    pub fn direction(&mut self, dir: ui::Direction) -> &mut Self {
        self.props
            .push(StyleProp::Direction(StyleExpr::Constant(dir)));
        self
    }

    pub fn left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Left(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Right(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Top(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Bottom(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Width(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Height(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn min_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MinWidth(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn min_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MinHeight(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn max_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MaxWidth(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn max_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MaxHeight(StyleExpr::Constant(length.as_val())));
        self
    }

    // pub aspect_ratio: StyleProp<f32>,

    pub fn margin(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props
            .push(StyleProp::Margin(StyleExpr::Constant(rect.as_uirect())));
        self
    }

    pub fn margin_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MarginLeft(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn margin_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MarginRight(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn margin_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::MarginTop(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn margin_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MarginBottom(StyleExpr::Constant(
            length.as_val(),
        )));
        self
    }

    pub fn padding(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props
            .push(StyleProp::Padding(StyleExpr::Constant(rect.as_uirect())));
        self
    }

    pub fn padding_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::PaddingLeft(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn padding_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::PaddingRight(StyleExpr::Constant(
            length.as_val(),
        )));
        self
    }

    pub fn padding_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::PaddingTop(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn padding_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::PaddingBottom(StyleExpr::Constant(
                length.as_val(),
            )));
        self
    }

    pub fn border(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props
            .push(StyleProp::Border(StyleExpr::Constant(rect.as_uirect())));
        self
    }

    pub fn border_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::BorderLeft(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn border_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::BorderRight(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn border_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::BorderTop(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn border_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::BorderBottom(StyleExpr::Constant(
            length.as_val(),
        )));
        self
    }

    pub fn flex_direction(&mut self, dir: ui::FlexDirection) -> &mut Self {
        self.props
            .push(StyleProp::FlexDirection(StyleExpr::Constant(dir)));
        self
    }

    pub fn flex_wrap(&mut self, w: ui::FlexWrap) -> &mut Self {
        self.props.push(StyleProp::FlexWrap(StyleExpr::Constant(w)));
        self
    }

    // Flex(ExprList),

    pub fn flex_grow(&mut self, n: f32) -> &mut Self {
        self.props.push(StyleProp::FlexGrow(StyleExpr::Constant(n)));
        self
    }

    pub fn flex_shrink(&mut self, n: f32) -> &mut Self {
        self.props
            .push(StyleProp::FlexShrink(StyleExpr::Constant(n)));
        self
    }

    pub fn flex_basis(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::FlexBasis(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn row_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::RowGap(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn column_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::ColumnGap(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::Gap(StyleExpr::Constant(length.as_val())));
        self
    }

    pub fn align_items(&mut self, align: ui::AlignItems) -> &mut Self {
        self.props
            .push(StyleProp::AlignItems(StyleExpr::Constant(align)));
        self
    }

    pub fn align_self(&mut self, align: ui::AlignSelf) -> &mut Self {
        self.props
            .push(StyleProp::AlignSelf(StyleExpr::Constant(align)));
        self
    }

    pub fn align_content(&mut self, align: ui::AlignContent) -> &mut Self {
        self.props
            .push(StyleProp::AlignContent(StyleExpr::Constant(align)));
        self
    }

    pub fn justify_items(&mut self, justify: ui::JustifyItems) -> &mut Self {
        self.props
            .push(StyleProp::JustifyItems(StyleExpr::Constant(justify)));
        self
    }

    pub fn justify_self(&mut self, justify: ui::JustifySelf) -> &mut Self {
        self.props
            .push(StyleProp::JustifySelf(StyleExpr::Constant(justify)));
        self
    }

    pub fn justify_content(&mut self, justify: ui::JustifyContent) -> &mut Self {
        self.props
            .push(StyleProp::JustifyContent(StyleExpr::Constant(justify)));
        self
    }

    // // TODO:
    // GridAutoFlow(bevy::ui::GridAutoFlow),
    // // pub grid_template_rows: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_template_columns: Option<Vec<RepeatedGridTrack>>,
    // // pub grid_auto_rows: Option<Vec<GridTrack>>,
    // // pub grid_auto_columns: Option<Vec<GridTrack>>,
    // GridRow(bevy::ui::GridPlacement),
    // GridRowStart(StyleExpr<i16>),
    // GridRowSpan(StyleExpr<u16>),
    // GridRowEnd(i16),
    // GridColumn(bevy::ui::GridPlacement),
    // GridColumnStart(i16),
    // GridColumnSpan(u16),
    // GridColumnEnd(i16),

    // LineBreak(BreakLineOn),

    pub fn pointer_events(&mut self, pe: PointerEvents) -> &mut Self {
        self.props
            .push(StyleProp::PointerEvents(StyleExpr::Constant(pe)));
        self
    }

    /// Add a selector expression to this style declaration.
    pub fn selector(
        &mut self,
        mut expr: &str,
        builder_fn: impl FnOnce(&mut StyleSetBuilder) -> &mut StyleSetBuilder,
    ) -> &mut Self {
        let mut builder = StyleSetBuilder::new();
        builder_fn(&mut builder);
        match Selector::parser(&mut expr) {
            Ok(selector) => {
                self.selectors.push((selector, builder.props));
            }
            Err(err) => {
                error!("{}: {}", err, expr)
            }
        }
        self
    }
}
