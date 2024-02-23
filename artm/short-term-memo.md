# intent messages

component: motion intent
    entity
    destination (pos)
motion systen
    for_each
    handle motion
    remove the message entity
insert the system in player/monster schedules
generate motion intent messages inside player input and random walk

# restruct

let map can enter accept point and fix all calls
consts for cardinal directions and reuse that in that one array of deltas

# review

tuples: tuples of single value don't exist?
legion's CommandBuffer::push()