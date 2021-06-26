module.exports = {
	mount: {
    public: '/',
		src: '/dist'
	},
	buildOptions: {
		watch: true
	},
	plugins: [
		["@snowpack/plugin-build-script", { cmd: "postcss", input: [".css"], output: [".css"] }]
	],
};