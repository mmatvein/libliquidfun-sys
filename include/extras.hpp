#pragma once

#include <box2d/box2d.h>

class b2CircleShape;
class b2DistanceJoint;
class b2FrictionJoint;
class b2GearJoint;
class b2MotorJoint;
class b2MouseJoint;
class b2PrismaticJoint;
class b2PulleyJoint;
class b2RevoluteJoint;
class b2RopeJoint;
class b2WeldJoint;
class b2WheelJoint;
class b2World;
enum b2JointType;
struct b2DistanceJointDef;
struct b2FrictionJointDef;
struct b2GearJointDef;
struct b2MotorJointDef;
struct b2MouseJointDef;
struct b2ParticleGroupDef;
struct b2PrismaticJointDef;
struct b2PulleyJointDef;
struct b2RevoluteJointDef;
struct b2RopeJointDef;
struct b2Vec2;
struct b2WeldJointDef;
struct b2WheelJointDef;

void SetCircleRadius(b2CircleShape& self, float radius);

void SetCirclePosition(b2CircleShape& self, const b2Vec2& position);

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
    );

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
        float motorSpeed);


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
        float motorSpeed);

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
 );

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
);

b2MouseJoint* CreateMouseJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 target,
    float maxForce,
    float stiffness,
    float damping
);

b2GearJoint* CreateGearJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Joint* joint1,
    b2Joint* joint2,
    float ratio
);

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
);

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
);

b2FrictionJoint* CreateFrictionJoint(
    b2World& world,
    b2Body* bodyA,
    b2Body* bodyB,
    bool collideConnected,
    b2Vec2 localAnchorA,
    b2Vec2 localAnchorB,
    float maxForce,
    float maxTorque
);

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
);
