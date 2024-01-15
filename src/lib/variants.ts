import { cva } from "class-variance-authority"

export type ColorVariant = typeof colorVariants

export const colorVariants = cva("text-inherit", {
  variants: {
    color: {
      primary: "text-primary",
      secondary: "text-secondary",
      error: "text-error",
      success: "text-success",
      warning: "text-warning",
      info: "text-info",
      muted: "text-gray-500"
    }
  }
})

export type BgColorVariant = ReturnType<typeof bgColorVariants>

export type Opacity =
  | 0
  | 5
  | 10
  | 20
  | 25
  | 30
  | 40
  | 50
  | 60
  | 70
  | 75
  | 80
  | 90
  | 95
  | 100

export const bgColorVariants = (opacity: Opacity = 75) =>
  cva("", {
    variants: {
      bgColor: {
        primary: `bg-primary/${opacity}`,
        secondary: `bg-secondary/${opacity}`,
        error: `bg-error/${opacity}`,
        success: `bg-success/${opacity}`,
        warning: `bg-warning/${opacity}`,
        info: `bg-info/${opacity}`,
        alert: `bg-alert/${opacity}`,
        muted: `bg-gray-400/${opacity}`,
        inherit: "bg-inherit",
        card: "bg-card"
      },
      variant: {
        default: "",
        filled: "",
        outline: "bg-transparent hover:bg-transparent",
        ghost: "bg-transparent hover:bg-transparent",
        link: "bg-transparent hover:bg-transparent"
      }
    },
    defaultVariants: {
      bgColor: "card"
    }
  })

export type BgGradientVariant = ReturnType<typeof bgGradientVariants>

export const bgGradientVariants = (opacity: Opacity = 75) =>
  cva("", {
    variants: {
      bgGradient: {
        primary: `bg-gradient-to-r from-primary to-primary-dark`,
        secondary: `bg-gradient-to-r from-secondary to-secondary-dark`,
        error: `bg-gradient-to-r from-error to-error-dark`,
        success: `bg-gradient-to-r from-success to-success-dark`,
        warning: `bg-gradient-to-r from-warning to-warning-dark`,
        info: `bg-gradient-to-r from-info to-info-dark`,
        muted: `bg-gradient-to-r from-gray-400 to-gray-500`,
        inherit: "bg-inherit",
        card: "bg-card"
      }
    },
    defaultVariants: {
      bgGradient: "card"
    }
  })

export type HoverColorVariant = typeof hoverColorVariants

export const hoverColorVariants = cva("", {
  variants: {
    hoverColor: {
      primary: "hover:text-primary-dark",
      secondary: "hover:text-secondary-dark",
      error: "hover:text-error-dark",
      success: "hover:text-success-dark",
      warning: "hover:text-warning-dark",
      info: "hover:text-info-dark",
      muted: "hover:text-gray-700"
    }
  }
})

export type HoverBgColorVariant = ReturnType<typeof hoverBgColorVariants>

export const hoverBgColorVariants = (opacity: number = 100) =>
  cva("", {
    variants: {
      hoverBgColor: {
        primary: `hover:bg-primary/${opacity}`,
        secondary: `hover:bg-secondary/${opacity}`,
        error: `hover:bg-error/${opacity}`,
        success: `hover:bg-success/${opacity}`,
        warning: `hover:bg-warning/${opacity}`,
        info: `hover:bg-info/${opacity}`,
        alert: `hover:bg-alert/${opacity}`,
        muted: `hover:bg-gray-400/${opacity}`
      }
    }
  })

export type BackgroundColorVariant = typeof backgroundColorVariants

export const backgroundColorVariants = cva("", {
  variants: {
    bgColor: {
      primary: "bg-primary",
      secondary: "bg-secondary",
      error: "bg-error",
      success: "bg-success",
      warning: "bg-warning",
      info: "bg-info",
      muted: "bg-gray-400"
    }
  },
  defaultVariants: {
    bgColor: "primary"
  }
})

export type BorderColorVariant = typeof borderColorVariants

export const borderColorVariants = cva("", {
  variants: {
    borderColor: {
      primary: "border-primary",
      secondary: "border-secondary",
      error: "border-error",
      success: "border-success",
      warning: "border-warning",
      info: "border-info",
      alert: "border-alert",
      muted: "border-gray-500"
    }
  },
  defaultVariants: {
    borderColor: "primary"
  }
})

export type BorderRadiusVariant = typeof borderRadiusVariants

export const borderRadiusVariants = cva("rounded-none", {
  variants: {
    borderRadius: {
      none: "rounded-none",
      sm: "rounded-sm",
      md: "rounded-md",
      lg: "rounded-lg",
      xl: "rounded-xl",
      full: "rounded-full"
    }
  }
})

export type BorderWidthVariant = typeof borderWidthVariants

export const borderWidthVariants = cva("border-0", {
  variants: {
    borderWidth: {
      none: "border-0",
      xs: "border-1",
      sm: "border-2",
      md: "border-3",
      lg: "border-4",
      xl: "border-5"
    }
  }
})

export type PaddingVariant = typeof paddingVariants

export const paddingVariants = cva("p-0", {
  variants: {
    padding: {
      none: "p-0",
      xs: "p-1",
      sm: "p-2",
      md: "p-3",
      lg: "p-4",
      xl: "p-5"
    },
    paddingTop: {
      none: "pt-0",
      xs: "pt-1",
      sm: "pt-2",
      md: "pt-3",
      lg: "pt-4",
      xl: "pt-5"
    },
    paddingRight: {
      none: "pr-0",
      xs: "pr-1",
      sm: "pr-2",
      md: "pr-3",
      lg: "pr-4",
      xl: "pr-5"
    },
    paddingBottom: {
      none: "pb-0",
      xs: "pb-1",
      sm: "pb-2",
      md: "pb-3",
      lg: "pb-4",
      xl: "pb-5"
    },
    paddingLeft: {
      none: "pl-0",
      xs: "pl-1",
      sm: "pl-2",
      md: "pl-3",
      lg: "pl-4",
      xl: "pl-5"
    }
  }
})

export type MarginVariant = typeof marginVariants

export const marginVariants = cva("m-0", {
  variants: {
    margin: {
      none: "m-0",
      xs: "m-1",
      sm: "m-2",
      md: "m-3",
      lg: "m-4",
      xl: "m-5"
    },
    marginTop: {
      none: "mt-0",
      xs: "mt-1",
      sm: "mt-2",
      md: "mt-3",
      lg: "mt-4",
      xl: "mt-5"
    },
    marginRight: {
      none: "mr-0",
      xs: "mr-1",
      sm: "mr-2",
      md: "mr-3",
      lg: "mr-4",
      xl: "mr-5"
    },
    marginBottom: {
      none: "mb-0",
      xs: "mb-1",
      sm: "mb-2",
      md: "mb-3",
      lg: "mb-4",
      xl: "mb-5"
    },
    marginLeft: {
      none: "ml-0",
      xs: "ml-1",
      sm: "ml-2",
      md: "ml-3",
      lg: "ml-4",
      xl: "ml-5"
    }
  }
})
