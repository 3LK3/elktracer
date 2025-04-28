mod model;
mod ui;

use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{
    EguiContext, EguiContextPass, EguiContextSettings, EguiPlugin, egui,
};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin, bevy_inspector::ui_for_world,
};
use egui_dock::{DockArea, DockState, NodeIndex, Style};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_framepace::FramepacePlugin) // reduces input lag
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(DefaultInspectorConfigPlugin)
        // .add_plugins(bevy_mod_picking::plugins::DefaultPickingPlugins)
        .insert_resource(UiState::new())
        .add_systems(Startup, setup)
        .add_systems(EguiContextPass, show_ui_system)
        .add_systems(PostUpdate, set_camera_viewport.after(show_ui_system))
        // .add_systems(Update, auto_add_raycast_target)
        // .add_systems(Update, handle_pick_events)
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .run();
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

#[derive(Debug)]
enum GuiWindow {
    SceneView,
    SceneObjects,
    Camera,
    Rendering,
    Debug,
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct UiState {
    state: DockState<GuiWindow>,
    viewport_rect: egui::Rect,
}

impl UiState {
    fn new() -> Self {
        let mut state = DockState::new(vec![GuiWindow::SceneView]);
        let tree = state.main_surface_mut();

        let [scene_view, _rendering] = tree.split_right(
            NodeIndex::root(),
            0.75,
            vec![GuiWindow::Rendering, GuiWindow::Debug],
        );
        let [_scene_view, scene_objects] =
            tree.split_left(scene_view, 0.3, vec![GuiWindow::SceneObjects]);
        tree.split_below(scene_objects, 0.7, vec![GuiWindow::Camera]);
        // tree.split_below(scene_view, 0.7, vec![GuiWindow::Debug]);

        Self {
            state,
            viewport_rect: egui::Rect::NOTHING,
        }
    }

    fn ui(&mut self, world: &mut World, ctx: &mut egui::Context) {
        let style = Style::from_egui(ctx.style().as_ref());

        let mut tab_viewer = EditorApplication {
            world,
            viewport_rect: &mut self.viewport_rect,
            camera: elktracer_core::Camera::default(),
        };
        DockArea::new(&mut self.state)
            .style(style)
            .show(ctx, &mut tab_viewer);
    }
}

struct EditorApplication<'a> {
    world: &'a mut World,
    viewport_rect: &'a mut egui::Rect,
    camera: elktracer_core::Camera,
}

impl egui_dock::TabViewer for EditorApplication<'_> {
    type Tab = GuiWindow;

    fn ui(&mut self, ui: &mut egui_dock::egui::Ui, window: &mut Self::Tab) {
        // let type_registry = self.world.resource::<AppTypeRegistry>().0.clone();
        // let type_registry = type_registry.read();

        match window {
            GuiWindow::SceneView => {
                *self.viewport_rect = ui.clip_rect();

                // draw_gizmo(
                //     ui,
                //     &mut self.gizmo,
                //     self.world,
                //     self.selected_entities,
                // );
            }
            GuiWindow::SceneObjects => {
                ui.label("Scene Objects");
                // let selected =
                //     hierarchy_ui(self.world, ui, self.selected_entities);
                // if selected {
                //     *self.selection = InspectorSelection::Entities;
                // }
            }
            GuiWindow::Camera => ui_for_camera_options(ui, self.camera),
            GuiWindow::Rendering => {
                ui.label("Rendering");
                // select_asset(ui, &type_registry, self.world, self.selection)
            }
            GuiWindow::Debug => ui_for_world(self.world, ui),
        }
    }

    fn title(&mut self, window: &mut Self::Tab) -> egui_dock::egui::WidgetText {
        format!("{window:?}").into()
    }

    fn clear_background(&self, window: &Self::Tab) -> bool {
        !matches!(window, GuiWindow::SceneView)
    }
}

