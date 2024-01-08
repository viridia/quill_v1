#![allow(missing_docs)]

use bevy::{
    asset::AssetPath,
    log::error,
    math::Vec3,
    prelude::Color,
    ui::{self, ZIndex},
};

use crate::{PointerEvents, StyleProp};

use super::{selector::Selector, style_props::SelectorList, transition::Transition};

/// Trait that represents a CSS color
pub trait ColorParam {
    fn to_val(self) -> Option<Color>;
}

impl ColorParam for Option<Color> {
    fn to_val(self) -> Option<Color> {
        self
    }
}

impl ColorParam for Color {
    fn to_val(self) -> Option<Color> {
        Some(self)
    }
}

impl ColorParam for &str {
    fn to_val(self) -> Option<Color> {
        Some(Color::hex(self).unwrap())
    }
}

/// Trait that represents a CSS "length"
pub trait LengthParam {
    fn to_val(self) -> ui::Val;
}

impl LengthParam for ui::Val {
    fn to_val(self) -> ui::Val {
        self
    }
}

impl LengthParam for f32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self)
    }
}

impl LengthParam for i32 {
    fn to_val(self) -> ui::Val {
        ui::Val::Px(self as f32)
    }
}

/// Trait that represents a CSS Z-index
pub trait ZIndexParam {
    fn to_val(self) -> Option<ZIndex>;
}

impl ZIndexParam for ZIndex {
    fn to_val(self) -> Option<ZIndex> {
        Some(self)
    }
}

impl ZIndexParam for i32 {
    fn to_val(self) -> Option<ZIndex> {
        Some(ZIndex::Local(self))
    }
}

/// Trait that represents CSS edge widths (margin, padding, etc.)
pub trait UiRectParam {
    fn to_uirect(self) -> ui::UiRect;
}

impl UiRectParam for ui::UiRect {
    fn to_uirect(self) -> ui::UiRect {
        self
    }
}

impl UiRectParam for ui::Val {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(self)
    }
}

impl UiRectParam for f32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self))
    }
}

impl UiRectParam for i32 {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::all(ui::Val::Px(self as f32))
    }
}

impl<H: LengthParam, V: LengthParam> UiRectParam for (H, V) {
    fn to_uirect(self) -> ui::UiRect {
        ui::UiRect::axes(self.0.to_val(), self.1.to_val())
    }
}

pub struct StyleBuilder {
    pub(crate) props: Vec<StyleProp>,
    pub(crate) selectors: SelectorList,
}

impl StyleBuilder {
    pub(super) fn new() -> Self {
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
        self.props.push(StyleProp::BackgroundColor(color.to_val()));
        self
    }

