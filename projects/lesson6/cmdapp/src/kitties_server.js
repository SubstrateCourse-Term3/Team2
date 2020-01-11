const Config = require("./config");
const Utils = require("./utils");

async function handleEvents(db, api, num) {
    // 读取块中所有事件
    const events = await Utils.getEventsByNumber(api, num);
    for (const record of events) {
        // console.log(record.toJSON());
        const {event} = record;
        // const types = event.typeDef;
        // 读取 kitties 相关的事件
        if (event.section === 'kitties') {
            console.log(`received kitties event: ${event.method}: ${event.meta.documentation.toString()}`);
            let kittyEvent = [];
            for (const data of event.data) {
                kittyEvent.push(data.toString());
            }
            switch (event.method) {
                case 'Created': {
                    // A kitty is created. (owner, kitty_id) Created(AccountId, KittyIndex)
                    let kitty = await api.query.kitties.kitties(kittyEvent[1]);
                    let dna = kitty.toJSON();
                    if (dna !== null) {
                        await db.runAsync("insert into t_kitties(owner_addr, kitty_index, kitty_dna) values (?,?,?)", kittyEvent[0], kittyEvent[1], dna);
                    }
                    break;
                }
                case 'Transferred': {
                    // A kitty is transferred. (from, to, kitty_id) Transferred(AccountId, AccountId, KittyIndex)
                    await db.runAsync("update t_kitties set owner_addr=? where kitty_index=?", kittyEvent[1], kittyEvent[2]);
                    break;
                }
                case 'Ask': {
                    // A kitty is available for sale. (owner, kitty_id, price) Ask(AccountId, KittyIndex, Option<Balance>),
                    if (kittyEvent[2] === null) {
                        await db.runAsync("delete from t_kitty_prices where kitty_index=?", kittyEvent[1]);
                    } else {
                        await db.runAsync("insert into t_kitty_prices(owner_addr, kitty_index, kitty_price) values (?,?,?)", kittyEvent[0], kittyEvent[1], kittyEvent[2]);
                    }
                    break;
                }
                case 'Sold': {
                    // A kitty is sold. (seller, buyer, kitty_id, price) Sold(AccountId, AccountId, KittyIndex, Balance),
                    // 此处最好是事务, 但是不是也没问题, 会重试, 直到这个区块被妥善处理.
                    await Promise.all([
                        db.runAsync("delete from t_kitty_prices where kitty_index=?", kittyEvent[2]),
                        db.runAsync("update t_kitties set owner_addr=? where kitty_index=?", kittyEvent[1], kittyEvent[2]),
                    ]);
                    break;
                }
                default: {
                    console.log(`unknown event: ${event.method}`);
                    break;
                }
            }
        }
    }
}

async function run(db) {
    const [api] = await Promise.all([
        Utils.getApi(),
        createTables(db),
    ]);
    let start = await Utils.getStartNumber(api);
    console.log(`-------------------start from ${start}`);
    // noinspection InfiniteLoopJS
    for (; ;) {
        if (Config.endBlockNumber !== -1) {
            if (Config.endBlockNumber < start) {
                process.exit(0);
            }
        }
        try {
            const last = await Utils.getFinalizedHeadNumber(api);
            if (start <= last) {
                console.log(`check block ${start}`);
                await handleEvents(db, api, start);
                start++;
            } else {
                // 等待区块链出现新的finalized区块.
                await Utils.sleep(200);
            }
        } catch (e) {
            console.log(e);
            await Utils.sleep(2000)
        }
    }
}

async function createTables(db) {
    return Promise.all([
        db.runAsync(` CREATE TABLE IF NOT EXISTS t_kitties
                      (
                          kitty_index INTEGER default 0 not null primary key,
                          owner_addr  TEXT    default '' not null,
                          kitty_dna   TEXT    default '' not null
                      )`)
            .then((_x) => {
                return db.runAsync(`create index if not exists kitty_owner on t_kitties (owner_addr)`);
            }),
        db.runAsync(` CREATE TABLE IF NOT EXISTS t_kitty_prices
                      (
                          kitty_index INTEGER default 0 not null primary key,
                          owner_addr  TEXT    default '' not null,
                          kitty_price INTEGER default 0 not null
                      )`)
            .then((_x) => {
                return db.runAsync(`create index if not exists price on t_kitty_prices (kitty_price)`);
            })
            .then((_x) => {
                return db.runAsync(`create index if not exists price_owner on t_kitty_prices (owner_addr)`);
            }),
    ]);
}

function main() {
    let db = Utils.newSqlite3DB("kitties-sqlite-db");
    run(db).catch(Utils.rethrow).finally(() => db.close(Utils.rethrow));
}

main();

