# IO Refactor
The IO resource is done is a silly way, I forces system to push and 
pop from the vector checking the value. This prohibits things happening in parallel. 

1. There should be be one input and one output resource
1. The commands should be kept in a hashmap rather than a vector.
2. There should be a function call to where in the argument you specify
   the Cmd::? you want, and that should return a vector of those commands.

```
Branch Name: io_fix
Depends On: nil
Required for: nil 
Related to: null?
```
