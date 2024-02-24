# combat and HUD

## intro and player

health component
attach it to the player
    start with 15/20
HUD
    font
    layer
        readable scale constants for tooltips etc
    clear it in tick
    render system
        print player's helth
        and display a bar
        schedule it

# refactor

name render systems render_...
const for layer indices
const for num layers
