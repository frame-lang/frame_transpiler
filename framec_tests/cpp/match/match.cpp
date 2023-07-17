// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <unordered_map>
#include <stdexcept>
#include <iostream>
#include <vector>
#include <any>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

class MatchCompartment
{
public:
    int state;
    
    MatchCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class Match
{
private:
    MatchCompartment *_compartment_;
    MatchCompartment *_nextCompartment_;
    
    
    
public:
    Match()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(MatchState::INIT);
        
        _compartment_ = new MatchCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class MatchState
    {
        INIT = 0,
        EMPTYMATCH = 1,
        SIMPLEMATCH = 2,
        MULTIMATCH = 3,
        NESTEDMATCH = 4,
        CHILDMATCH = 5,
        FINAL = 6
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(MatchState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::EMPTYMATCH))
        {
            this->_sEmptyMatch_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::SIMPLEMATCH))
        {
            this->_sSimpleMatch_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::MULTIMATCH))
        {
            this->_sMultiMatch_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::NESTEDMATCH))
        {
            this->_sNestedMatch_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::CHILDMATCH))
        {
            this->_sChildMatch_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(MatchState::FINAL))
        {
            this->_sFinal_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            MatchCompartment *nextCompartment = this->_nextCompartment_;
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
        void Empty() {
            FrameEvent e("Empty", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Simple() {
            FrameEvent e("Simple", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Multi() {
            FrameEvent e("Multi", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Nested() {
            FrameEvent e("Nested", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void Child() {
            FrameEvent e("Child", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void OnInt(int i) {
            unordered_map<string, any> params;
            params["i"] = i;

            
            FrameEvent e("OnInt", params);
            this->_mux_(&e);
        }
        
        void OnString(String s) {
            unordered_map<string, any> params;
            params["s"] = s;

            
            FrameEvent e("OnString", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "Empty") {
            MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::EMPTYMATCH));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Simple") {
            MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::SIMPLEMATCH));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Multi") {
            MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::MULTIMATCH));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Nested") {
            MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::NESTEDMATCH));
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "Child") {
            MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::CHILDMATCH));
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sEmptyMatch_(FrameEvent *e)
    {
        if (e->_message == "OnString") {
            if (any_cast<String>(e->_parameters["s"]) == "") || (any_cast<String>(e->_parameters["s"]) == "foo") {
                log_do("empty");
            } else {
                log_do("?");
            }
            return;
        }
    }  //  TODO: matching only the empty string is broken

    
    void _sSimpleMatch_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) == 0)) {
                log_do("0");
            } else if (any_cast<int>(e->_parameters["i"]) == 42)) {
                log_do("42");
            } else if (any_cast<int>(e->_parameters["i"]) == 42)) {
                log_do("!!!");
            } else if (any_cast<int>(e->_parameters["i"]) == -200)) {
                log_do("-200");
            } else {
                log_do("?");
            }
            return;
        }
        else if (e->_message == "OnString") {
            if (any_cast<String>(e->_parameters["s"]) == "hello") {
                log_do("hello");
            } else if (any_cast<String>(e->_parameters["s"]) == "hello") {
                log_do("!!!");
            } else if (any_cast<String>(e->_parameters["s"]) == "goodbye") {
                log_do("goodbye");
            } else if (any_cast<String>(e->_parameters["s"]) == "Testing 1, 2, 3...") {
                log_do("testing");
            } else if (any_cast<String>(e->_parameters["s"]) == "$10!") {
                log_do("money");
            } else {
                log_do("?");
            }
            return;
        }
    }
    
    void _sMultiMatch_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) == 3) || (any_cast<int>(e->_parameters["i"]) == -7)) {
                log_do("3|-7");
            } else if (any_cast<int>(e->_parameters["i"]) == -4) || (any_cast<int>(e->_parameters["i"]) == 5) || (any_cast<int>(e->_parameters["i"]) == 6)) {
                log_do("-4|5|6");
            } else {
                log_do("?");
            }
            return;
        }
        else if (e->_message == "OnString") {
            if (any_cast<String>(e->_parameters["s"]) == "$10") || (any_cast<String>(e->_parameters["s"]) == "12.5%") || (any_cast<String>(e->_parameters["s"]) == "@#*!") {
                log_do("symbols");
            } else if (any_cast<String>(e->_parameters["s"]) == " ") || (any_cast<String>(e->_parameters["s"]) == "  ") || (any_cast<String>(e->_parameters["s"]) == "\t") || (any_cast<String>(e->_parameters["s"]) == "\n") {
                log_do("whitespace");
            } else {
                log_do("?");
            }
            return;
        }
    }
    
    void _sNestedMatch_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) > 0) {
                if (any_cast<int>(e->_parameters["i"]) == 1) || (any_cast<int>(e->_parameters["i"]) == 2) || (any_cast<int>(e->_parameters["i"]) == 3)) {
                    log_do("1-3");
                    if (any_cast<int>(e->_parameters["i"]) == 1)) {
                        log_do("1");
                    } else if (any_cast<int>(e->_parameters["i"]) == 2)) {
                        log_do("2");
                    } else {
                        log_do("3");
                    }
                } else if (any_cast<int>(e->_parameters["i"]) == 4) || (any_cast<int>(e->_parameters["i"]) == 5)) {
                    log_do("4-5");
                    if (any_cast<int>(e->_parameters["i"]) == 4) {
                        log_do("4");
                    } else {
                        log_do("5");
                    }
                } else {
                    log_do("too big");
                }
            } else {
                log_do("too small");
            }
            return;
        }
        else if (e->_message == "OnString") {
            if (any_cast<String>(e->_parameters["s"]) == "hello") || (any_cast<String>(e->_parameters["s"]) == "hola") || (any_cast<String>(e->_parameters["s"]) == "bonjour") {
                log_do("greeting");
                if (any_cast<String>(e->_parameters["s"]) == "hello") {
                    log_do("English");
                } else if (any_cast<String>(e->_parameters["s"]) == "hola") {
                    log_do("Spanish");
                } else {
                    log_do("French");
                }
            } else if (any_cast<String>(e->_parameters["s"]) == "goodbye") || (any_cast<String>(e->_parameters["s"]) == "adios") || (any_cast<String>(e->_parameters["s"]) == "au revoir") {
                log_do("farewell");
                if (any_cast<String>(e->_parameters["s"]) == "goodbye") {
                    log_do("English");
                } else if (any_cast<String>(e->_parameters["s"]) == "adios") {
                    log_do("Spanish");
                } else {
                    log_do("French");
                }
            } else {
                log_do("?");
            }
            return;
        }
    }
    
    void _sChildMatch_(FrameEvent *e)
    {
        if (e->_message == "OnInt") {
            if (any_cast<int>(e->_parameters["i"]) == 0)) {
                MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::FINAL));
                
                this->_transition_(compartment);
            } else if (any_cast<int>(e->_parameters["i"]) == 3)) {
                log_do("3");
            } else if (any_cast<int>(e->_parameters["i"]) == 4)) {
                log_do("4");
                return;
            } else if (any_cast<int>(e->_parameters["i"]) == 42)) {
                log_do("42 in child");
            } else if (any_cast<int>(e->_parameters["i"]) == 5)) {
                log_do("5");
                MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::FINAL));
                
                this->_transition_(compartment);
                return;
            } else {
                log_do("no match in child");
            }
            
        }
        else if (e->_message == "OnString") {
            if (any_cast<String>(e->_parameters["s"]) == "hello") {
                log_do("hello in child");
            } else if (any_cast<String>(e->_parameters["s"]) == "goodbye") {
                MatchCompartment *compartment =  new MatchCompartment(static_cast<int>(MatchState::FINAL));
                
                this->_transition_(compartment);
            } else if (any_cast<String>(e->_parameters["s"]) == "Testing 1, 2, 3...") {
                log_do("testing in child");
                return;
            } else {
                log_do("no match in child");
            }
            
        }
        _sSimpleMatch_(e);
        
    }
    
    void _sFinal_(FrameEvent *e)
    {
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
    
    void _transition_(MatchCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(MatchCompartment *nextCompartment)
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

class MatchController : public Match
{
public:
	MatchController() : Match() {}
};

********************/

