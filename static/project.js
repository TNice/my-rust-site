const select = document.getElementById("selection");
const frame = document.getElementById("frame");

select.addEventListener('change', (event) => {
    frame.src = event.target.value;
    console.log("test");
}, false)

