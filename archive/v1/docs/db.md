# Database Structure

Note SQLite has no built-in `bool` or `time` data type.

### Table `site`

+ id 
+ firstRun:   bool
+ allow-guest: bool
+ lang: text
+ createAt: time

### Table `user`

When the site starts up the first time, the user will be an admin automatically.

+ id
+ username: text
+ password: text
+ group: number // id of group

### Table `group`

Initial group `Admin` and `User` cannot be removed
`Admin`: {id: 1, name: 'Admin', level: 9}
`User`: {id: 2, name: 'User', level: 2}

+ id
+ name: text
+ level: number
+ status: number    // whether the group has been removed. In case some items relate to a removed user group.

### Table `category`

+ id
+ name: text    // Could be *movie*, *tv series*, *photos*, etc..
+ permission: number

### Table `storage`

+ id
+ location: text    // like *d:/Movies* or */user/ma/Pictures*
+ name

### Table `item`

One movie is likely to have only one file, while TV shows may contain some sub folders with files inside them.

+ id
+ name: text
+ type: number // 1 means video, 2 images, 3 music, each has a different player.
+ category: the id of category
+ parent: number //the id of its parent item, only used in sub folders
+ permission
+ description: text

### Table `file`

+ id
+ name: text
+ item: number // the id of the item it belongs to
+ storage: number // the id of the storage it lives in 
+ path: text  // the relative path of this file on the storage

### Table `cast`

+ id
+ name: text
+ desription: text
+ permission: number // some artist may not be available to all users

### Table `item-cast`

+ id
+ item: number
+ artist: number
+ type: text // could be *actor*, *director*, *singer*, or null
+ order: number // indicates the importance of this artist in the specified item

### Table `tag`

+ id
+ name: text
+ permission: number

### Table `item-tag`

+ id
+ item
+ tag