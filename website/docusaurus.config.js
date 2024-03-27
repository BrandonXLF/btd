// @ts-check

import { themes as prismThemes } from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'btd',
    tagline: 'Build. Transform. Deploy.',
    favicon: 'img/favicon.ico',
    url: 'https://brandonxlf.github.io',
    baseUrl: '/btd/',
    organizationName: 'brandonxlf',
    projectName: 'btd',
    onBrokenLinks: 'throw',
    onBrokenMarkdownLinks: 'throw',
    i18n: {
        defaultLocale: 'en',
        locales: ['en'],
    },
    presets: [
        [
            'classic',
            /** @type {import('@docusaurus/preset-classic').Options} */
            {
                docs: {
                    path: '../docs',
                    routeBasePath: '/',
                    sidebarPath: './sidebars.js',
                    editUrl: 'https://github.com/brandonxlf/btd/tree/main/website/',
                },
                theme: {
                    customCss: './src/custom.css',
                },
            },
        ],
    ],
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    themeConfig: {
        navbar: {
            title: 'btd',
            items: [
                {
                    type: 'docSidebar',
                    sidebarId: 'docSidebar',
                    position: 'left',
                    label: 'Docs',
                },
                {
                    href: 'https://github.com/brandonxlf/btd',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            copyright: `Copyright © ${new Date().getFullYear()} Brandon Fowler`,
        },
        prism: {
            theme: prismThemes.github,
            darkTheme: prismThemes.dracula,
        },
    },
};

export default config;
