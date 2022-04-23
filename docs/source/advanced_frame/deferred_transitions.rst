Deferred Transitions
====================

Transitions in Frame were developed as the mechanism for providing enter and
exit events to state functions.

.. code-block::

    #BasicTransition

    -interface-

    doTransition

    -machine-

    $S0
        |doTransition| -> $S1 ^
        |<| print("Leaving $S0") ^

    $S1
        |>| print("Entering $S1") ^

  	##

This spec generates the following code:

.. code-block::

    public partial class BasicTransition {
        public BasicTransition() {

            _state_ = _sS0_;
        }

        //===================== Interface Block ===================//

        public void doTransition() {
            FrameEvent e = new FrameEvent("doTransition",null);
            _state_(e);
        }


        //===================== Machine Block ===================//

        private void _sS0_(FrameEvent e) {
            if (e._message.Equals("doTransition")) {
                _transition_(_sS1_);
                return;
            }
            else if (e._message.Equals("<")) {
                print("Leaving $S0");
                return;
            }
        }

        private void _sS1_(FrameEvent e) {
            if (e._message.Equals(">")) {
                print("Entering $S1");
                return;
            }
        }

        //=============== Machinery and Mechanisms ==============//

        private delegate void FrameState(FrameEvent e);
        private FrameState _state_;

        private void _transition_(FrameState newState) {
            FrameEvent exitEvent = new FrameEvent("<",null);
            _state_(exitEvent);
            _state_ = newState;
            FrameEvent enterEvent = new FrameEvent(">",null);
            _state_(enterEvent);
        }

    }

One aspect to this architecture is that the call stack grows with
each call to a transition. Lets inspect the call stack
when a client makes a call to the `doTransition()` interface. By the time we
are at the line `print("Entering $S1")`, the call the stack looks like this:

_sS1_
_transition_
_sS0_
doTransition

Now let us add a few more transitions inside of `|>|` handlers:

.. code-block::

    #BasicTransition

    -interface-

    doTransition

    -machine-

    $S0
        |doTransition| -> $S1 ^
        |<| print("Leaving $S0") ^

    $S1
        |>|
        	print("Entering $S1")
            -> $S2 ^

    $S2
        |>|
        	print("Entering $S2")
            -> $S3 ^

    $S3
        |>|
        	print("Entering $S3")  ^

  	##

Now our state stack will look like this by the time we are in `$S3`:

_sS3_
_transition_
_sS2_
_transition_
_sS1_
_transition_
_sS0_
doTransition

So we can start to see a problem with this implementation of transitions.
However this usually is not a problem in reactive systems as typically a
client will call the interface and usually only a single transition will
happen at most.

Where this situation becomes a problem is in a couple of cases. One case is
in trying to solve iterative problems with state machines. For instance, here
is a simple count down machine that does all its work in the enter events:

.. code-block::

    #Countdown $[i:int] --- pass the number of times to loop into the system

      -machine-

      $Test[i:int]                  --- start state initialized by system param
        |>|
            print(itoa(i))          --- print current value of i
            i <= 0 ? -> $Stop ^ ::  --- if i == 0 then transition to $Stop
            -> (i) $S1 ^            --- otherwise pass i as an enter event param
                                    --- to $Decrement

      $Decrement
        |>| [i:int]
            i = i - 1               --- decrement i
            -> $Test(i) ^           --- loop back to

      $Stop
        |>| print("done") ^

    ##

Here we can see that the machine has two states and no external interface.
Instead `$Test[i:int]` is initialized by the `#Countdown $[i:int]`
system parameter and the machine loops between `$Test` and `$Decrement`.
With every loop the call stack will grow by three stack frames. If `i`
is a large number, this could easily crash the process.

Deferred transitions are the solution to this problem.

Deferred Transition Mechanism
-----------------------------

A deferred transition means, at a high level, that the transition does not
actually happen when `_transition()` is called. Instead, a multistep process
is initiated by first caching a reference to the
next compartment:

.. code-block::

    // _transition_ call
    m._transition_(compartment)  --- deferred transition call
    return                       --- mandatory return

    ...

    func (m *countdownStruct) _transition_(compartment *CountdownCompartment) {
        m._nextCompartment_ = compartment
    }

    func (m *countdownStruct) _do_transition_(nextCompartment *CountdownCompartment) {
        m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
        m._compartment_ = nextCompartment
        m._mux_(&framelang.FrameEvent{Msg: ">", Params: m._compartment_.EnterArgs, Ret: nil})
    }

As we can see, the transition takes place in two steps:

#. _transition_()    - cache next compartment
#. _do_transition_() - perform transition

The question is - where does `_do_transition_()` get called. The answer is in
the last block in the `_mux_()`:

.. code-block::

    //====================== Multiplexer ====================//

    func (m *countdownStruct) _mux_(e *framelang.FrameEvent) {
        switch m._compartment_.State {
        case CountdownState_Test:
            m._CountdownState_Test_(e)
        case CountdownState_Decrement:
            m._CountdownState_Decrement_(e)
        case CountdownState_Stop:
            m._CountdownState_Stop_(e)
        }

        if m._nextCompartment_ != nil {
            // Note! This block is simplified to highlight the mechanisms for
            // deferred transitions.
            m._do_transition_(nextCompartment)
        }
    }

Above we can see that the `_mux_()` has two blocks. The first is a switch
statement that routes the Frame Event to the current state for processing. The
second block determines if a transition has occurred by testing the
`m._nextCompartment_` runtime variable. If so, it executes the transition.

Using this mechanism, transitions that happen inside an enter event handler will
not result in recursive additions to the call stack as the transition always
actually occurs in the context of the mux.
