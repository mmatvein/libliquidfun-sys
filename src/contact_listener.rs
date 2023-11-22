#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::box2d::ffi;
use crate::ffi::{
    b2Contact, b2ContactImpulse, b2Fixture, b2Manifold, b2ParticleBodyContact, b2ParticleContact,
    b2ParticleSystem, int32,
};
use autocxx::subclass::{CppSubclass, CppSubclassCppPeerHolder};
use std::cell::RefCell;
use std::ffi::c_int;
use std::rc::Rc;
use std::sync::{Arc, Weak};

#[allow(unused_variables)]
pub trait b2ContactListenerImpl {
    fn begin_contact(&mut self, contact: &mut b2Contact) {}
    fn end_contact(&mut self, contact: &mut b2Contact) {}
    fn begin_particle_body_contact(
        &mut self,
        particle_system: &mut b2ParticleSystem,
        contact: &mut b2ParticleBodyContact,
    ) {
    }
    fn end_particle_body_contact(
        &mut self,
        fixture: &mut b2Fixture,
        particleSystem: &mut b2ParticleSystem,
        particle_index: i32,
    ) {
    }
    fn begin_particle_particle_contact(
        &mut self,
        particle_system: &mut b2ParticleSystem,
        contact: &mut b2ParticleContact,
    ) {
    }
    fn end_particle_particle_contact(
        &mut self,
        particle_system: &mut b2ParticleSystem,
        index_a: i32,
        index_b: i32,
    ) {
    }
    fn pre_solve(&mut self, contact: &mut b2Contact, old_manifold: &b2Manifold) {}
    fn post_solve(&mut self, contact: &mut b2Contact, impulse: &b2ContactImpulse) {}
}
pub struct b2ContactListenerWrapper {
    cpp_peer: CppSubclassCppPeerHolder<ffi::b2ContactListenerWrapperCpp>,
    wrapper_impl: Weak<RefCell<dyn b2ContactListenerImpl>>,
}

impl b2ContactListenerWrapper {
    pub fn new(wrapper_impl: Arc<RefCell<dyn b2ContactListenerImpl>>) -> Rc<RefCell<Self>> {
        Self::new_rust_owned(Self {
            cpp_peer: Default::default(),
            wrapper_impl: Arc::downgrade(&wrapper_impl),
        })
    }
}

impl ffi::b2ContactListener_methods for b2ContactListenerWrapper {
    unsafe fn BeginContact(&mut self, contact: *mut b2Contact) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.begin_contact(contact.as_mut().unwrap());
        }
    }

    unsafe fn EndContact(&mut self, contact: *mut b2Contact) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.end_contact(contact.as_mut().unwrap());
        }
    }

    unsafe fn BeginContact1(
        &mut self,
        particleSystem: *mut b2ParticleSystem,
        particleBodyContact: *mut b2ParticleBodyContact,
    ) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.begin_particle_body_contact(
                particleSystem.as_mut().unwrap(),
                particleBodyContact.as_mut().unwrap(),
            );
        }
    }
    unsafe fn EndContact1(
        &mut self,
        fixture: *mut b2Fixture,
        particle_system: *mut b2ParticleSystem,
        index: autocxx::c_int,
    ) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.end_particle_body_contact(
                fixture.as_mut().unwrap(),
                particle_system.as_mut().unwrap(),
                int32::from(c_int::from(index)).into(),
            );
        }
    }
    unsafe fn BeginContact2(
        &mut self,
        particle_system: *mut b2ParticleSystem,
        particle_contact: *mut b2ParticleContact,
    ) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.begin_particle_particle_contact(
                particle_system.as_mut().unwrap(),
                particle_contact.as_mut().unwrap(),
            );
        }
    }
    unsafe fn EndContact2(
        &mut self,
        particle_system: *mut b2ParticleSystem,
        index_a: autocxx::c_int,
        index_b: autocxx::c_int,
    ) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.end_particle_particle_contact(
                particle_system.as_mut().unwrap(),
                int32::from(c_int::from(index_a)).into(),
                int32::from(c_int::from(index_b)).into(),
            );
        }
    }
    unsafe fn PreSolve(&mut self, contact: *mut b2Contact, old_manifold: *const b2Manifold) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.pre_solve(contact.as_mut().unwrap(), old_manifold.as_ref().unwrap());
        }
    }
    unsafe fn PostSolve(&mut self, contact: *mut b2Contact, impulse: *const b2ContactImpulse) {
        if let Some(wrapper_impl_rc) = self.wrapper_impl.upgrade() {
            let wrapper_impl = wrapper_impl_rc.as_ptr().as_mut().unwrap();
            wrapper_impl.post_solve(contact.as_mut().unwrap(), impulse.as_ref().unwrap());
        }
    }
}

