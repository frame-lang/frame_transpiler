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

class HierarchicalCompartment
{
public:
    int state;
    
    HierarchicalCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class Hierarchical
{
private:
    HierarchicalCompartment *_compartment_;
    HierarchicalCompartment *_nextCompartment_;
    
    
    
public:
    Hierarchical()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(HierarchicalState::I);
        
        _compartment_ = new HierarchicalCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class HierarchicalState
    {
        I = 0,
        S = 1,
        S0 = 2,
        S1 = 3,
        S2 = 4,
        S3 = 5,
        T = 6
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(HierarchicalState::I))
        {
            this->_sI_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::S))
        {
            this->_sS_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::S0))
        {
            this->_sS0_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::S1))
        {
            this->_sS1_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::S2))
        {
            this->_sS2_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::S3))
        {
            this->_sS3_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(HierarchicalState::T))
        {
            this->_sT_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            HierarchicalCompartment *nextCompartment = this->_nextCompartment_;
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
        
    
    //===================== Machine Block ===================//

private:
    
    void _sI_(FrameEvent *e)
    {
        if (e->_message == ">") {
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sS_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S");
            return;
        }
        else if (e->_message == "<") {
            exit_do("S");
            return;
        }
        else if (e->_message == "A") {
            log_do("S.A");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S0));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "B") {
            log_do("S.B");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S1));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sS0_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S0");
            
        }
        else if (e->_message == "<") {
            exit_do("S0");
            
        }
          //  override parent handler
		else if (e->_message == "A") {
            log_do("S0.A");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::T));
            
            this->_transition_(compartment);
            return;
        }
          //  do this, then parent handler
		else if (e->_message == "B") {
            log_do("S0.B");
            
        }
          //  extend parent handler
		else if (e->_message == "C") {
            log_do("S0.C");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S2));
            
            this->_transition_(compartment);
            return;
        }
        _sS_(e);
        
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
          //  defer to parent for A
		  //  do this, then parent, which transitions here
		else if (e->_message == "B") {
            log_do("S1.B");
            
        }
          //  propagate message not handled by parent
		else if (e->_message == "C") {
            log_do("S1.C");
            
        }
        _sS_(e);
        
    }
    
    void _sS2_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S2");
            
        }
        else if (e->_message == "<") {
            exit_do("S2");
            
        }
          //  will propagate to S0 and S
		else if (e->_message == "B") {
            log_do("S2.B");
            
        }
        else if (e->_message == "C") {
            log_do("S2.C");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::T));
            
            this->_transition_(compartment);
            return;
        }
        _sS0_(e);
        
    }  //  continue after transition (should be ignored)

    
    void _sS3_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("S3");
            
        }
        else if (e->_message == "<") {
            exit_do("S3");
            
        }
          //  defer to grandparent for A
		  //  override and move to sibling
		else if (e->_message == "B") {
            log_do("S3.B");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S2));
            
            this->_transition_(compartment);
            return;
        }
        _sS1_(e);
        
    }
    
    void _sT_(FrameEvent *e)
    {
        if (e->_message == ">") {
            enter_do("T");
            return;
        }
        else if (e->_message == "<") {
            exit_do("T");
            return;
        }
        else if (e->_message == "A") {
            log_do("T.A");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "B") {
            log_do("T.B");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S2));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "C") {
            log_do("T.C");
            HierarchicalCompartment *compartment =  new HierarchicalCompartment(static_cast<int>(HierarchicalState::S3));
            
            this->_transition_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void enter_do(const std::string& msg)
    {
         enters.push_back(msg);
    }
    
    
public:
    void exit_do(const std::string& msg)
    {
         exits.push_back(msg);
    }
    
    
public:
    void log_do(const std::string& msg)
    {
        tape.push_back(msg);
    }
    
    // Unimplemented Actions
    
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> enters ;
    public:
    std::vector<std::string> exits ;
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(HierarchicalCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(HierarchicalCompartment *nextCompartment)
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

class HierarchicalController : public Hierarchical
{
public:
	HierarchicalController() : Hierarchical() {}
};

********************/

