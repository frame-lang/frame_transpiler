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

class HierarchicalGuardCompartment
{
public:
    int state;
    
    HierarchicalGuardCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class HierarchicalGuard
{
private:
    HierarchicalGuardCompartment *_compartment_;
    HierarchicalGuardCompartment *_nextCompartment_;
    
    
    
public:
    HierarchicalGuard()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(HierarchicalGuardState::I);
        
        _compartment_ = new HierarchicalGuardCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class HierarchicalGuardState
    {
        I = 0,
        S = 1,
        S0 = 2,
        S1 = 3,
        S2 = 4,
        S3 = 5,
        S4 = 6
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::I))
        {
            this->_sI_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S))
        {
            this->_sS_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S0))
        {
            this->_sS0_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S1))
        {
            this->_sS1_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S2))
        {
            this->_sS2_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S3))
        {
            this->_sS3_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalGuardState::S4))
        {
            this->_sS4_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            HierarchicalGuardCompartment *nextCompartment = this->_nextCompartment_;
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
        void A(int i) {
            unordered_map<string, any> params;
            params["i"] = i;

            
            FrameEvent e("A", params);
            this->_mux_(&e);
        }
        
        void B(int i) {
            unordered_map<string, any> params;
            params["i"] = i;

            
            FrameEvent e("B", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sI_(FrameEvent *e)
    {
        if (e->_message == ">") {
            HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sS_(FrameEvent *e)
    {
        if (e->_message == "A") {
            log_do("S.A");
            if (any_cast<int>(e->_parameters["i"]) < 10) {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S0));
                
                this->_transition_(compartment);
                return;
            } else {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S1));
                
                this->_transition_(compartment);
            }
            return;
        }
        else if (e->_message == "B") {
            log_do("S.B");
            if (any_cast<int>(e->_parameters["i"]) < 10) {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S2));
                
                this->_transition_(compartment);
                return;
            } else {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S3));
                
                this->_transition_(compartment);
            }
            return;
        }
    }
    
    void _sS0_(FrameEvent *e)
    {
        if (e->_message == "A") {
            log_do("S0.A");
            if (any_cast<int>(e->_parameters["i"]) > 0) {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S2));
                
                this->_transition_(compartment);
                return;
            } else {
            }
            
        }
          //  fall through else branch
		else if (e->_message == "B") {
            log_do("S0.B");
            if (any_cast<int>(e->_parameters["i"]) > 0) {
            } else {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S1));
                
                this->_transition_(compartment);
                return;
            }
            
        }
        _sS_(e);
        
    }  //  fall through then branch

    
    void _sS1_(FrameEvent *e)
    {
        if (e->_message == "A") {
            log_do("S1.A");
            if (any_cast<int>(e->_parameters["i"]) > 5) {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S3));
                
                this->_transition_(compartment);
                return;
            } else {
            }
            
        }
        _sS0_(e);
        
    }  //  fall through else branch

    
    void _sS2_(FrameEvent *e)
    {
        if (e->_message == "A") {
            log_do("S2.A");
            if (any_cast<int>(e->_parameters["i"]) > 10) {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S4));
                
                this->_transition_(compartment);
                return;
            } else {
            }
            
        }
          //  fall through then branch
		else if (e->_message == "B") {
            log_do("S2.B");
            if (!(any_cast<int>(e->_parameters["i"]) > 10)) {
            } else {
                HierarchicalGuardCompartment *compartment =  new HierarchicalGuardCompartment(static_cast<int>(HierarchicalGuardState::S4));
                
                this->_transition_(compartment);
                return;
            }
            
        }
        _sS1_(e);
        
    }  //  fall through then branch

    
    void _sS3_(FrameEvent *e)
    {
        if (e->_message == "A") {
            log_do("S3.A");
            if (any_cast<int>(e->_parameters["i"]) > 0) {
                log_do("stop");
                return;
            } else {
                log_do("continue");
            }
            
        }
        else if (e->_message == "B") {
            log_do("S3.B");
            if (any_cast<int>(e->_parameters["i"]) > 0) {
                log_do("continue");
            } else {
                log_do("stop");
                return;
            }
            
        }
        _sS_(e);
        
    }
    
    void _sS4_(FrameEvent *e)
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
    
    void _transition_(HierarchicalGuardCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(HierarchicalGuardCompartment *nextCompartment)
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

class HierarchicalGuardController : public HierarchicalGuard
{
public:
	HierarchicalGuardController() : HierarchicalGuard() {}
};

********************/

