import { register } from "svelte-i18n";

register('en', () => import('./assets/i18n/en.json'));
register('cn', () => import('./assets/i18n/cn.json'));
