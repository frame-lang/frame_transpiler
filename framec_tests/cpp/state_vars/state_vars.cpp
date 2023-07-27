// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class StateVarsCompartment
{
public:
    int state;
    
    StateVarsCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class StateVars
{
public:
    StateVarsCompartment *_compartment_;
    StateVarsCompartment *_nextCompartment_;
    
    
    
public:
    StateVars()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(StateVarsState::INIT);
        
        _compartment_ = new StateVarsCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class StateVarsState
    {
        INIT = 0,
        A = 1,
        B = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(StateVarsState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateVarsState::A))
        {
            this->_sA_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateVarsState::B))
        {
            this->_sB_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            StateVarsCompartment *nextCompartment = this->_nextCompartment_;
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
        void X() {
            FrameEvent e("X", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Y() {
            FrameEvent e("Y", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Z() {
            FrameEvent e("Z", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == ">") {
            StateVarsCompartment *compartment =  new StateVarsCompartment(static_cast<int>(StateVarsState::A));
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sA_(FrameEvent *e)
    {
        if (e->_message == "X") {
            this->_compartment_->stateVars["x"] = (any_cast<int>(this->_compartment_->stateVars["x"])) + 1;
            return;
        }
        else if (e->_message == "Y") {
            StateVarsCompartment *compartment =  new StateVarsCompartment(static_cast<int>(StateVarsState::B));
            compartment->stateVars["y"] = 10;
            compartment->stateVars["z"] = 100;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Z") {
            StateVarsCompartment *compartment =  new StateVarsCompartment(static_cast<int>(StateVarsState::B));
            compartment->stateVars["y"] = 10;
            compartment->stateVars["z"] = 100;
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sB_(FrameEvent *e)
    {
        if (e->_message == "X") {
            StateVarsCompartment *compartment =  new StateVarsCompartment(static_cast<int>(StateVarsState::A));
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Y") {
            this->_compartment_->stateVars["y"] = (any_cast<int>(this->_compartment_->stateVars["y"])) + 1;
            return;
        }
        else if (e->_message == "Z") {
            this->_compartment_->stateVars["z"] = (any_cast<int>(this->_compartment_->stateVars["z"])) + 1;
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(StateVarsCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(StateVarsCompartment *nextCompartment)
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

class StateVarsController : public StateVars
{
public:
	StateVarsController() : StateVars() {}
};

********************/

