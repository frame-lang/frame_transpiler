using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace Match
{
    class MatchController : Match
    {
        public MatchController() : base()
        {
        }
        public void log_do(string msg)
        {
            this.tape.Add(msg);
        }
    }
    [TestFixture]
    public class MatchTest
    {
        [Test]
        public void TestEmptyString()
        {
            MatchController sm = new MatchController();
            sm.Empty();
            sm.OnString("");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "empty" }));

            sm.tape = new List<string>();
            sm.OnString("hi");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));
        }

        [Test]
        public void TestIntegerMatch()
        {
            MatchController sm = new MatchController();
            sm.Simple();
            sm.OnInt(0);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "0" }));

            sm.tape = new List<string>();
            sm.OnInt(42);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "42" }));

            sm.tape = new List<string>();
            sm.OnInt(-200);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "-200" }));
        }

        [Test]
        public void TestStringMatch()
        {
            MatchController sm = new MatchController();
            sm.Simple();
            sm.OnString("hello");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "hello" }));

            sm.tape = new List<string>();
            sm.OnString("goodbye");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "goodbye" }));

            sm.tape = new List<string>();
            sm.OnString("Testing 1, 2, 3...");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "testing" }));

            sm.tape = new List<string>();
            sm.OnString("$10!");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "money" }));

            sm.tape = new List<string>();
            sm.OnString("missing");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));

            sm.tape = new List<string>();
            sm.OnString("Testing");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));

            sm.tape = new List<string>();
            sm.OnString("");
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));
        }

        [Test]
        public void TestIntegerMultiMatch()
        {
            MatchController sm = new MatchController();
            sm.Multi();
            sm.OnInt(3);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "3|-7" }));

            sm.tape = new List<string>();
            sm.OnInt(-7);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "3|-7" }));

            sm.tape = new List<string>();
            sm.OnInt(-4);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "-4|5|6" }));

            sm.tape = new List<string>();
            sm.OnInt(5);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "-4|5|6" }));

            sm.tape = new List<string>();
            sm.OnInt(6);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "-4|5|6" }));

            sm.tape = new List<string>();
            sm.OnInt(4);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));

            sm.tape = new List<string>();
            sm.OnInt(0);
            Assert.That(sm.tape, Is.EqualTo(new List<string> { "?" }));
        }

        [Test]
        public void TestStringMultiMatch()
        {
            var sm = new MatchController();
            sm.Multi();
            sm.OnString("$10");
            Assert.AreEqual(new List<string> { "symbols" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("12.5%");
            Assert.AreEqual(new List<string> { "symbols" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("@#*!");
            Assert.AreEqual(new List<string> { "symbols" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString(" ");
            Assert.AreEqual(new List<string> { "whitespace" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("  ");
            Assert.AreEqual(new List<string> { "whitespace" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("\t");
            Assert.AreEqual(new List<string> { "whitespace" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("\n");
            Assert.AreEqual(new List<string> { "whitespace" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("10");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("#");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("   ");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
            sm.tape = new List<string>();
        }

        [Test]
        public void TestIntegerNestedMatch()
        {
            var sm = new MatchController();
            sm.Nested();
            sm.OnInt(1);
            Assert.AreEqual(new List<string> { "1-3", "1" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(2);
            Assert.AreEqual(new List<string> { "1-3", "2" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(3);
            Assert.AreEqual(new List<string> { "1-3", "3" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(4);
            Assert.AreEqual(new List<string> { "4-5", "4" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(5);
            Assert.AreEqual(new List<string> { "4-5", "5" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(10);
            Assert.AreEqual(new List<string> { "too big" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnInt(0);
            Assert.AreEqual(new List<string> { "too small" }, sm.tape);
        }

        [Test]
        public void TestStringNestedMatch()
        {
            var sm = new MatchController();
            sm.Nested();
            sm.OnString("hello");
            Assert.AreEqual(new List<string> { "greeting", "English" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("hola");
            Assert.AreEqual(new List<string> { "greeting", "Spanish" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("bonjour");
            Assert.AreEqual(new List<string> { "greeting", "French" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("goodbye");
            Assert.AreEqual(new List<string> { "farewell", "English" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("adios");
            Assert.AreEqual(new List<string> { "farewell", "Spanish" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("au revoir");
            Assert.AreEqual(new List<string> { "farewell", "French" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("hallo");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
            sm.tape = new List<string>();
            sm.OnString("ciao");
            Assert.AreEqual(new List<string> { "?" }, sm.tape);
        }

        [Test]
        public void TestIntegerHierarchicalMatch()
        {
            var sm = new MatchController();
            sm.Child();
            sm.OnInt(0);
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);

            sm = new MatchController();
            sm.Child();
            sm.OnInt(4);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "4" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnInt(5);
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(new List<string> { "5" }, sm.tape);

            sm = new MatchController();
            sm.Child();
            sm.OnInt(5);
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(new List<string> { "5" }, sm.tape);

            sm = new MatchController();
            sm.Child();
            sm.OnInt(3);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "3", "?" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnInt(42);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "42 in child", "42" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnInt(-200);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "no match in child", "-200" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnInt(100);
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "no match in child", "?" }, sm.tape);
        }

        [Test]
        public void TestStringHierarchicalMatch()
        {
            MatchController sm = new MatchController();
            sm.Child();
            sm.OnString("goodbye");
            Assert.AreEqual("6", sm.state_info());
            Assert.AreEqual(0, sm.tape.Count);

            sm = new MatchController();
            sm.Child();
            sm.OnString("hello");
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "hello in child", "hello" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnString("Testing 1, 2, 3...");
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "testing in child" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnString("$10!");
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "no match in child", "money" }, sm.tape);

            sm.tape = new List<string>();
            sm.OnString("testing 1, 2, 3...");
            Assert.AreEqual("5", sm.state_info());
            Assert.AreEqual(new List<string> { "no match in child", "?" }, sm.tape);
        }



    }
}