# random move

tag
system
flush schedule after collisions
    so removed entities aren't considered for random motion

# turn based

turn state
    awaiting input
    player
    monsters
turn state is a resource
end turn system
    replaces turn state with the next
        not for awaiting input
    player input advances to player turn after motion input
schedulers
    split base on turn state
    build all three
    execute the right one based on current state
