use fyrox::core::algebra::Vector3;
use fyrox::scene::base::{Property, PropertyValue};
use fyrox::scene::dim2::collider::{ColliderBuilder, ColliderShape, GeometrySource, TileMapShape};
use fyrox::scene::dim2::rigidbody::RigidBodyBuilder;
use fyrox::scene::rigidbody::RigidBodyType;
use fyrox::scene::tilemap::TileMap;
use fyrox::{
    asset::untyped::ResourceKind,
    core::{algebra::Vector2, color::Color, math::Rect, pool::Handle},
    material::{Material, MaterialResource},
    scene::{
        base::BaseBuilder,
        graph::Graph,
        node::Node,
        tilemap::{
            tileset::{TileCollider, TileDefinition, TileSet, TileSetResource},
            Tile, TileMapBuilder, Tiles,
        },
    },
};

// ANCHOR: create_tile_map
fn create_tile_map(graph: &mut Graph) -> Handle<Node> {
    // Each tile could have its own material, for simplicity it is just a standard 2D material.
    let material = MaterialResource::new_ok(ResourceKind::Embedded, Material::standard_2d());

    // Create a tile set - it is a data source for the tile map. Tile map will reference the tiles
    // stored in the tile set by handles. We'll create two tile types with different colors.
    let mut tile_set = TileSet::default();
    let stone_tile = tile_set.add_tile(TileDefinition {
        material: material.clone(),
        uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        collider: TileCollider::Rectangle,
        color: Color::BROWN,
        position: Default::default(),
        properties: vec![],
    });
    let grass_tile = tile_set.add_tile(TileDefinition {
        material,
        uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        collider: TileCollider::Rectangle,
        color: Color::GREEN,
        position: Default::default(),
        properties: vec![],
    });
    let tile_set = TileSetResource::new_ok(ResourceKind::Embedded, tile_set);

    let mut tiles = Tiles::default();

    // Create stone foundation.
    for x in 0..10 {
        for y in 0..2 {
            tiles.insert(Tile {
                position: Vector2::new(x, y),
                definition_handle: stone_tile,
            });
        }
    }

    // Add grass on top of it.
    for x in 0..10 {
        tiles.insert(Tile {
            position: Vector2::new(x, 2),
            definition_handle: grass_tile,
        });
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
        }))
        .build(graph);

    // Create a static rigid body with the tile map collider.
    let rigid_body = RigidBodyBuilder::new(BaseBuilder::new().with_children(&[collider]))
        .with_body_type(RigidBodyType::Static)
        .build(graph);
}
// ANCHOR_END: tile_map_physics

// ANCHOR: create_tile_map_with_props
const SOIL: u8 = 1;
const SLIME: u8 = 2;

fn create_tile_map_with_props(graph: &mut Graph) {
    let material = MaterialResource::new_ok(ResourceKind::Embedded, Material::standard_2d());

    let mut tile_set = TileSet::default();
    let stone_tile = tile_set.add_tile(TileDefinition {
        material: material.clone(),
        uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        collider: TileCollider::Rectangle,
        color: Color::BROWN,
        position: Default::default(),
        properties: vec![Property {
            name: "SurfaceType".to_string(),
            value: PropertyValue::U8(SOIL),
        }],
    });
    let slime_tile = tile_set.add_tile(TileDefinition {
        material,
        uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0),
        collider: TileCollider::Rectangle,
        color: Color::GREEN,
        position: Default::default(),
        properties: vec![Property {
            name: "SurfaceType".to_string(),
            value: PropertyValue::U8(SLIME),
        }],
    });
    let tile_set = TileSetResource::new_ok(ResourceKind::Embedded, tile_set);

    // ..
}

fn calculate_speed_factor(tile_map: &TileMap, player_position: Vector3<f32>) -> f32 {
    let grid_position = tile_map.world_to_grid(player_position);

    if let Some(tile) = tile_map.tiles.get(&grid_position) {
        if let Some(tile_set) = tile_map.tile_set() {
            if let Some(tile_set_data) = tile_set.data_ref().as_loaded_ref() {
                let tile_definition = &tile_set_data.tiles[tile.definition_handle];

                if let Some(property) = tile_definition
                    .properties
                    .iter()
                    .find(|p| p.name == "SurfaceType")
                {
                    if let PropertyValue::U8(surface_type) = property.value {
                        return match surface_type {
                            SOIL => 1.0,
                            // Green slime tile slows down the player.
                            SLIME => 0.7,
                            _ => 1.0,
                        };
                    }
                }
            }
        }
    }

    1.0
}

// ANCHOR_END: create_tile_map_with_props
