# Blood splat
When a player (or monster is attacked) there should be a visual indication of blood on the victom.

```
Branch Name: uid 
Depends On: nill
Required for: nill
```

## Components Required
- Animation (new) 
  - This should be a generic animation that contains
    - Vec::SpriteRenders
    - time
  - The animation should just cycle through those renders as time goes on
  - Eventually walk_animation cna be implemented using this

## Resources Required (Server)
- LifeFormUID (New)
  - This should init to zero when the server starts up
  - It should tick up when new players and monsters

## Systems Required (Client)
- AnimationSystem
  - Should check IO for incoming animation packs
    - Then create the entities for these packs
  - Should also handle animations from other client systems

## Systems Required (Server)
- PlayerManSystem
  - Should create animation pack and send during melee event

## Objects
  - Cmd::Animation
    - Should be sent from the server to players 
