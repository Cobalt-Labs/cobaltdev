document.addEventListener('DOMContentLoaded', () => {
    const magicBtn = document.getElementById('magicBtn');
    
    // Fun interactive bit of logic 
    magicBtn.addEventListener('click', () => {
        // Change button text
        magicBtn.textContent = 'Boom! 🎇';
        
        // Show an alert
        alert("Welcome to Ibrahim's custom landing page! Ready to code?");
        
        // Reset the button after 3 seconds so they can do it again
        setTimeout(() => {
            magicBtn.textContent = 'Click for Magic';
        }, 3000);
    });
});
