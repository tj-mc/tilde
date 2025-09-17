// Complex benchmark test - Fibonacci + factorial + nested loops
// Equivalent JavaScript version for Bun

function fibonacci(n) {
    if (n <= 1) {
        return n;
    } else {
        const n1 = n - 1;
        const n2 = n - 2;
        const fib1 = fibonacci(n1);
        const fib2 = fibonacci(n2);
        return fib1 + fib2;
    }
}

function factorial(n) {
    if (n <= 1) {
        return 1;
    } else {
        const prev = n - 1;
        const result = factorial(prev);
        return n * result;
    }
}

function complexCalculation(iterations) {
    let total = 0;
    let counter = 0;

    while (true) {
        if (counter >= iterations) {
            break;
        }

        const fibResult = fibonacci(18);
        const factResult = factorial(8);

        const combined = fibResult + factResult;
        total = total + combined;

        let innerCounter = 0;
        while (true) {
            if (innerCounter >= 1000) {
                break;
            }
            total = total + 1;
            innerCounter = innerCounter + 1;
        }

        counter = counter + 1;
    }

    return total;
}

// Run the benchmark
console.log("Starting complex benchmark...");
const startTime = performance.now();

const result = complexCalculation(30);

const endTime = performance.now();
console.log("Benchmark completed!");
console.log(`Final result: ${result}`);
console.log(`Time taken: ${(endTime - startTime).toFixed(2)}ms`);