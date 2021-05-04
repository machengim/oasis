module.exports = {
	mount: {
        public: '/',
		src: '/dist'
	},
	plugins: [
		["@snowpack/plugin-build-script", { cmd: "postcss", input: [".css"], output: [".css"] }]
	],
};