import * as React from "react"

import { VariantProps, cva } from "class-variance-authority"
import { cn, focusRing } from "@src/lib/utils"
import { Box } from "./box"
import { Icon } from "./icon"
import { Cross2Icon } from "@radix-ui/react-icons"

const inputVariants = cva(
  "ml-1px flex h-9 w-full px-2 py-1 shadow text-foreground hover:border-foreground text-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
  {
    compoundVariants: [
      {
        isIcon: true,
        size: "xs",
        className: "pl-5"
      },
      {
        isIcon: true,
        size: "sm",
        className: "pl-6"
      },
      {
        isIcon: true,
        size: "md",
        className: "pl-7"
      },
      {
        isIcon: true,
        size: "lg",
        className: "pl-8"
      },
      {
        isIcon: true,
        size: "xl",
        className: "pl-9"
      },
      {
        variant: ["outline", "default"],
        size: "xl",
        className: "border-3"
      }
    ],
    variants: {
      color: {
        primary: "bg-primary/10 border-primary hover:bg-primary/25",
        secondary: "bg-secondary/10 border-secondary hover:bg-secondary/25",
        error: "bg-error/10 border-error hover:bg-error/25",
        success: "bg-success/10 border-success hover:bg-success/25",
        warning: "bg-warning/10 border-warning hover:bg-warning/25",
        info: "bg-info/10 border-info hover:bg-info/25",
        muted: "bg-gray-400/10 border-gray-500 hover:bg-gray-400/25"
      },
      variant: {
        default: "border-2",
        filled: "",
        outline: "border-2 bg-transparent",
        ghost: "bg-transparent hover:text-foreground hover:border-foreground"
      },
      size: {
        xs: "h-5 text-xs",
        sm: "h-6 text-sm",
        md: "h-8 px-3 text-base",
        lg: "h-10 px-4 text-lg",
        xl: "h-12 px-4 text-xl"
      },
      isIcon: {
        true: "pr-0"
      }
    },
    defaultVariants: {
      variant: "outline",
      color: "primary",
      size: "md"
    }
  }
)

export interface InputProps
  extends Omit<
      React.InputHTMLAttributes<HTMLInputElement>,
      "size" | "color" | "height"
    >,
    VariantProps<typeof inputVariants> {
  height?: string
  children?: React.ReactNode
  icon?: React.ReactNode
  noShadow?: boolean
  asChild?: boolean
  closeHandler?: () => void
  iconSize?: "xs" | "sm" | "md" | "lg" | "xl" | null
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
  roundSide?: "t" | "b" | "l" | "r" | "tl" | "tr" | "bl" | "br" | "none" | null
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    {
      children,
      color,
      variant,
      icon,
      iconSize,
      height,
      className,
      noShadow,
      roundSize = "xl",
      roundSide,
      size,
      type,
      closeHandler,
      ...props
    },
    ref
  ) => {
    const closable = !!closeHandler
    const isIcon = !!icon

    const roundingClass = roundSide
      ? `rounded-${roundSide}-${roundSize}`
      : `rounded-${roundSize}`

    return (
      <Box className={cn("relative", className)} bgColor={color} hoverable>
        {isIcon && (
          <Icon
            color={color}
            className="absolute p-1.5 top-1/2 left-1 transform -translate-y-1/2"
            size={iconSize || size}
            variant="ghost"
            noShadow
            asChild
          >
            {icon}
          </Icon>
        )}
        <input
          type={type}
          className={cn(
            focusRing,
            inputVariants({ variant, size, color, isIcon }),
            height,
            roundingClass,
            noShadow && "shadow-none",
            closable && "pr-8"
          )}
          {...props}
          ref={ref}
        />
        {children}
        {closable && (
          <Icon
            color="error"
            variant="ghost"
            noShadow
            className="absolute top-1/2 right-0 transform -translate-y-1/2 cursor-pointer"
            size={iconSize || size}
            onClick={() => {
              closeHandler()
            }}
          >
            <Cross2Icon />
          </Icon>
        )}
      </Box>
    )
  }
)
Input.displayName = "Input"

export { Input }
