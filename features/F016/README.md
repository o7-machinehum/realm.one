# Items 
Players should be able to move their items from their wallet into realm.one,
this is the basis of Worlds.

[Diagram](../../docs/items.pdf)

* [x] The worlds wallet pads the first four bytes of the item packet with the
length of the packet.
* [x] The wallet sends this json string over TCP.
* [x] This string is recived on a TCP socket in the wallet system
src/system/client/wallet.rs The wallet systems then inserts the item into the
ECS systeam. Please see the entity list below.
* [ ] The "item_sync" system does a join to find items. If it contains a "new" component
it must be synced with the server. Create a Pack and push that out to the
server.
* [ ] The server then has to verify the item is real, through it own wallet system,
which opens a socket to the serverside wallet.
* [ ] The item system does a join on item and item_action, if it finds a match it will perform
that action.

```
Branch: items
```

## Systems Required (Client)
- WalletSystem
    - Recives the data through TCP and creates the item entity. 
    - Entity is described in the digram above.
    - Transform value is going to be a spot in the inventory.

- ItemSyncSystem
    - If an item contains a "Sync component", send a message to the server
    notifying it of the item.

- ItemSystem
    - If an item contains an ItemAction component, operate on the item
    
## Systems Required (Server)
- Item system
    - Things will happen to items in this system 

- WalletSystem
    - Verifies that items are all good

## Entities and Components 
- Item (Entity)
    - Item (Component)
        - This should contain the json string
    - Sync (Component)
        - This notifies the system the item has been synced with the 
        server
        - Remove Coponent after sync
    - Transform (Component)
        - This contains the location on the map.
    - SpriteRender (Component)
        - The item sprite
    - ItemAction (Component)
        - This is a component that will signal the ItemSytem to do something

- LifeformComponent (Component)
    - Add a new field called inventory

- Inventory
    - Vec<Items>
