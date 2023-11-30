#![allow(missing_docs)]

use std::sync::Arc;

use bevy::{
    asset::AssetPath,
    ecs::entity::Entity,
    log::error,
    math::{IVec2, Vec3},
    prelude::Color,
    ui::{self, ZIndex},
};

use crate::Cursor;

use super::{
    computed::ComputedStyle, selector::Selector, selector_matcher::SelectorMatcher,
    style_expr::StyleExpr, transition::Transition,
};

/// A sharable reference to a collection of UI style properties.
#[derive(Clone)]
pub struct StyleHandle(pub Arc<StyleSet>);

/// Handle which maintains a shared reference to a set of styles and selectors.
impl StyleHandle {
    /// Build a StyleSet using a builder callback.
    pub fn build(builder_fn: impl FnOnce(&mut StyleSetBuilder) -> &mut StyleSetBuilder) -> Self {
        let mut builder = StyleSetBuilder::new();
        builder_fn(&mut builder);
        Self(Arc::new(StyleSet {
            props: builder.props,
            selectors: builder.selectors,
        }))
    }

    /// Merge the style properties into a computed `Style` object.
    pub fn apply_to<'a>(
        &self,
        computed: &mut ComputedStyle,
        matcher: &SelectorMatcher,
        entity: &Entity,
    ) {
        self.0.as_ref().apply_to(computed, matcher, entity);
    }

    /// Return the number of UiNode levels referenced by selectors.
    pub fn depth(&self) -> usize {
        self.0.as_ref().depth()
    }

    /// Return whether any of the selectors use the ':hover' pseudo-class.
    pub fn uses_hover(&self) -> bool {
        self.0.as_ref().uses_hover()
    }
}

impl PartialEq for StyleHandle {
    fn eq(&self, other: &Self) -> bool {
        // Reference-equality is all we need.
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

/// Trait that adds syntactic sugar for making lists of references to style handles.
#[doc(hidden)]
pub trait StyleRef: Send {
    fn as_handle(self) -> StyleHandle;
}

impl StyleRef for StyleHandle {
    fn as_handle(self) -> StyleHandle {
        self
    }
}

impl StyleRef for &StyleHandle {
    fn as_handle(self) -> StyleHandle {
        self.clone()
    }
}

/// Controls behavior of bevy_mod_picking
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerEvents {
    /// No pointer events for this entity, or its children
    None,
    /// Pointer events from both self and children
    All,
}

/// The set of all style attributes. This is represented as a list of enums rather than
/// a map so that attributes can be both strongly typed and represented sparsely.
#[derive(Debug, Clone)]
pub enum StyleProp {
    BackgroundImage(Option<AssetPath<'static>>),
    BackgroundColor(StyleExpr<Option<Color>>),
    BorderColor(StyleExpr<Option<Color>>),
    Color(StyleExpr<Option<Color>>),

    ZIndex(StyleExpr<Option<ui::ZIndex>>),

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

    GridAutoFlow(StyleExpr<ui::GridAutoFlow>),
    GridTemplateRows(Vec<ui::RepeatedGridTrack>),
    GridTemplateColumns(Vec<ui::RepeatedGridTrack>),
    GridAutoRows(Vec<ui::GridTrack>),
    GridAutoColumns(Vec<ui::GridTrack>),
    GridRow(StyleExpr<ui::GridPlacement>),
    GridRowStart(StyleExpr<i16>),
    GridRowSpan(StyleExpr<u16>),
    GridRowEnd(StyleExpr<i16>),
    GridColumn(StyleExpr<ui::GridPlacement>),
    GridColumnStart(StyleExpr<i16>),
    GridColumnSpan(StyleExpr<u16>),
    GridColumnEnd(StyleExpr<i16>),

    // TODO:
    // LineBreak(BreakLineOn),
    PointerEvents(StyleExpr<PointerEvents>),

    // Outlines
    OutlineColor(StyleExpr<Option<Color>>),
    OutlineWidth(StyleExpr<ui::Val>),
    OutlineOffset(StyleExpr<ui::Val>),

    // TODO: Future planned features
    Cursor(StyleExpr<Cursor>),
    CursorImage(StyleExpr<AssetPath<'static>>),
    CursorOffset(StyleExpr<IVec2>),

    // Transforms
    Scale(StyleExpr<f32>),
    ScaleX(StyleExpr<f32>),
    ScaleY(StyleExpr<f32>),
    Rotation(StyleExpr<f32>),
    Translation(StyleExpr<Vec3>),

    // Transitions
    Transition(Vec<Transition>),
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
}

impl StyleSet {
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            // vars: VarsMap::new(),
            selectors: Vec::new(),
        }
    }

    /// Build a StyleSet using a builder callback.
    pub fn build(builder_fn: impl FnOnce(&mut StyleSetBuilder) -> &mut StyleSetBuilder) -> Self {
        let mut builder = StyleSetBuilder::new();
        builder_fn(&mut builder);
        Self {
            props: builder.props,
            selectors: builder.selectors,
        }
    }

    /// Return the number of UiNode levels referenced by selectors.
    pub fn depth(&self) -> usize {
        self.selectors
            .iter()
            .map(|s| s.0.depth())
            .max()
            .unwrap_or(0)
    }

    /// Return whether any of the selectors use the ':hover' pseudo-class.
    pub fn uses_hover(&self) -> bool {
        self.selectors
            .iter()
            .map(|s| s.0.uses_hover())
            .max()
            .unwrap_or(false)
    }

    /// Merge the style properties into a computed `Style` object.
    pub fn apply_to<'a>(
        &self,
        computed: &mut ComputedStyle,
        matcher: &SelectorMatcher,
        entity: &Entity,
    ) {
        // Apply unconditional styles
        self.apply_attrs_to(&self.props, computed);

        // Apply conditional styles
        for (selector, props) in self.selectors.iter() {
            if matcher.selector_match(selector, entity) {
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

                StyleProp::GridAutoFlow(expr) => {
                    if let Ok(af) = expr.eval() {
                        computed.style.grid_auto_flow = af;
                    }
                }

                StyleProp::GridTemplateRows(expr) => {
                    computed.style.grid_template_rows.clone_from(expr);
                }

                StyleProp::GridTemplateColumns(expr) => {
                    computed.style.grid_template_columns.clone_from(expr);
                }

                StyleProp::GridAutoRows(expr) => {
                    computed.style.grid_auto_rows.clone_from(expr);
                }

                StyleProp::GridAutoColumns(expr) => {
                    computed.style.grid_auto_columns.clone_from(expr);
                }

                StyleProp::GridRow(expr) => {
                    if let Ok(af) = expr.eval() {
                        computed.style.grid_row = af;
                    }
                }
                StyleProp::GridRowStart(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_row.set_start(val);
                    }
                }

                StyleProp::GridRowSpan(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_row.set_span(val);
                    }
                }

                StyleProp::GridRowEnd(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_row.set_end(val);
                    }
                }

                StyleProp::GridColumn(expr) => {
                    if let Ok(af) = expr.eval() {
                        computed.style.grid_column = af;
                    }
                }
                StyleProp::GridColumnStart(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_column.set_start(val);
                    }
                }

