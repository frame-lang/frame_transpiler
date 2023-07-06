using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace EventHandler
{
    class EventHandlerController : EventHandler
    {
        public EventHandlerController() : base()
        {
        }
        public new void log_do(String msg, int val)
        {
            String value = msg + "=" + val.ToString();
            this.tape.Add(value);
        }
    }

        [TestFixture]
        public class EventHandlerTest
        {
            [Test]
            public void TestSingleParameter(){
                EventHandlerController sm = new();
                sm.LogIt(2);
                Assert.AreEqual(new List<string> { "x=2" }, sm.tape);
                
            }

            [Test]
            public void TestComputeTwoParameter(){
                EventHandlerController sm = new();
                sm.LogAdd(-3, 10);
                Assert.AreEqual(new List<string> {"a=-3", "b=10", "a+b=7"}, sm.tape);
            }

            [Test]
            public void TestReturnLocalVariable(){
                EventHandlerController sm = new();
                int ret = sm.LogReturn(13, 21);
                Assert.AreEqual(new List<string> {"a=13", "b=21", "r=34"}, sm.tape);
                Assert.AreEqual(34, ret);
            }

            [Test]
            public void TestPassResult(){
                EventHandlerController sm = new();
                sm.PassAdd(5, -12);
                Assert.AreEqual(new List<string> { "p=-7" }, sm.tape);
            }

            [Test]
            public void TestPassAndReturnResult(){
                EventHandlerController sm = new();
                int ret = sm.PassReturn(101, -59);
                Assert.AreEqual(new List<string> { "r=42", "p=42"}, sm.tape);
                Assert.AreEqual(ret, 42);
        
            }

        }
    
}