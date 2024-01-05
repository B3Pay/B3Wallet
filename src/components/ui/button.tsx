import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"
import { cn, focusRing } from "@src/lib/utils"
import {
  bgColorVariants,
  borderColorVariants,
  hoverBgColorVariants
} from "@src/lib/variants"
import * as React from "react"

const buttonVariants = cva(
  "transition-border-radius inline-flex items-center justify-center whitespace-nowrap font-medium transition-colors whitespace-nowrap transition-colors focus-visible:outline-none disabled:pointer-events-none disabled:opacity-50",
  {
    compoundVariants: [
      {
        variant: "default",
        color: "muted",
        className: "text-foreground hover:text-background"
      },
      {
        variant: "outline",
        color: "muted",
        className: "text-gray-500"
      },
      {
        variant: ["outline", "default"],
        size: "xl",
        className: "border-3"
      },
      {
        variant: "filled",
        color: "primary",
        className: "bg-primary"
      },
      {
        variant: "filled",
        color: "secondary",
        className: "bg-secondary"
      },
      {
        variant: "filled",
        color: "error",
        className: "bg-error"
      },
      {
        variant: "filled",
        color: "success",
        className: "bg-success"
      },
      {
        variant: "filled",
        color: "warning",
        className: "bg-warning"
      },
      {
        variant: "filled",
        color: "info",
        className: "bg-info"
      },
      {
        asIconButton: true,
        size: "xs",
        className: "w-5"
      },
      {
        asIconButton: true,
        size: "sm",
        className: "w-6"
      },
      {
        asIconButton: true,
        size: "md",
        className: "w-8"
      },
      {
        asIconButton: true,
        size: "lg",
        className: "w-10"
      },
      {
        asIconButton: true,
        size: "xl",
        className: "w-12"
      },
      {
        variant: "ghost",
        color: ["primary", "secondary", "error", "success", "warning", "info"],
        className: "bg-transparent"
      }
    ],
    variants: {
      color: {
        primary: "focus:ring-primary",
        secondary: "focus:ring-secondary",
        error: "focus:ring-error",
        success: "focus:ring-success",
        warning: "focus:ring-warning",
        info: "focus:ring-info",
        muted: "border-gray-500"
      },
      variant: {
        default: "border-2 shadow text-foreground hover:border-foreground",
        filled: "text-foreground focus:border-1",
        outline:
          "border-2 shadow bg-transparent hover:text-foreground hover:border-foreground",
        ghost: "shadow hover:bg-accent hover:text-accent-foreground",
        link: "bg-transparent hover:bg-transparent underline focus:ring-offset-0 focus:ring-0"
      },
      size: {
        xs: "h-5 px-2 text-xs",
        sm: "h-6 px-3 text-sm",
        md: "h-8 px-4 text-base",
        lg: "h-10 px-8 text-lg",
        xl: "h-12 px-10 text-xl"
      },
      asIconButton: {
        true: "p-0"
      },
      innerShadow: {
        true: "shadow-button-inner"
      },
      noShadow: {
        true: "shadow-none"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "primary",
      size: "md"
    }
  }
)

export interface ButtonProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, "color">,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean
  height?: string
  fullWidth?: boolean
  asIconButton?: boolean
  innerShadow?: boolean
  noShadow?: boolean
  isLoading?: boolean
  iconSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
  diagonalRoundSide?: "r" | "l" | "none" | null
  roundSide?: "t" | "b" | "l" | "r" | "tl" | "tr" | "bl" | "br" | "none" | null
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    {
      className,
      children,
      color,
      variant,
      size,
      roundSize = "xl",
      diagonalRoundSide,
      roundSide,
      innerShadow,
      asIconButton,
      height,
      noShadow,
      isLoading,
      iconSize,
      fullWidth,
      asChild = false,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "button"

    const roundingClass = roundSide
      ? `rounded-${roundSide}-${roundSize}`
      : `rounded-${roundSize}`

    const diagonalRoundingClass =
      diagonalRoundSide === "r"
        ? `rounded-tr-${roundSize} rounded-bl-${roundSize}`
        : diagonalRoundSide === "l"
        ? `rounded-tl-${roundSize} rounded-br-${roundSize}`
        : ""

    return (
      <Comp
        className={cn(
          focusRing,
          buttonVariants({
            variant,
            color,
            size,
            asIconButton,
            noShadow,
            innerShadow
          }),
          borderColorVariants({ borderColor: color }),
          hoverBgColorVariants(50)({ hoverBgColor: color }),
          bgColorVariants(75)({ bgColor: color }),
          fullWidth && "w-full",
          roundingClass,
          diagonalRoundingClass,
          height,
          isLoading && "animate-border-pulse",
          className
        )}
        ref={ref}
        disabled={isLoading}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)
Button.displayName = "Button"

export { Button, buttonVariants }
