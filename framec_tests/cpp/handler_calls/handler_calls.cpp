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

class HandlerCallsCompartment
{
public:
    int state;
    
    HandlerCallsCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class HandlerCalls
{
private:
    HandlerCallsCompartment *_compartment_;
    HandlerCallsCompartment *_nextCompartment_;
    
    
    
public:
    HandlerCalls()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(HandlerCallsState::INIT);
        
        _compartment_ = new HandlerCallsCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class HandlerCallsState
    {
        INIT = 0,
        NONRECURSIVE = 1,
        SELFRECURSIVE = 2,
        MUTUALLYRECURSIVE = 3,
        FINAL = 4
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(HandlerCallsState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HandlerCallsState::NONRECURSIVE))
        {
            this->_sNonRecursive_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HandlerCallsState::SELFRECURSIVE))
        {
            this->_sSelfRecursive_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HandlerCallsState::MUTUALLYRECURSIVE))
        {
            this->_sMutuallyRecursive_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HandlerCallsState::FINAL))
        {
            this->_sFinal_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            HandlerCallsCompartment *nextCompartment = this->_nextCompartment_;
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
        void NonRec() {
            FrameEvent e("NonRec", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void SelfRec() {
            FrameEvent e("SelfRec", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void MutRec() {
            FrameEvent e("MutRec", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Call(string event,int arg) {
            unordered_map<string, any> params;
            params["event"] = event;

            
            params["arg"] = arg;

            
            FrameEvent e("Call", params);
            this->_mux_(&e);
        }
        
        void Foo(int arg) {
            unordered_map<string, any> params;
            params["arg"] = arg;

            
            FrameEvent e("Foo", params);
            this->_mux_(&e);
        }
        
        void Bar(int arg) {
            unordered_map<string, any> params;
            params["arg"] = arg;

            
            FrameEvent e("Bar", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "NonRec") {
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::NONRECURSIVE));
            compartment->stateVars["counter"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "SelfRec") {
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::SELFRECURSIVE));
            compartment->stateVars["counter"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "MutRec") {
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::MUTUALLYRECURSIVE));
            compartment->stateVars["counter"] = 0;
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sNonRecursive_(FrameEvent *e)
    {
        if (e->_message == "Foo") {
            log_do("Foo",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            Bar(any_cast<int>(e->_parameters["arg"]) * 2);
            return;
            log_do("Unreachable",0);
            return;
        }
          //  the front-end should report the next line as a static error
		else if (e->_message == "Bar") {
            log_do("Bar",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::FINAL));
            compartment->stateArgs["counter"] = any_cast<int>(this->_compartment_->stateVars["counter"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Call") {
            if ((any_cast<string>(e->_parameters["event"]) == "Foo")) {
                Foo(any_cast<int>(e->_parameters["arg"]));
                return;
            } else if ((any_cast<string>(e->_parameters["event"]) == "Bar")) {
                Bar(any_cast<int>(e->_parameters["arg"]));
                return;
            } else {
                Call("Foo",1000);
                return;
            }
            return;
        }
    }
    
    void _sSelfRecursive_(FrameEvent *e)
    {
        if (e->_message == "Foo") {
            log_do("Foo",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            if ((any_cast<int>(this->_compartment_->stateVars["counter"])) < 100) {
                Foo(any_cast<int>(e->_parameters["arg"]) * 2);
                return;
            } else {
                HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::FINAL));
                compartment->stateArgs["counter"] = any_cast<int>(this->_compartment_->stateVars["counter"]);
                
                this->_transition_(compartment);
            }
            return;
        }
        else if (e->_message == "Bar") {
            log_do("Bar",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::FINAL));
            compartment->stateArgs["counter"] = any_cast<int>(this->_compartment_->stateVars["counter"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Call") {
            if ((any_cast<string>(e->_parameters["event"]) == "Foo")) {
                Foo(any_cast<int>(e->_parameters["arg"]));
                return;
            } else if ((any_cast<string>(e->_parameters["event"]) == "Bar")) {
                Bar(any_cast<int>(e->_parameters["arg"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _sMutuallyRecursive_(FrameEvent *e)
    {
        if (e->_message == "Foo") {
            log_do("Foo",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            if ((any_cast<int>(this->_compartment_->stateVars["counter"])) > 100) {
                HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::FINAL));
                compartment->stateArgs["counter"] = any_cast<int>(this->_compartment_->stateVars["counter"]);
                
                this->_transition_(compartment);
            } else {
                Bar(any_cast<int>(e->_parameters["arg"]) * 2);
                return;
            }
            return;
        }
        else if (e->_message == "Bar") {
            log_do("Bar",any_cast<int>(e->_parameters["arg"]));
            this->_compartment_->stateVars["counter"] = (any_cast<int>(this->_compartment_->stateVars["counter"])) + any_cast<int>(e->_parameters["arg"]);
            if ((any_cast<int>(e->_parameters["arg"]) == 4)) {
                Foo(any_cast<int>(e->_parameters["arg"]));
                return;
            } else if ((any_cast<int>(e->_parameters["arg"]) == 8)) {
                Foo(any_cast<int>(e->_parameters["arg"]) * 2);
                return;
            } else {
                Foo(any_cast<int>(e->_parameters["arg"]) * 3);
                return;
            }
            return;
        }
        else if (e->_message == "Call") {
            if ((any_cast<string>(e->_parameters["event"]) == "Foo")) {
                Foo(any_cast<int>(e->_parameters["arg"]));
                return;
            } else if ((any_cast<string>(e->_parameters["event"]) == "Bar")) {
                Bar(any_cast<int>(e->_parameters["arg"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _sFinal_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("Final",any_cast<int>(this->_compartment_->stateArgs["counter"]));
            HandlerCallsCompartment *compartment =  new HandlerCallsCompartment(static_cast<int>(HandlerCallsState::INIT));
            
            this->_transition_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(string from, int val)
    {
        tape.push_back(from + "(" + std::to_string(val) + ")");
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(HandlerCallsCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(HandlerCallsCompartment *nextCompartment)
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

class HandlerCallsController : public HandlerCalls
{
public:
	HandlerCallsController() : HandlerCalls() {}
};

********************/

