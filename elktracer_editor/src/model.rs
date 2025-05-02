// use elktracer_core::Vec3f;

// #[derive(Debug, Clone)]
// pub enum ObjectType {
//     Sphere { radius: f64 },
// }

// #[derive(Debug, Clone)]
// pub struct ObjectModel {
//     pub id: String,
//     pub position: Vec3f,
//     pub material_id: String,
//     pub object_type: ObjectType,
// }

// impl ObjectModel {
//     pub fn new(
//         id: &str,
//         position: Vec3f,
//         material_id: &str,
//         object_type: ObjectType,
//     ) -> Self {
//         Self {
//             id: id.to_string(),
//             position,
//             material_id: material_id.to_string(),
//             object_type,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum MaterialType {
//     Lambert { albedo: [f32; 3] },
//     Metal { albedo: [f32; 3], fuzziness: f64 },
//     Transparent { refraction_index: f64 },
// }

// #[derive(Debug, Clone)]
// pub struct MaterialModel {
//     pub id: String,
//     pub material_type: MaterialType,
// }

// impl MaterialModel {
//     pub fn new(id: &str, material_type: MaterialType) -> Self {
//         Self {
//             id: id.to_string(),
//             material_type,
//         }
//     }
// }