    pub fn border_color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props.push(StyleProp::BorderColor(color.to_val()));
        self
    }

    pub fn color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props.push(StyleProp::Color(color.to_val()));
        self
    }

    pub fn z_index(&mut self, index: impl ZIndexParam) -> &mut Self {
        self.props.push(StyleProp::ZIndex(index.to_val()));
        self
    }

    pub fn display(&mut self, disp: ui::Display) -> &mut Self {
        self.props.push(StyleProp::Display(disp));
        self
    }

    pub fn position(&mut self, pos: ui::PositionType) -> &mut Self {
        self.props.push(StyleProp::Position(pos));
        self
    }

    pub fn overflow(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props.push(StyleProp::Overflow(ov));
        self
    }

    pub fn overflow_x(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props.push(StyleProp::OverflowX(ov));
        self
    }

    pub fn overflow_y(&mut self, ov: ui::OverflowAxis) -> &mut Self {
        self.props.push(StyleProp::OverflowY(ov));
        self
    }

    pub fn direction(&mut self, dir: ui::Direction) -> &mut Self {
        self.props.push(StyleProp::Direction(dir));
        self
    }

    pub fn left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Left(length.to_val()));
        self
    }

    pub fn right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Right(length.to_val()));
        self
    }

    pub fn top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Top(length.to_val()));
        self
    }

    pub fn bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Bottom(length.to_val()));
        self
    }

    pub fn width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Width(length.to_val()));
        self
    }

    pub fn height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Height(length.to_val()));
        self
    }

    pub fn min_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MinWidth(length.to_val()));
        self
    }

    pub fn min_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MinHeight(length.to_val()));
        self
    }

    pub fn max_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MaxWidth(length.to_val()));
        self
    }

    pub fn max_height(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MaxHeight(length.to_val()));
        self
    }

    // pub aspect_ratio: StyleProp<f32>,

    pub fn margin(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props.push(StyleProp::Margin(rect.to_uirect()));
        self
    }

    pub fn margin_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MarginLeft(length.to_val()));
        self
    }

    pub fn margin_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MarginRight(length.to_val()));
        self
    }

    pub fn margin_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MarginTop(length.to_val()));
        self
    }

    pub fn margin_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::MarginBottom(length.to_val()));
        self
    }

    pub fn padding(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props.push(StyleProp::Padding(rect.to_uirect()));
        self
    }

    pub fn padding_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::PaddingLeft(length.to_val()));
        self
    }

    pub fn padding_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::PaddingRight(length.to_val()));
        self
    }

    pub fn padding_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::PaddingTop(length.to_val()));
        self
    }

    pub fn padding_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::PaddingBottom(length.to_val()));
        self
    }

    pub fn border(&mut self, rect: impl UiRectParam) -> &mut Self {
        self.props.push(StyleProp::Border(rect.to_uirect()));
        self
    }

    pub fn border_left(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::BorderLeft(length.to_val()));
        self
    }

    pub fn border_right(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::BorderRight(length.to_val()));
        self
    }

    pub fn border_top(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::BorderTop(length.to_val()));
        self
    }

    pub fn border_bottom(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::BorderBottom(length.to_val()));
        self
    }

    pub fn flex_direction(&mut self, dir: ui::FlexDirection) -> &mut Self {
        self.props.push(StyleProp::FlexDirection(dir));
        self
    }

    pub fn flex_wrap(&mut self, w: ui::FlexWrap) -> &mut Self {
        self.props.push(StyleProp::FlexWrap(w));
        self
    }

    // Flex(ExprList),

    pub fn flex_grow(&mut self, n: f32) -> &mut Self {
        self.props.push(StyleProp::FlexGrow(n));
        self
    }

    pub fn flex_shrink(&mut self, n: f32) -> &mut Self {
        self.props.push(StyleProp::FlexShrink(n));
        self
    }

    pub fn flex_basis(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::FlexBasis(length.to_val()));
        self
    }

    pub fn row_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::RowGap(length.to_val()));
        self
    }

    pub fn column_gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::ColumnGap(length.to_val()));
        self
    }

    pub fn gap(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::Gap(length.to_val()));
        self
    }

    pub fn align_items(&mut self, align: ui::AlignItems) -> &mut Self {
        self.props.push(StyleProp::AlignItems(align));
        self
    }

    pub fn align_self(&mut self, align: ui::AlignSelf) -> &mut Self {
        self.props.push(StyleProp::AlignSelf(align));
        self
    }

    pub fn align_content(&mut self, align: ui::AlignContent) -> &mut Self {
        self.props.push(StyleProp::AlignContent(align));
        self
    }

    pub fn justify_items(&mut self, justify: ui::JustifyItems) -> &mut Self {
        self.props.push(StyleProp::JustifyItems(justify));
        self
    }

    pub fn justify_self(&mut self, justify: ui::JustifySelf) -> &mut Self {
        self.props.push(StyleProp::JustifySelf(justify));
        self
    }

    pub fn justify_content(&mut self, justify: ui::JustifyContent) -> &mut Self {
        self.props.push(StyleProp::JustifyContent(justify));
        self
    }

    pub fn grid_auto_flow(&mut self, flow: ui::GridAutoFlow) -> &mut Self {
        self.props.push(StyleProp::GridAutoFlow(flow));
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
        self.props.push(StyleProp::GridRow(val));
        self
    }

    pub fn grid_row_start(&mut self, val: i16) -> &mut Self {
        self.props.push(StyleProp::GridRowStart(val));
        self
    }

    pub fn grid_row_span(&mut self, val: u16) -> &mut Self {
        self.props.push(StyleProp::GridRowSpan(val));
        self
    }

    pub fn grid_row_end(&mut self, val: i16) -> &mut Self {
        self.props.push(StyleProp::GridRowEnd(val));
        self
    }

    pub fn grid_column(&mut self, val: ui::GridPlacement) -> &mut Self {
        self.props.push(StyleProp::GridColumn(val));
        self
    }

    pub fn grid_column_start(&mut self, val: i16) -> &mut Self {
        self.props.push(StyleProp::GridColumnStart(val));
        self
    }

    pub fn grid_column_span(&mut self, val: u16) -> &mut Self {
        self.props.push(StyleProp::GridColumnSpan(val));
        self
    }

    pub fn grid_column_end(&mut self, val: i16) -> &mut Self {
        self.props.push(StyleProp::GridColumnEnd(val));
        self
    }

    // LineBreak(BreakLineOn),

    pub fn outline_color(&mut self, color: impl ColorParam) -> &mut Self {
        self.props.push(StyleProp::OutlineColor(color.to_val()));
        self
    }

    pub fn outline_width(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::OutlineWidth(length.to_val()));
        self
    }

    pub fn outline_offset(&mut self, length: impl LengthParam) -> &mut Self {
        self.props.push(StyleProp::OutlineOffset(length.to_val()));
        self
    }

    pub fn pointer_events(&mut self, pe: PointerEvents) -> &mut Self {
        self.props.push(StyleProp::PointerEvents(pe));
        self
    }

    pub fn font(&mut self, path: Option<AssetPath<'static>>) -> &mut Self {
        self.props.push(StyleProp::Font(path));
        self
    }

    pub fn font_size(&mut self, val: f32) -> &mut Self {
        self.props.push(StyleProp::FontSize(val));
        self
    }

    pub fn scale_x(&mut self, scale: f32) -> &mut Self {
        self.props.push(StyleProp::ScaleX(scale));
        self
    }

    pub fn scale_y(&mut self, scale: f32) -> &mut Self {
        self.props.push(StyleProp::ScaleY(scale));
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.props.push(StyleProp::Scale(scale));
        self
    }

    pub fn rotation(&mut self, rot: f32) -> &mut Self {
        self.props.push(StyleProp::Rotation(rot));
        self
    }

    pub fn translation(&mut self, trans: Vec3) -> &mut Self {
        self.props.push(StyleProp::Translation(trans));
        self
    }

    pub fn transition(&mut self, transition: &[Transition]) -> &mut Self {
        self.props
            .push(StyleProp::Transition(Vec::from(transition)));
        self
    }

    /// Add a selector expression to this style declaration.
    pub fn selector(
        &mut self,
        mut expr: &str,
        builder_fn: impl FnOnce(&mut StyleBuilder) -> &mut StyleBuilder,
    ) -> &mut Self {
        let mut builder = StyleBuilder::new();
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
