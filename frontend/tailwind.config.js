module.exports = {
  purge: ['./src/**/*.{js,jsx,ts,tsx}', './public/index.html'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    fill: theme => ({
      'red': theme('colors.red.400'),
      'green': theme('colors.green.400'),
      'yellow': theme('colors.yellow.400'),
    })
   },
  variants: {
    extend: {},
  },
  plugins: [],
}
