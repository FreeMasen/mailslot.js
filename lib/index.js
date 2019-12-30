var addon = require('../native');

module.exports = {
    Server: addon.MailSlotServer,
    Client: addon.MailSlotClient,
};
