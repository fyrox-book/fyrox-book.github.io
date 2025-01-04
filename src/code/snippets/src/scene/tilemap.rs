use fyrox::core::algebra::Vector3;
use fyrox::core::log::Log;
use fyrox::core::{uuid, Uuid};
use fyrox::fxhash::FxHashMap;
use fyrox::scene::dim2::collider::{ColliderBuilder, ColliderShape, GeometrySource, TileMapShape};
use fyrox::scene::dim2::rigidbody::RigidBodyBuilder;
use fyrox::scene::rigidbody::RigidBodyType;
use fyrox::scene::tilemap::tileset::{
    TileBounds, TileData, TileMaterialBounds, TileSetPage, TileSetPageSource, TileSetPropertyLayer,
    TileSetPropertyType, TileSetPropertyValue,
};
use fyrox::scene::tilemap::{TileDefinitionHandle, TileGridMap, TileMap, TilePropertyError};
use fyrox::{
    asset::untyped::ResourceKind,
    core::{algebra::Vector2, color::Color, pool::Handle},
    material::{Material, MaterialResource},
    scene::{
        base::BaseBuilder,
        graph::Graph,
        node::Node,
        tilemap::{
            tileset::{TileDefinition, TileSet, TileSetResource},
            TileMapBuilder, Tiles,
        },
    },
};

// ANCHOR: create_tile_map
fn create_tile_map(graph: &mut Graph) -> Handle<Node> {
    // Each tile could have its own material, for simplicity it is just a standard 2D material.
    let material = MaterialResource::new_ok(ResourceKind::Embedded, Material::standard_tile());

    // Create a tile set - it is a data source for the tile map. Tile map will reference the tiles
    // stored in the tile set by handles. We'll create two tile types with different colors.
    let mut tiles = TileGridMap::default();
    let stone_tile = TileDefinitionHandle::new(0, 0, 0, 0);
    let grass_tile = TileDefinitionHandle::new(0, 0, 1, 0);
    tiles.insert(
        stone_tile.tile(),
        TileDefinition {
            material_bounds: TileMaterialBounds {
                material: material.clone(),
                bounds: TileBounds {
                    left_top_corner: Vector2::new(0, 0),
                    right_top_corner: Vector2::new(16, 0),
                    left_bottom_corner: Vector2::new(0, 16),
                    right_bottom_corner: Vector2::new(16, 16),
                },
            },
            data: TileData {
                color: Color::BROWN,
                properties: FxHashMap::default(),
                colliders: FxHashMap::default(),
            },
        },
    );
    tiles.insert(
        grass_tile.tile(),
        TileDefinition {
            material_bounds: TileMaterialBounds {
                material: material.clone(),
                bounds: TileBounds {
                    left_top_corner: Vector2::new(0, 0),
                    right_top_corner: Vector2::new(16, 0),
                    left_bottom_corner: Vector2::new(0, 16),
                    right_bottom_corner: Vector2::new(16, 16),
                },
            },
            data: TileData {
                color: Color::GREEN,
                properties: FxHashMap::default(),
                colliders: FxHashMap::default(),
            },
        },
    );
    let source = TileSetPageSource::Freeform(tiles);
    let page = TileSetPage {
        icon: TileDefinitionHandle::new(0, 0, 0, 0),
        source,
    };
    let mut tile_set = TileSet::default();
    tile_set.insert_page(Vector2::new(0, 0), page);
    let tile_set = TileSetResource::new_ok(ResourceKind::Embedded, tile_set);

    let mut tiles = Tiles::default();

    // Create stone foundation.
    for x in 0..10 {
        for y in 0..2 {
            tiles.insert(Vector2::new(x, y), stone_tile);
        }
    }

    // Add grass on top of it.
    for x in 0..10 {
        tiles.insert(Vector2::new(x, 2), grass_tile);
    }

    // Finally create the tile map.
    TileMapBuilder::new(BaseBuilder::new())
        .with_tile_set(tile_set)
        .with_tiles(tiles)
        .build(graph)
}
// ANCHOR_END: create_tile_map

