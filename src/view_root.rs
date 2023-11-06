use bevy::prelude::*;

use crate::ViewHandle;

#[derive(Resource)]
pub struct ViewRootResource(pub ViewHandle);
