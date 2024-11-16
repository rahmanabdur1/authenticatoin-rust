const countDisplay = document.getElementById("count-display");
const incrementButton = document.getElementById("increment-btn");

// Fetch and display the current count
async function fetchCount() {
    try {
        const response = await fetch("/api/count");
        const text = await response.text();
        countDisplay.textContent = text;
    } catch (error) {
        console.error("Error fetching count:", error);
        countDisplay.textContent = "Error fetching count";
    }
}

// Increment the count and update display
async function incrementCount() {
    try {
        const response = await fetch("/api/add");
        const text = await response.text();
        countDisplay.textContent = text;
    } catch (error) {
        console.error("Error incrementing count:", error);
        countDisplay.textContent = "Error incrementing count";
    }
}

// Attach event listener to button
incrementButton.addEventListener("click", incrementCount);

// Load the count on page load
fetchCount();
