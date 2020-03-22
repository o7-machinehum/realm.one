# AI/Monsters 
Server should instantiate NPC monsters into the game. These monsters are controlled through a serverside system called MonsterAiSystem. The engine should treat players and monsters exactly the same, this will reduce additional required code.

Branch Name: AI

## Components Required
( Monster Entity )
- lifeforms.rs
  - Add enum LifeFormType
    - This will be used on the serverside to determine if the AI should operate on it
- Transform
- SpriteRender

## Resources Required (Server)
- PlayerList
  - Refactor to be LifeformList
  - Contains a list of all the monsters (and players) in the game

## Systems Required (Client)
- PlayerManSystem
  - This should be refactored into "LifeformManSystem"

## Systems Required (Server)
- MonsterAiSystem (new)
  - Monsters should do some action every x ms (500 or something)
  - This should push actions into the io list
  - Actions are then acted on in the playerman system
- PlayerManSystem
  - Refactored to LifeformManSystem

## Other Objects
- Cmd::InsertPlayer
  - Refactor to IntertLifeform
- Cmd::UpdatePlayer
  - Refactor to UpdateLifeform
- Cmd::RemovePlayer
  - Refactor to RemoveLifeform

- Need a database to keep all the current monsters
-
