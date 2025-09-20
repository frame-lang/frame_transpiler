# DO NOT MODIFY THIS TEST WITHOUT EXPLICIT PERMISSION
# Formatter functions for complex multi-file test
# This module exports individual functions, not a module

fn formatNumber(value) {
    # Format number to 2 decimal places
    return str(round(value * 100) / 100.0)
}

fn parseNumber(text) {
    return float(text)
}

fn formatCurrency(amount) {
    return "$" + formatNumber(amount)
}

fn formatPercentage(value) {
    return str(round(value * 100)) + "%"
}