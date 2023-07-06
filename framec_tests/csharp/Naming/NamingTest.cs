using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using NUnit.Framework;

namespace Naming
{
    class NamingController : Naming
    {
        public NamingController() : base()
        {
        }
        public void snake_action_do(int snake_param)
        {
            this.snake_log.Add(snake_param);
        }

        public void CamelAction_do(int CamelParam)
        {
            this.CamelLog.Add(CamelParam);
        }

        public void action123_do(int param123)
        {
            this.log123.Add(param123);
        }

        public void logFinal_do(int r)
        {
            this.finalLog.Add(r);
        }
    }
    [TestFixture]
    public class NamingTest
    {
        [Test]
        public void TestFollowNamingWorks()
        {
            NamingController sm = new NamingController();

            sm.snake_event(1);
            Assert.AreEqual("1", sm.state_info());
            sm.snake_event(2);
            Assert.AreEqual("0", sm.state_info());
            sm.snake_event(1);
            Assert.AreEqual("1", sm.state_info());
            sm.CamelEvent(3);
            Assert.AreEqual("0", sm.state_info());
            sm.snake_event(1);
            Assert.AreEqual("1", sm.state_info());
            sm.event123(4);
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(new List<int> { 1103, 1104, 1105 }, sm.finalLog);
            sm.finalLog = new List<int>();

            sm.CamelEvent(11);
            Assert.AreEqual("2", sm.state_info());
            sm.snake_event(2);
            Assert.AreEqual("0", sm.state_info());
            sm.CamelEvent(11);
            Assert.AreEqual("2", sm.state_info());
            sm.CamelEvent(3);
            Assert.AreEqual("0", sm.state_info());
            sm.CamelEvent(11);
            Assert.AreEqual("2", sm.state_info());
            sm.event123(4);
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(new List<int> { 1213, 1214, 1215 }, sm.finalLog);
            sm.finalLog = new List<int>();

            sm.event123(21);
            Assert.AreEqual("3", sm.state_info());
            sm.snake_event(2);
            Assert.AreEqual("0", sm.state_info());
            sm.event123(21);
            Assert.AreEqual("3", sm.state_info());
            sm.CamelEvent(3);
            Assert.AreEqual("0", sm.state_info());
            sm.event123(21);
            Assert.AreEqual("3", sm.state_info());
            sm.event123(4);
            Assert.AreEqual("0", sm.state_info());
            Assert.AreEqual(new List<int> { 1323, 1324, 1325 }, sm.finalLog);

            Assert.AreEqual(new List<int> { 1103, 1213, 1323 }, sm.snake_log);
            Assert.AreEqual(new List<int> { 1104, 1214, 1324 }, sm.CamelLog);
            Assert.AreEqual(new List<int> { 1105, 1215, 1325 }, sm.log123);
        }

        [Test]
        public void TestInterfaceCalls()
        {
            NamingController sm = new NamingController();
            sm.call("snake_event", 1);
            sm.call("CamelEvent", 2);
            sm.call("event123", 3);
            sm.call("snake_event", 4);
            sm.call("CamelEvent", 5);
            sm.call("event123", 6);
            Assert.AreEqual(new List<int> { 1103, 1307, 1211 }, sm.finalLog);
            Assert.AreEqual(new List<int> { 1307 }, sm.snake_log);
            Assert.AreEqual(new List<int> { 1103 }, sm.CamelLog);
            Assert.AreEqual(new List<int> { 1211 }, sm.log123);
        }

    }
}