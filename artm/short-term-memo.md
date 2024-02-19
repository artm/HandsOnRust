map has a vec of tiles
map_idx converts x y to index
map's render renders tiles
main
    has state
        contains a map
        implements game state
        renders map in tick
    in main()
        constructs ctx
        starts main loop with ctx and state