const {ApiPromise, WsProvider} = require('@polkadot/api');
const Config = require("./config");
const Sqlite3 = require('sqlite3').verbose();

// noinspection JSUnusedGlobalSymbols
module.exports = {
    newSqlite3DB(dbName) {
        // https://github.com/mapbox/node-sqlite3/wiki/API#databaseexecsql-callback
        let db = new Sqlite3.Database(dbName, Sqlite3.OPEN_READWRITE | Sqlite3.OPEN_CREATE, this.rethrow);
        db.runAsync = function (sql, ...args) {
            let that = this;
            return new Promise(function (resolve, reject) {
                that.run(sql, ...args, function (err) {
                    if (err)
                        reject(err);
                    else
                        resolve({lastID: this.lastID, changes: this.changes});
                });
            });
        };
        db.getAsync = function (sql, ...args) {
            let that = this;
            return new Promise(function (resolve, reject) {
                that.get(sql, ...args, function (err, row) {
                    if (err)
                        reject(err);
                    else
                        resolve(row);
                });
            });
        };
        db.allAsync = function (sql, ...args) {
            let that = this;
            return new Promise(function (resolve, reject) {
                that.all(sql, ...args, function (err, rows) {
                    if (err)
                        reject(err);
                    else
                        resolve(rows);
                });
            });
        };
        return db;
    },
    rethrow(e) {
        if (e) throw e;
    },
    logErr(e) {
        if (e) console.log(e);
    },
    async getApi() {
        return ApiPromise.create({
            provider: new WsProvider(Config.url),
            types: Config.types,
        });
    },
    sleep(milliseconds) {
        return new Promise(resolve => setTimeout(resolve, milliseconds))
    },
    async getFinalizedHeadNumber(api) {
        return api.rpc.chain.getFinalizedHead()
            .then((hash) => api.rpc.chain.getBlock(hash))
            .then((block) => block.block.header.number.toNumber());
    },
    async getStartNumber(api) {
        // 确定从哪个块开始
        if (Config.startBlockNumber === 0) {
            return this.getFinalizedHeadNumber(api);
        } else {
            return Config.startBlockNumber;
        }
    },
    async getBlockByNumber(api, num) {
        return api.rpc.chain.getBlockHash(num).then(hash => api.rpc.chain.getBlock(hash));
    },
    async getEventsByNumber(api, num) {
        return api.rpc.chain.getBlockHash(num).then(hash => api.query.system.events.at(hash));
    },
};