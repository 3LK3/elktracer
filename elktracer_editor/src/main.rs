mod application;
mod error;
// mod model;
mod render_tasks;
mod ui;

use bevy::{
    prelude::*, render::camera::Viewport, window::PrimaryWindow,
    winit::WinitWindows,
};
use bevy_egui::{
    EguiContext, EguiContextPass, EguiContextSettings, EguiPlugin,
    egui::{self, CentralPanel, Frame, TopBottomPanel},
};
use bevy_inspector_egui::DefaultInspectorConfigPlugin;
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use elktracer_json::{load_scene_model, model::SceneModel, save_scene_model};
use render_tasks::{ElktracerRenderSystem, handle_elktracer_render_tasks};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1440.0, 900.0).into(),
                    position: WindowPosition::At((20, 20).into()),
                    title: "Elktracer".to_string(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            DefaultInspectorConfigPlugin,
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(UiState::new())
        .insert_resource(ElktracerRenderSystem::default())
        .add_systems(Startup, set_window_icon)
        .add_systems(EguiContextPass, show_ui_system)
        .add_systems(PostUpdate, set_camera_viewport.after(show_ui_system))
        .add_systems(Update, handle_elktracer_render_tasks)
        // .register_type::<Option<Handle<Image>>>()
        // .register_type::<AlphaMode>()
        .run();
}

fn set_camera_viewport(
    ui_state: Res<UiState>,
    primary_window: Query<&mut Window, With<PrimaryWindow>>,
    egui_settings: Single<&EguiContextSettings>,
    mut cam: Single<&mut Camera, With<MainCamera>>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };

    let scale_factor = window.scale_factor() * egui_settings.scale_factor;

    let viewport_pos =
        ui_state.viewport_rect.left_top().to_vec2() * scale_factor;
    let viewport_size = ui_state.viewport_rect.size() * scale_factor;

    let physical_position =
        UVec2::new(viewport_pos.x as u32, viewport_pos.y as u32);
    let physical_size =
        UVec2::new(viewport_size.x as u32, viewport_size.y as u32);

    // The desired viewport rectangle at its offset in "physical pixel space"
    let rect = physical_position + physical_size;

    let window_size = window.physical_size();
    // wgpu will panic if trying to set a viewport rect which has coordinates extending
    // past the size of the render target, i.e. the physical window in our case.
    // Typically this shouldn't happen- but during init and resizing etc. edge cases might occur.
    // Simply do nothing in those cases.
    if rect.x <= window_size.x && rect.y <= window_size.y {
        cam.viewport = Some(Viewport {
            physical_position,
            physical_size,
            depth: 0.0..1.0,
        });
    }
}

