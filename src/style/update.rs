use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use bevy_mod_picking::focus::{HoverMap, PreviousHoverMap};

use crate::{
    style::{ComputedStyle, UpdateComputedStyle},
    ElementClasses, ElementStyles, SelectorMatcher,
};

use super::tokens::{self, TokenMap};

pub(crate) fn update_styles(
    mut commands: Commands,
    query: Query<(
        Entity,
        Ref<'static, ElementStyles>,
        Option<Ref<'static, ElementClasses>>,
        Ref<'static, Style>,
    )>,
    mut tokens_query: Query<(Entity, &'static mut tokens::ElementTokens)>,
    parent_query: Query<&'static Parent, (With<Node>, With<Visibility>)>,
    children_query: Query<&'static Children, (With<Node>, With<Visibility>)>,
    hover_map: Res<HoverMap>,
    hover_map_prev: Res<PreviousHoverMap>,
    assets: Res<AssetServer>,
) {
    let matcher = SelectorMatcher::new(&query, &parent_query, &children_query, &hover_map.0);
    let matcher_prev =
        SelectorMatcher::new(&query, &parent_query, &children_query, &hover_map_prev.0);

    // Update style tokens
    for (entity, mut tokens) in tokens_query.iter_mut() {
        let Ok((_, styles, _, _)) = query.get(entity) else {
            continue;
        };
        let changed = is_changed(
            &styles,
            entity,
            &query,
            &matcher,
            &matcher_prev,
            &parent_query,
        );

        if changed {
            let mut next_tokens = TokenMap::default();
            for ss in styles.styles.iter() {
                ss.update_tokens(&mut next_tokens, &matcher, &entity);
            }
            if tokens.0 != next_tokens {
                std::mem::swap(&mut tokens.as_mut().0, &mut next_tokens);
            }
        }
    }

    // Update computed styles
    for (entity, styles, _, style) in query.iter() {
        // Style changes only affect current element, not children.
        let changed = is_changed(
            &styles,
            entity,
            &query,
            &matcher,
            &matcher_prev,
            &parent_query,
        );

        if changed {
            // Compute computed style. Initialize to the current state.
            let mut computed = ComputedStyle::new();
            computed.style = style.clone();
            let lookup = tokens::TokenLookup::new(entity, &tokens_query, &parent_query);
            for ss in styles.styles.iter() {
                ss.apply_to(&mut computed, &matcher, &lookup, &entity);
            }
            computed.font_handle = match computed.font {
                Some(ref path) => Some(assets.load(path)),
                None => None,
            };
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
}

/// Detects whether the given entity's styles have changed, or whether any of its ancestors
/// have changed in a way that would affect the computation of styles.
fn is_changed(
    styles: &Ref<'_, ElementStyles>,
    entity: Entity,
    query: &Query<
        '_,
        '_,
        (
            Entity,
            Ref<'_, ElementStyles>,
            Option<Ref<'_, ElementClasses>>,
            Ref<'_, Style>,
        ),
    >,
    matcher: &SelectorMatcher<'_, '_, '_>,
    matcher_prev: &SelectorMatcher<'_, '_, '_>,
    parent_query: &Query<'_, '_, &Parent, (With<Node>, With<Visibility>)>,
) -> bool {
    let mut changed = styles.is_changed();
    if !changed && styles.selector_depth > 0 {
        // Search ancestors to see if any have changed.
        // We want to know if either the class list or the hover state has changed.
        let mut e = entity;
        for _ in 0..styles.selector_depth {
            match query.get(e) {
                Ok((_, _, a_classes, _)) => {
                    if styles.uses_hover && matcher.is_hovering(&e) != matcher_prev.is_hovering(&e)
                    {
                        changed = true;
                    }
                    if a_classes.map_or(false, |f| f.is_changed()) {
                        changed = true;
                        break;
                    } else {
                        match parent_query.get(e) {
                            Ok(parent) => e = **parent,
                            _ => break,
                        }
                    }
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
    changed
}
