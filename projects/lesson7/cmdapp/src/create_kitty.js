const {Keyring} = require('@polkadot/api');
const Utils = require("./utils");

async function main() {
    const api = await Utils.getApi();
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice', {name: 'Alice'});
    let signal = false;

    // noinspection DuplicatedCode
    await api.tx.kitties.create().signAndSend(alice, ({events = [], status}) => {
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