fn set_window_icon(windows: NonSend<WinitWindows>) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = elktracer_core::image_rs::open("assets/icons/app.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon =
        winit::window::Icon::from_rgba(icon_rgba, icon_width, icon_height)
            .unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn show_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<UiState, _>(|world, mut ui_state| {
        ui_state.ui(world, egui_context.get_mut())
    });
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct UiState {
    state: DockState<application::GuiWindow>,
    viewport_rect: egui::Rect,
    scene_model: SceneModel,
    render_options: elktracer_core::RenderOptions,
    preview_render_options: elktracer_core::RenderOptions,
}

impl UiState {
    fn new() -> Self {
        let mut state = DockState::new(vec![
            application::GuiWindow::SceneView,
            application::GuiWindow::Rendering,
        ]);
        let tree = state.main_surface_mut();

        // tree.split_below(
        //     NodeIndex::root(),
        //     0.8,
        //     vec![application::GuiWindow::Rendering],
        // );

        let [_, right] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![
                application::GuiWindow::Materials,
                application::GuiWindow::Debug,
            ],
        );

        tree.split_below(right, 0.7, vec![application::GuiWindow::Camera]);

        let [_, left] = tree.split_left(
            NodeIndex::root(),
            0.2,
            vec![application::GuiWindow::SceneObjects],
        );
        tree.split_below(left, 0.7, vec![application::GuiWindow::Preview]);

        Self {
            state,
            viewport_rect: egui::Rect::NOTHING,
            // camera_options: elktracer_core::Camera::new(
            //     Vec3f::new(12.0, 2.0, 3.0),
            //     Vec3f::zero(),
            //     Vec3f::new(0.0, 1.0, 0.0),
            //     15.0,
            //     0.6,
            //     10.0,
            // ),
            // materials: vec![
            //     model::MaterialModel::new(
            //         "ground",
            //         model::MaterialType::Lambert {
            //             albedo: [0.8, 0.8, 0.0],
            //         },
            //     ),
            //     model::MaterialModel {
            //         id: "center".to_string(),
            //         material_type: model::MaterialType::Lambert {
            //             albedo: [0.1, 0.2, 0.5],
            //         },
            //     },
            //     model::MaterialModel {
            //         id: "left".to_string(),
            //         material_type: model::MaterialType::Transparent {
            //             refraction_index: 1.5,
            //         },
            //     },
            //     model::MaterialModel {
            //         id: "bubble".to_string(),
            //         material_type: model::MaterialType::Transparent {
            //             refraction_index: 1.0 / 1.5,
            //         },
            //     },
            //     model::MaterialModel {
            //         id: "right".to_string(),
            //         material_type: model::MaterialType::Metal {
            //             albedo: [0.8, 0.6, 0.2],
            //             fuzziness: 1.0,
            //         },
            //     },
            // ],
            // scene_objects: vec![
            //     model::ObjectModel::new(
            //         "Ground",
            //         Vec3f::new(0.0, -100.5, -1.0),
            //         "ground",
            //         model::ObjectType::Sphere { radius: 100.0 },
            //     ),
            //     model::ObjectModel::new(
            //         "Center",
            //         Vec3f::new(1.0, 0.0, 1.2),
            //         "center",
            //         model::ObjectType::Sphere { radius: 0.5 },
            //     ),
            //     model::ObjectModel::new(
            //         "Left",
            //         Vec3f::new(3.0, 0.5, 1.0),
            //         "left",
            //         model::ObjectType::Sphere { radius: 0.5 },
            //     ),
            //     model::ObjectModel::new(
            //         "Bubble",
            //         Vec3f::new(3.0, 0.5, 1.0),
            //         "bubble",
            //         model::ObjectType::Sphere { radius: 0.4 },
            //     ),
            //     model::ObjectModel::new(
            //         "Right",
            //         Vec3f::new(1.0, 0.2, -2.0),
            //         "right",
            //         model::ObjectType::Sphere { radius: 0.5 },
            //     ),
            // ],
            scene_model: SceneModel::default(),
            render_options: elktracer_core::RenderOptions::new(
                1920,
                16.0 / 9.0,
                50,
                50,
            ),
            preview_render_options: elktracer_core::RenderOptions::new(
                320,
                16.0 / 9.0,
                10,
                10,
            ),
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        // ctx.set_theme(ThemePreference::Light);

        let style = Style::from_egui(ctx.style().as_ref());

        TopBottomPanel::top("egui_dock::MenuBar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.load_scene_from_file();
                    }
                    if ui.button("Save").clicked() {
                        self.save_scene_to_file();
                    }
                    if ui.button("Exit").clicked() {
                        world.send_event(AppExit::Success);
                    }
                });
            })
        });

        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                let mut tab_viewer = application::Application {
                    world,
                    viewport_rect: &mut self.viewport_rect,
                    scene_model: &mut self.scene_model,
                    render_options: &mut self.render_options,
                    preview_render_options: &mut self.preview_render_options,
                };
                DockArea::new(&mut self.state)
                    .style(style)
                    .show_inside(ui, &mut tab_viewer);
            });
    }

    fn load_scene_from_file(&mut self) {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("Scene (json)", &["json"])
            .pick_file()
        {
            self.scene_model = load_scene_model(&file);
        }
    }

    fn save_scene_to_file(&mut self) {
        if let Some(file) = rfd::FileDialog::new()
            .add_filter("Scene (json)", &["json"])
            .save_file()
        {
            save_scene_model(&file, &self.scene_model);
        }
    }
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // window.set_maximized(true);

