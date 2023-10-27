#ifndef EXTRAS_HPP
#define EXTRAS_HPP

#include <box2d/box2d.h>

extern "C" {
    class b2CircleShape;
    class b2World;
    struct b2Vec2;
    struct b2ParticleGroupDef;

    class b2RevoluteJoint;
    class b2PrismaticJoint;
    class b2DistanceJoint;
    class b2PulleyJoint;
    class b2MouseJoint;
    class b2GearJoint;
    class b2WheelJoint;
    class b2WeldJoint;
    class b2FrictionJoint;
    class b2RopeJoint;
    class b2MotorJoint;
    struct b2RevoluteJointDef;
    struct b2PrismaticJointDef;
    struct b2DistanceJointDef;
    struct b2PulleyJointDef;
    struct b2MouseJointDef;
    struct b2GearJointDef;
    struct b2WheelJointDef;
    struct b2WeldJointDef;
    struct b2FrictionJointDef;
    struct b2RopeJointDef;
    struct b2MotorJointDef;
    enum b2JointType;

    void SetCircleRadius(b2CircleShape& self, float radius) {
        self.m_radius = radius;
    }

    void SetCirclePosition(b2CircleShape& self, const b2Vec2& position){
        self.m_p = position;
    }

    b2ParticleGroupDef* CreateParticleGroupDef(
        uint32 flags,
        uint32 groupFlags,
        b2Vec2 position,
        float angle,
        b2Vec2 linearVelocity,
        float angularVelocity,
        float strength,
        const b2Shape& shape,
        float stride,
        float lifetime
        ) {

        auto def = new b2ParticleGroupDef();
		def->flags = flags;
		def->groupFlags = groupFlags;
		def->position = position;
		def->angle = angle;
		def->linearVelocity = linearVelocity;
		def->angularVelocity = angularVelocity;
		def->strength = strength;
		def->shape = &shape,
		def->stride = stride;
		def->lifetime = lifetime;
		return def;
    }

    b2RevoluteJoint* CreateRevoluteJoint(
            b2World& world,
            b2Body* bodyA,
            b2Body* bodyB,
            bool collideConnected,
            b2Vec2 localAnchorA,
            b2Vec2 localAnchorB,
            float referenceAngle,
            bool enableLimit,
            float lowerAngle,
            float upperAngle,
            bool enableMotor,
            float maxMotorTorque,
            float motorSpeed) {
        b2RevoluteJointDef def;
        def.type = b2JointType::e_revoluteJoint;
        def.bodyA = bodyA,
        def.bodyB = bodyB,
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
        def.localAnchorB = localAnchorB;
        def.referenceAngle = referenceAngle;
        def.enableLimit = enableLimit;
        def.lowerAngle = lowerAngle;
        def.upperAngle = upperAngle;
        def.enableMotor = enableMotor;
        def.maxMotorTorque = maxMotorTorque;
        def.motorSpeed = motorSpeed;
        return static_cast<b2RevoluteJoint*>(world.CreateJoint(&def));
    }


    b2PrismaticJoint* CreatePrismaticJoint(
            b2World& world,
            b2Body* bodyA,
            b2Body* bodyB,
            bool collideConnected,
            b2Vec2 localAnchorA,
            b2Vec2 localAnchorB,
            b2Vec2 localAxisA,
            float referenceAngle,
            bool enableLimit,
            float lowerTranslation,
            float upperTranslation,
            bool enableMotor,
            float maxMotorForce,
            float motorSpeed) {
        b2PrismaticJointDef def;
        def.type = b2JointType::e_prismaticJoint;
        def.bodyA = bodyA,
        def.bodyB = bodyB,
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
        def.localAnchorB = localAnchorB;
        def.localAxisA = localAxisA;
        def.referenceAngle = referenceAngle;
        def.enableLimit = enableLimit;
        def.lowerTranslation = lowerTranslation;
        def.upperTranslation = upperTranslation;
        def.enableMotor = enableMotor;
        def.maxMotorForce = maxMotorForce;
        def.motorSpeed = motorSpeed;
        return static_cast<b2PrismaticJoint*>(world.CreateJoint(&def));
    }

    b2DistanceJoint* CreateDistanceJoint(b2World& world, const b2DistanceJointDef* def) {
        return static_cast<b2DistanceJoint*>(world.CreateJoint(def));
    }

    b2PulleyJoint* CreatePulleyJoint(b2World& world, const b2PulleyJointDef* def) {
        return static_cast<b2PulleyJoint*>(world.CreateJoint(def));
    }

    b2MouseJoint* CreateMouseJoint(b2World& world, const b2MouseJointDef* def) {
        return static_cast<b2MouseJoint*>(world.CreateJoint(def));
    }

    b2GearJoint* CreateGearJoint(b2World& world, const b2GearJointDef* def) {
        return static_cast<b2GearJoint*>(world.CreateJoint(def));
    }

    b2WheelJoint* CreateWheelJoint(b2World& world, const b2WheelJointDef* def) {
        return static_cast<b2WheelJoint*>(world.CreateJoint(def));
    }

    b2WeldJoint* CreateWeldJoint(b2World& world, const b2WeldJointDef* def) {
        return static_cast<b2WeldJoint*>(world.CreateJoint(def));
    }

    b2FrictionJoint* CreateFrictionJoint(b2World& world, const b2FrictionJointDef* def) {
        return static_cast<b2FrictionJoint*>(world.CreateJoint(def));
    }

    b2MotorJoint* CreateMotorJoint(b2World& world, const b2MotorJointDef* def) {
        return static_cast<b2MotorJoint*>(world.CreateJoint(def));
    }
}

#endif