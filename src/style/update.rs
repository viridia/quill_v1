use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use bevy_mod_picking::focus::{HoverMap, PreviousHoverMap};

use crate::{
    style::{ComputedStyle, UpdateComputedStyle},
    ElementClasses, ElementStyles, SelectorMatcher,
};

use super::style::TextStyles;

pub(crate) fn update_styles(
    mut commands: Commands,
    query_root: Query<Entity, (With<Node>, Without<Parent>)>,
    query_styles: Query<
        (
            Option<Ref<ElementStyles>>,
            Option<&TextStyles>,
            Ref<'static, Style>,
        ),
        With<Node>,
    >,
    query_element_classes: Query<Ref<'static, ElementClasses>>,
    query_parents: Query<&'static Parent, (With<Node>, With<Visibility>)>,
    query_children: Query<&'static Children, (With<Node>, With<Visibility>)>,
    hover_map: Res<HoverMap>,
    hover_map_prev: Res<PreviousHoverMap>,
    assets: Res<AssetServer>,
) {
    let matcher = SelectorMatcher::new(
        &query_element_classes,
        &query_parents,
        &query_children,
        &hover_map.0,
    );
    let matcher_prev = SelectorMatcher::new(
        &query_element_classes,
        &query_parents,
        &query_children,
        &hover_map_prev.0,
    );

    for root_node in &query_root {
        update_element_styles(
            &mut commands,
            &query_styles,
            &query_element_classes,
            &query_parents,
            &query_children,
            &matcher,
            &matcher_prev,
            &assets,
            root_node,
            &TextStyles::default(),
            false,
        )
    }
}

fn update_element_styles(
    commands: &mut Commands,
    query_styles: &Query<(Option<Ref<ElementStyles>>, Option<&TextStyles>, Ref<Style>), With<Node>>,
    classes_query: &Query<Ref<'static, ElementClasses>>,
    parent_query: &Query<'_, '_, &Parent, (With<Node>, With<Visibility>)>,
    children_query: &Query<'_, '_, &Children, (With<Node>, With<Visibility>)>,
    matcher: &SelectorMatcher<'_, '_, '_>,
    matcher_prev: &SelectorMatcher<'_, '_, '_>,
    assets: &Res<AssetServer>,
    entity: Entity,
    inherited_styles: &TextStyles,
    mut inherited_styled_changed: bool,
) {
    let mut text_styles = inherited_styles.clone();

    if let Ok((es, ts, style)) = query_styles.get(entity) {
        // Check if the element styles or ancestor classes have changed.
        let mut changed = match es {
            Some(ref element_style) => is_changed(
                element_style,
                entity,
                classes_query,
                &matcher,
                &matcher_prev,
                &parent_query,
            ),
            None => false,
        };

        if !changed && inherited_styled_changed {
            // Check if the text styles have changed.
            changed = ts != Some(&text_styles);
        }

        if changed {
            // Compute computed style. Initialize to the current state.
            let mut computed = ComputedStyle::new();
            computed.style = style.clone();

            // Inherited properties
            computed.font_handle = inherited_styles.font.clone();
            computed.color = inherited_styles.color;

            // Apply styles to computed
            if let Some(ref element_styles) = es {
                for ss in element_styles.styles.iter() {
                    ss.apply_to(&mut computed, &matcher, &entity);
                }
            }

            // Load font asset if non-null.
            if let Some(ref font_path) = computed.font {
                computed.font_handle = Some(assets.load(font_path));
            }

            // Update inherited text styles
            text_styles.color = computed.color;
            text_styles.font = computed.font_handle.clone();

            // Only store the text styles if they are different than the parent's.
            if ts != Some(&text_styles) {
                inherited_styled_changed = true;
                commands.entity(entity).insert(text_styles.clone());
            } else {
                commands.entity(entity).remove::<TextStyles>();
            }

            computed.image_handle = match computed.image {
                Some(ref path) => Some(
                    assets.load_with_settings(path, |s: &mut ImageLoaderSettings| {
                        s.sampler = ImageSampler::linear()
                    }),
                ),
                None => None,
            };
            commands.add(UpdateComputedStyle { entity, computed });
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_element_styles(
                commands,
                query_styles,
                &classes_query,
                parent_query,
                children_query,
                matcher,
                matcher_prev,
                assets,
                *child,
                &text_styles,
                inherited_styled_changed,
            );
        }
    }
}

/// Detects whether the given entity's styles have changed, or whether any of its ancestors
/// have changed in a way that would affect the computation of styles.
fn is_changed(
    element_styles: &Ref<'_, ElementStyles>,
    entity: Entity,
    classes_query: &Query<Ref<'static, ElementClasses>>,
    matcher: &SelectorMatcher<'_, '_, '_>,
    matcher_prev: &SelectorMatcher<'_, '_, '_>,
    parent_query: &Query<'_, '_, &Parent, (With<Node>, With<Visibility>)>,
) -> bool {
    // Style changes only affect current element, not children.
    let mut changed = element_styles.is_changed();

    // Search ancestors to see if any have changed.
    // We want to know if either the class list or the hover state has changed.
    if !changed && element_styles.selector_depth > 0 {
        let mut e = entity;
        for _ in 0..element_styles.selector_depth {
            match classes_query.get(e) {
                Ok(a_classes) => {
                    if element_styles.uses_hover
                        && matcher.is_hovering(&e) != matcher_prev.is_hovering(&e)
                    {
                        changed = true;
                        break;
                    }
                    if a_classes.is_changed() {
                        changed = true;
                        break;
                    }
                }
                _ => (),
            }

            match parent_query.get(e) {
                Ok(parent) => e = **parent,
                _ => break,
            }
        }
    }
    changed
}
