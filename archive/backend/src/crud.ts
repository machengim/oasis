import { Database } from "sqlite3";

export function readSingleRow(db: Database, sql: string, params: any = null): Promise<any> {
    if (!params) params = [];

    return new Promise(resolve => {
        db.get(sql, params, function(err, row) {
            if (err) {
                console.log(err);
            } else {
                return resolve(row);
            }
        });
    });
}