                StyleProp::GridColumnSpan(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_column.set_span(val);
                    }
                }

                StyleProp::GridColumnEnd(expr) => {
                    if let Ok(val) = expr.eval() {
                        computed.style.grid_column.set_end(val);
                    }
                }

                StyleProp::OutlineColor(expr) => {
                    if let Ok(color) = expr.eval() {
                        computed.outline_color = color;
                    }
                }

                StyleProp::OutlineWidth(expr) => {
                    if let Ok(width) = expr.eval() {
                        computed.outline_width = width;
                    }
                }

                StyleProp::OutlineOffset(expr) => {
                    if let Ok(offs) = expr.eval() {
                        computed.outline_offset = offs;
                    }
                }

                StyleProp::PointerEvents(expr) => {
                    if let Ok(pickable) = expr.eval() {
                        computed.pickable = Some(pickable);
                    }
                }

                StyleProp::Cursor(_) => todo!(),
                StyleProp::CursorImage(_) => todo!(),
                StyleProp::CursorOffset(_) => todo!(),

                StyleProp::Scale(expr) => {
                    if let Ok(scale) = expr.eval() {
                        computed.scale_x = Some(scale);
                        computed.scale_y = Some(scale);
                    }
                }
                StyleProp::ScaleX(expr) => {
                    if let Ok(scale) = expr.eval() {
                        computed.scale_x = Some(scale);
                    }
                }
                StyleProp::ScaleY(expr) => {
                    if let Ok(scale) = expr.eval() {
                        computed.scale_y = Some(scale);
                    }
                }
                StyleProp::Rotation(expr) => {
                    if let Ok(rot) = expr.eval() {
                        computed.rotation = Some(rot);
                        computed.scale_y = Some(rot);
                    }
                }
                StyleProp::Translation(expr) => {
                    if let Ok(trans) = expr.eval() {
                        computed.translation = Some(trans);
                    }
                }

                StyleProp::Transition(trans) => computed.transitions.clone_from(trans),
            }
        }
    }
}

/// Trait that represents a CSS color
pub trait ColorParam {
    fn as_val(self) -> Option<Color>;
}

impl ColorParam for Option<Color> {
    fn as_val(self) -> Option<Color> {
        self
    }
}

impl ColorParam for Color {
    fn as_val(self) -> Option<Color> {
        Some(self)
    }
}

