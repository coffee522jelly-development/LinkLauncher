#include "cLinkLauncher.h"
#include "link_launcher_core.h"

// 外部公開API
// 戻り値：成功=0，失敗-1
// 引数：パス
extern "C" {
    int core_launch_path(const char* path)
    {
        if (path == nullptr)    return -1;

        CLinkLauncher   aLauncher;
        return aLauncher.Launch(path);
    }

    int core_reveal_path(const char* path)
    {
        if (path == nullptr)    return -1;

        CLinkLauncher   aLauncher;
        return aLauncher.Reveal(path);
    }
}
