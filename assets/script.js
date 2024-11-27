const socket = new WebSocket('ws://localhost:4000/ws');

socket.addEventListener('open', function (event) {
    socket.send('Hello Server!');
});

socket.addEventListener('message', function (event) {
    console.log('Message from server ', event.data);
    const textArea = document.getElementById("text-area");
    // if (textArea.value == "") {
    //     textArea.value = event.data;
    // } else {
    //     textArea.value = textArea.value + "\n" + event.data;
    // }
    textArea.value = event.data;

    console.log(textArea.value);
});

setTimeout(() => {
    const obj = { hello: "world" };
    const blob = new Blob([JSON.stringify(obj, null, 2)], {
        type: "application/json",
    });
    console.log("Sending blob over websocket");
    socket.send(blob);
}, 1000);

// close connection button
// let closeConnectionButton = document.getElementById("close-connection-button");
// closeConnectionButton.addEventListener('input', function () {
//     socket.send('About done here...');
//     console.log("Sending close over websocket");
//     socket.close(4000, "Crash and Burn!");
// });

// ping button
let pingButton = document.getElementById("ping-button");
pingButton.addEventListener('click', function () {
    socket.send('Hello Server!');
    console.log("ping");
});

// text area listener
let textarea = document.getElementById("text-area");
let timeout;
textarea.addEventListener('input', function() {
    clearTimeout(timeout);

    timeout = setTimeout(() => {
        console.log(`Last updated text: "${textarea.value}"`);
        socket.send(textarea.value);
    }, 500)
});
