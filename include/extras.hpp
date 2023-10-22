#ifndef EXTRAS_HPP
#define EXTRAS_HPP

#include <box2d/box2d.h>

extern "C" {
    class b2CircleShape;
    struct b2Vec2;
    struct b2ParticleGroupDef;

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
        const b2Shape* shapes,
        int32 shapeCount,
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
		def->shapes = &shapes;
		def->shapeCount = shapeCount;
		def->stride = stride;
		def->lifetime = lifetime;
		return def;
    }
}

#endif