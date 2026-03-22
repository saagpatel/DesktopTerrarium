use crate::components::PlantSpecies;
use bevy::prelude::*;
use std::path::Path;

const ASSET_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");

#[derive(Clone, Default, Debug)]
pub struct PlantArtSet {
    pub stages: [Option<Handle<Scene>>; 5],
}

#[derive(Resource, Clone, Default, Debug)]
pub struct SceneArtCatalog {
    pub environment_scene: Option<Handle<Scene>>,
    pub vessel_scene: Option<Handle<Scene>>,
    pub substrate_scene: Option<Handle<Scene>>,
    pub hardscape_scene: Option<Handle<Scene>>,
    pub butterfly_scene: Option<Handle<Scene>>,
    pub beetle_scene: Option<Handle<Scene>>,
    pub fern: PlantArtSet,
    pub moss: PlantArtSet,
    pub succulent: PlantArtSet,
}

impl SceneArtCatalog {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            environment_scene: load_scene_if_present(
                asset_server,
                "models/terrarium/environment_desk.glb",
            ),
            vessel_scene: load_scene_if_present(asset_server, "models/terrarium/vessel.glb"),
            substrate_scene: load_scene_if_present(
                asset_server,
                "models/terrarium/substrate_stack.glb",
            ),
            hardscape_scene: load_scene_if_present(asset_server, "models/terrarium/hardscape.glb"),
            butterfly_scene: load_scene_if_present(asset_server, "models/critters/butterfly.glb"),
            beetle_scene: load_scene_if_present(asset_server, "models/critters/beetle.glb"),
            fern: load_plant_set(asset_server, "fern"),
            moss: load_plant_set(asset_server, "moss"),
            succulent: load_plant_set(asset_server, "succulent"),
        }
    }

    pub fn plant_stage(&self, species: PlantSpecies, stage: u8) -> Option<Handle<Scene>> {
        let stage_index = usize::from(stage.min(4));
        match species {
            PlantSpecies::Fern => self.fern.stages[stage_index].clone(),
            PlantSpecies::Moss => self.moss.stages[stage_index].clone(),
            PlantSpecies::Succulent => self.succulent.stages[stage_index].clone(),
        }
    }
}

fn load_plant_set(asset_server: &AssetServer, species: &str) -> PlantArtSet {
    let mut set = PlantArtSet::default();
    for stage in 0..5 {
        let relative = format!("models/plants/{}_stage_{}.glb", species, stage);
        set.stages[stage] = load_scene_if_present(asset_server, &relative);
    }
    set
}

fn load_scene_if_present(asset_server: &AssetServer, relative_path: &str) -> Option<Handle<Scene>> {
    let absolute_path = Path::new(ASSET_ROOT).join(relative_path);
    if absolute_path.exists() {
        Some(asset_server.load(format!("{relative_path}#Scene0")))
    } else {
        None
    }
}
