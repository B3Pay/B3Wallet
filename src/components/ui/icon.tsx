import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "@src/lib/utils"
import * as React from "react"

const iconVariants = cva(
  "shadow transition-border-radius inline-flex items-center justify-center text-inherit p-0.5",
  {
    variants: {
      asButton: {
        true: "cursor-pointer"
      },
      color: {
        primary: "bg-primary/75 border-primary text-primary",
        secondary: "bg-secondary/75 border-secondary text-secondary",
        error: "bg-error/75 border-error text-error",
        success: "bg-success/75 border-success text-success",
        warning: "bg-warning/75 border-warning text-warning",
        info: "bg-info/75 border-info text-info",
        muted: "border-gray-500"
      },
      variant: {
        default: "border-2 text-foreground",
        filled: "text-foreground",
        outline: "border-2 bg-transparent",
        ghost: "bg-transparent",
        link: "shadow-none bg-transparent underline"
      },
      size: {
        xs: "h-5 w-5 text-xs",
        sm: "h-6 w-6 text-sm",
        md: "h-8 w-8 text-base",
        lg: "h-10 w-10 text-lg",
        xl: "h-12 w-12 text-xl"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "primary",
      size: "md"
    }
  }
)

export interface IconProps
  extends Omit<React.HTMLAttributes<HTMLSpanElement>, "color">,
    VariantProps<typeof iconVariants> {
  asButton?: boolean
  asChild?: boolean
  noShadow?: boolean
  diagonalRoundSide?: "r" | "l" | "none" | null
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | null
  roundSide?: "t" | "b" | "l" | "r" | "tl" | "tr" | "bl" | "br" | "none" | null
}

const Icon = React.forwardRef<HTMLSpanElement, IconProps>(
  (
    {
      children,
      asButton,
      roundSize = "xl",
      diagonalRoundSide,
      roundSide,
      variant,
      className,
      noShadow,
      color,
      size,
      asChild,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "span"

    const roundingClass = roundSide ? `rounded-${roundSide}-${roundSize}` : ""

    const diagonalRoundingClass =
      diagonalRoundSide === "r"
        ? `rounded-tr-${roundSize} rounded-bl-${roundSize}`
        : diagonalRoundSide === "l"
        ? `rounded-tl-${roundSize} rounded-br-${roundSize}`
        : ""

    return (
      <Comp
        ref={ref}
        className={cn(
          iconVariants({ size, variant, color, asButton }),
          noShadow && "shadow-none",
          roundingClass,
          diagonalRoundingClass,
          className
        )}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)

Icon.displayName = "Icon"
Icon.defaultProps = {
  size: "md"
}

export { Icon }
