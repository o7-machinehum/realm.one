# Items 
Players should be able to move their items from their wallet into realm.one,
this is the basis of Worlds.

[Diagram](../../docs/items.pdf)

1. First the wallet sents a json string to the client.
2. This string is recived on a TCP socket in the wallet system
src/system/client/wallet. The wallet systems then does two things.
    1. Sends an event to the network system to inform the server the
    item has entered the game.
    2. Insert the item into the ECS system
3. The server then has to verify the item is real, through 
it own wallet system, which opens a socket to the serverside
wallet.

```
Branch: items
```

## Systems Required (Client)
- WalletSystem
    - Recives the data through TCP and creates the item entity
    - Send a command to the network system to tell the server this
    item has entered the game

- Item system
    - This is where things will happen with the items (undefined)

## Systems Required (Server)
- Item system
    - This is where things will happen with the items (undefined)

- WalletSystem
    - Verifies that items are all good

## Components Required (Client)
- Item
    - This should contain the json string
