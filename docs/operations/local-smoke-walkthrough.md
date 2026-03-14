# Local Smoke Walkthrough

## Fast automated smoke

Run the scripted smoke flow against a temp state directory:

```bash
./scripts/smoke/run_local_smoke.sh
```

Pass criteria:

- The app launches and exits cleanly.
- `state.json` is written to the temp smoke directory.
- The printed summary shows a persisted weather value, time phase, and plant stages.

## Interactive smoke

Launch normally:

```bash
./scripts/env/with_deterministic_env.sh cargo run
```

Use `Fn` with function keys if macOS media-key mode is enabled.

Debug controls:

- `F1`: toggle debug overlay
- `F2`: cycle forced time-of-day phase
- `F3`: cycle activity mode (`system`, `forced-active`, `forced-idle`)
- `F4`: cycle forced weather
- `F5`: cycle time scale (`1x`, `5x`, `20x`, `100x`)
- `F6`: cycle plant growth multiplier (`1x`, `25x`, `250x`)
- `F7`: advance all plants by one stage
- `F8`: spawn beetle
- `F9`: spawn butterfly
- `F10`: save state immediately

Pass/fail walkthrough:

1. Cold launch
   Pass: window opens, backdrop/glass/soil/plants render, no panic, no missing textures.
2. Time-of-day
   Pass: `F2` cycles visible morning/day/evening/night layers.
3. Weather
   Pass: `F4` cycles clear/fog/rain/wind behavior when weather transitions are enabled.
4. Plant progression
   Pass: `F7` advances plant sprites immediately and overlay updates the stage lines.
5. Critter lifecycle
   Pass: `F8` and `F9` spawn visible critters that traverse and despawn.
6. Persistence
   Pass: `F10` updates the save status line and relaunch restores the latest plant/weather/time state.

## Isolated state path

Use a temporary state directory to avoid touching your real terrarium save:

```bash
TERRARIUM_STATE_DIR="$(mktemp -d)" ./scripts/env/with_deterministic_env.sh cargo run
```
