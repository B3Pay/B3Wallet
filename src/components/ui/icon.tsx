import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const iconVariants = cva(
  "inline-flex items-center justify-center text-inherit p-1",
  {
    variants: {
      asButton: {
        true: "p-0 cursor-pointer"
      },
      color: {
        primary: "bg-primary/25 border-primary/50 text-primary",
        secondary: "bg-secondary/25 border-secondary/50 text-secondary",
        error: "bg-error/25 border-error/50 text-error",
        success: "bg-success/25 border-success/50 text-success",
        warning: "bg-warning/25 border-warning/50 text-warning",
        info: "bg-info/25 border-info/50 text-info",
        muted: "border-gray-500"
      },
      variant: {
        default: "border-2 shadow text-foreground",
        filled: "shadow text-foreground",
        outline: "border-2 shadow bg-transparent",
        ghost: "shadow bg-transparent",
        link: "bg-transparent underline focus:ring-offset-0 focus:ring-0"
      },
      round: {
        none: "rounded-none",
        both: "rounded-full",
        left: "rounded-l-full",
        right: "rounded-r-full",
        t: "rounded-t",
        b: "rounded-b",
        tl: "rounded-tl",
        tr: "rounded-tr",
        bl: "rounded-bl",
        br: "rounded-br"
      },
      size: {
        xs: "w-5 h-5",
        sm: "w-8 h-8 p-1.5",
        md: "w-9 h-9 p-1.5",
        lg: "w-10 h-10 p-2",
        xl: "w-12 h-12 p-2"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "primary",
      round: "none",
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
}

const Icon = React.forwardRef<HTMLSpanElement, IconProps>(
  (
    {
      children,
      asButton,
      round,
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

    return (
      <Comp
        ref={ref}
        className={cn(
          iconVariants({ size, variant, round, color, asButton }),
          noShadow && "shadow-none",
          size && size !== "xs" && `rounded-${round}-${size}`,
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
