
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;
use bevy_prototype_lyon::prelude::*;
use mouse_map_projection::*;
use if_chain::if_chain;

mod mouse_map_projection;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(ShapePlugin)
        .add_plugin(MouseWorldProjectionPlugin)
        .add_startup_system(setup_world)
        .add_system(player_movement)
        .add_system(contact_event_print)
        .add_system(mouse_ball_creator)
        .run();
}

enum WallSide {
    Left,
    Right,
}

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Player {
    standing: bool,
    on_wall: Option<WallSide>
}

impl Player {
    pub fn new() -> Self {
        Self {
            on_wall: None,
            standing: false
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_world(
    mut cmd: Commands,
    mut configuration: ResMut<RapierConfiguration>,

){ 
    let scale = 30.;
    configuration.scale = scale;
    cmd.spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let wall = |cmd: &mut Commands,color: Color, x: f32,y: f32, w: f32,h: f32| {
        let shape = shapes::Rectangle{
            extents: Vec2::new(w*scale,h*scale),
            origin: Default::default()
        };

        cmd.spawn_bundle(ColliderBundle{
            position: [x,y].into(),
            shape: ColliderShape::cuboid(w/2.,h/2.).into(),
            material: ColliderMaterial {
                friction: 2.,
                restitution: 0.01,
                ..Default::default()
            }.into(),
            ..Default::default()
        })
        .insert_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined{
                fill_mode: FillMode::color(color),
                outline_mode: StrokeMode::new(Color::BLACK,3.),
            },
            Transform::default(),
        ))
        .insert(ColliderPositionSync::Discrete)
        .insert(Platform);

    };
    wall(&mut cmd,Color::BLACK,0.,-10.,21.,1.);
    wall(&mut cmd,Color::BLACK,0.,10.,21.,1.);
    wall(&mut cmd,Color::BLACK,10.,0.,1.,21.);
    wall(&mut cmd,Color::BLACK,5.,0.,1.,15.);
    wall(&mut cmd,Color::BLACK,-10.,0.,1.,21.);
    wall(&mut cmd,Color::BLACK,-5.,0.,1.,15.);

    create_ball(&mut cmd,scale,[0.,3.].into());
    //create player
    cmd.spawn_bundle(RigidBodyBundle{
        position: Vec2::new(0.,0.).into(),
        forces: RigidBodyForces {
            gravity_scale: 1.,
            ..Default::default()
        }.into(),
        mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle{
        shape: ColliderShape::cuboid(0.5,0.5,).into(),
        flags: (ActiveEvents::CONTACT_EVENTS).into(),
        ..Default::default()
    })
    .insert_bundle(SpriteBundle{
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(30.,30.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(RigidBodyPositionSync::Discrete)
    .insert(Player::new());
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocityComponent, &RigidBodyMassPropsComponent)>
) {
    for (player_state,mut rb_vels,rb_mprops) in player_info.iter_mut() {
        let jump = keyboard_input.just_pressed(KeyCode::Space);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        
        let mut move_delta = Vector2::new(x_axis as f32, 0.);

        if move_delta != Vector::zeros() {
            let speed = if player_state.standing {20.} else {1.};
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
            rb_vels.apply_impulse(rb_mprops,move_delta*speed);
        }

        if jump {
            if player_state.standing{
                rb_vels.linvel.y +=10.;
            } else {
                if let Some(side) = &player_state.on_wall {
                    match side {
                        WallSide::Left => {
                            rb_vels.linvel.y +=10.;
                            rb_vels.linvel.x +=5.;
                        }
                        WallSide::Right => {
                            rb_vels.linvel.y +=10.;
                            rb_vels.linvel.x -=5.;
                        }
                    }
                }
            }
        }
    }
}

fn contact_event_print(
    mut player_query: Query<(Entity,&mut Player)>,
    mut wall_query: Query<(&Platform,)>,
    narrow_phase: Res<NarrowPhase>,
) {
    let (player_ent,mut player_state) = player_query.single_mut();
    let mut standing = false;
    let mut on_wall = None;

    for contact_pair in narrow_phase.contacts_with(player_ent.handle()) {
        let other_collider = if contact_pair.collider1 == player_ent.handle() {
            contact_pair.collider2
        } else {
            contact_pair.collider1
        };

        if let Ok(platform) = wall_query.get(other_collider.entity()) {
            for manifold in &contact_pair.manifolds {

                if manifold.data.normal.y == 1. {
                    standing = true;
                }
                if manifold.data.normal.x == 1. {
                    on_wall = Some(WallSide::Left);
                }
                if manifold.data.normal.x == -1. {
                    on_wall = Some(WallSide::Right);
                }
            }
        }
    }

    player_state.standing = standing;
    player_state.on_wall = on_wall;
}

fn create_ball(
    cmd: &mut Commands,
    scale: f32,
    pos: Vector2<f32>,
) {
    //create ball
    let ball_size = 1.;
    cmd.spawn_bundle(RigidBodyBundle{
        position: Vec2::new(pos.x,pos.y).into(),
        forces: RigidBodyForces {
            gravity_scale: 1.,
            ..Default::default()
        }.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle{
        shape: ColliderShape::ball(ball_size).into(),
        mass_properties: ColliderMassProps::Density(0.001).into(),
        material: ColliderMaterial {
            restitution:0.9,
            ..Default::default()
        }.into(),
        ..Default::default()
    })
    .insert_bundle(GeometryBuilder::build_as(
        &shapes::Circle {
            radius: ball_size*scale,
            center: Vec2::ZERO
        },
        DrawMode::Outlined{
            fill_mode: FillMode::color(Color::RED),
            outline_mode: StrokeMode::new(Color::BLACK,1.),
        },
        Transform::default(),
    ))
    .insert(RigidBodyPositionSync::Discrete);
}

fn mouse_ball_creator(
    mut cmd: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mouse_pos: Res<MouseWorldPosition>,
    rapier_parameters: Res<RapierConfiguration>,
) {
    if_chain!{
        if let Some(mouse_pos) = mouse_pos.0;
        if mouse_input.just_pressed(MouseButton::Left);
        then {
            let scale = rapier_parameters.scale;
            let rapier_pos = mouse_pos /scale;
            create_ball(&mut cmd,scale,rapier_pos.into());
        }
    }
}
