## Design
This document aims to outline aspects of the game's features. 

## Themes
* Medieval (grim)dark fantasy:
  * Skeletons, wizards, swords, goblins, and orcs.
  * Magic and melee combat.
  * Castles, dungeons, and... lore? 

## Gameplay
The goal of ascii survivors is to provide a challenging, rewarding, and replayable experience, with a focus on meaningful choices and build identity. The game is designed to be difficult, but fair, with a high skill ceiling and a low barrier to entry.
* Survivors-esque game with roguelike mechanics:
  * Exploration, combat, and resource management.
  * Large, in-depth pool of items and upgrades.
  * In-depth build identity and complexity, meaningful choices in upgrades.
  * Diverse upgrades for spells, weapons, etc.
  * Die once, your character's upgrades/stats are reset, apart from persistent ones.
  * High difficulty regardless of the player's power level.
* 'Roguelite' persistence:
  * Upgrades and revives
  * Gold
  * Upgrades
  * Ruleset changes, modifiers, and quests
* Play with kb/m or gamepads.
* Several simple but hand-crafted levels.
#### Features:
* Unique, upgradeable torch item, provides light in dark areas like caves or dungeons. Acquired early into the game.
  * Can be upgraded to provide more light, last longer, and could have special effects(e.g. puzzles)
  * Cool lighting effect which interacts with the environment. Shader or Bresenham's line algorithm?
  * Maybe it could be used to light campfires in levels?
  * Could be used to scare away certain enemies(e.g. bats)
* Rest area including a campfire, where the player can heal, and a shop, where players can permanently upgrade their character and purchase items.
* [Multi-cell enemies](https://www.gridsagegames.com/blog/2020/04/developing-multitile-creatures-roguelikes/)(e.g. dragons, snakes, giant stick man)
* Companion system:
  * Find and recruit companions throughout levels.
  * Companions assist in combat and provide buffs.
  * Companions can be upgraded and customized in the rest area.
* Items and equipment:
  * Weapons, armor, and accessories.
  * Items can be found in levels, purchased from shops, or looted from bosses.
* Map:
  * Several hand-crafted levels with unique themes(grassland, desert, cave, dungeon, etc.) and enemies.
  * Puzzles, traps, incentives to explore.
  * Creative use of lighting.
    * Puzzles utilizing lightning 
  * Breakable environment objects(e.g. barrels, crates, pots) that drop gold, experience, or items.
  * Map Editor(Utilizing [REXPaint](https://www.gridsagegames.com/rexpaint/)):
    * Ability to place placeholder cells that tell the game to decide whatever to place on that tile, such as items.
    * Zones
      * Spawn only a specific category of objects, such as items, monsters that fit the area's theme, keys, stairs/portals/ladders, etc.
      * Safezones, like our rest area. The player cannot take damage here, maybe this is the only area which a player can swap items or upgrade certain attributes.
* Magnet.

## Visuals
* **ASCII graphics**
* Unique, subtle, and informative use of colors and glyphs. Including an in-game legend/key.
* Pleasing, simple, intuitive UI/UX.

## [Sound](https://github.com/proficiency/ascii_survivors/issues/1)
Pleasing, cozy, fantasy sound effects and music. _todo._

## Code
Written in Rust using Bevy — the code is simple, documentation is light but meaningful, aiming to provide clarity without being overbearing.
* Efficient use of Bevy's [ECS](https://docs.rs/bevy_ecs/latest/bevy_ecs/).
* Lightweight, maintainable code.
* Filesystem and modules designed with separation of concerns in mind.

---
> [!NOTE]
> as design and features evolve, this document will be updated to reflect the direction of the project.
