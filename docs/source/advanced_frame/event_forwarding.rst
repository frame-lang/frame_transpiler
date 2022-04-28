Event Forwarding
================

As has been previously discussed, the function of software is to map events
to behavior based on the logical state of the system. Traditionally structured software systems can loosely be characterized as
"event-oriented" in the sense that most of the software is organized
in event handlers, of one form or another.

(event,state) -> behavior

Unlike state functions, event handlers have no formal internal structure
around state. This lack of structure actually makes it easy to slip between
one state and another by performing arbitrary boolean tests and updating
data values in an ad hoc manner. This lack of structure is liberating when
systems are simple, but becomes the definition of "spaghetti code" when they
become complex.

State functions rigidly isolate one state's behavior from other
state's behavior. A side effect of this segregation of logical state, however,
is that the system can't easily slip from one
state to another during a single call to an event handler. This is the goal,
but also creates some curious situations that need to be understood as a
byproduct of this segregation.

One of the most common situations occurs when an event is sent with the system
in one state but needs to be handled in another. An example will illustrate
the point.

Say you are at the mailbox when a letter
arrives, but you don't want to read it right then as a nosey neighbor
might read it. Instead you wait until you are on your couch in the privacy
of your home before you read it.

.. code-block::

    #YouGotMail

    -interface-

    newMail [letter:Letter]

    -machine-

    $AtMailBox                    --- start state at the mailbox
        |newMail| [letter:Letter] --- new mail arrives
            newLetter = letter    --- cache off letter reference in domain variable
            -> $OnCouch ^         --- transition to couch

    $OnCouch
        |>|                       --- once at couch
            #.newLetter != nil ?  --- see if we have a letter. we won't always
                readLetter()      --- read letter if we do
                newLetter = nil   --- throw away letter
            :: ^
        |newMail| [letter:Letter] --- if we are already on the couch
            readLetter() ^        --- and someone brings us the mail, read it

    -domain-

    var newLetter:Letter = nil

    ##

This solution is fine, however we have to do some shuffling around of the letter
to deal with. It also feels a bit like a hack in differentiating between the two
situations of having a letter or not when transitioning to the couch.

As this is a common situation, it would be nice to have a more elegant way
to deal with it. Event forwarding provides just that solution.

Event forwarding syntax annotates a transition operator with a dispatch
operator:

.. code-block::

    |newMail| [letter:Letter]
        -> => $OnCouch ^

This syntax basically reads "do a transition and then dispatch **this** event to
the new state". The important point is that **a new event is not created**. Instead
the system machinery handles the caching and forwarding of the original event
to the new state. Lets see how our mail scenario works using event forwarding.

.. code-block::

    #YouGotMail

    -interface-

    newMail [letter:Letter]

    -machine-

    $AtMailBox --- start state is at mailbox
        |newMail| [letter:Letter] --- new mail arrives
            -> => $OnCouch ^      --- transition to couch and forward mail

    $OnCouch
        |newMail| [letter:Letter] --- handle original mail event
            readLetter() ^

    ##

Obviously there has been a significant simplification in the system's
specification as we have completely eliminated the `$OnCouch` `|>|` event handler.

This simplification in the Frame spec comes at the cost of complexity in the
generated code. This, of course, shouldn't matter to anyone as it is handled
by boiler plate code that no one needs to understand.

But lets understand it anyway.

Event Forwarding Mechanism
--------------------------

The first step in event forwarding is to use the runtime compartment variable
_forwardEvent_ to cache off the current event and then start the transition:

.. code-block:: go

    func (m *youGotMailStruct) _YouGotMailState_AtMailBox_(e *framelang.FrameEvent) {
        switch e.Msg {
        case "newMail":
            compartment := NewYouGotMailCompartment(YouGotMailState_OnCouch)
            compartment._forwardEvent_ = e // <--- compartment stores the event
            m._transition_(compartment)
            return
        }
    }

The heart of the event forwarding mechanism lives in the multiplexer method.
The multiplexer has two big sections. The first is the real "state machine"
switch statement that routes events to the current state function. The second
is the event forwarding logic which is commented below:

.. _multiplexer:

