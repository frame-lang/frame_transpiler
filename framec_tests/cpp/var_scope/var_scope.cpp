// emitted from framec_v0.10.0
// get include files at https://github.com/frame-lang/frame-ancillary-files

#include <iostream>
#include <vector>
#include <any>
#include <string>
#include <unordered_map>
using namespace std;
#include "../FrameLang/FrameLang.h"

//=============== Compartment ==============//

using FuncPtr = void(*)();

class VarScopeCompartment
{
public:
    int state;
    
    VarScopeCompartment(int state)
    {
        this->state = state;
    }
    
    std::unordered_map<std::string, std::any> stateArgs;
    std::unordered_map<std::string, std::any> stateVars;
    std::unordered_map<std::string, std::any> enterArgs;
    std::unordered_map<std::string, std::any> exitArgs;
    FrameEvent *_forwardEvent = nullptr;
};

class VarScope
{
private:
    VarScopeCompartment *_compartment_;
    VarScopeCompartment *_nextCompartment_;
    
    
    
public:
    VarScope()
    {
        // Create and intialize start state compartment.
        _state_ = static_cast<int>(VarScopeState::INIT);
        
        _compartment_ = new VarScopeCompartment(this->_state_);
        _nextCompartment_ = nullptr;
        
        
        // Send system start event
        FrameEvent frame_event(">", std::unordered_map<std::string, std::any>());
        _mux_(&frame_event);
    
    }
    
    // states enum
private:
    enum class VarScopeState
    {
        INIT = 0,
        NN = 1,
        NY = 2,
        YN = 3,
        YY = 4
    };
    
