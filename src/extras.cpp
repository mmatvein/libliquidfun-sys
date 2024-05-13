#include <box2d/box2d.h>
#include "extras.hpp"

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

b2DistanceJoint* CreateDistanceJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    float length,
    float minLength,
    float maxLength,
    float stiffness,
    float damping
 ) {
    b2DistanceJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.localAnchorA = localAnchorA;
    def.localAnchorB = localAnchorB;
    def.length = length;
    def.minLength = minLength;
    def.maxLength = maxLength;
    def.stiffness = stiffness;
    def.damping = damping;
    return static_cast<b2DistanceJoint*>(world.CreateJoint(&def));
}

b2PulleyJoint* CreatePulleyJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 groundAnchorA,
    b2Vec2 groundAnchorB,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    float lengthA,
    float lengthB,
    float ratio
) {
    b2PulleyJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.groundAnchorA = groundAnchorA;
    def.groundAnchorB = groundAnchorB;
    def.localAnchorA = localAnchorA;
    def.localAnchorB = localAnchorB;
    def.lengthA = lengthA;
    def.lengthB = lengthB;
    def.ratio = ratio;
    return static_cast<b2PulleyJoint*>(world.CreateJoint(&def));
}

b2MouseJoint* CreateMouseJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 target,
    float maxForce,
    float stiffness,
    float damping
) {
    b2MouseJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.target = target;
    def.maxForce = maxForce;
    def.stiffness = stiffness;
    def.damping = damping;
    return static_cast<b2MouseJoint*>(world.CreateJoint(&def));
}

b2GearJoint* CreateGearJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Joint* joint1,
    b2Joint* joint2,
    float ratio
) {
    b2GearJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.joint1 = joint1;
    def.joint2 = joint2;
    def.ratio = ratio;
    return static_cast<b2GearJoint*>(world.CreateJoint(&def));
}

b2WheelJoint* CreateWheelJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    b2Vec2 localAxisA,
    bool enableLimit,
    float lowerTranslation,
    float upperTranslation,
    bool enableMotor,
    float maxMotorTorque,
    float motorSpeed,
    float stiffness,
    float damping
) {
    b2WheelJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.localAnchorA = localAnchorA;
    def.localAnchorB = localAnchorB;
    def.localAxisA = localAxisA;
    def.enableLimit = enableLimit;
    def.lowerTranslation = lowerTranslation;
    def.upperTranslation = upperTranslation;
    def.enableMotor = enableMotor;
    def.maxMotorTorque = maxMotorTorque;
    def.motorSpeed = motorSpeed;
    def.stiffness = stiffness;
    def.damping = damping;
    return static_cast<b2WheelJoint*>(world.CreateJoint(&def));
}

b2WeldJoint* CreateWeldJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    float referenceAngle,
    float stiffness,
    float damping
) {
    b2WeldJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.localAnchorA = localAnchorA;
    def.localAnchorB = localAnchorB;
    def.referenceAngle = referenceAngle;
    def.stiffness = stiffness;
    def.damping = damping;
    return static_cast<b2WeldJoint*>(world.CreateJoint(&def));
}

b2FrictionJoint* CreateFrictionJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    float maxForce,
    float maxTorque
) {
    b2FrictionJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.localAnchorA = localAnchorA;
    def.localAnchorB = localAnchorB;
    def.maxForce = maxForce;
    def.maxTorque = maxTorque;
    return static_cast<b2FrictionJoint*>(world.CreateJoint(&def));
}

b2MotorJoint* CreateMotorJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 linearOffset,
    float angularOffset,
    float maxForce,
    float maxTorque,
    float correctionFactor
) {
    b2MotorJointDef def;
    def.bodyA = bodyA;
    def.bodyB = bodyB;
    def.collideConnected = collideConnected;
    def.linearOffset = linearOffset;
    def.angularOffset = angularOffset;
    def.maxForce = maxForce;
    def.maxTorque = maxTorque;
    def.correctionFactor = correctionFactor;
    return static_cast<b2MotorJoint*>(world.CreateJoint(&def));
}
