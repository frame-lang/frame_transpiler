// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class StateParamsCompartment
{
public:
    int state;
    
    StateParamsCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class StateParams
{
private:
    StateParamsCompartment *_compartment_;
    StateParamsCompartment *_nextCompartment_;
    
    
    
public:
    StateParams()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(StateParamsState::INIT);
        
        _compartment_ = new StateParamsCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class StateParamsState
    {
        INIT = 0,
        SPLIT = 1,
        MERGE = 2
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(StateParamsState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateParamsState::SPLIT))
        {
            this->_sSplit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(StateParamsState::MERGE))
        {
            this->_sMerge_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            StateParamsCompartment *nextCompartment = this->_nextCompartment_;
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
        
        void Prev() {
            FrameEvent e("Prev", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Log() {
            FrameEvent e("Log", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "Next") {
            StateParamsCompartment *compartment =  new StateParamsCompartment(static_cast<int>(StateParamsState::SPLIT));
            compartment->stateArgs["val"] = 1;
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sSplit_(FrameEvent *e)
    {
        if (e->_message == "Next") {
            StateParamsCompartment *compartment =  new StateParamsCompartment(static_cast<int>(StateParamsState::MERGE));
            compartment->stateArgs["left"] = any_cast<int>(this->_compartment_->stateArgs["val"]);
            compartment->stateArgs["right"] = any_cast<int>(this->_compartment_->stateArgs["val"]) + 1;
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Prev") {
            StateParamsCompartment *compartment =  new StateParamsCompartment(static_cast<int>(StateParamsState::MERGE));
            compartment->stateArgs["left"] = any_cast<int>(this->_compartment_->stateArgs["val"]) + 1;
            compartment->stateArgs["right"] = any_cast<int>(this->_compartment_->stateArgs["val"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Log") {
            got_param_do("val",any_cast<int>(this->_compartment_->stateArgs["val"]));
            return;
        }
    }
    
    void _sMerge_(FrameEvent *e)
    {
        if (e->_message == "Next") {
            StateParamsCompartment *compartment =  new StateParamsCompartment(static_cast<int>(StateParamsState::SPLIT));
            compartment->stateArgs["val"] = any_cast<int>(this->_compartment_->stateArgs["left"]) + any_cast<int>(this->_compartment_->stateArgs["right"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Prev") {
            StateParamsCompartment *compartment =  new StateParamsCompartment(static_cast<int>(StateParamsState::SPLIT));
            compartment->stateArgs["val"] = any_cast<int>(this->_compartment_->stateArgs["left"]) * any_cast<int>(this->_compartment_->stateArgs["right"]);
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Log") {
            got_param_do("left",any_cast<int>(this->_compartment_->stateArgs["left"]));
            got_param_do("right",any_cast<int>(this->_compartment_->stateArgs["right"]));
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void got_param_do(const std::string& name, int val)
    {
        param_log.push_back(name + "=" + std::to_string(val));
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    std::vector<std::string> param_log ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(StateParamsCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(StateParamsCompartment *nextCompartment)
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

class StateParamsController : public StateParams
{
public:
	StateParamsController() : StateParams() {}
};

********************/

