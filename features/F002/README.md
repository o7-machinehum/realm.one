# Player UID
Players are currently identified by their name. This will cause the game to explode if there are more than one player with the same name. Monsters will have the same name as well, which will cause havoc.

```
Branch Name: uid 
Depends On: nill
Required for: F001 
```

## Components Required
- LifeformComponent
  - Needs additonal fiend (uid)

## Resources Required (Server)
- LifeFormUID (New)
  - This should init to zero when the server starts up
  - It should tick up when new players and monsters

## Systems Required (Client)
- PlayerManSystem
  - In the UpdatePlayer match statement it should look for UID, not name
  - Remove player should use ID, not ip

## Systems Required (Server)
- AuthSystem should give the UID
