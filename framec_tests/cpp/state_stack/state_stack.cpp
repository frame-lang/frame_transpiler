// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <vector>
#include <any>
#include <stack>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class StateStackCompartment
{
public:
    int state;
    
    StateStackCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class StateStack
{
private:
    StateStackCompartment *_compartment_;
    StateStackCompartment *_nextCompartment_;
    
    
    
public:
    StateStack()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(StateStackState::A);
        // Create state stack.
        
        this->_stateStack_ = new stack<StateStackCompartment>();
        
        
        _compartment_ = new StateStackCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class StateStackState
    {
        A = 0,
        B = 1,
        C = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(StateStackState::A))
        {
            this->_sA_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateStackState::B))
        {
            this->_sB_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateStackState::C))
        {
            this->_sC_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            StateStackCompartment *nextCompartment = this->_nextCompartment_;
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
        void to_a() {
            FrameEvent e("to_a", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void to_b() {
            FrameEvent e("to_b", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void to_c() {
            FrameEvent e("to_c", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void push() {
            FrameEvent e("push", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void pop() {
            FrameEvent e("pop", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void pop_change() {
            FrameEvent e("pop_change", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sA_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("A:>");
            return;
        }
        else if (e->_message == "<") {
            log_do("A:<");
            return;
        }
        else if (e->_message == "to_a") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::A));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::B));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::C));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sB_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("B:>");
            return;
        }
        else if (e->_message == "<") {
            log_do("B:<");
            return;
        }
        else if (e->_message == "to_a") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::A));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::B));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::C));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_changeState_(compartment);
            return;
        }
    }
    
    void _sC_(FrameEvent *e)
    {
        if (e->_message == ">") {
            log_do("C:>");
            return;
        }
        else if (e->_message == "<") {
            log_do("C:<");
            return;
        }
        else if (e->_message == "to_a") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::A));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::B));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateStackCompartment *compartment =  new StateStackCompartment(static_cast<int>(StateStackState::C));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateStackCompartment *compartment = this->_stateStack_pop_();
            this->_changeState_(compartment);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(const std::string& msg)
    {
        tape.push_back(msg);
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(StateStackCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(StateStackCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    private:
    stack<StateStackCompartment>* _stateStack_ = nullptr;
    
    public:
    void _stateStack_push_(StateStackCompartment* compartment)
    {
        _stateStack_->push(*compartment);
    }
    
    StateStackCompartment* _stateStack_pop_()
    {
        StateStackCompartment* copyCompartment = &_stateStack_->top();
        _stateStack_->pop();
        return copyCompartment;
    }
    
    private: void _changeState_(StateStackCompartment* compartment)
    {
        this->_compartment_ = compartment;
    }
    
    public:
    string state_info(){
        return std::to_string(_compartment_->state);
        }
        
};

/********************

class StateStackController : public StateStack
{
public:
	StateStackController() : StateStack() {}
};

********************/

