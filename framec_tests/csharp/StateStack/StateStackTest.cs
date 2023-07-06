using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace StateStack
{
    class StateStackController : StateStack
    {
        public StateStackController() : base()
        {
        }
        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }
    [TestFixture]
    public class StateStackTest
    {
        // Test that a pop restores a pushed state.
        [Test]
        public void TestPushPop()
        {
            StateStackController sm = new StateStackController();
            Assert.AreEqual("0", sm.state_info());
            sm.push();
            sm.to_b();
            Assert.AreEqual("1", sm.state_info());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
        }

        // Test that multiple states can be pushed and subsequently restored by pops, LIFO style.
        [Test]
        public void TestMultiplePushPops()
        {
            StateStackController sm = new StateStackController();
            Assert.AreEqual("0", sm.state_info());
            sm.push();
            sm.to_c();
            sm.push();
            sm.to_a();
            sm.push();
            sm.push();
            sm.to_c(); // no push
            sm.to_b();
            sm.push();
            sm.to_c();
            sm.push(); // stack top-to-bottom: C, B, A, A, C, A
            sm.to_a();
            Assert.AreEqual("0", sm.state_info());
            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            sm.to_a();
            Assert.AreEqual("0", sm.state_info());
            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            sm.to_b();
            sm.push();
            sm.to_c();
            sm.push(); // stack top-to-bottom: C, B, A
            sm.to_a();
            sm.to_b();
            Assert.AreEqual("1", sm.state_info());
            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
        }

        // Test that pop transitions trigger enter/exit events.
        [Test]
        public void TestPopTransitionEvents()
        {
            StateStackController sm = new StateStackController();
            sm.to_b();
            sm.push();
            sm.to_a();
            sm.push();
            sm.to_c();
            sm.push(); // stack top-to-bottom: C, A, B
            sm.to_a();
            sm.tape = new List<string>();
            Assert.AreEqual("0", sm.state_info());
            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            Assert.IsTrue(sm.tape.SequenceEqual(new List<string> { "A:<", "C:>" }));
            sm.tape = new List<string>();
            sm.pop();
            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            Assert.IsTrue(sm.tape.SequenceEqual(new List<string> { "C:<", "A:>", "A:<", "B:>" }));
        }

        // Test that pop change-states do not trigger enter/exit events.
        [Test]
        public void TestPopChangeStateNoEvents()
        {
            StateStackController sm = new StateStackController();
            sm.to_b();
            sm.push();
            sm.to_a();
            sm.push();
            sm.to_c();
            sm.push(); // stack top-to-bottom: C, A, B
            sm.to_a();
            sm.tape = new List<string>();
            Assert.AreEqual("0", sm.state_info());
            sm.pop_change();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);
            sm.pop();
            sm.pop_change();
            Assert.AreEqual(new List<string> { "C:<", "A:>" }, sm.tape);
        }

    }
}