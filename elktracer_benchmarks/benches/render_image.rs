use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use elktracer_core::{
    color::Color,
    material::{
        lambert::LambertMaterial, metal::MetalMaterial,
        transparent::TransparentMaterial,
    },
    math::vector3::Vec3f,
    raytracer::Raytracer,
    scene::sphere::Sphere,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 20;
    let max_ray_depth = 20;

    let mut raytracer = Raytracer::new(
        image_width,
        aspect_ratio,
        samples_per_pixel,
        max_ray_depth,
    );

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
        Vec3f::new(0.0, 0.0, -1.2),
        0.5,
        Box::new(material_center),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(-1.0, 0.0, -1.0),
        0.4,
        Box::new(material_bubble),
    ));

    raytracer.add_scene_object(Sphere::new(
        Vec3f::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    ));

    let mut group = c.benchmark_group("sample-size-example");
    group.warm_up_time(Duration::from_secs(5));
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(15));
    group.bench_function("render_image", |b| {
        b.iter(|| {
            raytracer.render_image(
                Vec3f::new(-2.0, 2.0, 1.0),
                Vec3f::new(0.0, 0.0, -1.0),
                Vec3f::new(0.0, 1.0, 0.0),
                20.0,
                10.0,
                3.4,
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
