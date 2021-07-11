import App from './App.svelte';

const path = window.location.pathname;

const app = new App({
	target: document.body,
	props: {
		path
	}
});

export default app;