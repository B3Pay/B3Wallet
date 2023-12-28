import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const colorVariants = cva("", {
  variants: {
    color: {
      primary: "bg-primary/5 border-primary",
      secondary: "bg-secondary/5 border-secondary",
      error: "bg-error/5 border-error",
      success: "bg-success/5 border-success",
      warning: "bg-warning/5 border-warning",
      info: "bg-info/5 border-info",
      muted: "bg-gray-400/5 border-gray-500"
    }
  }
})

const textColorVariants = cva("text-inherit", {
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

const paddingVariants = cva("p-0", {
  variants: {
    padding: {
      xs: "p-1",
      sm: "p-2",
      md: "p-3",
      lg: "p-4",
      xl: "p-5"
    }
  }
})

const marginVariants = cva("m-0", {
  variants: {
    margin: {
      xs: "m-1",
      sm: "m-2",
      md: "m-3",
      lg: "m-4",
      xl: "m-5"
    }
  }
})

const boxVariants = cva("transition-colors", {
  variants: {
    size: {
      xs: "text-xs",
      sm: "text-sm",
      md: "text-base",
      lg: "text-lg",
      xl: "text-xl"
    },
    hoverColor: {
      primary: "hover:text-primary-dark",
      secondary: "hover:text-secondary-dark",
      error: "hover:text-error-dark",
      success: "hover:text-success-dark",
      warning: "hover:text-warning-dark",
      info: "hover:text-info-dark",
      muted: "hover:text-gray-700"
    },
    hoverBgColor: {
      primary: "hover:bg-primary/50",
      secondary: "hover:bg-secondary/50",
      error: "hover:bg-error/50",
      success: "hover:bg-success/50",
      warning: "hover:bg-warning/50",
      info: "hover:bg-info/50",
      muted: "hover:bg-gray-200"
    },
    hoverable: {
      true: "hover:text-foreground"
    },
    border: {
      0: "border-0",
      1: "border-1",
      2: "border-2",
      3: "border-3",
      4: "border-4"
    }
  }
})

export interface BoxProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof boxVariants>,
    VariantProps<typeof colorVariants>,
    VariantProps<typeof paddingVariants>,
    VariantProps<typeof marginVariants> {
  asChild?: boolean
  hoverable?: boolean
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
}

const Box = React.forwardRef<HTMLDivElement, BoxProps>(
  (
    {
      asChild,
      color,
      hoverColor,
      hoverBgColor,
      hoverable,
      padding,
      margin,
      roundSize,
      border,
      size,
      children,
      className,
      ...props
    },
    ref
  ) => {
    const Comp = asChild ? Slot : "div"

    return (
      <Comp
        ref={ref}
        className={cn(
          boxVariants({
            border,
            size,
            hoverColor,
            hoverBgColor,
            hoverable
          }),
          colorVariants({ color }),
          paddingVariants({ padding }),
          marginVariants({ margin }),
          roundSize && `rounded-${roundSize}`,
          className
        )}
        {...props}
      >
        {children}
      </Comp>
    )
  }
)

Box.displayName = "Box"

export {
  Box,
  boxVariants,
  colorVariants,
  textColorVariants,
  paddingVariants,
  marginVariants
}
