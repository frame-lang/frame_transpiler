using System.IO;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace StateContextStack
{
    class StateContextStackController : StateContextStack
    {
        public StateContextStackController() : base()
        {
        }
        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }
    [TestFixture]
    public class StateContextStackTest
    {
        [Test]
        public void TestPushPop()
        {
            StateContextStackController sm = new StateContextStackController();
            Assert.AreEqual("0", sm.state_info());
            sm.push();
            sm.to_b();
            Assert.AreEqual("1", sm.state_info());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
        }

        /* Test that multiple states can be pushed and subsequently restored by pops, LIFO style. */
        [Test]
        public void TestMultiplePushPops()
        {
            StateContextStackController sm = new StateContextStackController();
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

        /* Test that pop transitions trigger enter/exit events. */
        [Test]
        public void TestPopTransitionEvents()
        {
            StateContextStackController sm = new StateContextStackController();
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
            Assert.AreEqual(new List<string> { "A:<", "C:>" }, sm.tape);
            sm.tape = new List<string>();
            sm.pop();
            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(new List<string> { "C:<", "A:>", "A:<", "B:>" }, sm.tape);
        }

        /* Test that pop change-states do not trigger enter/exit events. */
        [Test]
        public void TestPopChangesStateNoEvents()
        {
            StateContextStackController sm = new StateContextStackController();
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
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(new List<string> { "C:<", "A:>" }, sm.tape);
        }

        [Test]
        public void TestPopRestoresStateVariables()
        {
            StateContextStackController sm = new StateContextStackController();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(2, sm.value());
            sm.to_b();
            sm.inc();
            sm.push();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(5, sm.value());
            sm.to_c();
            sm.inc();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(30, sm.value());
            sm.to_a();
            sm.inc();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(1, sm.value());
            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(30, sm.value());
            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(5, sm.value());
            sm.to_a();
            sm.inc();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(3, sm.value());
            sm.to_c();
            sm.inc();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(10, sm.value());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(3, sm.value());
            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(2, sm.value());
        }

        [Test]
        public void TestPushStoresStateVariableSnapshot()
        {
            StateContextStackController sm = new StateContextStackController();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(2, sm.value());
            sm.inc();
            sm.inc();
            Assert.AreEqual(4, sm.value());

            sm.to_b();
            sm.inc();
            sm.push();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(5, sm.value());
            sm.inc();
            sm.inc();
            Assert.AreEqual(15, sm.value()); // these changes should be forgotten

            sm.to_c();
            sm.inc();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(30, sm.value());
            sm.inc();
            Assert.AreEqual(40, sm.value()); // forgotten

            sm.to_a();
            sm.inc();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(1, sm.value());

            sm.pop();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(30, sm.value());

            sm.pop();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(5, sm.value());

            sm.to_a();
            sm.inc();
            sm.inc();
            sm.inc();
            sm.push();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(3, sm.value());
            sm.inc();
            Assert.AreEqual(4, sm.value()); //forgotten

            sm.to_c();
            sm.inc();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(10, sm.value());

            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(3, sm.value());

            sm.pop();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(2, sm.value());
        }



    }
}