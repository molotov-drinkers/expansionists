use godot::{classes::{CharacterBody3D, ICharacterBody3D}, prelude::*};

use crate::globe::coordinates_system::{coordinates_system::CoordinatesSystem, virtual_planet::VirtualPlanet};

pub enum TypesOfTarget {
  Troop,
}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Projectile {
  base: Base<CharacterBody3D>,
  showing: bool,
  
  pub trajectory: Vec<Vector3>,
  pub trajectory_is_set: bool,
  current_trajectory_point: usize,

  pub target: Option<TypesOfTarget>,

  pub up_to_date_target_position: Vector3,
  current_position: Vector3,
}

#[godot_api]
impl ICharacterBody3D for Projectile {
  fn init(base: Base<CharacterBody3D>) -> Projectile {

    Projectile {
      base: base,
      showing: true,

      trajectory: Vec::new(),
      trajectory_is_set: false,
      current_trajectory_point: 0,

      target: None,

      up_to_date_target_position: Vector3::ZERO,
      current_position: Vector3::ZERO,
    }
  }

  fn ready(&mut self) {
    let showing = self.showing;
    self.base_mut().set_visible(showing);
  }

  fn process(&mut self, delta: f64) {
    self.maybe_upsert_trajectory();
    self.move_towards_target(delta);
  }
}

impl Projectile {
  fn maybe_upsert_trajectory(&mut self) {
    if self.trajectory_is_set {
      return;
    }

    let trajectory = CoordinatesSystem::get_geodesic_trajectory(
      self.base().get_global_transform().origin,
      // todo: was self.up_to_date_target_position alredy set at this point?
      self.up_to_date_target_position,
      VirtualPlanet::get_planet_radius() as f32
    );
    self.trajectory = trajectory.to_vec();
    self.trajectory_is_set = true;
  }

  fn move_towards_target(&mut self, _delta: f64) {
    let target_position = self.up_to_date_target_position;
    let current_position = self.base().get_global_transform().origin;

    let direction = (target_position - current_position).try_normalized();
    let on_the_last_waypoint = self.current_trajectory_point == (self.trajectory.len() -1);

    if direction.is_none() && !on_the_last_waypoint {
      self.current_trajectory_point = self.current_trajectory_point + 1;
      return;
    }

    let Some(direction) = direction else {
      godot_error!("Expected Projectile direction to be a Vector3");
      self.base_mut().queue_free();

      // todo: Decide if should deal damage to the enemy troop
      return
    };

    const PROJECTILE_SPEED: f32 = 0.95;
    let velocity = direction * PROJECTILE_SPEED;

    // self.set_orientation(direction);
    self.base_mut().set_velocity(velocity);
    self.base_mut().move_and_slide();

    let current_distance = current_position.distance_to(target_position);
    let too_close_to_the_waypoint = current_distance < 0.1;

    if too_close_to_the_waypoint && !on_the_last_waypoint {
      self.current_trajectory_point = self.current_trajectory_point + 1;
    }

    if too_close_to_the_waypoint && on_the_last_waypoint {
      self.base_mut().queue_free();
      // todo: should deal damage to the enemy troop
    }

  }


  // todo: copy from troop, rename and refactor it properly
  //// Sets orientation to respect the globe trajectory and gravity
  //// if the troop is moving, it will set the orientation to the direction it's moving
  // fn set_orientation(&mut self, trajectory_vector: Vector3) {
  //   // This is the "up" direction on the surface
  //   let normal = self.base().get_global_position().normalized();

  //   // Calculate the right vector using the cross product (normal x forward)
  //   let right = normal
  //     .cross(trajectory_vector)
  //     .try_normalized()
  //     .expect("normal and forward expected to exist");
  
  //   // Calculate the new forward vector as the cross product of right and normal
  //   let new_forward = right
  //     .cross(normal)
  //     .try_normalized()
  //     .expect("right vector expected to exist");
  
  //   // Create a new rotation basis
  //   let basis = Basis::new_looking_at(new_forward, normal, true);

  //   let origin = self.base().get_global_position();
  //   self.base_mut().set_global_transform(Transform3D::new(
  //     basis, 
  //     origin
  //   ));
  // }
}