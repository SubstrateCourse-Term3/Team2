module.exports = {
    startBlockNumber: 0,
    endBlockNumber: -1,
    url: "ws://127.0.0.1:9944",
    types: {
        "Kitty": "[u8;16]",
        "KittyIndex": "u32",
        "KittyLinkedItem": {
            "prev": "Option<KittyIndex>",
            "next": "Option<KittyIndex>"
        }
    },
};