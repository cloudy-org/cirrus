import plugin from "tailwindcss/plugin";

export const Colours = plugin(
    ({ addUtilities, addComponents, e, config }) => {},
    {
        theme: {
            extend: {
                colors: {
                    cloudyDark: "#0a0909",
                    cloudyLight: "#b3c4c4",

                    exeBlack: "#090b11",
                    exeGray: "#9ca3af",
                    goldyPink: "#fb89ab",
                    goldyDarky: {
                        DEFAULT: "#0e1114",
                        200: "#0a0b0d",
                        300: "#0b0d0f",
                        500: "#0e1114"
                    },
                    goldyGreyy: {
                        DEFAULT: "#222930",
                        100: "#222930",
                        300: "#2A2B2C"
                    },
                    goldyCream: "#fbc689",
                    goldyWhite: "#f1f1f1",
                    goldyOrangy: {
                        DEFAULT: "#f5671b",
                        100: "#f5671b",
                        300: "#f57d3d",
                        800: "#f5be3d"
                    },
                    goldyGreen: "#d0f54c"
                }
            },
        }
    },
);