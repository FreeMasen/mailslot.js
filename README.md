# mailslot.js
A Rust implementation of the Windows Mailslot API wrapped in a [Neon](https://neon-bindings.com/) node module

## Basic Usage
```js
const mailslot = require('mailslot');
const NAME = 'node-mailslot-example';
const server = new mailslot.Server(NAME);
const client = new mailslot.Client(NAME);
function clientTick(ct) {
    if (ct > 10) return;
    client.send_message();
    setTimeout(serverTick, 1000, ct+1);
}
function serverTick(ct) {
    if (ct > 10) return;
    console.log(server.get_next_unread());
    setTimeout(clientTick, 500, ct);
}
clientTick(1);
```
> NOTE: The mailslot is ideally utilized from different processes so the server's `get_next_unread` method
> is blocking. 