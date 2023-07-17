// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <unordered_map>
#include <stdexcept>
#include <iostream>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class NamingCompartment
{
public:
    int state;
    
    NamingCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class Naming
{
private:
    NamingCompartment *_compartment_;
    NamingCompartment *_nextCompartment_;
    
    
    
public:
    Naming()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(NamingState::INIT);
        
        _compartment_ = new NamingCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class NamingState
    {
        INIT = 0,
        SNAKE_STATE = 1,
        CAMELSTATE = 2,
        STATE123 = 3,
        FINAL = 4
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(NamingState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(NamingState::SNAKE_STATE))
        {
            this->_ssnake_state_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(NamingState::CAMELSTATE))
        {
            this->_sCamelState_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(NamingState::STATE123))
        {
            this->_sstate123_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(NamingState::FINAL))
        {
            this->_sFinal_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            NamingCompartment *nextCompartment = this->_nextCompartment_;
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
        void snake_event(int snake_param) {
            unordered_map<string, any> params;
            params["snake_param"] = snake_param;

            
            FrameEvent e("snake_event", params);
            this->_mux_(&e);
        }
        
        void CamelEvent(int CamelParam) {
            unordered_map<string, any> params;
            params["CamelParam"] = CamelParam;

            
            FrameEvent e("CamelEvent", params);
            this->_mux_(&e);
        }
        
        void event123(int param123) {
            unordered_map<string, any> params;
            params["param123"] = param123;

            
            FrameEvent e("event123", params);
            this->_mux_(&e);
        }
        
        void call(const std::string& event,int param) {
            unordered_map<string, any> params;
            params["event"] = event;

            
            params["param"] = param;

            
            FrameEvent e("call", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "snake_event") {
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::SNAKE_STATE));
            compartment->stateArgs["snake_state_param"] = any_cast<int>(e->_parameters["snake_param"]);
            compartment->stateVars["snake_state_var"] = this->snake_domain_var + this->CamelDomainVar + this->domainVar123 + 100;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "CamelEvent") {
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::CAMELSTATE));
            compartment->stateArgs["CamelStateParam"] = any_cast<int>(e->_parameters["CamelParam"]);
            compartment->stateVars["CamelStateVar"] = this->snake_domain_var + this->CamelDomainVar + this->domainVar123 + 200;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "event123") {
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::STATE123));
            compartment->stateArgs["stateParam123"] = any_cast<int>(e->_parameters["param123"]);
            compartment->stateVars["stateVar123"] = this->snake_domain_var + this->CamelDomainVar + this->domainVar123 + 300;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "call") {
            if (any_cast<std::string>(e->_parameters["event"]) == "snake_event") {
                snake_event(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "CamelEvent") {
                CamelEvent(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "event123") {
                event123(any_cast<int>(e->_parameters["param"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _ssnake_state_(FrameEvent *e)
    {
          //  1100
		if (e->_message == "snake_event") {
            int snake_local_var  = any_cast<int>(this->_compartment_->stateVars["snake_state_var"]) + any_cast<int>(this->_compartment_->stateArgs["snake_state_param"]) + any_cast<int>(e->_parameters["snake_param"]);
            snake_action_do(snake_local_var);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = snake_local_var;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "CamelEvent") {
            int CamelLocalVar  = any_cast<int>(this->_compartment_->stateVars["snake_state_var"]) + any_cast<int>(this->_compartment_->stateArgs["snake_state_param"]) + any_cast<int>(e->_parameters["CamelParam"]);
            CamelAction_do(CamelLocalVar);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = CamelLocalVar;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "event123") {
            int localVar123  = any_cast<int>(this->_compartment_->stateVars["snake_state_var"]) + any_cast<int>(this->_compartment_->stateArgs["snake_state_param"]) + any_cast<int>(e->_parameters["param123"]);
            action123_do(localVar123);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = localVar123;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "call") {
            if (any_cast<std::string>(e->_parameters["event"]) == "snake_event") {
                snake_event(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "CamelEvent") {
                CamelEvent(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "event123") {
                event123(any_cast<int>(e->_parameters["param"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _sCamelState_(FrameEvent *e)
    {
          //  1200
		if (e->_message == "snake_event") {
            int snake_local_var  = any_cast<int>(this->_compartment_->stateVars["CamelStateVar"]) + any_cast<int>(this->_compartment_->stateArgs["CamelStateParam"]) + any_cast<int>(e->_parameters["snake_param"]);
            snake_action_do(snake_local_var);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = snake_local_var;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "CamelEvent") {
            int CamelLocalVar  = any_cast<int>(this->_compartment_->stateVars["CamelStateVar"]) + any_cast<int>(this->_compartment_->stateArgs["CamelStateParam"]) + any_cast<int>(e->_parameters["CamelParam"]);
            CamelAction_do(CamelLocalVar);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = CamelLocalVar;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "event123") {
            int localVar123  = any_cast<int>(this->_compartment_->stateVars["CamelStateVar"]) + any_cast<int>(this->_compartment_->stateArgs["CamelStateParam"]) + any_cast<int>(e->_parameters["param123"]);
            action123_do(localVar123);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = localVar123;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "call") {
            if (any_cast<std::string>(e->_parameters["event"]) == "snake_event") {
                snake_event(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "CamelEvent") {
                CamelEvent(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "event123") {
                event123(any_cast<int>(e->_parameters["param"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _sstate123_(FrameEvent *e)
    {
          //  1300
		if (e->_message == "snake_event") {
            int snake_local_var  = any_cast<int>(this->_compartment_->stateVars["stateVar123"]) + any_cast<int>(this->_compartment_->stateArgs["stateParam123"]) + any_cast<int>(e->_parameters["snake_param"]);
            snake_action_do(snake_local_var);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = snake_local_var;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "CamelEvent") {
            int CamelLocalVar  = any_cast<int>(this->_compartment_->stateVars["stateVar123"]) + any_cast<int>(this->_compartment_->stateArgs["stateParam123"]) + any_cast<int>(e->_parameters["CamelParam"]);
            CamelAction_do(CamelLocalVar);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = CamelLocalVar;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "event123") {
            int localVar123  = any_cast<int>(this->_compartment_->stateVars["stateVar123"]) + any_cast<int>(this->_compartment_->stateArgs["stateParam123"]) + any_cast<int>(e->_parameters["param123"]);
            action123_do(localVar123);
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::FINAL));
            compartment->stateArgs["result"] = localVar123;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "call") {
            if (any_cast<std::string>(e->_parameters["event"]) == "snake_event") {
                snake_event(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "CamelEvent") {
                CamelEvent(any_cast<int>(e->_parameters["param"]));
                return;
            } else if (any_cast<std::string>(e->_parameters["event"]) == "event123") {
                event123(any_cast<int>(e->_parameters["param"]));
                return;
            } else {
            }
            return;
        }
    }
    
    void _sFinal_(FrameEvent *e)
    {
        if (e->_message == ">") {
            logFinal_do(any_cast<int>(this->_compartment_->stateArgs["result"]));
            NamingCompartment *compartment =  new NamingCompartment(static_cast<int>(NamingState::INIT));
            
            this->_transition_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void snake_action_do(int snake_param)
    {
        snake_log.push_back(snake_param);
    }
    
    
public:
    void CamelAction_do(int CamelParam)
    {
        CamelLog.push_back(CamelParam);
    }
    
    
public:
    void action123_do(int param123)
    {
        log123.push_back(param123);
    }
    
    
public:
    void logFinal_do(int r)
    {
        finalLog.push_back(r);
    }
    
    // Unimplemented Actions
    
    
    //===================== Domain Block ===================//
    
    
    public:
    int snake_domain_var  = 300;
    public:
    int CamelDomainVar  = 550;
    public:
    int domainVar123  = 150;
    public:
    std::vector<int> snake_log ;
    public:
    std::vector<int> CamelLog ;
    public:
    std::vector<int> log123 ;
    public:
    std::vector<int> finalLog ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(NamingCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(NamingCompartment *nextCompartment)
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

class NamingController : public Naming
{
public:
	NamingController() : Naming() {}
};

********************/

