using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace TransitionParams
{
    class TransitParamsController : TransitParams
    {
        public TransitParamsController() : base()
        {
        }
        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }
    [TestFixture]
    public class TransitionParamsTest
    {
        [Test]
        public void Enter()
        {
            var sm = new TransitParamsController();
            sm.Next();
            Assert.AreEqual(new List<string> { "hi A" }, sm.tape);
        }

        [Test]
        public void EnterAndExit()
        {
            var sm = new TransitParamsController();
            sm.Next();
            sm.tape = new List<string>();
            sm.Next();
            Assert.AreEqual(new List<string> { "bye A", "hi B", "42" }, sm.tape);
            sm.tape = new List<string>();
            sm.Next();
            Assert.AreEqual(new List<string> { "True", "bye B", "hi again A" }, sm.tape);
        }

        [Test]
        public void ChangeState()
        {
            var sm = new TransitParamsController();
            Assert.AreEqual("0", sm.state_info());
            sm.Change();
            Assert.AreEqual("1", sm.state_info());
            sm.Change();
            Assert.AreEqual("2", sm.state_info());
            sm.Change();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);
        }

        [Test]
        public void ChangeAndTransition()
        {
            var sm = new TransitParamsController();
            sm.Change();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);
            sm.Next();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(new List<string> { "bye A", "hi B", "42" }, sm.tape);
            sm.tape = new List<string>();
            sm.Change();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);
            sm.Change();
            sm.Next();
            Assert.AreEqual("1", sm.state_info());
            Assert.AreEqual(new List<string> { "True", "bye B", "hi again A" }, sm.tape);
        }
    }



}