impl CppSubclass<ffi::b2ContactListenerWrapperCpp> for b2ContactListenerWrapper {
    fn peer_holder(&self) -> &CppSubclassCppPeerHolder<ffi::b2ContactListenerWrapperCpp> {
        &self.cpp_peer
    }

    fn peer_holder_mut(
        &mut self,
    ) -> &mut CppSubclassCppPeerHolder<ffi::b2ContactListenerWrapperCpp> {
        &mut self.cpp_peer
    }
}

#[cfg(test)]
mod tests {
    use crate::contact_listener::{b2ContactListenerImpl, b2ContactListenerWrapper};
    use crate::ffi::b2BodyType::{b2_dynamicBody, b2_staticBody};
    use crate::ffi::{
        b2BodyDef, b2CircleShape, b2Contact, b2ContactListener, b2Shape, b2Vec2, b2World,
        SetCircleRadius,
    };
    use autocxx::prelude::*;
    use std::cell::RefCell;
    use std::pin::Pin;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn contact_listener_triggered() {
        unsafe {
            let gravity = b2Vec2::new1(0., -10.).within_box();
            let mut world = b2World::new(&*gravity).within_box();

            {
                let mut body_def = b2BodyDef::new().within_box();
                body_def.type_ = b2_dynamicBody;
                body_def.position = b2Vec2 { x: 0., y: 2.001 };
                let body_def = &*body_def;
                let body = world.as_mut().CreateBody(&*body_def);
                let mut body = Pin::new_unchecked(body.as_mut().unwrap());

                let mut shape = b2CircleShape::new().within_box();
                SetCircleRadius(shape.as_mut(), 1.);
                let shape: &b2Shape = (&*shape).as_ref();
                body.as_mut().CreateFixture1(&*shape, 5.);
            }
            {
                let mut body_def = b2BodyDef::new().within_box();
                body_def.type_ = b2_staticBody;
                body_def.position = b2Vec2 { x: 0., y: 0. };
                let body_def = &*body_def;
                let body = world.as_mut().CreateBody(&*body_def);
                let mut body = Pin::new_unchecked(body.as_mut().unwrap());

                let mut shape = b2CircleShape::new().within_box();
                SetCircleRadius(shape.as_mut(), 1.);
                let shape: &b2Shape = (&*shape).as_ref();
                body.as_mut().CreateFixture1(&*shape, 5.);
            }

            let begin_contact_called = Rc::new(RefCell::new(false));
            let listener = ListenerImpl {
                begin_contact_called: false,
            };
            let listener_ref = &listener;
            let baa = Arc::new(RefCell::new(listener));
            let contact_listener = b2ContactListenerWrapper::new(baa.clone());
            {
                let contact_listener: *mut b2ContactListener = contact_listener
                    .as_ref()
                    .borrow_mut()
                    .pin_mut()
                    .as_mut()
                    .get_unchecked_mut();
                world.as_mut().SetContactListener(contact_listener);
            }

            for _ in 0..10 {
                world
                    .as_mut()
                    .Step(0.02, c_int::from(8), c_int::from(3), c_int::from(100));
            }

            let listener_impl = baa.borrow();
            assert!(listener_impl.begin_contact_called)
        }

        struct ListenerImpl {
            pub begin_contact_called: bool,
        }

        impl b2ContactListenerImpl for ListenerImpl {
            fn begin_contact(&mut self, _contact: &mut b2Contact) {
                self.begin_contact_called = true;
            }
        }
    }
}
