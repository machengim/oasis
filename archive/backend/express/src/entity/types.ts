export type Config = {
    id: number;
    firstRun: number;
    port: number;
	allowGuest: number;
    defaultUserGroup: number;
	lang: number;
	createAt: string;
}

export type ConfigPick = Pick<Config, 'firstRun'| 'allowGuest'|'lang'>;

interface Entity {
    checkEntity() : boolean;
}