fn ui_for_camera_options(ui: &mut egui::Ui, camera: elktracer_core::Camera) {
    ui.label("Camera Options");
    ui.horizontal(|ui| {
        ui.label("Position");
        ui.label(format!("{:?}", camera.position));
    });
    ui.horizontal(|ui| {
        ui.label("Look At");
        ui.label(format!("{:?}", camera.look_at));
    });
    ui.horizontal(|ui| {
        ui.label("Up");
        ui.label(format!("{:?}", camera.up));
    });
    ui.horizontal(|ui| {
        ui.label("FOV");
        ui.label(format!("{:?}", camera.fov_vertical_degrees));
    });
    //    ui.label("Camera");
    //     ui.with_layout(
    //         egui::Layout::top_down_justified(egui::Align::Center),
    //         |ui| {
    //             ui.label("world!");
    //             ui.text_edit_singleline(&mut )
    //             ui.label("Hello");
    //         }
    // );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let box_size = 2.0;
    // let box_thickness = 0.15;
    // let box_offset = (box_size + box_thickness) / 2.0;

    // // left - red
    // let mut transform = Transform::from_xyz(-box_offset, box_offset, 0.0);
    // transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.63, 0.065, 0.05),
    //         ..Default::default()
    //     })),
    //     transform,
    // ));
    // // right - green
    // let mut transform = Transform::from_xyz(box_offset, box_offset, 0.0);
    // transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
    //     transform,
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.14, 0.45, 0.091),
    //         ..Default::default()
    //     })),
    // ));
    // // bottom - white
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(
    //         box_size + 2.0 * box_thickness,
    //         box_thickness,
    //         box_size,
    //     ))),
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.725, 0.71, 0.68),
    //         ..Default::default()
    //     })),
    // ));
    // // top - white
    // let transform = Transform::from_xyz(0.0, 2.0 * box_offset, 0.0);
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(
    //         box_size + 2.0 * box_thickness,
    //         box_thickness,
    //         box_size,
    //     ))),
    //     transform,
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.725, 0.71, 0.68),
    //         ..Default::default()
    //     })),
    // ));
    // // back - white
    // let mut transform = Transform::from_xyz(0.0, box_offset, -box_offset);
    // transform.rotate(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(
    //         box_size + 2.0 * box_thickness,
    //         box_thickness,
    //         box_size + 2.0 * box_thickness,
    //     ))),
    //     transform,
    //     MeshMaterial3d(materials.add(StandardMaterial {
    //         base_color: Color::srgb(0.725, 0.71, 0.68),
    //         ..Default::default()
    //     })),
    // ));

    // // ambient light
    // commands.insert_resource(AmbientLight {
    //     color: Color::WHITE,
    //     brightness: 0.02,
    //     ..default()
    // });
    // // top light
    // commands
    //     .spawn((
    //         Mesh3d(meshes.add(Plane3d::default().mesh().size(0.4, 0.4))),
    //         Transform::from_matrix(Mat4::from_scale_rotation_translation(
    //             Vec3::ONE,
    //             Quat::from_rotation_x(std::f32::consts::PI),
    //             Vec3::new(0.0, box_size + 0.5 * box_thickness, 0.0),
    //         )),
    //         MeshMaterial3d(materials.add(StandardMaterial {
    //             base_color: Color::WHITE,
    //             emissive: LinearRgba::WHITE * 100.0,
    //             ..Default::default()
    //         })),
    //     ))
    //     .with_children(|builder| {
    //         builder.spawn((
    //             PointLight {
    //                 color: Color::WHITE,
    //                 intensity: 25000.0,
    //                 ..Default::default()
    //             },
    //             Transform::from_translation((box_thickness + 0.05) * Vec3::Y),
    //         ));
    //     });
    // // directional light
    // commands.spawn((
    //     DirectionalLight {
    //         illuminance: 2000.0,
    //         ..default()
    //     },
    //     Transform::from_rotation(Quat::from_rotation_x(
    //         -std::f32::consts::PI / 2.0,
    //     )),
    // ));

    // // camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(0.0, box_offset, 4.0)
    //         .looking_at(Vec3::new(0.0, box_offset, 0.0), Vec3::Y),
    //     MainCamera,
    //     // PickRaycastSource,
    // ));
}
