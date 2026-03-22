# DesktopTerrarium Asset Production Brief

## Goal
Create a premium, stylized-real terrarium art pack that replaces the current procedural placeholder geometry without changing the app's core behavior.

## Deliverable scope
- One desk/environment composition.
- One glass vessel.
- One substrate stack.
- One hardscape set.
- Three plant species with five growth stages each.
- Two critters.
- Small FX support assets for condensation, haze, and debris.

## Core shot
- Fixed three-quarter hero camera.
- Target frame: 1440x900.
- The terrarium should read clearly at a glance and feel like a physical object on a desk.

## Required asset list

### Environment
- `assets/models/terrarium/environment_desk.glb`
  - Includes desk surface and background support.
  - Should not overpower the terrarium silhouette.

### Vessel
- `assets/models/terrarium/vessel.glb`
  - Clear glass vessel.
  - Clean silhouette with readable edge highlights.
  - Leave enough interior volume for plants and hardscape.

### Substrate
- `assets/models/terrarium/substrate_stack.glb`
  - Drainage pebble layer.
  - Charcoal layer.
  - Soil body and top breakup.
  - Slight asymmetry is preferred over perfect radial uniformity.

### Hardscape
- `assets/models/terrarium/hardscape.glb`
  - Bark, branch, stone, moss support forms.
  - Should help frame the hero plant and ground the supporting plants.

### Plants
- Fern:
  - `assets/models/plants/fern_stage_0.glb`
  - `assets/models/plants/fern_stage_1.glb`
  - `assets/models/plants/fern_stage_2.glb`
  - `assets/models/plants/fern_stage_3.glb`
  - `assets/models/plants/fern_stage_4.glb`
- Moss:
  - `assets/models/plants/moss_stage_0.glb`
  - `assets/models/plants/moss_stage_1.glb`
  - `assets/models/plants/moss_stage_2.glb`
  - `assets/models/plants/moss_stage_3.glb`
  - `assets/models/plants/moss_stage_4.glb`
- Succulent:
  - `assets/models/plants/succulent_stage_0.glb`
  - `assets/models/plants/succulent_stage_1.glb`
  - `assets/models/plants/succulent_stage_2.glb`
  - `assets/models/plants/succulent_stage_3.glb`
  - `assets/models/plants/succulent_stage_4.glb`

### Critters
- `assets/models/critters/butterfly.glb`
- `assets/models/critters/beetle.glb`

## Artistic rules
- Follow [visual-style-bible.md](/Users/d/Projects/Fun:GamePrjs/DesktopTerrarium/desktop_terrarium/docs/operations/visual-style-bible.md).
- Favor believable materials over high-frequency noise.
- Keep the palette restrained and earthy.
- Avoid flat neon greens, cartoon outlines, or exaggerated fantasy shapes.
- Growth stages must change silhouette, density, and maturity, not just scale.

## Technical constraints
- Format: `.glb`
- Expect the runtime to load `#Scene0` from each file.
- Keep transforms authored near origin so the app can place assets predictably.
- Keep plant pivots at the base where they meet substrate.
- Keep critter pivots near center of mass for path-based animation.
- Keep scale consistent across the pack.

## Integration behavior
- If a file exists at the contract path, the runtime will prefer it.
- If a file is missing, the app falls back to its procedural placeholder for that category.
- Plants are loaded per stage; missing stages fall back individually.

## Priority order
1. Vessel
2. Substrate stack
3. Hero/support plant stages
4. Hardscape
5. Critters
6. Micro-FX support
