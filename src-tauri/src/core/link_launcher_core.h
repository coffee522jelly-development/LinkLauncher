#pragma once
#include <string>

extern "C" {
    /**
     * core_launch_path
     * Executes a given URL or file path using the system's default handler.
     * Returns 0 on success, or an error code.
     */
    int core_launch_path(const char* path);
}
