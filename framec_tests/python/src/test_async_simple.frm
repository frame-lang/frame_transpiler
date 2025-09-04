from fsl import str

async fn fetch_data(url) {
    print("Fetching from " + url)
    return "data from " + url
}

system SimpleAsync {
    interface:
        async getData(id)
    
    machine:
        $Ready {
            getData(id) {
                print("Getting data for id: " + str(id))
                self.result = "data_" + str(id)
                return = self.result
            }
        }
    
    actions:
        logMessage(msg) {
            print("LOG: " + msg)
        }
    
    domain:
        var result = None
}

fn main() {
    print("Simple async test started")
    return
}