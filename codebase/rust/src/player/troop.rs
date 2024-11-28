use godot::{classes::{ICharacterBody3D, CharacterBody3D, StaticBody3D}, prelude::*};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Troop {
  base: Base<CharacterBody3D>,
  pub territory: String,
  pub owner: String,
  pub count: i32,

  pub damage: i32,
  pub hp: i32,
  pub speed: i32,
  pub alive: bool,
}

#[godot_api]
impl ICharacterBody3D for Troop {
  fn init(base: Base<CharacterBody3D>) -> Troop {

    Troop {
      base: base,
      territory: "".to_string(),
      owner: "".to_string(),
      count: 0,

      hp: 100,
      damage: 5,
      speed: 20,
      alive: true,
    }
  }

  fn ready(&mut self) {
    godot_print!("Troop ready");
    // TODO: how to attach a troop to a territory?


    // According to Godot doc:
    // https://docs.godotengine.org/en/stable/classes/class_CharacterBody3D.html#class-CharacterBody3D-method-get-colliding-bodies
    // contact_monitor has to be enabled to get colliding bodies
    // self.base_mut().set_contact_monitor(true);

    // self.base_mut().set_max_contacts_reported(1);
  }

  fn physics_process(&mut self, _delta: f64) {
    // godot_print!("Troop process");

    // let gg = self.base_mut();

    // let colliding_bodies = self.base_mut().get_colliding_bodies();

    // colliding_bodies.iter_shared().for_each(|colliding_body| {
    //   let colliding_body = colliding_body.cast::<Node3D>();

    //   let parent = colliding_body.get_parent().unwrap();
    //   // let parent = parent.cast::<StaticBody3D>();
    //   let parent_name = parent.get_name();
    //   // godot_print!("parent: {:?}", parent_name);
    // });


    // colliding_bodies.iter_shared().for_each(|body| {
    //   let body = body.cast::<Node3D>();
    //   let body_name = body.get_name();
    //   godot_print!("Colliding with: {:?}", body_name);
    // });
  }
}