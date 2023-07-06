#include <string>
#include <unordered_map>
#include <any>
using namespace std;

class FrameEvent {
public:
    std::string _message;
    std::unordered_map<std::string, any> _parameters;
    void* _return = nullptr;

    FrameEvent(std::string message, std::unordered_map<std::string, any> parameters) :
        _message(message), _parameters(parameters) {}


    FrameEvent() {}
};