impl ColorParam for &str {
    fn as_val(self) -> Option<Color> {
        Some(Color::hex(self).unwrap())
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

/// Trait that represents a CSS Z-index
pub trait ZIndexParam {
    fn as_val(self) -> Option<ZIndex>;
}

impl ZIndexParam for ZIndex {
    fn as_val(self) -> Option<ZIndex> {
        Some(self)
    }
}

impl ZIndexParam for i32 {
    fn as_val(self) -> Option<ZIndex> {
        Some(ZIndex::Local(self))
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

impl<H: LengthParam, V: LengthParam> UiRectParam for (H, V) {
    fn as_uirect(self) -> ui::UiRect {
        ui::UiRect::axes(self.0.as_val(), self.1.as_val())
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

    pub fn background_image(&mut self, img: Option<AssetPath<'static>>) -> &mut Self {
        self.props.push(StyleProp::BackgroundImage(img));
        self
    }

    pub fn background_color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props
            .push(StyleProp::BackgroundColor(StyleExpr::Constant(
                color.as_val(),
            )));
        self
    }

    pub fn border_color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props
            .push(StyleProp::BorderColor(StyleExpr::Constant(color.as_val())));
        self
    }

    pub fn color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props
            .push(StyleProp::Color(StyleExpr::Constant(color.as_val())));
        self
    }

    pub fn z_index(&mut self, index: impl ZIndexParam) -> &mut Self {
        self.props
            .push(StyleProp::ZIndex(StyleExpr::Constant(index.as_val())));
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

    pub fn grid_auto_flow(&mut self, flow: ui::GridAutoFlow) -> &mut Self {
        self.props
            .push(StyleProp::GridAutoFlow(StyleExpr::Constant(flow)));
        self
    }

    pub fn grid_template_rows(&mut self, rows: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.props.push(StyleProp::GridTemplateRows(rows));
        self
    }

    pub fn grid_template_columns(&mut self, columns: Vec<ui::RepeatedGridTrack>) -> &mut Self {
        self.props.push(StyleProp::GridTemplateColumns(columns));
        self
    }

    pub fn grid_auto_rows(&mut self, rows: Vec<ui::GridTrack>) -> &mut Self {
        self.props.push(StyleProp::GridAutoRows(rows));
        self
    }

    pub fn grid_auto_columns(&mut self, columns: Vec<ui::GridTrack>) -> &mut Self {
        self.props.push(StyleProp::GridAutoColumns(columns));
        self
    }

    pub fn grid_row(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.props
            .push(StyleProp::GridRow(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_row_start(&mut self, val: i16) -> &mut Self {
        self.props
            .push(StyleProp::GridRowStart(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_row_span(&mut self, val: u16) -> &mut Self {
        self.props
            .push(StyleProp::GridRowSpan(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_row_end(&mut self, val: i16) -> &mut Self {
        self.props
            .push(StyleProp::GridRowEnd(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_column(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.props
            .push(StyleProp::GridColumn(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_column_start(&mut self, val: i16) -> &mut Self {
        self.props
            .push(StyleProp::GridColumnStart(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_column_span(&mut self, val: u16) -> &mut Self {
        self.props
            .push(StyleProp::GridColumnSpan(StyleExpr::Constant(val)));
        self
    }

    pub fn grid_column_end(&mut self, val: i16) -> &mut Self {
        self.props
            .push(StyleProp::GridColumnEnd(StyleExpr::Constant(val)));
        self
    }

    // LineBreak(BreakLineOn),

    pub fn outline_color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props
            .push(StyleProp::OutlineColor(StyleExpr::Constant(color.as_val())));
        self
    }

    pub fn outline_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::OutlineWidth(StyleExpr::Constant(
            length.as_val(),
        )));
        self
    }

    pub fn outline_offset(&mut self, length: impl LengthParam) -> &mut Self {
        self.props
            .push(StyleProp::OutlineOffset(StyleExpr::Constant(
                length.as_val(),
            )));
        self
    }

    pub fn pointer_events(&mut self, pe: PointerEvents) -> &mut Self {
        self.props
            .push(StyleProp::PointerEvents(StyleExpr::Constant(pe)));
        self
    }

    pub fn scale_x(&mut self, scale: f32) -> &mut Self {
        self.props
            .push(StyleProp::ScaleX(StyleExpr::Constant(scale)));
        self
    }

    pub fn scale_y(&mut self, scale: f32) -> &mut Self {
        self.props
            .push(StyleProp::ScaleY(StyleExpr::Constant(scale)));
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.props
            .push(StyleProp::Scale(StyleExpr::Constant(scale)));
        self
    }

    pub fn rotation(&mut self, rot: f32) -> &mut Self {
        self.props
            .push(StyleProp::Rotation(StyleExpr::Constant(rot)));
        self
    }

    pub fn translation(&mut self, trans: Vec3) -> &mut Self {
        self.props
            .push(StyleProp::Translation(StyleExpr::Constant(trans)));
        self
    }

    pub fn transition(&mut self, transition: &Vec<Transition>) -> &mut Self {
        self.props.push(StyleProp::Transition(transition.clone()));
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
