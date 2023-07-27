// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <unordered_map>
#include <vector>
#include <any>
#include <string>
#include <iostream>

#include "../FrameLang/FrameLang.h"
using namespace std;

//=============== Compartment ==============//

class TransitParamsCompartment
{
public:
    int state;
    
    TransitParamsCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class TransitParams
{
private:
    TransitParamsCompartment *_compartment_;
    TransitParamsCompartment *_nextCompartment_;
    
    
    
public:
    TransitParams()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(TransitParamsState::INIT);
        
        _compartment_ = new TransitParamsCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class TransitParamsState
    {
        INIT = 0,
        A = 1,
        B = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(TransitParamsState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitParamsState::A))
        {
            this->_sA_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitParamsState::B))
        {
            this->_sB_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            TransitParamsCompartment *nextCompartment = this->_nextCompartment_;
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
        void Next() {
            FrameEvent e("Next", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Change() {
            FrameEvent e("Change", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "Next") {
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::A));
            compartment->enterArgs["msg"] = std::string("hi A");
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Change") {
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::A));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sA_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do(any_cast<string>(e->_parameters["msg"]));
            return;
        }
        else if (e->_message == "<") {
            log_do("bye A");
            return;
        }
        else if (e->_message == "Next") {
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::B));
            compartment->enterArgs["msg"] = std::string("hi B");
            compartment->enterArgs["val"] = 42;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Change") {
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::B));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sB_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do(any_cast<string>(e->_parameters["msg"]));
            log_do(to_string(e->_parameters["val"]));
            return;
        }
        else if (e->_message == "<") {
            log_do(to_string(e->_parameters["val"]));
            log_do(any_cast<string>(e->_parameters["msg"]));
            return;
        }
        else if (e->_message == "Next") {
            this->_compartment_->exitArgs["val"] = true;
            this->_compartment_->exitArgs["msg"] = std::string("bye B");
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::A));
            compartment->enterArgs["msg"] = std::string("hi again A");
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Change") {
            TransitParamsCompartment *compartment =  new TransitParamsCompartment(static_cast<int>(TransitParamsState::A));
            
            this->_changeState_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(const std::string& msg)
    {
        tape.push_back(msg);
        for (const auto& msg : tape)
        {
            std::cout << msg << std::endl;
        }
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(TransitParamsCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(TransitParamsCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    private: void _changeState_(TransitParamsCompartment* compartment)
    {
        this->_compartment_ = compartment;
    }
    
    public:
    string state_info(){
        return std::to_string(_compartment_->state);
        }
        
};

/********************

class TransitParamsController : public TransitParams
{
public:
	TransitParamsController() : TransitParams() {}
};

********************/

