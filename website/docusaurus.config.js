// @ts-check

import { themes as prismThemes } from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: 'btd',
    tagline: 'Build. Transform. Deploy.',
    favicon: 'img/favicon.svg',
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
            logo: {
                alt: 'btd',
                src: 'img/logo.svg',
                srcDark: 'img/logo-dark.svg',
                height: '32px',
                width: '64px'
            },
            items: [
                {
                    type: 'docSidebar',
                    sidebarId: 'docSidebar',
                    position: 'left',
                    label: 'Docs',
                },
                {
                    href: 'https://crates.io/crates/btd',
                    label: 'crates.io',
                    position: 'right',
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
            copyright: `Copyright © ${new Date().getFullYear()} <a href="https://www.brandonfowler.me/">Brandon Fowler</a>`,
        },
        prism: {
            theme: prismThemes.github,
            darkTheme: prismThemes.dracula,
        },
    },
};

export default config;
