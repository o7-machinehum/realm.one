# Network Refactor
Broadcast packet no longer works as there are several rooms. 

```
Branch Name: broadcast
Depends On: nil
Required for: Monster AI [F001] / Move Up/Down Stairs [F004]
  -> This will make development of monster AI easier
Related to: null?
```

## Resources Required (Server)
- TcpSystem
    - Should only broadcast to players in the room
    - It should look at Pack::room, then go through the
      the playerlist and only broadcast to those people

## Objects 
- network::Pack
    - Should have another field called Room
