const fs = require('fs');
const path = require('path');

module.exports = {
    entry: {
        global: './assets/js/global.js',
        contact: './assets/js/contact.js',
        carousel: './assets/js/carousel.js',
        website_creation: './assets/js/website_creation.js',
        motion_home_player: './assets/js/motion_home_player.js',
        project: './assets/js/project.js',
        faq: './assets/js/faq.js'
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
        syncWebAssembly: true
    }
}