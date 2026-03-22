# DesktopTerrarium Asset Import Contract

## Purpose
This document defines the runtime-facing asset paths for the 3D terrarium art pack.

The engine-side catalog lives in [src/resources/art.rs](/Users/d/Projects/Fun:GamePrjs/DesktopTerrarium/desktop_terrarium/src/resources/art.rs). If an asset file exists at one of the paths below, the runtime will load `#Scene0` from that `.glb`. If it does not exist, the app will use its procedural fallback.

## Scene categories

### Environment
- `assets/models/terrarium/environment_desk.glb`

### Vessel
- `assets/models/terrarium/vessel.glb`

### Substrate
- `assets/models/terrarium/substrate_stack.glb`

### Hardscape
- `assets/models/terrarium/hardscape.glb`

## Plant stages

### Fern
- `assets/models/plants/fern_stage_0.glb`
- `assets/models/plants/fern_stage_1.glb`
- `assets/models/plants/fern_stage_2.glb`
- `assets/models/plants/fern_stage_3.glb`
- `assets/models/plants/fern_stage_4.glb`

### Moss
- `assets/models/plants/moss_stage_0.glb`
- `assets/models/plants/moss_stage_1.glb`
- `assets/models/plants/moss_stage_2.glb`
- `assets/models/plants/moss_stage_3.glb`
- `assets/models/plants/moss_stage_4.glb`

### Succulent
- `assets/models/plants/succulent_stage_0.glb`
- `assets/models/plants/succulent_stage_1.glb`
- `assets/models/plants/succulent_stage_2.glb`
- `assets/models/plants/succulent_stage_3.glb`
- `assets/models/plants/succulent_stage_4.glb`

## Critters
- `assets/models/critters/butterfly.glb`
- `assets/models/critters/beetle.glb`

## Runtime behavior
- Environment, vessel, substrate, and hardscape are loaded as category-level scene replacements.
- Plants are loaded per species and per growth stage.
- Critters are loaded per species.
- Missing files do not fail the app. They only fall back to the built-in placeholder render path.

## Authoring notes
- Use `.glb` scene exports with a single primary scene.
- Keep pivots clean and intentional.
- Keep scale and orientation consistent across files.
- Avoid embedding unnecessary cameras or lights.
