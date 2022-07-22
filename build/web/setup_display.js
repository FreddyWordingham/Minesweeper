(() => {
    const check_interval = 500; // ms
    let init_display = setInterval(function () {
        let canvas = document.body.getElementsByTagName("canvas");

        if (canvas.length > 0) {
            var fragment = document.createDocumentFragment();
            fragment.appendChild(canvas[0]);
            document.getElementById("wasm-window").appendChild(fragment);
            clearInterval(init_display);
        }
    }, check_interval);
})();
