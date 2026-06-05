#include "link_launcher.h"

#include <windows.h>
#include <shellapi.h>

// UTF-8をUTF-16に変換する
// 戻り値：UTF-16テキスト
// 引数：UTF-8テキスト
std::wstring CLinkLauncher::Utf8ToWide(const std::string& text) const
{
    const int size = MultiByteToWideChar(
        CP_UTF8,
        0,
        text.c_str(),
        -1,
        nullptr,
        0);

    if (size <= 0)  return L"";

    std::wstring result(size, L'\0');

    MultiByteToWideChar(
        CP_UTF8,
        0,
        text.c_str(),
        -1,
        result.data(),
        size);

    return result;
}


// パスを開く関数
// 戻り値：既定のアプリでパスのファイルを開けたか？
// 引数：パス
bool CLinkLauncher::Launch(const std::string& path) const
{
    const std::wstring widePath = Utf8ToWide(path);
    if (widePath.empty())   return false;

    const HINSTANCE hResult = ShellExecuteW(
        nullptr,
        L"open",
        widePath.c_str(),
        nullptr,
        nullptr,
        SW_SHOWNORMAL);

    return reinterpret_cast<INT_PTR>(hResult) > 32;
}