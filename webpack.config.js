const fs = require('fs');
const path = require('path');

module.exports = {
    entry: {
        global: './assets/js/global.js',
        agency: './assets/js/agency.js',
        contact: './assets/js/contact.js',
        swiper: './assets/js/swiper.js',
        website_creation: './assets/js/website_creation.js'
    },
    watch: process.env.NODE_ENV === 'development',
    watchOptions: {
        ignored: /node_modules/
    },
    target: "browserslist",
    output: {
        path: path.resolve(__dirname, 'dist')
    },
    experiments: {
        syncWebAssembly: true,
        asset: true
    }
}