//     // let box_size = 2.0;
//     // let box_thickness = 0.15;
//     // let box_offset = (box_size + box_thickness) / 2.0;

//     // // left - red
//     // let mut transform = Transform::from_xyz(-box_offset, box_offset, 0.0);
//     // transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

//     // commands.spawn((
//     //     Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::srgb(0.63, 0.065, 0.05),
//     //         ..Default::default()
//     //     })),
//     //     transform,
//     // ));
//     // // right - green
//     // let mut transform = Transform::from_xyz(box_offset, box_offset, 0.0);
//     // transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
//     // commands.spawn((
//     //     Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
//     //     transform,
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::srgb(0.14, 0.45, 0.091),
//     //         ..Default::default()
//     //     })),
//     // ));
//     // // bottom - white
//     // commands.spawn((
//     //     Mesh3d(meshes.add(Cuboid::new(
//     //         box_size + 2.0 * box_thickness,
//     //         box_thickness,
//     //         box_size,
//     //     ))),
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::srgb(0.725, 0.71, 0.68),
//     //         ..Default::default()
//     //     })),
//     // ));
//     // // top - white
//     // let transform = Transform::from_xyz(0.0, 2.0 * box_offset, 0.0);
//     // commands.spawn((
//     //     Mesh3d(meshes.add(Cuboid::new(
//     //         box_size + 2.0 * box_thickness,
//     //         box_thickness,
//     //         box_size,
//     //     ))),
//     //     transform,
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::srgb(0.725, 0.71, 0.68),
//     //         ..Default::default()
//     //     })),
//     // ));
//     // // back - white
//     // let mut transform = Transform::from_xyz(0.0, box_offset, -box_offset);
//     // transform.rotate(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
//     // commands.spawn((
//     //     Mesh3d(meshes.add(Cuboid::new(
//     //         box_size + 2.0 * box_thickness,
//     //         box_thickness,
//     //         box_size + 2.0 * box_thickness,
//     //     ))),
//     //     transform,
//     //     MeshMaterial3d(materials.add(StandardMaterial {
//     //         base_color: Color::srgb(0.725, 0.71, 0.68),
//     //         ..Default::default()
//     //     })),
//     // ));

//     // // ambient light
//     // commands.insert_resource(AmbientLight {
//     //     color: Color::WHITE,
//     //     brightness: 0.02,
//     //     ..default()
//     // });
//     // // top light
//     // commands
//     //     .spawn((
//     //         Mesh3d(meshes.add(Plane3d::default().mesh().size(0.4, 0.4))),
//     //         Transform::from_matrix(Mat4::from_scale_rotation_translation(
//     //             Vec3::ONE,
//     //             Quat::from_rotation_x(std::f32::consts::PI),
//     //             Vec3::new(0.0, box_size + 0.5 * box_thickness, 0.0),
//     //         )),
//     //         MeshMaterial3d(materials.add(StandardMaterial {
//     //             base_color: Color::WHITE,
//     //             emissive: LinearRgba::WHITE * 100.0,
//     //             ..Default::default()
//     //         })),
//     //     ))
//     //     .with_children(|builder| {
//     //         builder.spawn((
//     //             PointLight {
//     //                 color: Color::WHITE,
//     //                 intensity: 25000.0,
//     //                 ..Default::default()
//     //             },
//     //             Transform::from_translation((box_thickness + 0.05) * Vec3::Y),
//     //         ));
//     //     });
//     // // directional light
//     // commands.spawn((
//     //     DirectionalLight {
//     //         illuminance: 2000.0,
//     //         ..default()
//     //     },
//     //     Transform::from_rotation(Quat::from_rotation_x(
//     //         -std::f32::consts::PI / 2.0,
//     //     )),
//     // ));

//     // // camera
//     // commands.spawn((
//     //     Camera3d::default(),
//     //     Transform::from_xyz(0.0, box_offset, 4.0)
//     //         .looking_at(Vec3::new(0.0, box_offset, 0.0), Vec3::Y),
//     //     MainCamera,
//     //     // PickRaycastSource,
//     // ));
// }
