#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::fmt::{Debug, Formatter};

use autocxx::prelude::*;
use contact_listener::b2ContactListenerWrapper;

include_cpp! {
    #include "box2d/box2d.h"
    #include "extras.hpp"
    safety!(unsafe_ffi)

    generate!("b2Body")
    generate!("b2Contact")
    generate!("b2ContactListener")
    generate!("b2ContactFilter")
    generate!("b2DestructionListener")
    generate!("b2Filter")
    generate!("b2Fixture")
    generate!("b2Manifold")
    generate!("b2QueryCallback")
    generate!("b2World")
    generate!("b2WorldManifold")

    generate!("b2Shape")
    generate!("b2ChainShape")
    generate!("b2CircleShape")
    generate!("b2EdgeShape")
    generate!("b2PolygonShape")

    generate!("b2Joint")
    generate!("b2DistanceJoint")
    generate!("b2FrictionJoint")
    generate!("b2GearJoint")
    generate!("b2MotorJoint")
    generate!("b2PrismaticJoint")
    generate!("b2PulleyJoint")
    generate!("b2RevoluteJoint")
    generate!("b2WeldJoint")
    generate!("b2WheelJoint")
    generate!("b2JointUserData")
    generate!("b2JointDef")
    generate!("b2DistanceJointDef")
    generate!("b2FrictionJointDef")
    generate!("b2GearJointDef")
    generate!("b2MotorJointDef")
    generate!("b2PrismaticJointDef")
    generate!("b2PulleyJointDef")
    generate!("b2RevoluteJointDef")
    generate!("b2WeldJointDef")
    generate!("b2WheelJointDef")

    generate!("b2ParticleContact")
    generate!("b2ParticleGroup")
    generate!("b2ParticleHandle")
    generate!("b2ParticleSystem")
    generate_pod!("b2ParticleBodyContact")
    generate_pod!("b2ParticleDef")
    generate_pod!("b2ParticleFlag")
    generate_pod!("b2ParticleSystemDef")

    generate_pod!("b2AABB")
    generate_pod!("b2BodyDef")
    generate_pod!("b2FixtureDef")
    generate_pod!("b2MassData")
    generate_pod!("b2Transform")
    generate_pod!("b2Vec2")
    generate_pod!("b2Rot")

    // extras.hpp
    generate!("SetCircleRadius")
    generate!("SetCirclePosition")
    generate!("CreateParticleGroupDef")

    generate!("CreateRevoluteJoint")
    generate!("CreatePrismaticJoint")
    generate!("CreateDistanceJoint")
    generate!("CreatePulleyJoint")
    generate!("CreateMouseJoint")
    generate!("CreateGearJoint")
    generate!("CreateWheelJoint")
    generate!("CreateWeldJoint")
    generate!("CreateFrictionJoint")
    generate!("CreateMotorJoint")

    subclass!("b2ContactListener", b2ContactListenerWrapper)
}

mod contact_listener;
pub mod box2d {
    pub mod ffi {
        pub use crate::contact_listener::*;
        pub use crate::ffi::*;
    }
}

impl PartialEq for ffi::b2Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Debug for ffi::b2Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("b2Vec2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::contact_listener::b2ContactListenerWrapper;
    use std::pin::Pin;

    use crate::ffi::b2BodyType::{b2_dynamicBody, b2_staticBody};
    use crate::ffi::SetCircleRadius;
    use crate::ffi::{b2BodyDef, b2CircleShape, b2Shape, b2Vec2};

    use super::*;

    #[test]
    fn create_world() {
        moveit! { let gravity = ffi::b2Vec2::new1(0., -9.81); }
        let world = ffi::b2World::new(&gravity).within_box();

        let world_gravity = world.GetGravity();
        let expected_gravity = gravity.as_ref().get_ref();
        assert_eq!(*expected_gravity, world_gravity);
    }

    #[test]
    fn gravity_affects_position() {
        unsafe {
            let gravity = ffi::b2Vec2::new1(0., -10.).within_box();
            let mut world = ffi::b2World::new(&*gravity).within_box();
            let mut body_def = b2BodyDef::new().within_box();
            body_def.type_ = b2_dynamicBody;
            let body_def = &*body_def;
            let body = world.as_mut().CreateBody(&*body_def);
            let mut body = Pin::new_unchecked(body.as_mut().unwrap());

            let mut shape = b2CircleShape::new().within_box();
            SetCircleRadius(shape.as_mut(), 5.);
            let shape: &b2Shape = (&*shape).as_ref();
            body.as_mut().CreateFixture1(&*shape, 5.);

            for _ in 0..10 {
                world
                    .as_mut()
                    .Step(0.02, c_int::from(8), c_int::from(3), c_int::from(100));
            }

            assert!(
                body.as_ref().GetPosition().y < 0.,
                "Body needs to move downwards due to gravity"
            );
        }
    }
}

impl Debug for ffi::b2Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("b2Body")
            .field("position", &self.GetPosition())
            .field("angle", &self.GetAngle())
            .field("type", &(self.GetType() as u32))
            .field("mass", &self.GetMass())
            .finish()
    }
}
