using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace Transition
{
    class TransitionSmController : TransitionSm
    {
        public TransitionSmController() : base()
        {
        }
        public void enter_do(string state)
        {
            this.enters.Add(state);
        }

        public void exit_do(string state)
        {
            this.exits.Add(state);
        }

        public void clear_all()
        {
            this.enters = new List<string>();
            this.exits = new List<string>();
        }
    }
    [TestFixture]
    public class TransitionTest
    {
        [Test]
        public void TestTransitionEvents()
        {
            TransitionSmController sm = new TransitionSmController();
            sm.clear_all();
            sm.transit();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(new List<string> { "S0" }, sm.exits);
            Assert.AreEqual(new List<string> { "S1" }, sm.enters);
        }

        [Test]
        public void TestChangeStateNoEvents()
        {
            TransitionSmController sm = new TransitionSmController();
            sm.clear_all();
            sm.change();
            Assert.AreEqual("1", sm.state_info());
            sm.change();
            Assert.AreEqual("2", sm.state_info());
            sm.change();
            Assert.AreEqual("3", sm.state_info());
            sm.change();
            Assert.AreEqual("4", sm.state_info());
            Assert.IsTrue(sm.exits.Count == 0);
            Assert.IsTrue(sm.enters.Count == 0);
        }

        [Test]
        public void TestCascadingTransition()
        {
            TransitionSmController sm = new TransitionSmController();
            sm.change();
            sm.clear_all();
            Assert.AreEqual("1", sm.state_info());
            sm.transit();
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S1", "S2" }, sm.exits);
            Assert.AreEqual(new List<string> { "S2", "S3" }, sm.enters);
        }

        [Test]
        public void TestCascadingChangeSheet()
        {
            TransitionSmController sm = new TransitionSmController();
            sm.change();
            sm.change();
            sm.change();
            sm.clear_all();
            Assert.AreEqual("3", sm.state_info());
            sm.transit();
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(new List<string> { "S3" }, sm.exits);
            Assert.AreEqual(new List<string> { "S4" }, sm.enters);
        }


    }
}