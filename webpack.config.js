const path = require('path');
const entry_path = './assets/js/pages';

module.exports = {
    resolve: {
        alias: {
            '@js': path.resolve(__dirname, 'assets/js')
        }
    },
    entry: {
        global: './assets/js/global.js',
        admin: './assets/js/admin.js',
        index: `${entry_path}/index.js`,
        contact: `${entry_path}/contact.js`,
        agency: `${entry_path}/agency.js`,
        website_creation: `${entry_path}/website_creation.js`,
        project: `${entry_path}/portfolio_project.js`,
        faq: `${entry_path}/faq.js`,
        post: `${entry_path}/blog_post.js`,
        'admin/login': `${entry_path}/admin/login.js`,
        'admin/dashboard': `${entry_path}/admin/dashboard.js`
    },
    watch: process.env.NODE_ENV === 'development',
    watchOptions: {
        ignored: /node_modules/
    },
    target: 'browserslist',
    output: {
        path: path.resolve(__dirname, 'dist')
    },
    experiments: {
        syncWebAssembly: true
    }
}