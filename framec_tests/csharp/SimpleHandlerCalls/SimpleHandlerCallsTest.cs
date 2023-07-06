using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace SimpleHandlerCalls
{
    class SimpleHandlerCallsController : SimpleHandlerCalls
    {
        public SimpleHandlerCallsController() : base()
        {
        }
    }
    [TestFixture]
    public class SimpleHandlerCallsTest
    {
        [Test]
        public void TestSimpleCall()
        {
            SimpleHandlerCallsController sm = new SimpleHandlerCallsController();
            sm.C();
            Assert.AreEqual("1", sm.state_info());
        }

        /* Test that a handler call terminates the current handler. */
        [Test]
        public void TestCallsTerminateHandler()
        {
            SimpleHandlerCallsController sm = new SimpleHandlerCallsController();
            sm.D();
            Assert.AreEqual("2", sm.state_info());

            sm = new SimpleHandlerCallsController();
            sm.E();
            Assert.AreEqual("2", sm.state_info());
        }

    }
}