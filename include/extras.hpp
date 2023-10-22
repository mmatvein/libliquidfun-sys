#ifndef EXTRAS_HPP
#define EXTRAS_HPP

#include <box2d/box2d.h>

extern "C" {
    class b2CircleShape;
    struct b2Vec2;

    void SetCircleRadius(b2CircleShape& self, float radius) {
        self.m_radius = radius;
    }

    void SetCirclePosition(b2CircleShape& self, const b2Vec2& position){
        self.m_p = position;
    }
}

#endif