    //====================== Multiplexer ====================//

private:
    void _mux_(FrameEvent *e)
    {
        if(this->_compartment_->state == static_cast<int>(VarScopeState::INIT))
        {
            this->_sInit_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(VarScopeState::NN))
        {
            this->_sNN_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(VarScopeState::NY))
        {
            this->_sNY_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(VarScopeState::YN))
        {
            this->_sYN_(e);
        }
        else if(this->_compartment_->state == static_cast<int>(VarScopeState::YY))
        {
            this->_sYY_(e);
        }
        
        
        if(this->_nextCompartment_ != nullptr)
        {
            VarScopeCompartment *nextCompartment = this->_nextCompartment_;
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
        void to_nn() {
            FrameEvent e("to_nn", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void to_ny() {
            FrameEvent e("to_ny", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void to_yn() {
            FrameEvent e("to_yn", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void to_yy() {
            FrameEvent e("to_yy", unordered_map<string, any>());
            this->_mux_(&e);
        }
        
        void nn(string d) {
            unordered_map<string, any> params;
            params["d"] = d;

            
            FrameEvent e("nn", params);
            this->_mux_(&e);
        }
        
        void ny(string d) {
            unordered_map<string, any> params;
            params["d"] = d;

            
            FrameEvent e("ny", params);
            this->_mux_(&e);
        }
        
        void yn(string d,string x) {
            unordered_map<string, any> params;
            params["d"] = d;

            
            params["x"] = x;

            
            FrameEvent e("yn", params);
            this->_mux_(&e);
        }
        
        void yy(string d,string x) {
            unordered_map<string, any> params;
            params["d"] = d;

            
            params["x"] = x;

            
            FrameEvent e("yy", params);
            this->_mux_(&e);
        }
        
        void sigils(string x) {
            unordered_map<string, any> params;
            params["x"] = x;

            
            FrameEvent e("sigils", params);
            this->_mux_(&e);
        }
        
    
    //===================== Machine Block ===================//

private:
    
    void _sInit_(FrameEvent *e)
    {
        if (e->_message == "to_nn") {
             std::cout << "Transition to NN state." << std::endl;
            VarScopeCompartment *compartment =  new VarScopeCompartment(static_cast<int>(VarScopeState::NN));
            compartment->stateArgs["b"] = std::string("$NN[b]");
            compartment->stateVars["c"] = std::string("$NN.c");
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_ny") {
            VarScopeCompartment *compartment =  new VarScopeCompartment(static_cast<int>(VarScopeState::NY));
             compartment->stateArgs["b"] = std::string("$NY[b]");
            compartment->stateVars["c"] = std::string("$NY.c");
            compartment->stateVars["x"] = std::string("$NY.x");
            
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_yn") {
            VarScopeCompartment *compartment =  new VarScopeCompartment(static_cast<int>(VarScopeState::YN));
            compartment->stateArgs["b"] = std::string("$YN[b]");
            compartment->stateArgs["x"] = std::string("$YN[x]");
            compartment->stateVars["c"] = std::string("$YN.c");
            
            this->_transition_(compartment);
            return;
        }
        else if (e->_message == "to_yy") {
            VarScopeCompartment *compartment =  new VarScopeCompartment(static_cast<int>(VarScopeState::YY));
            compartment->stateArgs["b"] = std::string("$YY[b]");
            compartment->stateArgs["x"] = std::string("$YY[x]");
            compartment->stateVars["c"] = std::string("$YY.c");
            compartment->stateVars["x"] = std::string("$YY.x");
            
            this->_transition_(compartment);
            return;
        }
    }
    
    void _sNN_(FrameEvent *e)
    {
        if (e->_message == "nn") {
            string et  = "|nn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(this->x);
            return;
        }
        else if (e->_message == "ny") {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "yn") {
            string et  = "|yn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(any_cast<string>(e->_parameters["x"]));
            return;
        }
        else if (e->_message == "yy") {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "sigils") {
            log_do(this->x);
            return;
        }
    }  //  var x:string = "|sigils|.x"
  //  log(||[x])
  //  log(||.x)

    
    void _sNY_(FrameEvent *e)
    {
        if (e->_message == "nn") {
            string et  = "|nn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do((any_cast<string>(this->_compartment_->stateVars["x"])));
            return;
        }
        else if (e->_message == "ny") {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "yn") {
            string et  = "|yn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(any_cast<string>(e->_parameters["x"]));
            return;
        }
        else if (e->_message == "yy") {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "sigils") {
            log_do(this->x);
            return;
        }
    }  //  var x:string = "|sigils|.x"
  //  log($.x)
  //  log(||[x])
  //  log(||.x)

    
    void _sYN_(FrameEvent *e)
    {
        if (e->_message == "nn") {
            string et  = "|nn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(any_cast<string>(this->_compartment_->stateArgs["x"]));
            return;
        }
        else if (e->_message == "ny") {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "yn") {
            string et  = "|yn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(any_cast<string>(e->_parameters["x"]));
            return;
        }
        else if (e->_message == "yy") {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "sigils") {
            log_do(this->x);
            return;
        }
    }  //  var x:string = "|sigils|.x"
  //  log($[x])
  //  log(||[x])
  //  log(||.x)

    
    void _sYY_(FrameEvent *e)
    {
        if (e->_message == "nn") {
            string et  = "|nn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do((any_cast<string>(this->_compartment_->stateVars["x"])));
            return;
        }
        else if (e->_message == "ny") {
            string et  = "|ny|.e";
            string x  = "|ny|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "yn") {
            string et  = "|yn|.e";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(any_cast<string>(e->_parameters["x"]));
            return;
        }
        else if (e->_message == "yy") {
            string et  = "|yy|.e";
            string x  = "|yy|.x";
            log_do(this->a);
            log_do(any_cast<string>(this->_compartment_->stateArgs["b"]));
            log_do((any_cast<string>(this->_compartment_->stateVars["c"])));
            log_do(any_cast<string>(e->_parameters["d"]));
            log_do(et);
            log_do(x);
            return;
        }
        else if (e->_message == "sigils") {
            log_do(this->x);
            return;
        }
    }

//===================== Actions Block ===================//
    
    
    
public:
    void log_do(string s)
    {
         tape.push_back(s);
    }
    
    // Unimplemented Actions
    public:
    
    //===================== Domain Block ===================//
    
    
    public:
    string a  = "#.a";
    public:
    string x  = "#.x";
    public:
    std::vector<std::string> tape ;
    
    
    //=============== Machinery and Mechanisms ==============//

private:
    int _state_;
    
    void _transition_(VarScopeCompartment *compartment)
    {
        _nextCompartment_ = compartment;
    }
    
    void _doTransition_(VarScopeCompartment *nextCompartment)
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

class VarScopeController : public VarScope
{
public:
	VarScopeController() : VarScope() {}
};

********************/

