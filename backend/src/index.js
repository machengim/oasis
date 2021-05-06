import fs from 'fs';
import os from 'os';
import path from 'path';
import sqlite3 from 'sqlite3';

function getOS() {
    let sys = os.platform();

    if (sys !== 'win32' && sys !== 'darwin' && sys !== 'linux') {
        console.error('Error: only Windows, MacOS and Linux are supported!');
        process.exit(1);
    }

    return sys;
}

function getDb(sys) {
    let dbName = 'main.db';

    let dbFile = null;
    if (sys == 'win32') {
        dbFile = path.join(__dirname, dbName);  
    } else {
        dbFile = path.join(os.homedir(), '.config', 'oasis', dbName);
    }

    if (!fs.existsSync(dbFile)) {
        createNewDb(dbFile);
    }
}

function createNewDb(filename) {
    let dir = path.dirname(filename);
    console.log(dir);
    
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, {recursive: true});
        let db = new sqlite3.Database(filename);
        console.log(db);
    }
}

let sys = getOS();
getDb(sys);