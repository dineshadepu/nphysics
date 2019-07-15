extern crate nalgebra as na;

use na::{Point2, Vector2};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, DefaultBodySet, DefaultColliderSet, Ground, BodyPartHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultDynamicWorld, DefaultColliderWorld};
use nphysics_testbed2d::Testbed;



pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let dynamic_world = DefaultDynamicWorld::new(Vector2::new(0.0, -9.81));
    let collider_world = DefaultColliderWorld::new();
    let mut bodies = DefaultBodySet::new();
    let mut colliders = DefaultColliderSet::new();
    let joint_constraints = DefaultJointConstraintSet::new();
    let force_generators = DefaultForceGeneratorSet::new();

    /*
     * Setup a static body used as the ground.
     */
    let ground_handle = bodies.insert(Ground::new());

    /*
     * Ground
     */
    let ground_size = 5.0;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 0.2)));
    let conveyor_material1 = BasicMaterial { surface_velocity: Some(Vector2::x()), ..BasicMaterial::default() };
    let conveyor_material2 = BasicMaterial { surface_velocity: Some(-Vector2::x()), ..BasicMaterial::default() };

    for i in 0..10 {
        let co = ColliderDesc::new(ground_shape.clone())
            .translation(Vector2::new(-2.0, 5.0 - i as f32 * 4.0))
            .rotation(0.1)
            .material(MaterialHandle::new(conveyor_material1))
            .build(BodyPartHandle(ground_handle, 0));
        colliders.insert(co);


        let co = ColliderDesc::new(ground_shape.clone())
            .translation(Vector2::new(2.0, 3.0 - i as f32 * 4.0))
            .rotation(-0.1)
            .material(MaterialHandle::new(conveyor_material2))
            .build(BodyPartHandle(ground_handle, 0));
        colliders.insert(co);
    }

    /*
     * Create the boxes
     */
    let num = 5;
    let rad = 0.1;

    let cuboid = ShapeHandle::new(Cuboid::new(Vector2::repeat(rad)));

    let shift = (rad + ColliderDesc::<f32>::default_margin()) * 2.0;
    let centerx = shift * (num as f32) / 2.0 + 5.0;
    let centery = shift / 2.0 + 5.0;

    for i in 0usize..num {
        for j in 0..num {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery;

            // Build the rigid body.
            let rb = RigidBodyDesc::new()
                .translation(Vector2::new(x, y))
                .build();
            let rb_handle = bodies.insert(rb);

            // Build the collider.
            let co = ColliderDesc::new(cuboid.clone())
                .density(1.0)
                .build(BodyPartHandle(rb_handle, 0));
            colliders.insert(co);
        }
    }

    /*
     * Set up the testbed.
     */
    testbed.set_ground_handle(Some(ground_handle));
    testbed.set_world(dynamic_world, collider_world, bodies, colliders, joint_constraints, force_generators);
    testbed.look_at(Point2::new(0.0, -2.5), 95.0);
}


fn main() {
    let testbed = Testbed::from_builders(0, vec![
        ("Conveyor belt", init_world),
    ]);
    testbed.run()
}