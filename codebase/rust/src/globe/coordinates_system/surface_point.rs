
use godot::{classes::{Area3D, IArea3D}, meta::GodotType, prelude::*};
use crate::globe::territory::types::TerritoryId;

type Latitude = i16;
type Longitude = i16;
pub type Coordinates = (Latitude, Longitude);

#[derive(Debug, Clone)]
pub struct SurfacePointMetadata {
  pub cartesian: Vector3,
  pub lat_long: Coordinates,
  pub territory_id: Option<TerritoryId>,
}

#[derive(Debug, GodotClass)]
#[class(base=Area3D)]
pub struct SurfacePoint {
  base: Base<Area3D>,
  surface_point_metadata: SurfacePointMetadata,
}

#[godot_api]
impl IArea3D for SurfacePoint {
  fn init(base: Base<Area3D>) -> SurfacePoint {
    SurfacePoint {
      base: base,
      surface_point_metadata: get_blank_surface_point_metadata(),
    }
  }
}

impl GodotConvert for SurfacePoint {
  type Via = Gd<SurfacePoint>;
}

// impl FromGodot for SurfacePoint {
//   fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
//     // let pp: Gd<SurfacePoint> = via;

//     godot_print_rich!("SurfacePoint::try_from_godot: {:?}", via);

//     let gg = via.try_cast::<SurfacePoint>()
//       .expect("Expected SurfacePoint");

//     godot_print!("gg: {:?}", gg);

//     let aa: GdRef<'_, SurfacePoint> = gg.bind();
//     godot_print!("meta: {:?}", aa);
//     // let meta = aa.get_surface_point_metadata();

//     // godot_print!("meta: {:?}", meta);

//     // let aa =gg.bind_mut();

//     // let a = gg.bind();
//     // godot_print_rich!("a: {:?}", a);

//     // let aa = a.base_field();

//     // let ff: SurfacePoint = SurfacePoint {
//     //   base: aa.clone(),
//     //   surface_point_metadata: get_blank_surface_point_metadata(),
//     // };

//     todo!()
//     // Ok(gg)
    
//     // let mut surface_point = SurfacePoint::init(Area3D::new());
//     // let surface_point_metadata = SurfacePointMetadata {
//     //   cartesian: via.get("cartesian").try_to()?,
//     //   lat_long: via.get("lat_long").try_to()?,
//     //   territory_id: via.get("territory_id").try_to()?,
//     // };
//     // Ok(SurfacePoint {
//     //   base: Gd<Area3D::new_alloc()>,
//     //   surface_point_metadata: get_blank_surface_point_metadata(),
//     // })

//     // todo!()
//   }

//   // fn try_from_variant(variant: &Variant) -> Result<Self, ConvertError> {
//   //   godot_print_rich!("SurfacePoint::try_from_variant: {:?}", variant);


//   //   variant.
//   //   let ping = variant.get_property();
//   //   godot_print_rich!("get_property: {:?}", ping);

//   //   let typea = ping.get_type();
//   //   godot_print_rich!("get_type: {:?}", typea);

//   //   // let gg: SurfacePoint = typea.into();
      
//   //   todo!()
//   // }
// }

// impl Into for SurfacePoint {
//   fn into(self) -> Variant {
//     godot_print_rich!("SurfacePoint::into: {:?}", self);
//     let mut variant = Variant::();
//     variant.set("cartesian", self.surface_point_metadata.cartesian);
//     variant.set("lat_long", self.surface_point_metadata.lat_long);
//     variant.set("territory_id", self.surface_point_metadata.territory_id);
//     variant
//   }
// }



impl SurfacePoint {
  pub fn set_surface_point_metadata(&mut self, surface_point_metadata: SurfacePointMetadata) {
    self.surface_point_metadata = surface_point_metadata;
  }

  pub fn get_surface_point_metadata(&self) -> &SurfacePointMetadata {
    &self.surface_point_metadata
  }
  pub fn get_surface_point_metadata_mut(&mut self) -> &mut SurfacePointMetadata {
    &mut self.surface_point_metadata
  }
}

fn get_blank_surface_point_metadata() -> SurfacePointMetadata {
  SurfacePointMetadata {
    cartesian: Vector3::new(0.0, 0.0, 0.0),
    lat_long: (0, 0),
    territory_id: None,
  }
}