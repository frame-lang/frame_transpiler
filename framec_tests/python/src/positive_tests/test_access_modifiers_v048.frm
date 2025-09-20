# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Test Python-style access modifiers (v0.48)

class BankAccount {
    var account_count = 0  # Public class variable
    
    fn init(owner, balance) {
        self.owner = owner           # Public instance variable
        self._balance = balance      # Protected instance variable
        self.__pin = 1234           # Private instance variable
        BankAccount.account_count = BankAccount.account_count + 1
    }
    
    # Public method
    fn get_owner() {
        return self.owner
    }
    
    # Protected method
    fn _check_balance() {
        return self._balance
    }
    
    # Private method
    fn __validate_pin(pin) {
        return pin == self.__pin
    }
    
    # Public method using protected and private
    fn withdraw(amount, pin) {
        if not self.__validate_pin(pin) {
            return "Invalid PIN"
        }
        
        if amount > self._balance {
            return "Insufficient funds"
        }
        
        self._balance = self._balance - amount
        return "Withdrew: " + str(amount)
    }
    
    fn deposit(amount) {
        self._balance = self._balance + amount
        return "Deposited: " + str(amount)
    }
}

fn test_access_modifiers() {
    print("=== Testing Access Modifiers ===")
    
    var account = BankAccount("Alice", 1000)
    
    # Test public access
    print("Owner: " + account.get_owner())
    print("Account count: " + str(BankAccount.account_count))
    
    # Test public methods
    print(account.deposit(500))
    print(account.withdraw(200, 1234))
    print(account.withdraw(100, 9999))  # Wrong PIN
    
    print("=== Access Modifier Tests Complete ===")
}

test_access_modifiers()