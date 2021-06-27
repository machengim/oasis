module.exports = {
	mount: {
    public: '/',
		src: '/dist'
	},
	buildOptions: {
		watch: true
	},
	devOptions: {
		tailwindConfig: './tailwind.config.js',
	},
	plugins: [
		'@snowpack/plugin-postcss',
		["@snowpack/plugin-build-script", { cmd: "postcss", input: [".css"], output: [".css"] }]
	],
};