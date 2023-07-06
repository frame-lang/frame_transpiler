using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;


namespace Hierarchical
{
    class HierarchicalController : Hierarchical
    {
        public HierarchicalController() : base()
        {
        }
        public void enter_do(string msg)
        {
            this.enters.Add(msg);
        }

        public void exit_do(string msg)
        {
            this.exits.Add(msg);
        }

        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }

    [TestFixture]
    public class HierarchicalControllerTests
    {
        [Test]
        public void TestEnterContinue()
        {
            var sm = new HierarchicalController();
            sm.enters.Clear();
            sm.A();
            Assert.AreEqual(new[] { "S0", "S" }, sm.enters);
            sm.enters.Clear();
            sm.C();
            Assert.AreEqual(new[] { "S2", "S0", "S" }, sm.enters);
        }

        [Test]
        public void TestExitContinue()
        {
            var sm = new HierarchicalController();
            sm.A();
            sm.exits.Clear();
            sm.C();
            Assert.AreEqual(new[] { "S0", "S" }, sm.exits);
            sm.exits.Clear();
            sm.A();
            Assert.AreEqual(new[] { "S2", "S0", "S" }, sm.exits);
        }

        [Test]
        public void TestEnterReturn()
        {
            var sm = new HierarchicalController();
            sm.enters.Clear();
            sm.B();
            Assert.AreEqual(new[] { "S1" }, sm.enters);

            sm = new HierarchicalController();
            sm.A();
            sm.A();
            Assert.AreEqual("6", sm.state_info());
            sm.enters.Clear();
            sm.C();
            Assert.AreEqual(new[] { "S3", "S1" }, sm.enters);
        }

        [Test]
        public void TestExitReturn()
        {
            var sm = new HierarchicalController();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            sm.exits.Clear();
            sm.A();
            Assert.AreEqual(new[] { "S1" }, sm.exits);

            sm = new HierarchicalController();
            sm.A();
            sm.A();
            sm.C();
            Assert.AreEqual("5", sm.state_info());
            sm.exits.Clear();
            sm.B();
            Assert.AreEqual(new[] { "S3", "S1" }, sm.exits);
        }

        [Test]
        public void TestCurrentStateSimple()
        {
            var sm = new HierarchicalController();
            Assert.AreEqual("1", sm.state_info());
            sm.A();
            Assert.AreEqual("2", sm.state_info());
            sm.A();
            Assert.AreEqual("6", sm.state_info());
            sm.C();
            Assert.AreEqual("5", sm.state_info());
            sm.B();
            Assert.AreEqual("4", sm.state_info());
        }

        [Test]
        public void TestCurrentStateWithPropagation()
        {
            var sm = new HierarchicalController();
            Assert.AreEqual("1", sm.state_info());
            sm.A();
            Assert.AreEqual("2", sm.state_info());
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            sm.C();
            Assert.AreEqual("3", sm.state_info());
            sm.A();
            Assert.AreEqual("2", sm.state_info());
            sm.C();
            Assert.AreEqual("4", sm.state_info());
            sm.B();
            Assert.AreEqual("3", sm.state_info());
        }

        [Test]
        public void TestOverrideParentHandler()
        {
            var sm = new HierarchicalController();
            sm.A();
            sm.tape = new List<string>();
            sm.A();
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.A" }, sm.tape);

            sm.C();
            sm.tape = new List<string>();
            sm.B();
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string> { "S3.B" }, sm.tape);
        }

        [Test]
        public void TestBeforeParentHandler()
        {
            var sm = new HierarchicalController();
            sm.A();
            sm.tape = new List<string>();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S0.B", "S.B" }, sm.tape);
            sm.tape = new List<string>();
            sm.exits = new List<string>();
            sm.enters = new List<string>();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S1.B", "S.B" }, sm.tape);
            Assert.AreEqual(new List<string> { "S1" }, sm.exits);
            Assert.AreEqual(new List<string> { "S1" }, sm.enters);

            sm = new HierarchicalController();
            sm.A();
            sm.C();
            Assert.AreEqual("4", sm.state_info());
            sm.tape = new List<string>();
            sm.exits = new List<string>();
            sm.enters = new List<string>();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.B", "S0.B", "S.B" }, sm.tape);
            Assert.AreEqual(new List<string> { "S2", "S0", "S" }, sm.exits);
            Assert.AreEqual(new List<string> { "S1" }, sm.enters);
        }

        [Test]
        public void TestDeferToParentHandler()
        {
            var sm = new HierarchicalController();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            sm.tape = new List<string>();
            sm.A();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(new List<string> { "S.A" }, sm.tape);
            sm.A();
            sm.C();
            Assert.AreEqual("5", sm.state_info());
            sm.tape = new List<string>();
            sm.A();
            Assert.AreEqual("2", sm.state_info());
            Assert.AreEqual(new List<string> { "S.A" }, sm.tape);
        }

        [Test]
        public void TestBeforeMissingHandler()
        {
            var sm = new HierarchicalController();
            sm.B();
            Assert.AreEqual("3", sm.state_info());
            sm.tape = new List<string>();
            sm.exits = new List<string>();
            sm.enters = new List<string>();
            sm.C();
            Assert.AreEqual("3", sm.state_info());
            Assert.AreEqual(new List<string> { "S1.C" }, sm.tape);
            Assert.AreEqual(0, sm.exits.Count);
            Assert.AreEqual(0, sm.enters.Count);
        }

        [Test]
        public void TestContinueAfterTransitionIgnored()
        {
            var sm = new HierarchicalController();
            sm.A();
            sm.C();
            Assert.AreEqual("4", sm.state_info());
            sm.tape = new List<string>();
            sm.enters = new List<string>();
            sm.C();
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(new List<string> { "S2.C" }, sm.tape);
            Assert.AreEqual(new List<string> { "T" }, sm.enters);
        }



    }
}