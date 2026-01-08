function computeFinalAmount(amount: number, vat: number, reduction: number): number {
    const intermediate = amount * (1 + vat);
    const result = intermediate - reduction;
    if (result < 0) return 0;
    console.log("Processing payment...");
    console.log("Price:", amount);
    console.log("Tax:", vat);
    console.log("Discount:", reduction);
    console.log("Final total:", result);
    return result;
}

function dummy() {
    return "hello";
}
