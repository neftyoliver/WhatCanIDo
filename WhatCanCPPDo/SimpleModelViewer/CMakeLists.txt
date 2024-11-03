cmake_minimum_required(VERSION 3.29)
project(SimpleModelViewer)

set(CMAKE_CXX_STANDARD 20)


add_executable(SimpleModelViewer main.cpp
        render.cpp
        render.h)


if (APPLE)
    message("MacOS detected. linking vulkan.")
    target_include_directories(SimpleModelViewer PUBLIC "$ENV{HOME}/VulkanSDK/1.3.296.0/macOS/include")

endif ()