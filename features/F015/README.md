# AI
There should be a system (monster AI) that creates LifeformEvents::Actions 
and pipes them over to the lifeform system. These are AI actions that
are reasonable things that monsters do.

```
Branch: AI
```

## Systems Required (Server)
- MonsterAiSystem (new)
    - This should loop through all lifeforms in the lifeform list
    - Monsters should do some action every x ms (500 or something)
    - Action: Move towards player
    - Action: Attack player

