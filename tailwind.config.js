/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: ["class"],
  content: [
    "./pages/**/*.{ts,tsx}",
    "./components/**/*.{ts,tsx}",
    "./app/**/*.{ts,tsx}",
    "./src/**/*.{ts,tsx}"
  ],
  safelist: [
    "rounded-t-sm",
    "rounded-b-sm",
    "rounded-sm",
    "rounded-t-md",
    "rounded-b-md",
    "rounded-md",
    "rounded-t-lg",
    "rounded-b-lg",
    "rounded-l-lg",
    "rounded-r-lg",
    "rounded-lg",
    "rounded-t-xl",
    "rounded-b-xl",
    "rounded-l-xl",
    "rounded-r-xl",
    "rounded-xl",
    "rounded-tl-sm",
    "rounded-bl-sm",
    "rounded-tr-sm",
    "rounded-br-sm",
    "rounded-tl-md",
    "rounded-bl-md",
    "rounded-tr-md",
    "rounded-br-md",
    "rounded-tl-lg",
    "rounded-bl-lg",
    "rounded-tr-lg",
    "rounded-br-lg",
    "rounded-tl-xl",
    "rounded-bl-xl",
    "rounded-tr-xl",
    "rounded-br-xl"
  ],
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px"
      }
    },
    extend: {
      fontFamily: {
        sans: ["var(--font-inter)"],
        mono: ["var(--font-roboto-mono)"]
      },
      borderWidth: {
        DEFAULT: "var(--border-width)",
        0: "0",
        1: "1px",
        2: "2px",
        3: "3px",
        4: "4px",
        5: "5px",
        6: "6px",
        8: "8px"
      },
      spacing: {
        1: "1px",
        5: "20px"
      },
      margin: {
        "1px": "1px",
        "5px": "5px"
      },
      padding: {
        "1px": "1px",
        "5px": "5px"
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)"
      },
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
          background: "hsl(var(--primary-background))"
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
          background: "hsl(var(--secondary-background))"
        },
        error: {
          DEFAULT: "hsl(var(--error))",
          foreground: "hsl(var(--error-foreground))"
        },
        success: {
          DEFAULT: "hsl(var(--success))",
          foreground: "hsl(var(--success-foreground))"
        },
        warning: {
          DEFAULT: "hsl(var(--warning))",
          foreground: "hsl(var(--warning-foreground))"
        },
        info: {
          DEFAULT: "hsl(var(--info))",
          foreground: "hsl(var(--info-foreground))"
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))"
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))"
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))"
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))"
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))"
        }
      },
      keyframes: {
        "accordion-down": {
          from: { height: 0 },
          to: { height: "var(--radix-accordion-content-height)" }
        },
        "accordion-up": {
          from: { height: "var(--radix-accordion-content-height)" },
          to: { height: 0 }
        },
        "border-pulse": {
          "50%": { borderColor: "transparent" }
        }
      },
      animation: {
        "accordion-down": "accordion-down 0.2s ease-out",
        "accordion-up": "accordion-up 0.2s ease-out",
        "border-pulse": "border-pulse 2s ease-in-out infinite"
      }
    }
  },
  plugins: [require("tailwindcss-animate")]
}
