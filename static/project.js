const select = document.getElementById("selection");
const frame = document.getElementById("frame");

if (select.addEventListener){
    select.addEventListener('change', (event) => {
        frame.src = event.target.value;
        console.log("test");
    }, false)
}
else{
    select.attachEvent('onchange', (event) => {
        frame.src = event.target.value;
        console.log("test");
    }, false)
}