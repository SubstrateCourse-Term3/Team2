const Utils = require("./utils");
const assert = require('assert');

async function main(db) {
    const api = await Utils.getApi();
    // let [a0, a1] = await Promise.all([
    //     api.query.kitties.kitties(0),
    //     api.query.kitties.kitties(1110),
    // ]);
    // console.log(a0.toJSON() === null);
    // console.log(a1.toJSON() === null);
    let count = await api.query.kitties.kittiesCount().then(c => c.toNumber());
    for (let i = 0; i < count; i++) {
        let [dna, owner, price, db_kitty, db_price] = await Promise.all([
            api.query.kitties.kitties(i).then(a => a.toJSON()),
            api.query.kitties.kittyOwners(i).then(a => a.toJSON()),
            api.query.kitties.kittyPrices(i).then(a => a.toJSON()),
            db.getAsync("Select owner_addr, kitty_dna from t_kitties where kitty_index=?", i),
            db.getAsync("Select owner_addr, kitty_price from t_kitty_prices where kitty_index=?", i),
        ]);

        assert.strictEqual(db_kitty.owner_addr, owner);
        assert.strictEqual(db_kitty.kitty_dna, dna);

        if (price !== null) {
            assert.strictEqual(db_price.owner_addr, owner);
            assert.strictEqual(db_price.kitty_price, price);
        }

        console.log(`${i}: dna: ${dna}, owner: ${owner}, price: ${price}`);
    }
}

main(Utils.newSqlite3DB("kitties-sqlite-db"))
    .catch(Utils.logErr)
    .finally(() => process.exit(0));




