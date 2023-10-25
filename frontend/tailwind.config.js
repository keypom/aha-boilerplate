/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx}', './node_modules/flowbite/**/*.js'],
    theme: {
        extend: {
            dropShadow: {
                hard: '4px 4px 0px rgba(0,0,0,1)',
            },
            colors: {
                'brand-blue': '#1351B2',
                'brand-cream': '#F5F6F0',
            },
            fontFamily: {
                dela: ['Dela Gothic One', 'cursive'],
            },
        },
    },
    plugins: [require('flowbite/plugin')],
};
