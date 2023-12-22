use bevy::{
    render::color::Color,
    utils::{CowArc, HashMap},
};

pub enum VarValue<'a> {
    String(CowArc<'a, str>),
    Number(f32),
    Color(Color),
    Length(bevy::ui::Val),
}

pub type VarsMap<'a> = HashMap<CowArc<'a, str>, VarValue<'a>>;

// #[derive(Eq, PartialEq, Hash, Clone, Default)]
// pub struct AssetPath<'a> {
//     source: AssetSourceId<'a>,
//     path: CowArc<'a, Path>,
//     label: Option<CowArc<'a, str>>,
// }
