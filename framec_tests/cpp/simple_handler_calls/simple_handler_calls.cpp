// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <unordered_map>
#include <vector>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class SimpleHandlerCallsCompartment
{
public:
    int state;
    
    SimpleHandlerCallsCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class SimpleHandlerCalls
{
private:
    SimpleHandlerCallsCompartment *_compartment_;
    SimpleHandlerCallsCompartment *_nextCompartment_;
    
    
    
public:
    SimpleHandlerCalls()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(SimpleHandlerCallsState::INIT);
        
        _compartment_ = new SimpleHandlerCallsCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class SimpleHandlerCallsState
    {
        INIT = 0,
        A = 1,
        B = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(SimpleHandlerCallsState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(SimpleHandlerCallsState::A))
        {
            this->_sA_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(SimpleHandlerCallsState::B))
        {
            this->_sB_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            SimpleHandlerCallsCompartment *nextCompartment = this->_nextCompartment_;
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
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "A") {
            SimpleHandlerCallsCompartment *compartment =  new SimpleHandlerCallsCompartment(static_cast<int>(SimpleHandlerCallsState::A));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "B") {
            SimpleHandlerCallsCompartment *compartment =  new SimpleHandlerCallsCompartment(static_cast<int>(SimpleHandlerCallsState::B));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "C") {
            A();
            return;
            return;
        }
        else if (e->_message == "D") {
            B();
            return;
            SimpleHandlerCallsCompartment *compartment =  new SimpleHandlerCallsCompartment(static_cast<int>(SimpleHandlerCallsState::A));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "E") {
            D();
            return;
            C();
            return;
            return;
        }
    }
    
    void _sA_(FrameEvent *e)
    {
    }
    
    void _sB_(FrameEvent *e)
    {
    }

//===================== Actions Block ===================//
    
    
    // Unimplemented Actions
    
    //===================== Domain Block ===================//
    
    
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(SimpleHandlerCallsCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(SimpleHandlerCallsCompartment *nextCompartment)
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

class SimpleHandlerCallsController : public SimpleHandlerCalls
{
public:
	SimpleHandlerCallsController() : SimpleHandlerCalls() {}
};

********************/

