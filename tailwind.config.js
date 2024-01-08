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
    ...generateTailwindSafelist(),
    "-rotate-y-0",
    "-rotate-y-180",
    "rotate-y-180"
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
      backfaceVisibility: {
        hidden: "hidden",
        visible: "visible"
      },
      transitionDuration: {
        custom: "0.2s"
      },
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
      boxShadow: {
        "button-inner": "inset 0 0px 5px 0 rgba(0, 0, 0, 0.25)"
      },
      backgroundImage: {
        "line-middle":
          "linear-gradient(to right, transparent 49.8%, grey 49.8%, grey 50.2%, transparent 50.2%)"
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
  plugins: [
    require("tailwindcss-animate"),
    require("@xpd/tailwind-3dtransforms")
  ]
}

function generateTailwindSafelist() {
  const sizes = ["sm", "md", "lg", "xl", "2xl", "none", "0", "2", "3", "4"]
  const sides = ["", "t", "b", "l", "r", "tl", "tr", "bl", "br"]
  const properties = ["rounded", "border"]
  const colors = [
    "primary",
    "secondary",
    "error",
    "success",
    "warning",
    "info",
    "muted",
    "inherit",
    "card"
  ]
  const opacity = [
    "5",
    "10",
    "20",
    "30",
    "40",
    "50",
    "60",
    "70",
    "80",
    "90",
    "100"
  ]

  const safelist = []

  properties.forEach(prop => {
    if (prop === "rounded") {
      sides.forEach(side => {
        sizes.slice(0, -3).forEach(size => {
          safelist.push(`${prop}-${side ? side + "-" : ""}${size}`)
        })
      })
    } else if (prop === "border") {
      ;["t", "b", "l", "r", ""].forEach(side => {
        sizes.slice(-3).forEach(size => {
          safelist.push(`${prop}-${side}${side ? "-" : ""}${size}`)
        })
      })
    }
  })

  colors.forEach(color => {
    opacity.forEach(opacity => {
      safelist.push(`bg-${color}/${opacity}`)
    })
  })

  return safelist
}
