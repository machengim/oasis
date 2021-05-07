export function readSingleRow(db, sql, params = null) {
    if (!params) params = [];

    return new Promise(resolve => {
        db.get('SELECT * FROM site;', params, function(err, row) {
            if (err) {
                console.log(err);
            } else {
                return resolve(row);
            }
        });
    });
}