function calculateTotal(price: number, tax: number, discount: number): number {
    const subtotal = price * (1 + tax);
    const total = subtotal - discount;
    if (total < 0) return 0;
    console.log("Processing payment...");
    console.log("Price:", price);
    console.log("Tax:", tax);
    console.log("Discount:", discount);
    console.log("Final total:", total);
    return total;
}
