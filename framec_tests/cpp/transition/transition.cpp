// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class TransitionSmCompartment
{
public:
    int state;
    
    TransitionSmCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class TransitionSm
{
private:
    TransitionSmCompartment *_compartment_;
    TransitionSmCompartment *_nextCompartment_;
    
    
    
public:
    TransitionSm()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(TransitionSmState::S0);
        
        _compartment_ = new TransitionSmCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class TransitionSmState
    {
        S0 = 0,
        S1 = 1,
        S2 = 2,
        S3 = 3,
        S4 = 4
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(TransitionSmState::S0))
        {
            this->_sS0_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitionSmState::S1))
        {
            this->_sS1_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitionSmState::S2))
        {
            this->_sS2_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitionSmState::S3))
        {
            this->_sS3_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(TransitionSmState::S4))
        {
            this->_sS4_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            TransitionSmCompartment *nextCompartment = this->_nextCompartment_;
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
        void transit() {
            FrameEvent e("transit", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void change() {
            FrameEvent e("change", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sS0_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S0");
            return;
        }
        else if (e->_message == "<") {
            exit_do("S0");
            return;
        }
        else if (e->_message == "transit") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S1));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "change") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S1));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sS1_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S1");
            return;
        }
        else if (e->_message == "<") {
            exit_do("S1");
            return;
        }
        else if (e->_message == "transit") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S2));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "change") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S2));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sS2_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S2");
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S3));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "<") {
            exit_do("S2");
            return;
        }
        else if (e->_message == "transit") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S3));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "change") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S3));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sS3_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S3");
            return;
        }
        else if (e->_message == "<") {
            exit_do("S3");
            return;
        }
        else if (e->_message == "transit") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S4));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "change") {
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S4));
            
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sS4_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S4");
            TransitionSmCompartment *compartment =  new TransitionSmCompartment(static_cast<int>(TransitionSmState::S0));
            
            this->_changeState_(compartment);
            return;
        }
        else if (e->_message == "<") {
            exit_do("S4");
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void enter_do(const std::string& state)
    {
        enters.push_back(state);
    }
    
    
public:
    void exit_do(const std::string& state)
    {
        exits.push_back(state);
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> enters ;
    public:
    std::vector<std::string> exits ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(TransitionSmCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(TransitionSmCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    private: void _changeState_(TransitionSmCompartment* compartment)
    {
        this->_compartment_ = compartment;
    }
    
    public:
    string state_info(){
        return std::to_string(_compartment_->state);
        }
        
};

/********************

class TransitionSmController : public TransitionSm
{
public:
	TransitionSmController() : TransitionSm() {}
};

********************/

