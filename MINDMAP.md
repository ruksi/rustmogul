Well, this isn't actually a mindmap but naming different entities and musing how things should work.

There are numerous Players:

+ a Player represents a single controller like a human or a bot
+ each Player has a player_id:Uuid
+ each Player has a name:String

There are numerous Conflicts:

+ each Conflict has a conflict_id:Uuid
+ each Conflict has a name:String
+ each Conflict has a join_closes_at:DateTime
+ each Conflict has a min_player_count:u8
+ each Conflict has a max_player_count:u8
+ each Conflict has a start_conditions:StartConditions
    + if max_player_count is reached, don't wait for join_closes_at even one exists
    + if join_closes_at is reached:
        + if min_player_count is reached, start the Conflict
+ different Conflicts can have different rules

Players can AutoJoin Conflicts:

+ AutoJoin can have filters about which kind of Conflicts to join
+ then Player will get automatically matched to a Conflict

Players can ManualJoin Conflicts:

+ Player must know the ConflictId to ManualJoin it
+ Player might need to know the Conflict's password to join it

KEEP THE ABOVE IN MIND ^^^ WILL IMPLEMENT LATER ➡️

Ledge is the central database for game logic.
Currently, it is simplified to host only one Conflict.

Ledge has:

+ _Actions_: async events that can be triggered by Players to change the state of the Conflict
+ _Reactions_: async events that sent by Ledger to tell Players what is the state of the Conflict
+ _Reflexes_: sync calls that the Ledger will respond immediately. Avoid these as they will block.

The idea was to mildly start splitting the logic to the server-client style, not fully
but just to get the hang of it. The idea is that the server will be the source of truth
and clients shouldn't be allowed to directly change the game state, only how it's presented.

There are some admin-level actions that should be removed later, like spawning robots.

I envision that these events are turned to some form of web socket or HTTP messages.

Currently, the game logic is also ran by bevy's Entity-Component-System, but I'm not sure
if that's the best way to do it in the long run as most of the logic is in simple structs.
The ECS was just a simple way to get event handling, multi-threading and timers out-of-the-box.
