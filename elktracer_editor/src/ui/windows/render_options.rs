use std::error::Error;

use elkengine_core::{
    glow::{self, HasContext},
    imgui::{self, TextureId},
};
use elktracer_core::{
    color::Color,
    material::{
        lambert::LambertMaterial, metal::MetalMaterial,
        transparent::TransparentMaterial,
    },
    math::vector3::Vec3f,
    raytracer::{CameraRenderOptions, Raytracer, image::Image},
    scene::sphere::Sphere,
};

use super::UiWindow;

pub struct RenderOptionsWindow {
    raytracer: Box<Raytracer>,
    camera_options: CameraRenderOptions,
    result_texture: Option<TextureId>,
}

impl RenderOptionsWindow {
    pub fn new() -> Self {
        let mut raytracer = Raytracer::new();

        let material_ground = LambertMaterial::new(Color::new(0.8, 0.8, 0.0));
        let material_center = LambertMaterial::new(Color::new(0.1, 0.2, 0.5));
        let material_left = TransparentMaterial::new(1.5);
        let material_bubble = TransparentMaterial::new(1.0 / 1.5);
        let material_right = MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

        raytracer.add_scene_object(Sphere::new(
            Vec3f::new(0.0, -100.5, -1.0),
            100.0,
            Box::new(material_ground),
        ));

        raytracer.add_scene_object(Sphere::new(
            Vec3f::new(1.0, 0.0, 1.2),
            0.5,
            Box::new(material_center),
        ));

        raytracer.add_scene_object(Sphere::new(
            Vec3f::new(3.0, 0.5, 1.0),
            0.5,
            Box::new(material_left),
        ));

        raytracer.add_scene_object(Sphere::new(
            Vec3f::new(3.0, 0.5, 1.0),
            0.4,
            Box::new(material_bubble),
        ));

        raytracer.add_scene_object(Sphere::new(
            Vec3f::new(1.0, 0.2, -2.0),
            0.5,
            Box::new(material_right),
        ));

        Self {
            raytracer: Box::new(raytracer),
            camera_options: CameraRenderOptions {
                aspect_ratio: 16.0 / 9.0,
                image_width: 600,
                position: Vec3f::new(12.0, 2.0, 3.0),
                look_at: Vec3f::new(0.0, 0.0, 0.0),
                up: Vec3f::new(0.0, 1.0, 0.0),
                fov_vertical_degrees: 15.0,
                defocus_angle: 0.6,
                focus_distance: 10.0,
            },
            result_texture: None,
        }
    }

    fn load_texture(
        glow_context: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
        image: &Image,
    ) -> Result<TextureId, Box<dyn Error>> {
        let gl_texture = unsafe {
            glow_context
                .create_texture()
                .expect("unable to create OpenGL texture")
        };

        unsafe {
            glow_context.bind_texture(glow::TEXTURE_2D, Some(gl_texture));
            glow_context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as _,
            );
            glow_context.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as _,
            );
            glow_context.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as _, // When generating a texture like this, you're probably working in linear color space
                image.width() as _,
                image.height() as _,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(image.data()),
            )
        }

        Ok(textures.insert(gl_texture))
    }
}

impl Default for RenderOptionsWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl UiWindow for RenderOptionsWindow {
    fn update(
        &mut self,
        _delta_time: std::time::Duration,
        ui: &mut elkengine_core::imgui::Ui,
        glow_context: &glow::Context,
        textures: &mut imgui::Textures<glow::Texture>,
    ) {
        ui.window("Render_Options")
            .size([300.0, 200.0], imgui::Condition::FirstUseEver)
            .build(|| {
                if ui.button("Generate image") {
                    log::info!("Generating image ...");

                    let image = self.raytracer.render_image(
                        &self.camera_options,
                        10,
                        10,
                    );

                    self.result_texture = Some(
                        Self::load_texture(glow_context, textures, &image)
                            .expect("Failed to load image texture"),
                    );
                }

                if self.result_texture.is_some() {
                    imgui::Image::new(
                        self.result_texture.unwrap(),
                        [600.0, 337.0],
                    )
                    .build(ui);
                }
            });
    }
}
