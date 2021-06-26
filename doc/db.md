# Database Structure

Note SQLite has no built-in `bool` or `time` data type.

### Table `site`

+ site_id
+ version: number // Database version number should be same with app version
+ first_run:   bool
+ lang: text
+ created_at: time
+ storage: text // Like `/home/ma/Oasis`

### Table `user`

When the site starts up the first time, the user will be an admin automatically.

+ user_id
+ username: text
+ password: text
+ permission: number   
+ created_at: time

### Table `node`

+ node_id
+ node_name: text  // Like `a001.png`
+ is_dir: bool 
+ owner_id       // The id of owner
+ parent_node_id      // The id of parent node.
+ created_at: time
+ updated_at: time

Notes: Categories include `Files`, `Starred` or `Vault`, are also considerred as `node`, but only controlled by the system instead of the user. Categories have owner so that every user's category update can be tracked for syncing needs. This implies every user has a set of categories with the same names in the database.

The final path of a file consists of several parts:
1. `storage` in `site` table, like `/home/ma/oasis`
2. user id: like `/1`
3. Category name, like `/files`
4. Node name with all its parent name, like `/documents/work/a001.png`

Currently rule 3 seems useless, but leave it here in case of future needs.