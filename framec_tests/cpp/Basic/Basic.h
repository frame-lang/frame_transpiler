#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>
#include "../FrameLang/FrameEvent.h"
using namespace std;

//=============== Compartment ==============//

class BasicCompartment
{
public:
    int state;
    
    BasicCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class Basic
{
private:
    BasicCompartment *_compartment_;
    BasicCompartment *_nextCompartment_;
    
    
    
public:
    Basic()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(BasicState::S0);
        
        _compartment_ = new BasicCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class BasicState
    {
        S0 = 0,
        S1 = 1
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(BasicState::S0))
        {
            this->_sS0_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(BasicState::S1))
        {
            this->_sS1_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            BasicCompartment *nextCompartment = this->_nextCompartment_;
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
        
    
    //===================== Machine Block ===================//

private:
    
    void _sS0_(FrameEvent *e)
    {
        if (e->_message == ">") {
            entered_do("S0");
            return;
        }
        else if (e->_message == "<") {
            left_do("S0");
            return;
        }
        else if (e->_message == "A") {
            // ooh
            BasicCompartment *compartment =  new BasicCompartment(static_cast<int>(BasicState::S1));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sS1_(FrameEvent *e)
    {
        if (e->_message == ">") {
            entered_do("S1");
            return;
        }
        else if (e->_message == "<") {
            left_do("S1");
            return;
        }
        else if (e->_message == "B") {
            // aah
            BasicCompartment *compartment =  new BasicCompartment(static_cast<int>(BasicState::S0));
            
            this->_transition_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void entered_do(string msg)
    {
        entry_log.push_back(msg);
    }
    
    
public:
    void left_do(string msg)
    {
        exit_log.push_back(msg);
    }
    
    // Unimplemented Actions
    
    
    //===================== Domain Block ===================//
    
    

    vector<string> entry_log  = {};

    vector<string> exit_log  = {};
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(BasicCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(BasicCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    public:
    string state_info(){
        return any_cast<string>(_compartment_->state);
        }
        
};

/********************

class BasicController : public Basic
{
public:
	BasicController() : Basic() {}
};

********************/
