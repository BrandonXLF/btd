// @ts-check

import { themes as prismThemes } from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'Transform Deploy',
    tagline: 'Production build to deployment file transformer',
    favicon: 'img/favicon.ico',
    url: 'https://brandonxlf.github.io',
    baseUrl: '/tdep/',
    organizationName: 'brandonxlf',
    projectName: 'tdep',
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
                    editUrl: 'https://github.com/brandonxlf/tdep/tree/main/website/',
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
            title: 'Transform Deploy',
            items: [
                {
                    type: 'docSidebar',
                    sidebarId: 'docSidebar',
                    position: 'left',
                    label: 'Docs',
                },
                {
                    href: 'https://github.com/brandonxlf/tdep',
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
