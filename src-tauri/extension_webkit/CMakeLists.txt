project(dorion_csp_killer)
cmake_minimum_required(VERSION 3.25)

include(FindPkgConfig)

set(EXT_SRCS main.cpp)

# Target executable names.
set(EXT_TARGET "extension")

# find gtk
PKG_CHECK_MODULES(GTK3 REQUIRED gtk+-3.0)
include_directories(${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})
add_definitions(${GTK3_CFLAGS_OTHER})

# include webkitgtk
PKG_CHECK_MODULES(WEBKIT REQUIRED webkit2gtk-web-extension-4.1)
include_directories(${WEBKIT_INCLUDE_DIRS})
link_directories(${WEBKIT_LIBRARY_DIRS})
add_definitions(${WEBKIT_CFLAGS_OTHER})

# add extension lib
add_library(${EXT_TARGET} SHARED ${EXT_SRCS})
target_link_libraries(${EXT_TARGET} ${WEBKIT_LIBRARIES})