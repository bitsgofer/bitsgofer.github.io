// Toggle .active class when clicking the mobile navigation button (mobile only)
const headerNavToggle = document.getElementById("header-nav-toggle");
const navList = document.querySelector(".header .nav-list");

headerNavToggle.onclick = function() {
    headerNavToggle.classList.toggle("active");
    navList.classList.toggle("active");
}