// ANCHOR: tile_map_physics
fn add_tile_map_physics(tile_map: Handle<Node>, graph: &mut Graph) {
    // Create a new collider with tile map shape.
    let collider = ColliderBuilder::new(BaseBuilder::new())
        .with_shape(ColliderShape::TileMap(TileMapShape {
            tile_map: GeometrySource(tile_map),
            layer_name: "MainColliders".into(),
        }))
        .build(graph);

    // Create a static rigid body with the tile map collider.
    let rigid_body = RigidBodyBuilder::new(BaseBuilder::new().with_children(&[collider]))
        .with_body_type(RigidBodyType::Static)
        .build(graph);
}
// ANCHOR_END: tile_map_physics

// ANCHOR: create_tile_map_with_props
const SOIL: i32 = 1;
const SLIME: i32 = 2;
const SURFACE_TYPE_UUID: Uuid = uuid!("a70a754b-eed5-4e60-bf8a-3239f0b6004b");

fn create_tile_map_with_props(graph: &mut Graph) {
    let material = MaterialResource::new_ok(ResourceKind::Embedded, Material::standard_2d());

    let mut tiles = TileGridMap::default();
    let stone_tile = TileDefinitionHandle::new(0, 0, 0, 0);
    let grass_tile = TileDefinitionHandle::new(0, 0, 1, 0);
    tiles.insert(
        stone_tile.tile(),
        TileDefinition {
            material_bounds: TileMaterialBounds {
                material: material.clone(),
                bounds: TileBounds {
                    left_top_corner: Vector2::new(0, 0),
                    right_top_corner: Vector2::new(16, 0),
                    left_bottom_corner: Vector2::new(0, 16),
                    right_bottom_corner: Vector2::new(16, 16),
                },
            },
            data: TileData {
                color: Color::BROWN,
                properties: [(SURFACE_TYPE_UUID, TileSetPropertyValue::I32(SLIME))]
                    .into_iter()
                    .collect(),
                colliders: FxHashMap::default(),
            },
        },
    );
    tiles.insert(
        grass_tile.tile(),
        TileDefinition {
            material_bounds: TileMaterialBounds {
                material: material.clone(),
                bounds: TileBounds {
                    left_top_corner: Vector2::new(0, 0),
                    right_top_corner: Vector2::new(16, 0),
                    left_bottom_corner: Vector2::new(0, 16),
                    right_bottom_corner: Vector2::new(16, 16),
                },
            },
            data: TileData {
                color: Color::GREEN,
                properties: [(SURFACE_TYPE_UUID, TileSetPropertyValue::I32(SOIL))]
                    .into_iter()
                    .collect(),
                colliders: FxHashMap::default(),
            },
        },
    );
    let source = TileSetPageSource::Freeform(tiles);
    let page = TileSetPage {
        icon: TileDefinitionHandle::new(0, 0, 0, 0),
        source,
    };
    let mut tile_set = TileSet::default();
    tile_set.insert_page(Vector2::new(0, 0), page);
    tile_set.properties.push(TileSetPropertyLayer {
        name: "SurfaceType".into(),
        uuid: SURFACE_TYPE_UUID,
        prop_type: TileSetPropertyType::F32,
        named_values: Vec::default(),
    });
    let tile_set = TileSetResource::new_ok(ResourceKind::Embedded, tile_set);

    // ..
}

fn calculate_speed_factor(tile_map: &TileMap, player_position: Vector3<f32>) -> f32 {
    let grid_position = tile_map.world_to_grid(player_position);

    match tile_map.tile_property_value(grid_position, SURFACE_TYPE_UUID) {
        Ok(SOIL) => 1.0,
        Ok(SLIME) => 0.7,
        Ok(_) => {
            Log::err("Unknown surface type");
            1.0
        }
        // See fyrox::scene::tilemap::TilePropertyError for a list of possible errors.
        Err(err) => {
            Log::err(err.to_string());
            1.0
        }
    }
}

// ANCHOR_END: create_tile_map_with_props
