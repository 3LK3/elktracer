use std::{sync::Arc, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use elktracer_core::{
    Camera, Color, LambertMaterial, MetalMaterial, RayHitTest, Raytracer,
    RenderOptions, Sphere, TransparentMaterial, Vec3f,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 20;
    let max_ray_depth = 20;

    let camera = Camera::new(
        Vec3f::new(-2.0, 2.0, 1.0),
        Vec3f::new(0.0, 0.0, -1.0),
        Vec3f::new(0.0, 1.0, 0.0),
        15.0,
        0.6,
        10.0,
    );

    let mut raytracer = Raytracer::new();

    let mut group = c.benchmark_group("sample-size-example");
    group.warm_up_time(Duration::from_secs(5));
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(15));
    group.bench_function("render_image", |b| {
        b.iter(|| {
            let material_ground =
                LambertMaterial::new(Color::new(0.8, 0.8, 0.0));
            let material_center =
                LambertMaterial::new(Color::new(0.1, 0.2, 0.5));
            let material_left = TransparentMaterial::new(1.5);
            let material_bubble = TransparentMaterial::new(1.0 / 1.5);
            let material_right =
                MetalMaterial::new(Color::new(0.8, 0.6, 0.2), 1.0);

            let objects: Vec<Box<dyn RayHitTest>> = vec![
                Box::new(Sphere::new(
                    Vec3f::new(0.0, -100.5, -1.0),
                    100.0,
                    Arc::new(material_ground),
                )),
                Box::new(Sphere::new(
                    Vec3f::new(0.0, 0.0, -1.2),
                    0.5,
                    Arc::new(material_center),
                )),
                Box::new(Sphere::new(
                    Vec3f::new(-1.0, 0.0, -1.0),
                    0.5,
                    Arc::new(material_left),
                )),
                Box::new(Sphere::new(
                    Vec3f::new(-1.0, 0.0, -1.0),
                    0.4,
                    Arc::new(material_bubble),
                )),
                Box::new(Sphere::new(
                    Vec3f::new(1.0, 0.0, -1.0),
                    0.5,
                    Arc::new(material_right),
                )),
            ];

            raytracer.render_image(
                &camera,
                objects,
                RenderOptions {
                    aspect_ratio,
                    image_width,
                    samples_per_pixel,
                    max_ray_depth,
                },
            )
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
