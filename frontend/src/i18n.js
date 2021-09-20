import { register } from "svelte-i18n";

register('en', () => import('./i18n/en.json'));
register('cn', () => import('./i18n/cn.json'));
