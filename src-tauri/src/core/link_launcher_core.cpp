#include "link_launcher_core.h"
#include <windows.h>
#include <shellapi.h>
#include <string>

extern "C" {
    int core_launch_path(const char* path) {
        // Convert UTF-8 to Wide String for Windows API
        int wchars_num = MultiByteToWideChar(CP_UTF8, 0, path, -1, NULL, 0);
        if (wchars_num <= 0) return -1;

        wchar_t* w_path = new wchar_t[wchars_num];
        MultiByteToWideChar(CP_UTF8, 0, path, -1, w_path, wchars_num);

        // Use ShellExecuteW for opening files or URLs
        HINSTANCE result = ShellExecuteW(NULL, L"open", w_path, NULL, NULL, SW_SHOWNORMAL);

        delete[] w_path;

        // ShellExecute returns > 32 on success
        if ((intptr_t)result > 32) {
            return 0;
        } else {
            return (int)(intptr_t)result;
        }
    }
}
