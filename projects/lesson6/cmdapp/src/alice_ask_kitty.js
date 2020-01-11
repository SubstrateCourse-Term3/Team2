const {Keyring} = require('@polkadot/api');
const Utils = require("./utils");
const assert = require('assert');

async function main() {
    const api = await Utils.getApi();
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice', {name: 'Alice'});
    let signal = false;

    console.log(process.argv);
    assert(process.argv.length === 4);
    const kitty_index = Number(process.argv[2]);
    const kitty_price = Number(process.argv[3]);

    // noinspection DuplicatedCode
    await api.tx.kitties.ask(kitty_index, kitty_price).signAndSend(alice, ({events = [], status}) => {
        events.forEach(({phase, event: {data, method, section}}) => {
            console.log(phase.toString() + ' : ' + section + '.' + method + ' ' + data.toString());
        });

        if (status.isFinalized) {
            console.log('Success', status.asFinalized.toHex());
            signal = true;
        } else {
            console.log('Status of transfer: ' + status.type);
        }
    });
    // noinspection InfiniteLoopJS
    for (; ;) {
        await Utils.sleep(100);
        if (signal) break;
    }
}

main().catch(console.error).finally(() => process.exit());






