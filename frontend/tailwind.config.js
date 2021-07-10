module.exports = {
    purge: ['./public/**/*.html', './src/**/*.{js,jsx,ts,tsx,vue}'],
    theme: {
        fill: theme => ({
          'red': theme('colors.red.400'),
          'green': theme('colors.green.400'),
          'yellow': theme('colors.yellow.400'),
        })
       }
};