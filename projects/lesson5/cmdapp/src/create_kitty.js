const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

const sleep = (milliseconds) => {
    return new Promise(resolve => setTimeout(resolve, milliseconds))
}

async function main() {
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({
        provider,
        types: {
            "Kitty": "[u8;16]",
            "KittyIndex": "u32",
            "KittyLinkedItem": {
                "prev": "Option<KittyIndex>",
                "next": "Option<KittyIndex>"
            }
        }
    });
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice', { name: 'Alice' });


    await api.tx.kitties.create().signAndSend(alice, ({ events = [], status }) => {
        if (status.isFinalized) {
            console.log('Success', status.asFinalized.toHex());
        } else {
            console.log('Status of transfer: ' + status.type);
        }

        events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(phase.toString() + ' : ' + section + '.' + method + ' ' + data.toString());
        });
    });
    await sleep(3000);
}

main().catch(console.error).finally(() => process.exit());




// const keyring = new Keyring({ type: 'sr25519' });
// const alice = keyring.addFromUri('//Alice', { name: 'Alice' });
// const bob = keyring.addFromUri('//Bob', { name: 'Bob' });
// const txHash = await api.tx.balances
//     .transfer("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", 123400500000)
//     .signAndSend(bob);
// console.log(`Submitted with hash ${txHash}`);


// const [chain, nodeName, nodeVersion] = await Promise.all([
//     api.rpc.system.chain(),
//     api.rpc.system.name(),
//     api.rpc.system.version()
// ]);
// console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`);


