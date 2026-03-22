use bevy::prelude::*;

#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub enum PlantAnchorId {
    LeftSupport,
    Hero,
    RightSupport,
}

impl PlantAnchorId {
    pub fn from_slot(slot: u8) -> Self {
        match slot {
            0 => Self::LeftSupport,
            1 => Self::Hero,
            _ => Self::RightSupport,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::LeftSupport => "left-support",
            Self::Hero => "hero",
            Self::RightSupport => "right-support",
        }
    }
}

#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub struct PlantAnchor {
    pub id: PlantAnchorId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CritterPathId {
    SoilTraverse,
    GlassSweep,
    CanopyDrift,
}

impl CritterPathId {
    pub fn label(self) -> &'static str {
        match self {
            Self::SoilTraverse => "soil-traverse",
            Self::GlassSweep => "glass-sweep",
            Self::CanopyDrift => "canopy-drift",
        }
    }
}

#[derive(Component)]
pub struct PlantVisualPart;

#[derive(Component)]
pub struct MainTerrariumCamera {
    pub base_translation: Vec3,
    pub target: Vec3,
}

#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub enum LightRigRole {
    Key,
    Fill,
    Rim,
}

#[derive(Component)]
pub struct WeatherReactiveSurface;

#[derive(Component)]
pub struct HazeVolume;

#[derive(Component)]
pub struct WindReactive {
    pub amplitude: f32,
    pub phase_offset: f32,
}
