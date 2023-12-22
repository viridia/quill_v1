use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};
use bevy_mod_picking::focus::{HoverMap, PreviousHoverMap};

use crate::{
    style::{ComputedStyle, UpdateComputedStyle},
    ElementClasses, ElementStyles, SelectorMatcher,
};

pub(crate) fn update_styles(
    mut commands: Commands,
    query: Query<(
        Entity,
        Ref<'static, ElementStyles>,
        Option<Ref<'static, ElementClasses>>,
        Ref<'static, Style>,
    )>,
    parent_query: Query<&'static Parent, (With<Node>, With<Visibility>)>,
    children_query: Query<&'static Children, (With<Node>, With<Visibility>)>,
    hover_map: Res<HoverMap>,
    hover_map_prev: Res<PreviousHoverMap>,
    assets: Res<AssetServer>,
) {
    let matcher = SelectorMatcher::new(&query, &parent_query, &children_query, &hover_map.0);
    let matcher_prev =
        SelectorMatcher::new(&query, &parent_query, &children_query, &hover_map_prev.0);
    for (entity, styles, _, style) in query.iter() {
        // Style changes only affect current element, not children.
        let mut changed = styles.is_changed();
        if !changed && styles.selector_depth > 0 {
            // Search ancestors to see if any have changed.
            // We want to know if either the class list or the hover state has changed.
            let mut e = entity;
            for _ in 0..styles.selector_depth {
                match query.get(e) {
                    Ok((_, _, a_classes, _)) => {
                        if styles.uses_hover
                            && matcher.is_hovering(&e) != matcher_prev.is_hovering(&e)
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

        if changed {
            // Compute computed style. Initialize to the current state.
            let mut computed = ComputedStyle::new();
            computed.style = style.clone();
            for ss in styles.styles.iter() {
                ss.apply_to(&mut computed, &matcher, &entity);
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
