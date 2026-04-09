document.addEventListener("DOMContentLoaded", function() {
    const yearSpan = document.getElementById("year");
    if(yearSpan) {
        yearSpan.textContent = new Date().getFullYear();
    }
    
    const galleryItems = document.querySelectorAll(".gallery-item img");
    galleryItems.forEach(img => {
        img.addEventListener("click", () => {
            console.log("You clicked on picture showing:", img.alt);
        });
    });
});