Multiplexer
^^^^^^^^^^^
.. code-block:: go

    //====================== Multiplexer ====================//

    func (m *youGotMailStruct) _mux_(e *framelang.FrameEvent) {

        // Send event to state for processing.
        // This is the core "state machine".
        switch m._compartment_.State {
        case YouGotMailState_AtMailBox:
            m._YouGotMailState_AtMailBox_(e)
        case YouGotMailState_OnCouch:
            m._YouGotMailState_OnCouch_(e)
        }

        // detect if a transition started when handling the event
        // by seeing if there is a _nextCompartment_ set
        if m._nextCompartment_ != nil {
            // make a local reference to the next compartment
            nextCompartment := m._nextCompartment_
            // remove system runtime reference - we have it locally now
            m._nextCompartment_ = nil
            // if the next compartment has a forwarded event to handle
            if nextCompartment._forwardEvent_ != nil &&
               // and if the forwarded event was the enter event
               nextCompartment._forwardEvent_.Msg == ">" {
                // then we won't do a normal transition. Instead,
                // first send the exit event like a normal transition.
                m._mux_(&framelang.FrameEvent{Msg: "<", Params: m._compartment_.ExitArgs, Ret: nil})
                // do the state/compartment change
                m._compartment_ = nextCompartment
                // now, rather than send a new enter event
                // send the old, forwarded one to the new state/compartment
                m._mux_(nextCompartment._forwardEvent_)
            } else {
                // if there was a forwarded event then it wasn't
                // an enter event. Go ahead and transition like normal
                m._do_transition_(nextCompartment)
                // now detect if there was a forwarded event
                if nextCompartment._forwardEvent_ != nil {
                    // and forward it
                    m._mux_(nextCompartment._forwardEvent_)
                }
            }

            // remove reference to any handled forwarded event
            nextCompartment._forwardEvent_ = nil
        }
    }

From the event forwarding perspective there are three categories of events of
concern:

#. One
#. Two
#. Three

1. Enter events - the mechanism support for forwarding these adds the most
complexity to the machine as it doesn't do a normal transition
2. Exit events - disallowed and will result in a parse error
3. All other events - handled simply in the else clause above by transitioning
and then forwarding into the multiplexer

Full Event Forwarding Syntax
----------------------------

The full syntax for an event forwarding transition is shown here:

.. code-block::

    #EventForwardSyntax

    -machine-

    $S0
    	|<| [exit_msg:string] ^
        |e1| ("exit") -> ("enter") "Transition Label" => $S1 ^

    $S1
    	|>| [enter_msg:string] ^
        |e1| ^

    ##


Returning Values from Forwarded Events
--------------------------------------

Forwarded events should work exactly like non-forwarded events in the machine.
As an example of this, `#StringTools` takes a small set of requests for
string editing operations and routes them to the correct state for processing:

.. code-block::

    #StringTools

    -interface-

    reverse [str:string] : string
    makePalindrome [str:string] : string

    -machine-

    $Router
    	|makePalindrome| [str:string] : string
            -> "make\npalindrome" => $MakePalindrome ^
        |reverse| [str:string] : string
            -> "reverse" => $Reverse ^

    $Reverse
        |reverse| [str:string] : string
            @^ = reverse_str(str)
            -> "ready" $Router ^

    $MakePalindrome
        |makePalindrome| [str:string] : string
            @^ = str + reverse_str(str)
            -> "ready" $Router ^

    ##

Conclusion
----------

Event forwarding is a very nice to have, but not essential, capability
in Frame. The need for it arises as a byproduct of having a better organized
system. It is a lot like having taken a messy room with everything available
and in reach but strewn about and hard to find and then put all the stuff in
boxes. The stuff is now better organized, but now you have to deal with
organizing and accessing boxes.

The need for event forwarding was recognized early in the development of
Frame but had no simple solution at the time. It required the concept of the
compartment in order to provide a key part of the solution to
the puzzle to be developed first. Additionally, at the time, there was also
not a multiplexer
method to put the logic to implement the feature. Therefore it required the
evolution
of other mechanisms to unlock a practical way to finally implement it.

It is hoped that as Frame continues to mature, similar discoveries about
useful combinations of
features, Frame syntax and low level code mechanisms will continue to be identified
and able to build on each other.
