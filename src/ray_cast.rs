#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{
    cell::RefCell,
    ffi::c_int,
    rc::Rc,
    sync::{Arc, Weak},
};

use autocxx::subclass::{CppSubclass, CppSubclassCppPeerHolder};

use crate::{
    box2d::ffi,
    ffi::{b2Fixture, b2ParticleSystem, b2Vec2, int32},
};

#[allow(unused_variables)]
pub trait b2RayCastCallbackImpl {
    fn report_fixture(
        &mut self,
        fixture: &mut b2Fixture,
        point: &b2Vec2,
        normal: &b2Vec2,
        fraction: f32,
    ) -> f32;

    fn report_particle(
        &mut self,
        particle_system: &b2ParticleSystem,
        index: i32,
        point: &b2Vec2,
        normal: &b2Vec2,
        fraction: f32,
    ) -> f32;

    fn should_query_particle_system(&mut self, particle_system: *const b2ParticleSystem) -> bool;
}

pub struct b2RayCastCallbackWrapper {
    cpp_peer: CppSubclassCppPeerHolder<ffi::b2RayCastCallbackWrapperCpp>,
    wrapper_impl: Weak<RefCell<dyn b2RayCastCallbackImpl>>,
}

impl b2RayCastCallbackWrapper {
    pub fn new(wrapper_impl: Arc<RefCell<dyn b2RayCastCallbackImpl>>) -> Rc<RefCell<Self>> {
        Self::new_rust_owned(Self {
            cpp_peer: Default::default(),
            wrapper_impl: Arc::downgrade(&wrapper_impl),
        })
    }
}

impl ffi::b2RayCastCallback_methods for b2RayCastCallbackWrapper {
    unsafe fn ReportFixture(
        &mut self,
        fixture: *mut b2Fixture,
        point: &b2Vec2,
        normal: &b2Vec2,
        fraction: f32,
    ) -> f32 {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            return wrapper_impl.report_fixture(fixture.as_mut().unwrap(), point, normal, fraction);
        } else {
            return 0.;
        }
    }

    unsafe fn ReportParticle(
        &mut self,
        particle_system: *const b2ParticleSystem,
        index: autocxx::c_int,
        point: &b2Vec2,
        normal: &b2Vec2,
        fraction: f32,
    ) -> f32 {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            return wrapper_impl.report_particle(
                particle_system.as_ref().unwrap(),
                i32::from(int32::from(c_int::from(index))),
                point,
                normal,
                fraction,
            );
        } else {
            return 0.;
        }
    }

    unsafe fn ShouldQueryParticleSystem(
        &mut self,
        particle_system: *const b2ParticleSystem,
    ) -> bool {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            return wrapper_impl.should_query_particle_system(particle_system.as_ref().unwrap());
        } else {
            return true;
        }
    }
}

impl CppSubclass<ffi::b2RayCastCallbackWrapperCpp> for b2RayCastCallbackWrapper {
    fn peer_holder(&self) -> &CppSubclassCppPeerHolder<ffi::b2RayCastCallbackWrapperCpp> {
        &self.cpp_peer
    }

    fn peer_holder_mut(
        &mut self,
    ) -> &mut CppSubclassCppPeerHolder<ffi::b2RayCastCallbackWrapperCpp> {
        &mut self.cpp_peer
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, pin::Pin, sync::Arc};

    use autocxx::prelude::*;

    use crate::{
        ffi::{
            b2BodyDef,
            b2BodyType::b2_staticBody,
            b2CircleShape,
            b2Fixture,
            b2ParticleSystem,
            b2RayCastCallback,
            b2Shape,
            b2Vec2,
            b2World,
            SetCircleRadius,
        },
        ray_cast::{b2RayCastCallbackImpl, b2RayCastCallbackWrapper},
    };

    #[test]
    fn raycast_hits_correct_fixture() {
        unsafe {
            let gravity = b2Vec2::new1(0., -10.).within_box();
            let mut world = b2World::new(&*gravity).within_box();

            let mut body_def = b2BodyDef::new().within_box();
            body_def.type_ = b2_staticBody;
            body_def.position = b2Vec2 { x: 0., y: 0. };
            let body_def = &*body_def;
            let body = world.as_mut().CreateBody(&*body_def);
            let mut body = Pin::new_unchecked(body.as_mut().unwrap());

            let mut shape = b2CircleShape::new().within_box();
            SetCircleRadius(shape.as_mut(), 1.);
            let shape: &b2Shape = (&*shape).as_ref();
            let fixture = body.as_mut().CreateFixture1(&*shape, 5.);

            let callback = CallbackImpl::new(fixture.as_ref().unwrap());
            let callback_ref = Arc::new(RefCell::new(callback));
            let b2_callback = b2RayCastCallbackWrapper::new(callback_ref.clone());
            let b2_callback: *mut b2RayCastCallback = b2_callback
                .as_ref()
                .borrow_mut()
                .pin_mut()
                .as_mut()
                .get_unchecked_mut();
            world.as_ref().RayCast(
                b2_callback,
                &b2Vec2 { x: -2., y: 0. },
                &b2Vec2 { x: 2., y: 0. },
            );

            assert!(callback_ref.borrow().did_encounter_fixture);
        }

        struct CallbackImpl<'a> {
            expected_fixture: &'a b2Fixture,
            did_encounter_fixture: bool,
        }

        impl<'a> CallbackImpl<'a> {
            fn new(expected_fixture: &'a b2Fixture) -> Self {
                Self {
                    expected_fixture,
                    did_encounter_fixture: false,
                }
            }
        }

        #[allow(unused_variables)]
        impl b2RayCastCallbackImpl for CallbackImpl<'_> {
            fn report_fixture(
                &mut self,
                fixture: &mut b2Fixture,
                point: &b2Vec2,
                normal: &b2Vec2,
                fraction: f32,
            ) -> f32 {
                assert!(std::ptr::eq(fixture, self.expected_fixture));
                self.did_encounter_fixture = true;
                return 0.;
            }

            fn report_particle(
                &mut self,
                particle_system: &b2ParticleSystem,
                index: i32,
                point: &b2Vec2,
                normal: &b2Vec2,
                fraction: f32,
            ) -> f32 {
                assert!(false, "Should not get here");
                return 0.;
            }

            fn should_query_particle_system(
                &mut self,
                particle_system: *const b2ParticleSystem,
            ) -> bool {
                assert!(false, "Should not get here");
                return false;
            }
        }
    }
}
