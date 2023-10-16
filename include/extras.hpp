#ifndef EXTRAS_HPP
#define EXTRAS_HPP

#include <box2d/box2d.h>

extern "C" {
    class b2CircleShape;
    void SetCircleRadius(b2CircleShape& self, float radius) {
        self.m_radius = radius;
    }
}

#endif