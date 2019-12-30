const mailslot = require('../lib/index');
const NAME = 'node-mailslot-example';
const server = new mailslot.Server(NAME);
const client = new mailslot.Client(NAME);
function clientTick(ct) {
    if (ct > 10) return;
    client.send_message(`message: ${ct}`);
    setTimeout(serverTick, 500, ct+1);
}
function serverTick(ct) {
    if (ct > 10) return;
    console.log(server.get_next_unread());
    setTimeout(clientTick, 500, ct);
}
clientTick(1);