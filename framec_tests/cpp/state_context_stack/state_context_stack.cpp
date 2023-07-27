// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <stack>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class StateContextStackCompartment
{
public:
    int state;
    
    StateContextStackCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;

    StateContextStackCompartment* _deepCopyCompartment(const StateContextStackCompartment* originalCompartment)
    {
        StateContextStackCompartment* newCompartment = new StateContextStackCompartment(originalCompartment->state);
        newCompartment->stateArgs = originalCompartment->stateArgs;
        newCompartment->stateVars = originalCompartment->stateVars;
        newCompartment->enterArgs = originalCompartment->enterArgs;
        newCompartment->exitArgs = originalCompartment->exitArgs;
        return newCompartment;
    }
};

class StateContextStack
{
private:
    StateContextStackCompartment *_compartment_;
    StateContextStackCompartment *_nextCompartment_;
    
    
    
public:
    StateContextStack()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(StateContextStackState::A);
        // Create state stack.
        
        this->_stateStack_ = new stack<StateContextStackCompartment>();
        
        
        _compartment_ = new StateContextStackCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        _compartment_->stateVars["x"] = 0;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class StateContextStackState
    {
        A = 0,
        B = 1,
        C = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(StateContextStackState::A))
        {
            this->_sA_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateContextStackState::B))
        {
            this->_sB_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateContextStackState::C))
        {
            this->_sC_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            StateContextStackCompartment *nextCompartment = this->_nextCompartment_;
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
        
        void inc() {
            FrameEvent e("inc", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        int value() {
            FrameEvent e("value", unordered_map<string, any>());
            this->_mux_(&e);
            return any_cast<int>(*static_cast<any*>(e._return));
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
        else if (e->_message == "inc") {
            this->_compartment_->stateVars["x"] = (any_cast<int>(this->_compartment_->stateVars["x"])) + 1;
            return;
        }
        else if (e->_message == "value") {
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["x"])));
            
            return;
            
        }
        else if (e->_message == "to_a") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::A));
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::B));
            compartment->stateVars["y"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::C));
            compartment->stateVars["z"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
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
        else if (e->_message == "inc") {
            this->_compartment_->stateVars["y"] = (any_cast<int>(this->_compartment_->stateVars["y"])) + 5;
            return;
        }
        else if (e->_message == "value") {
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["y"])));
            
            return;
            
        }
        else if (e->_message == "to_a") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::A));
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::B));
            compartment->stateVars["y"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::C));
            compartment->stateVars["z"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
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
        else if (e->_message == "inc") {
            this->_compartment_->stateVars["z"] = (any_cast<int>(this->_compartment_->stateVars["z"])) + 10;
            return;
        }
        else if (e->_message == "value") {
            e->_return = (void*) new int((any_cast<int>(this->_compartment_->stateVars["z"])));
            
            return;
            
        }
        else if (e->_message == "to_a") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::A));
            compartment->stateVars["x"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_b") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::B));
            compartment->stateVars["y"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_c") {
            StateContextStackCompartment *compartment =  new StateContextStackCompartment(static_cast<int>(StateContextStackState::C));
            compartment->stateVars["z"] = 0;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "push") {
            _stateStack_push_(_compartment_);
            return;
        }
        else if (e->_message == "pop") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "pop_change") {
            StateContextStackCompartment *compartment = this->_stateStack_pop_();
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
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(StateContextStackCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(StateContextStackCompartment *nextCompartment)
    {
        this->_mux_(new FrameEvent("<", _compartment_->exitArgs));
        this->_compartment_ = nextCompartment;
        this->_mux_(new FrameEvent(">", this->_compartment_->enterArgs));
    }
    
    public:
        stack<StateContextStackCompartment> *_stateStack_ = nullptr;
    
    public:
        void _stateStack_push_(StateContextStackCompartment *compartment)
        {
             
            _stateStack_->push(*compartment);
        }
        
        
        StateContextStackCompartment *_stateStack_pop_()
        {
            StateContextStackCompartment *copyCompartment = &_stateStack_->top();
            _stateStack_->pop();
            return copyCompartment;
        }
        
        private: void _changeState_(StateContextStackCompartment* compartment)
        {
            this->_compartment_ = compartment;
        }
        
        public:
        string state_info(){
            return std::to_string(_compartment_->state);
            }
            
    };
    
    /********************

    class StateContextStackController : public StateContextStack
    {
    public:
    	StateContextStackController() : StateContextStack() {}
    };
    
********************/
    
