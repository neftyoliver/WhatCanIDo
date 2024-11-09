cmake_minimum_required(VERSION 3.29)
project(SimpleModelViewer LANGUAGES C CXX Swift)

set(CMAKE_CXX_STANDARD 20)

add_executable(SimpleModelViewer main.cpp
        render.cpp
        render.h
        SimpleModelViewer.h)


if (APPLE)
    message("MacOS detected. linking vulkan.")
    target_include_directories(SimpleModelViewer PUBLIC "$ENV{HOME}/VulkanSDK/1.3.296.0/macOS/include")

    target_include_directories(SimpleModelViewer PUBLIC "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/AppKit.framework/Versions/C/Headers")
    target_include_directories(SimpleModelViewer PUBLIC "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/System/Library/Frameworks/Foundation.framework/Versions/C/Headers")

    add_subdirectory(macos)

    target_link_libraries(SimpleModelViewer PUBLIC WindowSwift)

endif ()