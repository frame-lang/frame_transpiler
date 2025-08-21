fn main() {
    var lamp = Lamp()
    lamp.turnOn()
    lamp.turnOff()
}

system Lamp {

    interface:

        turnOn()      
        turnOff()

    machine:

        $Off {   
            $>() {
                print("Entering $Off")
                return
            }
            <$() {
                print("Exiting $Off")
                return
            }

            turnOn() {            
                -> $On
                return
            }              
        }

        $On {  
            $>() {
                print("Entering $On")
                return
            }
            <$() {
                print("Exiting $On")
                return
            }
            
            turnOff() {           
                -> $Off
                return
            }               
        }

}