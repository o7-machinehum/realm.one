# IO Refactor
The IO resource is done is a silly way, I forces system to push and 
pop from the vector checking the value. This prohibits things happening in parallel. 

There should be a event channel for each system. At the top of the system there should
an enum for each possible event, these are the inputs into the system itself.

The networking system on the server and client side should take the Pack and create and event from
it. This should then be piped into event where the corresponding system will deal with that data.

```
Branch Name: io_fix
Depends On: nil
Required for: nil 
Related to: null?
```
