# chase system

impl Algorithm2D for Map
    dimensions() -> Point
    in_bounds() -> bool
    the rest will be implemented
impl BaseMap for Map
    get_available_exits(idx) -> SmallVec<...>
        follow the book
    get_path_finding_distance(idx1, idx2) -> f32
        pythagorean

# learn

same method name in different impls for same type

