# Move Up/Down Stairs
When a player moves to a "TP" loaction, the server should execute the following acitons.
  - Check if the player is on a TP location. If this is so.
  - Send a RemovePlayer to all the other players in the old map
  - Send a InsertPlayer to all the other players in the new map
  - Create a CMD::TransferMap and send it to the place that TP'd
  - Update the player with the new monsters and players in that area

This poses several problems, as our broadcast packet no longer works. You shouldn't broadcast packets to players that are not in the same room. See F005 for the fix.

```
Branch Name: tp 
Depends On: network-refactor [F005]
Required for: Monster AI [F001]
  -> This will make development of monster AI easier
Related to: null?
```

## Components Required

## Resources Required (Server)

## Systems Required (Client)
- player.rs
  - Can't use allowed_move anymore, refactor to get_step() and match out

## Systems Required (Server)
- server/lifeform_man.rs
  - Can't use allowed_move anymore needs to match get_step()
  
## Objects
- Room
  - There should be a function call get_step() -> tile_prop 
  - This should replace allowed_move
  - This should return TileProp, which is a new enum
  - This is then matched out in the systems

```
pub enum TileProp {
  collision,
  tp(Tp),
  ... etc
}

```

```
pub struct Tp {
  - String -> Map this tp leads to
  - Loc -> Location on the NEXT map this leads to
}
