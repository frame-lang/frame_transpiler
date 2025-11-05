@target typescript

system TabsSpacesTS {
    operations:
    op1() {
		// mixed tabs and spaces above
		const a = 1;
        -> $Next
    }
    machine:
        $Init { op1() { } }
        $Next {}
}

