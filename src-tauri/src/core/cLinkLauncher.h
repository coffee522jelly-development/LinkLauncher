#pragma once

#include <string>

class CLinkLauncher
{
public:
    int Launch(const std::string& path) const;

private:
    std::wstring Utf8ToWide(const std::string& text) const;
};