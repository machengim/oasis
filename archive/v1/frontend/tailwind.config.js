module.exports = {
    purge: ['./public/**/*.html', './src/**/*.{js,jsx,ts,tsx,vue}'],
    theme: {
        extend: {
            colors: {
                'dark-gray': '#2b2b31',
                'light-gray': '#28282d',
            },
            fontSize: {
                icon: ['1.5rem', '1.5rem'],
            },
            backgroundImage: theme => ({
                'deer': "url('/home/images/deer.png')",
            }),
            maxWidth: {
                '4/5': '80%',
                '9/10': '90%',
            }
        }
    }
};