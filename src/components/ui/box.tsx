import { Slot } from "@radix-ui/react-slot"
import { cva, VariantProps } from "class-variance-authority"
import { cn } from "lib/utils"
import * as React from "react"

const boxVariants = cva("p-0 m-0 transition-colors", {
  variants: {
    color: {
      default: "text-foreground",
      primary: "text-primary",
      secondary: "text-secondary",
      error: "text-error",
      success: "text-success",
      warning: "text-warning",
      info: "text-info",
      muted: "text-gray-500"
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
    bgColor: {
      primary: "bg-primary",
      secondary: "bg-secondary",
      error: "bg-error",
      success: "bg-success",
      warning: "bg-warning",
      info: "bg-info",
      muted: "bg-gray-400"
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
    }
  },
  defaultVariants: {
    color: "default"
  }
})

export interface BoxProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, "color">,
    VariantProps<typeof boxVariants> {
  asChild?: boolean
  hoverable?: boolean
}

const Box = React.forwardRef<HTMLDivElement, BoxProps>(
  (
    {
      asChild,
      color,
      bgColor,
      hoverColor,
      hoverBgColor,
      hoverable,
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
            color,
            bgColor,
            hoverColor,
            hoverBgColor,
            hoverable
          }),
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

export { Box }
