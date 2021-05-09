"use strict";

var _init = _interopRequireDefault(require("./init.js"));

var _express = _interopRequireDefault(require("express"));

var _cors = _interopRequireDefault(require("cors"));

var _crud = require("./crud.js");

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var db = (0, _init.default)();
var sql = 'SELECT * FROM site;';
(0, _crud.readSingleRow)(db, sql).then(function (res) {
  console.log(res);
  var config = res;
  console.log(config);

  if (res.firstRun == 1) {
    console.log('first');
    initSite(db, res.port);
  } else {
    console.log('not first');
  }
});
var app = (0, _express.default)();

function initSite(db, port) {
  app.use((0, _cors.default)());
  app.get('/', function (req, res) {
    res.send('Hello world');
  });
  app.listen(port, function () {
    return console.log("Listening on port " + port);
  });
  app.get('/config', function (req, res) {
    var sql = 'SELECT * FROM site;';
    (0, _crud.readSingleRow)(db, sql).then(function (v) {
      res.send(v);
    });
  });
}