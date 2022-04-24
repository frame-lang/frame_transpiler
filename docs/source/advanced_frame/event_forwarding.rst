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

One of the situations that is easy to handle in event orgiented systems is
dealing with

events are
One of the side effects of compartmentalization of system behavior is that
now event handling is contextualized and we can't easily slip from one
state to another during a single call to an event handler.

This matters when an event happens in one state but needs to be dealt with
in another. For instance, let's say you are at the mailbox when a letter
arrives, but you don't want to read it right then as a nosey neighbor
might read it. Instead you wait until you are on your couch at home before
you open and read it.

.. code-block::

    #YouGotMail

    -interface-

    newMail [letter:Letter]

    -machine-

    $AtMailBox --- start state is at mailbox
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

This solution is fine, however we have to do some shuffling of the letter
and what feels like a bit of a hack in differentiating between the two
situations of having a letter or not when reaching it.

As this is a common situation, it would be nice to have a more elegant way
to deal with it. Event forwarding provides just that solution.

Event forwarding syntax annotates a transition operator with a dispatch
operator:

.. code-block::

    |newMail| [letter:Letter]
        -> => $OnCouch ^

This syntax basically reads "do a transition and then dispatch **this** event to
the new state". The important point is that a new event is not created. Instead
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
specification as we can completely eliminate the `$OnCouch` `|>|` event handler.

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


.. code-block:: go

    //====================== Multiplexer ====================//

    func (m *youGotMailStruct) _mux_(e *framelang.FrameEvent) {

        // send event to state for processing
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
