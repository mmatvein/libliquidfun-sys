#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// pub type int16 = ::std::os::raw::c_short;
// pub type int32 = ::std::os::raw::c_int;
// pub type uint8 = ::std::os::raw::c_uchar;
// pub type uint16 = ::std::os::raw::c_ushort;
// pub type uint32 = ::std::os::raw::c_uint;
// pub type int64 = ::std::os::raw::c_longlong;

use autocxx::prelude::*;
use std::fmt::{Debug, Formatter};

include_cpp! {
    #include "box2d/box2d.h"
    safety!(unsafe_ffi)

    generate!("b2Body")
    generate!("b2Contact")
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
    generate_pod!("b2JointDef")
    generate_pod!("b2DistanceJointDef")
    generate_pod!("b2FrictionJointDef")
    generate_pod!("b2GearJointDef")
    generate_pod!("b2MotorJointDef")
    generate_pod!("b2PrismaticJointDef")
    generate_pod!("b2PulleyJointDef")
    generate_pod!("b2RevoluteJointDef")
    generate_pod!("b2WeldJointDef")
    generate_pod!("b2WheelJointDef")

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
}

pub mod box2d {
    pub mod ffi {
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
        f.debug_struct("b2Vec2").field("x", &self.x).field("y", &self.y).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_world() {
        moveit! { let gravity = ffi::b2Vec2::new1(0., -9.81); }
        let world = ffi::b2World::new(&gravity).within_box();

        let world_gravity = world.GetGravity();
        let expected_gravity = gravity.as_ref().get_ref();
        assert_eq!(*expected_gravity, world_gravity);
    }
}
