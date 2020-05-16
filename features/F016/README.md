# Items 
Players should be able to move their items from their wallet into realm.one,
this is the basis of Worlds.

[Diagram](../../docs/items.pdf)

1. First the wallet sends a json string to the client.
2. This string is recived on a TCP socket in the wallet system
src/system/client/wallet.rs The wallet systems then inserts the 
item into the ECS systeam.
3. The Item system does a join to find items. If it does not
contain a sync component it must be synced with the server.
Create a Pack and push that out to the server.
4. The server then has to verify the item is real, through 
it own wallet system, which opens a socket to the serverside
wallet.

```
Branch: items
```

## Systems Required (Client)
- WalletSystem
    - Recives the data through TCP and creates the item entity. 
    - Entity is described in the digram above.

- Item system
    - If an item does not contain a sync component, send a message
    to the server notifying it of the item.
    - This is where things will happen with the items (undefined)

## Systems Required (Server)
- Item system
    - This is where things will happen with the items (undefined)

- WalletSystem
    - Verifies that items are all good

## Components Required (Client)
- Item
    - This should contain the json string
- Synced
    - This notifies the system the item has been synced with the 
    server
