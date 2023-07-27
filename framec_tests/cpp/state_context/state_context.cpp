// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <vector>
#include <any>
#include <unordered_map>
#include <string>
#include <iostream>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class StateContextSmCompartment
{
public:
    int state;
    
    StateContextSmCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class StateContextSm
{
private:
    StateContextSmCompartment *_compartment_;
    StateContextSmCompartment *_nextCompartment_;
    
    
    
public:
    StateContextSm()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(StateContextSmState::INIT);
        
        _compartment_ = new StateContextSmCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        _compartment_->stateVars["w"] = 0;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class StateContextSmState
    {
        INIT = 0,
        FOO = 1,
        BAR = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(StateContextSmState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateContextSmState::FOO))
        {
            this->_sFoo_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateContextSmState::BAR))
        {
            this->_sBar_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            StateContextSmCompartment *nextCompartment = this->_nextCompartment_;
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
        void Start() {
            FrameEvent e("Start", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void LogState() {
            FrameEvent e("LogState", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        int Inc() {
            FrameEvent e("Inc", unordered_map<string, any>());
            this->_mux_(&e);
            return any_cast<int>(e._return);
        }
        
        void Next(int arg) {
            unordered_map<string, any> params;
            params["arg"] = arg;

            
            FrameEvent e("Next", params);
            this->_mux_(&e);
        }
        
        void Change(int arg) {
            unordered_map<string, any> params;
            params["arg"] = arg;

            
            FrameEvent e("Change", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == ">") {
            this->_compartment_->stateVars["w"] = 3;
            log_do("w",(any_cast<int>(this->_compartment_->stateVars["w"])));
            return;
        }
        else if (e->_message == "Inc") {
            this->_compartment_->stateVars["w"] = (any_cast<int>(this->_compartment_->stateVars["w"])) + 1;
            log_do("w",(any_cast<int>(this->_compartment_->stateVars["w"])));
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["w"])));
            
            return;
            
        }
        else if (e->_message == "LogState") {
            log_do("w",(any_cast<int>(this->_compartment_->stateVars["w"])));
            return;
        }
        else if (e->_message == "Start") {
            StateContextSmCompartment *compartment =  new StateContextSmCompartment(static_cast<int>(StateContextSmState::FOO));
            compartment->enterArgs["a"] = 3;
            compartment->enterArgs["b"] = any_cast<int>(this->_compartment_->stateVars["w"]);
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sFoo_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("a",any_cast<int>(e->_parameters["a"]));
            log_do("b",any_cast<int>(e->_parameters["b"]));
            this->_compartment_->stateVars["x"] = any_cast<int>(e->_parameters["a"]) * any_cast<int>(e->_parameters["b"]);
            log_do("x",(any_cast<int>(this->_compartment_->stateVars["x"])));
            return;
        }
        else if (e->_message == "<") {
            log_do("c",any_cast<int>(e->_parameters["c"]));
            this->_compartment_->stateVars["x"] = (any_cast<int>(this->_compartment_->stateVars["x"])) + any_cast<int>(e->_parameters["c"]);
            log_do("x",(any_cast<int>(this->_compartment_->stateVars["x"])));
            return;
        }
        else if (e->_message == "LogState") {
            log_do("x",(any_cast<int>(this->_compartment_->stateVars["x"])));
            return;
        }
        else if (e->_message == "Inc") {
            this->_compartment_->stateVars["x"] = (any_cast<int>(this->_compartment_->stateVars["x"])) + 1;
            log_do("x",(any_cast<int>(this->_compartment_->stateVars["x"])));
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["x"])));
            
            return;
            
        }
        else if (e->_message == "Next") {
            int tmp  = any_cast<int>(e->_parameters["arg"]) * 10;
            this->_compartment_->exitArgs["c"] = 10;
            StateContextSmCompartment *compartment =  new StateContextSmCompartment(static_cast<int>(StateContextSmState::BAR));
            compartment->enterArgs["a"] = tmp;
            compartment->stateArgs["y"] = any_cast<int>(this->_compartment_->stateVars["x"]);
            compartment->stateVars["z"] = 0;
            
            this->_transition_(compartment);
            return;
        }
          //  FIXME: Swapping this to 10 * arg causes a parse error!
		else if (e->_message == "Change") {
            int tmp  = any_cast<int>(this->_compartment_->stateVars["x"]) + any_cast<int>(e->_parameters["arg"]);
            StateContextSmCompartment *compartment =  new StateContextSmCompartment(static_cast<int>(StateContextSmState::BAR));
            compartment->stateArgs["y"] = tmp;
            compartment->stateVars["z"] = 0;
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sBar_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("a",any_cast<int>(e->_parameters["a"]));
            log_do("y",any_cast<int>(this->_compartment_->stateArgs["y"]));
            this->_compartment_->stateVars["z"] = any_cast<int>(e->_parameters["a"]) + any_cast<int>(this->_compartment_->stateArgs["y"]);
            log_do("z",(any_cast<int>(this->_compartment_->stateVars["z"])));
            return;
        }
        else if (e->_message == "LogState") {
            log_do("y",any_cast<int>(this->_compartment_->stateArgs["y"]));
            log_do("z",(any_cast<int>(this->_compartment_->stateVars["z"])));
            return;
        }
        else if (e->_message == "Inc") {
            this->_compartment_->stateVars["z"] = (any_cast<int>(this->_compartment_->stateVars["z"])) + 1;
            log_do("z",(any_cast<int>(this->_compartment_->stateVars["z"])));
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["z"])));
            
            return;
            
        }
        else if (e->_message == "Change") {
            int tmp  = any_cast<int>(this->_compartment_->stateArgs["y"]) + any_cast<int>(this->_compartment_->stateVars["z"]) + any_cast<int>(e->_parameters["arg"]);
            log_do("tmp",tmp);
            StateContextSmCompartment *compartment =  new StateContextSmCompartment(static_cast<int>(StateContextSmState::INIT));
            compartment->stateVars["w"] = 0;
            
            this->_changeState_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(const std::string& name, int val)
    {
        
    tape.push_back(name + "=" + std::to_string(val));
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(StateContextSmCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(StateContextSmCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    private: void _changeState_(StateContextSmCompartment* compartment)
    {
        this->_compartment_ = compartment;
    }
    
    public:
    string state_info(){
        return std::to_string(_compartment_->state);
        }
        
};

/********************

class StateContextSmController : public StateContextSm
{
public:
	StateContextSmController() : StateContextSm() {}
};

********************/

