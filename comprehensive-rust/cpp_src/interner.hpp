#ifndef INTERNER_HPP
#define INTERNER_HPP

#include <string>
#include <unordered_set>

class StringInterner {
    std::unordered_set<std::string> strings;

public:
    const char* intern(const char* s) {
        auto [it, _] = strings.emplace(s);
        return it->c_str();
    }

    size_t count() const {
        return strings.size();
    }
};

#endif