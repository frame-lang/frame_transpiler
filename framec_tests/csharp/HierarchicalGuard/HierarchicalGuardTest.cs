using System.IO;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace HierarchicalGuard
{
    class HierarchicalGuardController : HierarchicalGuard
    {
        public HierarchicalGuardController() : base()
        {
        }
        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }
    [TestFixture]
    public class HierarchicalGuardTest
    {
        [Test]
        public void TestPropagateToParent()
        {
            var sm = new HierarchicalGuardController();
            sm.A(0);
            sm.tape = new List<string>();
            Assert.AreEqual("2", sm.state_info());
            sm.A(20);
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.A" }, sm.tape);

            sm = new HierarchicalGuardController();
            sm.A(0);
            sm.tape = new List<string>();
            Assert.AreEqual("2", sm.state_info());
            sm.A(-5);
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.A", "S.A" }, sm.tape);

            sm = new HierarchicalGuardController();
            sm.A(0);
            sm.tape = new List<string>();
            Assert.AreEqual("2", sm.state_info());
            sm.B(-5);
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.B" }, sm.tape);

            sm = new HierarchicalGuardController();
            sm.A(0);
            sm.tape = new List<string>();
            Assert.AreEqual("2", sm.state_info());
            sm.B(5);
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.B", "S.B" }, sm.tape);
        }

        [Test]
        public void TestPropagateMultipleLevels()
        {
            var sm = new HierarchicalGuardController();
            sm.B(0);
            sm.tape = new List<string>();
            Assert.AreEqual("4", sm.state_info());
            sm.A(7);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.A", "S1.A" }, sm.tape);

            sm = new HierarchicalGuardController();
            sm.B(0);
            sm.tape = new List<string>();
            Assert.AreEqual("4", sm.state_info());
            sm.A(-5);
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.A", "S1.A", "S0.A", "S.A" }, sm.tape);
        }

        [Test]
        public void TestPropagateSkipsLevels()
        {
            var sm = new HierarchicalGuardController();
            sm.B(0);
            sm.tape = new List<string>();
            Assert.AreEqual("4", sm.state_info());
            sm.B(-5);
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.B", "S0.B" }, sm.tape);

            sm = new HierarchicalGuardController();
            sm.B(0);
            sm.tape = new List<string>();
            Assert.AreEqual("4", sm.state_info());
            sm.B(5);
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.B", "S0.B", "S.B" }, sm.tape);
        }

        [Test]
        public void TestConditionalReturnTest()
        {
            var sm = new HierarchicalGuardController();
            sm.B(20);
            sm.tape = new List<string>();
            Assert.AreEqual(sm.state_info(), "5");
            sm.A(5);
            Assert.AreEqual(sm.state_info(), "5");
            Assert.AreEqual(sm.tape, new List<string> { "S3.A", "stop" });

            sm = new HierarchicalGuardController();
            sm.B(20);
            sm.tape = new List<string>();
            Assert.AreEqual(sm.state_info(), "5");
            sm.A(-5);
            Assert.AreEqual(sm.state_info(), "2");
            Assert.AreEqual(sm.tape, new List<string> { "S3.A", "continue", "S.A" });

            sm = new HierarchicalGuardController();
            sm.B(20);
            sm.tape = new List<string>();
            Assert.AreEqual(sm.state_info(), "5");
            sm.B(-5);
            Assert.AreEqual(sm.state_info(), "5");
            Assert.AreEqual(sm.tape, new List<string> { "S3.B", "stop" });

            sm = new HierarchicalGuardController();
            sm.B(20);
            sm.tape = new List<string>();
            Assert.AreEqual(sm.state_info(), "5");
            sm.B(5);
            Assert.AreEqual(sm.state_info(), "4");
            Assert.AreEqual(sm.tape, new List<string> { "S3.B", "continue", "S.B" });
        }

    }
}