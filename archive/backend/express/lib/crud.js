"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.readSingleRow = readSingleRow;

require("core-js/modules/es.promise.js");

function readSingleRow(db, sql, params) {
  if (params === void 0) {
    params = null;
  }

  if (!params) params = [];
  return new Promise(function (resolve) {
    db.get(sql, params, function (err, row) {
      if (err) {
        console.log(err);
      } else {
        return resolve(row);
      }
    });
  });
}