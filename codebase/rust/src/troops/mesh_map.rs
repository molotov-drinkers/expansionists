use std::collections::HashMap;

use super::surface::surface::Surface;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MeshId {
  Cannon,
  Tank1,
  Tonk,
  Truck1,

  Boat1,
  Boat2,
  Boat3,
  Boat4,
  Boat5,
  Boat6,
  Galleon,
}

pub struct TroopMesh {
  pub scene_name: String,
  pub surface: Surface,
  pub surface_to_be_colored: i32,
}

type MeshMap = HashMap<MeshId, TroopMesh>;

impl TroopMesh {

  pub fn get_land_meshes() -> MeshMap {
    let mut meshes = MeshMap::new();
    meshes.insert(
      MeshId::Cannon,
      TroopMesh { scene_name: "cannon".to_string(), surface: Surface::Land, surface_to_be_colored: 1, },
    );
    meshes.insert(
      MeshId::Tank1,
      TroopMesh { scene_name: "tank_1".to_string(), surface: Surface::Land, surface_to_be_colored: 0, },
    );
    meshes.insert(
      MeshId::Tonk,
      TroopMesh { scene_name: "tonk".to_string(), surface: Surface::Land, surface_to_be_colored: 0, },
    );
    meshes.insert(
      MeshId::Truck1,
      TroopMesh { scene_name: "truck_1".to_string(), surface: Surface::Land, surface_to_be_colored: 4, },
    );

    meshes
  }

  pub fn get_sea_meshes() -> MeshMap {
    let mut meshes = MeshMap::new();

    meshes.insert(
      MeshId::Boat1,
      TroopMesh { scene_name: "boat_1".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );
    meshes.insert(
      MeshId::Boat2,
      TroopMesh { scene_name: "boat_2".to_string(), surface: Surface::Sea, surface_to_be_colored: 0, },
    );
    meshes.insert(
      MeshId::Boat3,
      TroopMesh { scene_name: "boat_3".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );
    meshes.insert(
      MeshId::Boat4,
      TroopMesh { scene_name: "boat_4".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );
    meshes.insert(
      MeshId::Boat5,
      TroopMesh { scene_name: "boat_5".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );
    meshes.insert(
      MeshId::Boat6,
      TroopMesh { scene_name: "boat_6".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );
    meshes.insert(
      MeshId::Galleon,
      TroopMesh { scene_name: "galleon".to_string(), surface: Surface::Sea, surface_to_be_colored: 2, },
    );

    meshes
  }
}