using System.IO;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace StateContext
{
    class StateContextSmController : StateContextSm
    {
        public StateContextSmController() : base()
        {
        }

        public void log_do(String name, int val)
        {
            this.tape.Add(name + "=" + val.ToString());
        }
    }

    [TestFixture]
    public class StateContextTest
    {
        [TestFixture]
        public class StateContextSmControllerTest
        {
            [Test]
            public void TestInitialState()
            {
                StateContextSmController sm = new StateContextSmController();
                int r = sm.Inc();
                Assert.AreEqual(4, r);
                sm.LogState();
                CollectionAssert.AreEqual(new List<string> { "w=3", "w=4", "w=4" }, sm.tape);
            }

            [Test]
            public void TestTransition()
            {
                StateContextSmController sm = new StateContextSmController();
                sm.Inc();
                sm.Inc();
                sm.tape = new List<string>();

                sm.Start();
                CollectionAssert.AreEqual(new List<string> { "a=3", "b=5", "x=15" }, sm.tape);
                sm.tape = new List<string>();

                sm.Inc();
                int r = sm.Inc();
                Assert.AreEqual(17, r);
                CollectionAssert.AreEqual(new List<string> { "x=16", "x=17" }, sm.tape);
                sm.tape = new List<string>();

                sm.Next(3);
                CollectionAssert.AreEqual(new List<string> { "c=10", "x=27", "a=30", "y=17", "z=47" }, sm.tape);
                sm.tape = new List<string>();

                sm.Inc();
                sm.Inc();
                r = sm.Inc();
                Assert.AreEqual(50, r);
                CollectionAssert.AreEqual(new List<string> { "z=48", "z=49", "z=50" }, sm.tape);
            }

            [Test]
            public void TestChangeState()
            {
                StateContextSmController sm = new StateContextSmController();
                sm.Inc();
                sm.Inc();
                sm.Start();
                sm.tape = new List<string>();

                sm.Inc();
                CollectionAssert.AreEqual(new List<string> { "x=16" }, sm.tape);
                sm.tape = new List<string>();

                sm.Change(10);
                sm.LogState();
                CollectionAssert.AreEqual(new List<string> { "y=26", "z=0" }, sm.tape);
                sm.tape = new List<string>();

                sm.Inc();
                sm.Change(100);
                sm.LogState();
                Assert.AreEqual("0", sm.state_info());
                CollectionAssert.AreEqual(new List<string> { "z=1", "tmp=127", "w=0" }, sm.tape);
            }
        }


    }
}