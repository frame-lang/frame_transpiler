using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;
namespace Branch
{
    class BranchController : Branch
    {
        public BranchController() : base()
        {
        }
        public new void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }

    [TestFixture]
    public class BranchControllerTests
    {
        [Test]
        public void TestSimpleIfBool()
        {
            BranchController sm = new BranchController();
            sm.A();
            sm.OnBool(true);

            Assert.AreEqual("7", sm.state_info());
            Assert.AreEqual(new List<string> { "then 1", "then 2" }, sm.tape);

            sm = new BranchController();
            sm.A();
            sm.OnBool(false);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { "else 1", "else 2" }, sm.tape);
        }

        [Test]
        public void TestSimpleIfInt()
        {
            BranchController sm = new BranchController();
            sm.A();
            sm.OnInt(7);

            Assert.AreEqual("7", sm.state_info());
            Assert.AreEqual(new List<string> { "> 5", "< 10", "== 7" }, sm.tape);

            sm = new BranchController();
            sm.A();
            sm.OnInt(-3);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { "<= 5", "< 10", "!= 7" }, sm.tape);

            sm = new BranchController();
            sm.A();
            sm.OnInt(12);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { "> 5", ">= 10", "!= 7" }, sm.tape);
        }

        [Test]
        public void TestNegatedIfBool()
        {
            BranchController sm = new BranchController();
            sm.B();
            sm.OnBool(true);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { "else 1", "else 2" }, sm.tape);

            sm = new BranchController();
            sm.B();
            sm.OnBool(false);

            Assert.AreEqual("7", sm.state_info());
            Assert.AreEqual(new List<string> { "then 1", "then 2" }, sm.tape);
        }

        [Test]
        public void TestNegatedIfInt()
        {
            BranchController sm = new BranchController();
            sm.B();
            sm.OnInt(7);

            Assert.AreEqual("7", sm.state_info());
            Assert.AreEqual(new List<string> { ">= 5", "<= 10", "== 7" }, sm.tape);

            sm = new BranchController();
            sm.B();
            sm.OnInt(5);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { ">= 5", "<= 10", "!= 7" }, sm.tape);

            sm = new BranchController();
            sm.B();
            sm.OnInt(10);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string> { ">= 5", "<= 10", "!= 7" }, sm.tape);

            sm = new BranchController();
            sm.B();
            sm.OnInt(0);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string>() { "< 5", "<= 10", "!= 7" }, sm.tape);

            sm = new BranchController();

            sm.B();
            sm.OnInt(100);

            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(new List<string>() { ">= 5", "> 10", "!= 7" }, sm.tape);
        }

        [Test]
        public void OperatorPrecedenceTest()
        {
            BranchController sm = new BranchController();

            sm.C();
            sm.OnInt(0);
            Assert.AreEqual(new List<string>() { "then 1", "else 2", "then 3", "then 4" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(7);
            Assert.AreEqual(new List<string>() { "else 1", "then 2", "else 3", "then 4" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(-3);
            Assert.AreEqual(new List<string>() { "then 1", "else 2", "else 3", "else 4" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(12);
            Assert.AreEqual(new List<string>() { "else 1", "else 2", "then 3", "else 4" }, sm.tape);
        }

        [Test]
        public void NestedIfTest()
        {
            BranchController sm = new BranchController();

            sm.D();
            sm.OnInt(50);
            Assert.AreEqual("7", sm.state_info());
            Assert.AreEqual(new List<string>() { "> 0", "< 100" }, sm.tape);

            sm = new BranchController();
            sm.D();
            sm.OnInt(200);
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string>() { "> 0", ">= 100" }, sm.tape);

            sm = new BranchController();
            sm.D();
            sm.OnInt(-5);
            Assert.AreEqual("4", sm.state_info());
            Assert.AreEqual(new List<string>() { "<= 0", "> -10" }, sm.tape);

            sm = new BranchController();
            sm.D();
            sm.OnInt(-10);
            Assert.AreEqual("8", sm.state_info());
            Assert.AreEqual(sm.tape, new List<string> { "<= 0", "<= -10" });
        }

        [Test]
        public void TestGuardedTransition()
        {
            BranchController sm = new BranchController();
            sm.E();
            sm.OnInt(5);
            Assert.AreEqual(sm.state_info(), "9");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F3" });

            sm = new BranchController();
            sm.E();
            sm.OnInt(15);
            Assert.AreEqual(sm.state_info(), "8");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F2" });

            sm = new BranchController();
            sm.E();
            sm.OnInt(115);
            Assert.AreEqual(sm.state_info(), "7");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F1" });
        }

        [Test]
        public void TestNestedGuardedTransition()
        {
            BranchController sm = new BranchController();
            sm.F();
            sm.OnInt(5);
            Assert.AreEqual(sm.state_info(), "9");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F3" });

            sm = new BranchController();
            sm.F();
            sm.OnInt(15);
            Assert.AreEqual(sm.state_info(), "8");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F2" });

            sm = new BranchController();
            sm.F();
            sm.OnInt(65);
            Assert.AreEqual(sm.state_info(), "9");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F3" });

            sm = new BranchController();
            sm.F();
            sm.OnInt(115);
            Assert.AreEqual(sm.state_info(), "7");
            Assert.AreEqual(sm.tape, new List<string> { "-> $F1" });
        }




    }

}
