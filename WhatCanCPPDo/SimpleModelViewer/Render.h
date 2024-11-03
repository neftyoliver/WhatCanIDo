//
// Created by 송서혁 on 11/3/24.
//

#ifndef RENDER_H
#define RENDER_H
#include <cstdint>

namespace renderer {

    class WindowManager {
        public:
        WindowManager(uint32_t width, uint32_t height);
        ~WindowManager();
    };

    class Render {
        public:
        Render();

        ~Render();
    };

} // renderer

#endif //RENDER_H
