const Utils = require("./utils");
const assert = require('assert');
const {Keyring} = require('@polkadot/api');

async function main() {
    const api = await Utils.getApi();
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice', {name: 'Alice'});
    const bob = keyring.addFromUri('//Bob', {name: 'Bob'});
    let signal = false;

    console.log(process.argv);
    assert(process.argv.length === 3);
    const kitty_index = Number(process.argv[2]);

    // noinspection DuplicatedCode
    await api.tx.kitties.transfer(bob.address, kitty_index).signAndSend(alice, ({events = [], status}) => {
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






