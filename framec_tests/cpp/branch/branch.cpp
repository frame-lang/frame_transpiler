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

class BranchCompartment
{
public:
    int state;
    
    BranchCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class Branch
{
private:
    BranchCompartment *_compartment_;
    BranchCompartment *_nextCompartment_;
    
    
    
public:
    Branch()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(BranchState::I);
        
        _compartment_ = new BranchCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class BranchState
    {
        I = 0,
        SIMPLEIF = 1,
        NEGATEDIF = 2,
        PRECEDENCE = 3,
        NESTEDIF = 4,
        GUARDEDTRANSITION = 5,
        NESTEDGUARDEDTRANSITION = 6,
        F1 = 7,
        F2 = 8,
        F3 = 9
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(BranchState::I))
        {
            this->_sI_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::SIMPLEIF))
        {
            this->_sSimpleIf_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::NEGATEDIF))
        {
            this->_sNegatedIf_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::PRECEDENCE))
        {
            this->_sPrecedence_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::NESTEDIF))
        {
            this->_sNestedIf_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::GUARDEDTRANSITION))
        {
            this->_sGuardedTransition_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::NESTEDGUARDEDTRANSITION))
        {
            this->_sNestedGuardedTransition_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::F1))
        {
            this->_sF1_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::F2))
        {
            this->_sF2_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BranchState::F3))
        {
            this->_sF3_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            BranchCompartment *nextCompartment = this->_nextCompartment_;
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
        void A() {
            FrameEvent e("A", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void B() {
            FrameEvent e("B", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void C() {
            FrameEvent e("C", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void D() {
            FrameEvent e("D", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void E() {
            FrameEvent e("E", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void F() {
            FrameEvent e("F", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void OnBool(bool b) {
            unordered_map<string, any> params;
            params["b"] = b;

            
            FrameEvent e("OnBool", params);
            this->_mux_(&e);
        }
        
        void OnInt(int i) {
            unordered_map<string, any> params;
            params["i"] = i;

            
            FrameEvent e("OnInt", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sI_(FrameEvent *e)
    {
        if (e->_message == "A") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::SIMPLEIF));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "B") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::NEGATEDIF));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "C") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::PRECEDENCE));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "D") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::NESTEDIF));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "E") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::GUARDEDTRANSITION));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "F") {
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::NESTEDGUARDEDTRANSITION));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sSimpleIf_(FrameEvent *e)
    {
        if (e->_message == "OnBool") {
            if (any_cast<bool>(e->_parameters["b"])) {
                log_do("then 1");
            } else {
            }
            if (any_cast<bool>(e->_parameters["b"])) {
            } else {
                log_do("else 1");
            }
            if (any_cast<bool>(e->_parameters["b"])) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (any_cast<bool>(e->_parameters["b"])) {
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                
                this->_transition_(compartment);
            } else {
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                
                this->_transition_(compartment);
            }
            return;
        }
        else if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) > 5) {
                log_do("> 5");
            } else {
                log_do("<= 5");
            }
            if (any_cast<int>(e->_parameters["i"]) < 10) {
                log_do("< 10");
            } else {
                log_do(">= 10");
            }
            if (any_cast<int>(e->_parameters["i"]) == 7) {
                log_do("== 7");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                
                this->_transition_(compartment);
            } else {
                log_do("!= 7");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                
                this->_transition_(compartment);
            }
            return;
        }
    }
    
    void _sNegatedIf_(FrameEvent *e)
    {
        if (e->_message == "OnBool") {
            if (!(any_cast<bool>(e->_parameters["b"]))) {
                log_do("then 1");
            } else {
            }
            if (!(any_cast<bool>(e->_parameters["b"]))) {
            } else {
                log_do("else 1");
            }
            if (!(any_cast<bool>(e->_parameters["b"]))) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (!(any_cast<bool>(e->_parameters["b"]))) {
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                
                this->_transition_(compartment);
            } else {
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                
                this->_transition_(compartment);
            }
            return;
        }
        else if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) < 5) {
                log_do("< 5");
            } else {
                log_do(">= 5");
            }
            if (any_cast<int>(e->_parameters["i"]) > 10) {
                log_do("> 10");
            } else {
                log_do("<= 10");
            }
            if (any_cast<int>(e->_parameters["i"]) == 7) {
                log_do("== 7");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                
                this->_transition_(compartment);
            } else {
                log_do("!= 7");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                
                this->_transition_(compartment);
            }
            return;
        }
    }
    
    void _sPrecedence_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (-any_cast<int>(e->_parameters["i"]) >= 0 && -any_cast<int>(e->_parameters["i"]) <= 5) {
                log_do("then 1");
            } else {
                log_do("else 1");
            }
            if ((any_cast<int>(e->_parameters["i"]) > -5 && any_cast<int>(e->_parameters["i"]) > 5) && (any_cast<int>(e->_parameters["i"]) > -10 && any_cast<int>(e->_parameters["i"]) < 10)) {
                log_do("then 2");
            } else {
                log_do("else 2");
            }
            if (any_cast<int>(e->_parameters["i"]) >= 0 && any_cast<int>(e->_parameters["i"]) <= 5 || any_cast<int>(e->_parameters["i"]) >= 10 && any_cast<int>(e->_parameters["i"]) <= 20) {
                log_do("then 3");
            } else {
                log_do("else 3");
            }
            if ((any_cast<int>(e->_parameters["i"]) >= 0 && any_cast<int>(e->_parameters["i"]) < 10) && any_cast<int>(e->_parameters["i"]) + 5 < 20) {
                log_do("then 4");
            } else {
                log_do("else 4");
            }
            return;
        }
    }
    
    void _sNestedIf_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) > 0) {
                log_do("> 0");
                if (any_cast<int>(e->_parameters["i"]) < 100) {
                    log_do("< 100");
                    BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                    
                    this->_transition_(compartment);
                } else {
                    log_do(">= 100");
                }
            } else {
                log_do("<= 0");
                if (any_cast<int>(e->_parameters["i"]) > -10) {
                    log_do("> -10");
                } else {
                    log_do("<= -10");
                    BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                    
                    this->_transition_(compartment);
                }
            }
            return;
        }
    }
    
    void _sGuardedTransition_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) > 100) {
                log_do("-> $F1");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                
                this->_transition_(compartment);
                return;
            } else {
            }
            if (any_cast<int>(e->_parameters["i"]) < 10) {
            } else {
                log_do("-> $F2");
                BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                
                this->_transition_(compartment);
                return;
            }
            log_do("-> $F3");
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F3));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sNestedGuardedTransition_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) > 10) {
                if (any_cast<int>(e->_parameters["i"]) > 100) {
                    log_do("-> $F1");
                    BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F1));
                    
                    this->_transition_(compartment);
                    return;
                } else {
                }
                if (any_cast<int>(e->_parameters["i"]) > 50) {
                } else {
                    log_do("-> $F2");
                    BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F2));
                    
                    this->_transition_(compartment);
                    return;
                }
            } else {
            }
            log_do("-> $F3");
            BranchCompartment *compartment =  new BranchCompartment(static_cast<int>(BranchState::F3));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sF1_(FrameEvent *e)
    {
    }
    
    void _sF2_(FrameEvent *e)
    {
    }
    
    void _sF3_(FrameEvent *e)
    {
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(const std::string& msg)
    {
        tape.push_back(msg);
    }
    
    // Unimplemented Actions
    
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(BranchCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(BranchCompartment *nextCompartment)
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

class BranchController : public Branch
{
public:
	BranchController() : Branch() {}
};

********************/

