import getDb from './init.js';
import express from 'express';
import cors from 'cors';
import { readSingleRow } from './crud.js';

const db = getDb();
let sql = 'SELECT firstRun FROM site;';

readSingleRow(db, sql)
    .then(res => {
        if (res.firstRun == 1) {
            console.log('first');
            initSite(db, res.port);
        } else {
            console.log('not first');
        }
    });

const app = express();

function initSite(db, port) {
    app.use(cors());

    app.get('/', (req, res) => {
        res.send('Hello world');
    });

    app.listen(port, () => console.log(`Listening on port ${port}`));

    app.get('/config', (req, res) => {
        let sql = 'SELECT * FROM site;';
        readSingleRow(db, sql)
            .then(v => {
                res.send(v);
            });
    });
}



