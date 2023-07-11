// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <unordered_map>
#include <stdexcept>
#include <string>
#include <iostream>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class EventHandlerCompartment
{
public:
    int state;
    
    EventHandlerCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class EventHandler
{
private:
    EventHandlerCompartment *_compartment_;
    EventHandlerCompartment *_nextCompartment_;
    
    
    
public:
    EventHandler()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(EventHandlerState::S1);
        
        _compartment_ = new EventHandlerCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class EventHandlerState
    {
        S1 = 0,
        S2 = 1
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(EventHandlerState::S1))
        {
            this->_sS1_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(EventHandlerState::S2))
        {
            this->_sS2_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            EventHandlerCompartment *nextCompartment = this->_nextCompartment_;
            this->_nextCompartment_ = nullptr;
            if (nextCompartment->_forwardEvent != nullptr && 
                nextCompartment->_forwardEvent->_message == ">")
            {
                this->_mux_(new FrameEvent( "<", this->_compartment_->exitArgs));
                this->_compartment_ = nextCompartment;
                this->_mux_(nextCompartment->_forwardEvent);
            }
            else
            {
                this->_doTransition_(nextCompartment);
                if (nextCompartment->_forwardEvent != nullptr)
                {
                    this->_mux_(nextCompartment->_forwardEvent);
                }
            }
            nextCompartment->_forwardEvent = nullptr;
        }
    }
    
        
        //===================== Interface Block ===================//
    public:
        void LogIt(int x) {
            unordered_map<string, any> params;
            params["x"] = x;

            FrameEvent e("LogIt", params);
            this->_mux_(&e);
        }
        
        void LogAdd(int a,int b) {
            unordered_map<string, any> params;
            params["a"] = a;

            // FrameEvent e("LogAdd", params);
            // this->_mux_(&e);
            params["b"] = b;

            FrameEvent e("LogAdd", params);
            this->_mux_(&e);
        }
        
        int LogReturn(int a,int b) {
            unordered_map<string, any> params;
            params["a"] = a;

            // FrameEvent e("LogReturn", params);
            // this->_mux_(&e);
            params["b"] = b;

            FrameEvent e("LogReturn", params);
            this->_mux_(&e);
            return any_cast<int>(e._return);
        }
        
        void PassAdd(int a,int b) {
            unordered_map<string, any> params;
            params["a"] = a;

            // FrameEvent e("PassAdd", params);
            // this->_mux_(&e);
            params["b"] = b;

            FrameEvent e("PassAdd", params);
            this->_mux_(&e);
        }
        
        int PassReturn(int a,int b) {
            unordered_map<string, any> params;
            params["a"] = a;

            // FrameEvent e("PassReturn", params);
            // this->_mux_(&e);
            params["b"] = b;

            FrameEvent e("PassReturn", params);
            this->_mux_(&e);
            return any_cast<int>(e._return);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sS1_(FrameEvent *e)
    {
        if (e->_message == "LogIt") {
            log_do("x",any_cast<int>(e->_parameters["x"]));
            return;
        }
        else if (e->_message == "LogAdd") {
            log_do("a",any_cast<int>(e->_parameters["a"]));
            log_do("b",any_cast<int>(e->_parameters["b"]));
            log_do("a+b",any_cast<int>(e->_parameters["a"]) + any_cast<int>(e->_parameters["b"]));
            return;
        }
        else if (e->_message == "LogReturn") {
            log_do("a",any_cast<int>(e->_parameters["a"]));
            log_do("b",any_cast<int>(e->_parameters["b"]));
            int r  = any_cast<int>(e->_parameters["a"]) + any_cast<int>(e->_parameters["b"]);
            log_do("r",r);
            e->_return = (void*) new int(r);
            
            return;
            
        }
        else if (e->_message == "PassAdd") {
            EventHandlerCompartment *compartment =  new EventHandlerCompartment(static_cast<int>(EventHandlerState::S2));
            compartment->stateArgs["p"] = any_cast<int>(e->_parameters["a"]) + any_cast<int>(e->_parameters["b"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "PassReturn") {
            int r  = any_cast<int>(e->_parameters["a"]) + any_cast<int>(e->_parameters["b"]);
            log_do("r",r);
            EventHandlerCompartment *compartment =  new EventHandlerCompartment(static_cast<int>(EventHandlerState::S2));
            compartment->stateArgs["p"] = r;
            
            this->_transition_(compartment);
            e->_return = (void*) new int(r);
            
            return;
            
        }
    }
    
    void _sS2_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("p",any_cast<int>(this->_compartment_->stateArgs["p"]));
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    // Unimplemented Actions
    
    // void log_do(std::string msg, int val) {  throw std::logic_error("Not implemented");  }

    public:
    void log_do(std::string msg, int val) {
        std::string value = msg + "=" + to_string(val); 
		tape.push_back(value);
	}
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(EventHandlerCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(EventHandlerCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    public:
    string state_info(){
        return std::to_string(_compartment_->state);
        }
        
};

/********************

class EventHandlerController : public EventHandler
{
public:
	EventHandlerController() : EventHandler() {}
std::string msg, int val) {}
};

********************/

