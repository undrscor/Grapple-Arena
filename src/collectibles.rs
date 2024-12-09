use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Clone, Debug, Default, Bundle, LdtkEntity)]
pub struct CollectibleBundle{}

pub struct Collectible{}


pub struct CollectiblePlugin;
impl Plugin for CollectiblePlugin {
    fn build(&self, app: &mut App) {
        //app
            //.register_ldtk_entity::<CollectibleBundle>("Boots");
            //.register_ldtk_entity::<CollectibleBundle>("Pills");
            //.register_ldtk_entity::<CollectibleBundle>("Hook");
            //.register_ldtk_entity::<CollectibleBundle>("Coin");

    }
}