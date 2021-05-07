import fs from 'fs';
import os from 'os';
import path from 'path';
import sqlite3 from 'sqlite3';

// Get database connection.
// Create a database if not existed.
export default function getDb() {
    const sys = getOS();
    const dbName = 'main.db';

    let dbFile = null;
    if (sys == 'win32') {
        dbFile = path.join(__dirname, dbName);  
    } else {
        dbFile = path.join(os.homedir(), '.config', 'oasis', dbName);
    }

    if (!fs.existsSync(dbFile)) {
        return createNewDb(dbFile);
    } else {
        return connectDb(dbFile);
    }
}

// Get the Platform name and return it.
// Could be 'win32', 'darwin' or 'linux'.
function getOS() {
    let sys = os.platform();

    if (sys !== 'win32' && sys !== 'darwin' && sys !== 'linux') {
        console.error('Error: only Windows, MacOS and Linux are supported!');
        process.exit(1);
    }

    return sys;
}



// Open connection with a database.
function connectDb(filename) {
    const db = new sqlite3.Database(filename, e => {
        if (e) {
            console.error('Cannot connect to the database. ' + e);
            process.exit(1);
        }
    });

    return db;
}

// Create a database according to the file path provided.
function createNewDb(filename) {
    const dir = path.dirname(filename);
    console.log(dir);
    
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, {recursive: true});
    }

    const db = connectDb(filename);
    runInitSql(db);

    return db;
}

// Run the initial SQL commands from the provided `init.sql` file.
function runInitSql(db) {
    const sqlCmdFile = fs.readFileSync('./assets/init.sql').toString();
    const sqlCmdArray = sqlCmdFile.split(');');
    
    db.serialize(() => {
        db.run('BEGIN TRANSACTION;');

        sqlCmdArray.forEach((query) => {
            if (query) {
                query += ');';
                db.run(query, e => {
                    if (e) {
                        console.log('Cannot initialize databse. ' + e);
                        process.exit(1);
                    }
                });
            }
        });

        db.run('COMMIT;');
    });
}