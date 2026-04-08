// NOTE TO STUDENT: 
// This is pure "Vanilla JavaScript". No frameworks, making it easy to explain!

// 1. This function handles the login form submission
function handleLogin(event) {
    // preventDefault stops the page from reloading.
    event.preventDefault();
    
    // Because we have no backend server, we don't actually verify a real password.
    // We just pretend it was successful and send (redirect) the user to the dashboard.
    window.location.href = "dashboard.html";
    
    return false; 
}

// 2. This function handles submitting the application form
function handleApplicationSubmit(event) {
    event.preventDefault(); // Stop page reload

    // Show a small browser popup to confirm
    alert("Success! Your scholarship application has been sent.");

    // TRICK: We use the browser's "localStorage" to save a quick temporary note.
    // This lets us remember across page loads that a new application was just added.
    localStorage.setItem("newApplicationAdded", "true");

    // Send them back to the dashboard to see their new status
    window.location.href = "dashboard.html";

    return false;
}

// 3. This runs automatically every time any page loads.
window.onload = function() {
    
    // We try to find the table on the page. 
    // It only exists on the dashboard.html page!
    const tableBody = document.getElementById("status-table-body");
    
    // If the table Body exists, we are definitely on the Dashboard page
    if (tableBody) {
        
        // Did we just submit a new application on the previous page?
        if (localStorage.getItem("newApplicationAdded") === "true") {
            
            // Create a brand new table row <tr>
            const newRow = document.createElement("tr");
            
            // Get today's local date (e.g., "Oct 12, 2024") to make it look realistic
            const todayDate = new Date().toLocaleDateString('en-US', { 
                month: 'short', day: 'numeric', year: 'numeric' 
            });
            
            // Add HTML data into the row 
            newRow.innerHTML = `
                <td>Newly Applied Scholarship</td>
                <td>${todayDate}</td>
                <td><span class="status pending">Pending Review</span></td>
            `;
            
            // Inject our new row at the bottom of the table
            tableBody.appendChild(newRow);
            
            // Clear out localStorage so it doesn't keep adding rows if they refresh the page
            localStorage.removeItem("newApplicationAdded");
        }
    }
}
