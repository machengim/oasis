"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.default = getDb;

require("core-js/modules/es.regexp.to-string.js");

require("core-js/modules/es.string.split.js");

var _fs = _interopRequireDefault(require("fs"));

var _os = _interopRequireDefault(require("os"));

var _path = _interopRequireDefault(require("path"));

var _sqlite = _interopRequireDefault(require("sqlite3"));

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

// Get database connection.
// Create a database if not existed.
function getDb() {
  var sys = getOS();
  var dbName = 'main.db';
  var dbFile = '';

  if (sys == 'win32') {
    dbFile = _path.default.join(__dirname, dbName);
  } else {
    dbFile = _path.default.join(_os.default.homedir(), '.config', 'oasis', dbName);
  }

  if (!_fs.default.existsSync(dbFile)) {
    return createNewDb(dbFile);
  } else {
    return connectDb(dbFile);
  }
} // Get the Platform name and return it.
// Could be 'win32', 'darwin' or 'linux'.


function getOS() {
  var sys = _os.default.platform();

  if (sys !== 'win32' && sys !== 'darwin' && sys !== 'linux') {
    console.error('Error: only Windows, MacOS and Linux are supported!');
    process.exit(1);
  }

  return sys;
} // Open connection with a database.


function connectDb(filename) {
  var db = new _sqlite.default.Database(filename, function (e) {
    if (e) {
      console.error('Cannot connect to the database. ' + e);
      process.exit(1);
    }
  });
  return db;
} // Create a database according to the file path provided.


function createNewDb(filename) {
  var dir = _path.default.dirname(filename);

  console.log(dir);

  if (!_fs.default.existsSync(dir)) {
    _fs.default.mkdirSync(dir, {
      recursive: true
    });
  }

  var db = connectDb(filename);
  runInitSql(db);
  return db;
} // Run the initial SQL commands from the provided `init.sql` file.


function runInitSql(db) {
  var sqlCmdFile = _fs.default.readFileSync('./assets/init.sql').toString();

  var sqlCmdArray = sqlCmdFile.split(');');
  db.serialize(function () {
    db.run('BEGIN TRANSACTION;');
    sqlCmdArray.forEach(function (query) {
      if (query) {
        query += ');';
        db.run(query, function (e) {
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