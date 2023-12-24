#![allow(missing_docs)]

use bevy::{
    asset::AssetPath,
    ecs::entity::Entity,
    math::{IVec2, Vec3},
    prelude::*,
    ui,
};

use crate::Cursor;

use super::{
    builder::StyleBuilder, computed::ComputedStyle, selector::Selector,
    selector_matcher::SelectorMatcher, transition::Transition,
};

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
    BackgroundColor(Option<Color>),
    BorderColor(Option<Color>),
    Color(Option<Color>),

    ZIndex(Option<ui::ZIndex>),

    Display(ui::Display),
    Position(ui::PositionType),
    Overflow(ui::OverflowAxis),
    OverflowX(ui::OverflowAxis),
    OverflowY(ui::OverflowAxis),
    Direction(ui::Direction),

    Left(ui::Val),
    Right(ui::Val),
    Top(ui::Val),
    Bottom(ui::Val),

    Width(ui::Val),
    Height(ui::Val),
    MinWidth(ui::Val),
    MinHeight(ui::Val),
    MaxWidth(ui::Val),
    MaxHeight(ui::Val),
    // // pub aspect_ratio: StyleProp<f32>,

    // Allow margin sides to be set individually
    Margin(ui::UiRect),
    MarginLeft(ui::Val),
    MarginRight(ui::Val),
    MarginTop(ui::Val),
    MarginBottom(ui::Val),

    Padding(ui::UiRect),
    PaddingLeft(ui::Val),
    PaddingRight(ui::Val),
    PaddingTop(ui::Val),
    PaddingBottom(ui::Val),

    Border(ui::UiRect),
    BorderLeft(ui::Val),
    BorderRight(ui::Val),
    BorderTop(ui::Val),
    BorderBottom(ui::Val),

    FlexDirection(ui::FlexDirection),
    FlexWrap(ui::FlexWrap),
    // Flex(ExprList),
    FlexGrow(f32),
    FlexShrink(f32),
    FlexBasis(ui::Val),
    RowGap(ui::Val),
    ColumnGap(ui::Val),
    Gap(ui::Val),

    AlignItems(ui::AlignItems),
    AlignSelf(ui::AlignSelf),
    AlignContent(ui::AlignContent),
    JustifyItems(ui::JustifyItems),
    JustifySelf(ui::JustifySelf),
    JustifyContent(ui::JustifyContent),

    GridAutoFlow(ui::GridAutoFlow),
    GridTemplateRows(Vec<ui::RepeatedGridTrack>),
    GridTemplateColumns(Vec<ui::RepeatedGridTrack>),
    GridAutoRows(Vec<ui::GridTrack>),
    GridAutoColumns(Vec<ui::GridTrack>),
    GridRow(ui::GridPlacement),
    GridRowStart(i16),
    GridRowSpan(u16),
    GridRowEnd(i16),
    GridColumn(ui::GridPlacement),
    GridColumnStart(i16),
    GridColumnSpan(u16),
    GridColumnEnd(i16),

    // TODO:
    // LineBreak(BreakLineOn),
    PointerEvents(PointerEvents),

    // Text
    Font(Option<AssetPath<'static>>),
    FontSize(f32),

    // Outlines
    OutlineColor(Option<Color>),
    OutlineWidth(ui::Val),
    OutlineOffset(ui::Val),

    // TODO: Future planned features
    Cursor(Cursor),
    CursorImage(AssetPath<'static>),
    CursorOffset(IVec2),

    // Transforms
    Scale(f32),
    ScaleX(f32),
    ScaleY(f32),
    Rotation(f32),
    Translation(Vec3),

    // Transitions
    Transition(Vec<Transition>),
}

pub(crate) type SelectorList = Vec<(Box<Selector>, Vec<StyleProp>)>;

/// A collection of style attributes which can be merged to create a `ComputedStyle`.
#[derive(Debug, Default, Clone)]
pub struct StyleSet {
    /// List of style attributes.
    /// Rather than storing the attributes in a struct full of optional fields, we store a flat
    /// vector of enums, each of which stores a single style attribute. This "sparse" representation
    /// allows for fast merging of styles, particularly for styles which have few or no attributes.
    pub(crate) props: Vec<StyleProp>,

    /// List of conditional styles
    pub(crate) selectors: SelectorList,
}

impl StyleSet {
    pub fn new() -> Self {
        Self {
            props: Vec::new(),
            selectors: Vec::new(),
        }
    }

    /// Build a StyleSet using a builder callback.
    pub fn build(builder_fn: impl FnOnce(&mut StyleBuilder) -> &mut StyleBuilder) -> Self {
        let mut builder = StyleBuilder::new();
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
        self.selectors.iter().any(|s| s.0.uses_hover())
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
                    computed.background_color = *expr;
                }
                StyleProp::BorderColor(expr) => {
                    computed.border_color = *expr;
                }
                StyleProp::Color(expr) => {
                    computed.color = *expr;
                }
                StyleProp::ZIndex(expr) => {
                    computed.z_index = *expr;
                }
                StyleProp::Display(expr) => {
                    computed.style.display = *expr;
                }
                StyleProp::Position(expr) => {
                    computed.style.position_type = *expr;
                }
                StyleProp::OverflowX(expr) => {
                    computed.style.overflow.x = *expr;
                }
                StyleProp::OverflowY(expr) => {
                    computed.style.overflow.y = *expr;
                }
                StyleProp::Overflow(expr) => {
                    computed.style.overflow.x = *expr;
                    computed.style.overflow.y = *expr;
                }

                StyleProp::Direction(expr) => {
                    computed.style.direction = *expr;
                }
                StyleProp::Left(expr) => {
                    computed.style.left = *expr;
                }
                StyleProp::Right(expr) => {
                    computed.style.right = *expr;
                }
                StyleProp::Top(expr) => {
                    computed.style.top = *expr;
                }
                StyleProp::Bottom(expr) => {
                    computed.style.bottom = *expr;
                }
                StyleProp::Width(expr) => {
                    computed.style.width = *expr;
                }
                StyleProp::Height(expr) => {
                    computed.style.height = *expr;
                }
                StyleProp::MinWidth(expr) => {
                    computed.style.min_width = *expr;
                }
                StyleProp::MinHeight(expr) => {
                    computed.style.min_height = *expr;
                }
                StyleProp::MaxWidth(expr) => {
                    computed.style.max_width = *expr;
                }
                StyleProp::MaxHeight(expr) => {
                    computed.style.max_height = *expr;
                }
                StyleProp::Margin(expr) => {
                    computed.style.margin = *expr;
                }
                StyleProp::MarginLeft(expr) => {
                    computed.style.margin.left = *expr;
                }
                StyleProp::MarginRight(expr) => {
                    computed.style.margin.right = *expr;
                }
                StyleProp::MarginTop(expr) => {
                    computed.style.margin.top = *expr;
                }
                StyleProp::MarginBottom(expr) => {
                    computed.style.margin.bottom = *expr;
                }
                StyleProp::Padding(expr) => {
                    computed.style.padding = *expr;
                }
                StyleProp::PaddingLeft(expr) => {
                    computed.style.padding.left = *expr;
                }
                StyleProp::PaddingRight(expr) => {
                    computed.style.padding.right = *expr;
                }
                StyleProp::PaddingTop(expr) => {
                    computed.style.padding.top = *expr;
                }
                StyleProp::PaddingBottom(expr) => {
                    computed.style.padding.bottom = *expr;
                }
                StyleProp::Border(expr) => {
                    computed.style.border = *expr;
                }
                StyleProp::BorderLeft(expr) => {
                    computed.style.border.left = *expr;
                }
                StyleProp::BorderRight(expr) => {
                    computed.style.border.right = *expr;
                }
                StyleProp::BorderTop(expr) => {
                    computed.style.border.top = *expr;
                }
                StyleProp::BorderBottom(expr) => {
                    computed.style.border.bottom = *expr;
                }
                StyleProp::FlexDirection(expr) => {
                    computed.style.flex_direction = *expr;
                }
                StyleProp::FlexWrap(expr) => {
                    computed.style.flex_wrap = *expr;
                }
                StyleProp::FlexGrow(expr) => {
                    computed.style.flex_grow = *expr;
                }
                StyleProp::FlexShrink(expr) => {
                    computed.style.flex_shrink = *expr;
                }
                StyleProp::FlexBasis(expr) => {
                    computed.style.flex_basis = *expr;
                }
                StyleProp::ColumnGap(expr) => {
                    computed.style.column_gap = *expr;
                }
                StyleProp::RowGap(expr) => {
                    computed.style.row_gap = *expr;
                }
                StyleProp::Gap(expr) => {
                    computed.style.column_gap = *expr;
                    computed.style.row_gap = *expr;
                }

                StyleProp::AlignItems(expr) => {
                    computed.style.align_items = *expr;
                }
                StyleProp::AlignSelf(expr) => {
                    computed.style.align_self = *expr;
                }
                StyleProp::AlignContent(expr) => {
                    computed.style.align_content = *expr;
                }
                StyleProp::JustifyItems(expr) => {
                    computed.style.justify_items = *expr;
                }
                StyleProp::JustifySelf(expr) => {
                    computed.style.justify_self = *expr;
                }
                StyleProp::JustifyContent(expr) => {
                    computed.style.justify_content = *expr;
                }

                StyleProp::GridAutoFlow(expr) => {
                    computed.style.grid_auto_flow = *expr;
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
                    computed.style.grid_row = *expr;
                }
                StyleProp::GridRowStart(expr) => {
                    computed.style.grid_row.set_start(*expr);
                }

                StyleProp::GridRowSpan(expr) => {
                    computed.style.grid_row.set_span(*expr);
                }

                StyleProp::GridRowEnd(expr) => {
                    computed.style.grid_row.set_end(*expr);
                }

                StyleProp::GridColumn(expr) => {
                    computed.style.grid_column = *expr;
                }
                StyleProp::GridColumnStart(expr) => {
                    computed.style.grid_column.set_start(*expr);
                }

                StyleProp::GridColumnSpan(expr) => {
                    computed.style.grid_column.set_span(*expr);
                }

                StyleProp::GridColumnEnd(expr) => {
                    computed.style.grid_column.set_end(*expr);
                }

                StyleProp::OutlineColor(expr) => {
                    computed.outline_color = *expr;
                }

                StyleProp::OutlineWidth(expr) => {
                    computed.outline_width = *expr;
                }

                StyleProp::OutlineOffset(expr) => {
                    computed.outline_offset = *expr;
                }

                StyleProp::PointerEvents(expr) => {
                    computed.pickable = Some(*expr);
                }

                StyleProp::Font(expr) => {
                    computed.font = expr.clone();
                }

                StyleProp::FontSize(expr) => {
                    computed.font_size = Some(*expr);
                }

                StyleProp::Cursor(_) => todo!(),
                StyleProp::CursorImage(_) => todo!(),
                StyleProp::CursorOffset(_) => todo!(),

                StyleProp::Scale(expr) => {
                    computed.scale_x = Some(*expr);
                    computed.scale_y = Some(*expr);
                }
                StyleProp::ScaleX(expr) => {
                    computed.scale_x = Some(*expr);
                }
                StyleProp::ScaleY(expr) => {
                    computed.scale_y = Some(*expr);
                }
                StyleProp::Rotation(expr) => {
                    computed.rotation = Some(*expr);
                }
                StyleProp::Translation(expr) => {
                    computed.translation = Some(*expr);
                }

                StyleProp::Transition(trans) => computed.transitions.clone_from(trans),
            }
        }
    }
}
