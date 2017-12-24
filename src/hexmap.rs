pub enum TerrainKind {
    //Dirt,
    //Grass,
    //Sand,
    Stone
}

pub struct Hexmap {
    // FIXME: dep-publish; make a thing to get these.
    pub tiles: [[TerrainKind; 4]; 2]
}

impl Hexmap {
    pub fn new() -> Hexmap {
        Hexmap {
            tiles: [
//                [ TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone ],
//               [ TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone ],
                [ TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone ],
                [ TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone, TerrainKind::Stone ]
            ]
        }
    }
}