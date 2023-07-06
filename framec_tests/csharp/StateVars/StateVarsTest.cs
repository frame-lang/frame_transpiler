using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace StateVars
{
    class StateVarsController : StateVars
    {
        public StateVarsController() : base()
        {
        }
    }
    [TestFixture]
    public class StateVarsTest
    {
        [Test]
        public void TestSingleVariable()
        {
            StateVarsController sm = new StateVarsController();
            Assert.AreEqual("1", sm.state_info());
            sm.X(); // increment x
            sm.X(); // increment x
            Assert.AreEqual(2, (int)sm._compartment_.StateVars["x"]);
        }

        [Test]
        public void TestMultipleVariables()
        {
            StateVarsController sm = new StateVarsController();
            sm.Y();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(10, (int)sm._compartment_.StateVars["y"]);
            Assert.AreEqual(100, (int)sm._compartment_.StateVars["z"]);
            sm.Y();
            sm.Y();
            sm.Z();
            sm.Y();
            Assert.AreEqual(13, (int)sm._compartment_.StateVars["y"]);
            Assert.AreEqual(101, (int)sm._compartment_.StateVars["z"]);
        }

        [Test]
        public void TestVariablesAreReset()
        {
            StateVarsController sm = new StateVarsController();
            sm.X(); // increment x
            sm.X(); // increment x
            Assert.AreEqual(2, (int)sm._compartment_.StateVars["x"]);
            sm.Z(); // transition to B
            sm.Z(); // increment z
            sm.Y(); // increment y
            sm.Z(); // increment z
            Assert.AreEqual(11, (int)sm._compartment_.StateVars["y"]);
            Assert.AreEqual(102, (int)sm._compartment_.StateVars["z"]);
            sm.X(); // transition to A
            Assert.AreEqual(0, (int)sm._compartment_.StateVars["x"]);
            sm.Y(); // transition to B
            Assert.AreEqual(10, (int)sm._compartment_.StateVars["y"]);
            Assert.AreEqual(100, (int)sm._compartment_.StateVars["z"]);
        }


    }
}