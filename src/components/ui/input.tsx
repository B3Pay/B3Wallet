import * as React from "react"

import { VariantProps, cva } from "class-variance-authority"
import { cn, focusRing } from "lib/utils"
import { Box } from "./box"
import { Icon } from "./icon"

const inputVariants = cva(
  "ml-1px flex h-9 w-full px-2 py-1 shadow text-foreground hover:border-foreground text-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
  {
    compoundVariants: [
      {
        isIcon: true,
        size: "xs",
        className: "pl-6"
      },
      {
        isIcon: true,
        size: "sm",
        className: "pl-7"
      },
      {
        isIcon: true,
        size: "md",
        className: "pl-8"
      },
      {
        isIcon: true,
        size: "lg",
        className: "pl-9"
      },
      {
        isIcon: true,
        size: "xl",
        className: "pl-10"
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
      round: {
        none: "rounded-none",
        both: "rounded-full",
        left: "rounded-l-full",
        right: "rounded-r-full",
        top: "rounded-t-sm",
        bottom: "rounded-b-sm",
        topLeft: "rounded-tl-sm",
        topRight: "rounded-tr-sm",
        bottomLeft: "rounded-bl-sm",
        bottomRight: "rounded-br-sm"
      },
      isIcon: {
        true: "pr-0"
      }
    },
    defaultVariants: {
      variant: "outline",
      color: "muted",
      round: "both",
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
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    {
      children,
      color,
      variant,
      icon,
      height,
      className,
      noShadow,
      round,
      size,
      type,
      ...props
    },
    ref
  ) => {
    const isIcon = !!icon
    const inputRef = React.useRef<HTMLInputElement>(null)

    function refHandler(instance: HTMLInputElement) {
      if (ref) {
        if (typeof ref === "function") {
          ref(instance)
        } else {
          ref.current = instance
        }
      }
      // @ts-ignore
      inputRef.current = instance
    }

    return (
      <Box className="relative flex items-center" color={color} hoverable>
        {isIcon && (
          <Icon
            className="absolute left-0"
            size={size}
            onClick={() => {
              inputRef.current && inputRef.current.focus()
            }}
          >
            {icon}
          </Icon>
        )}
        <input
          type={type}
          className={cn(
            focusRing,
            inputVariants({ round, variant, size, color, isIcon }),
            height,
            `rounded-${round}-${size}`,
            noShadow && "shadow-none",
            className
          )}
          ref={refHandler}
          {...props}
        />
        {children}
      </Box>
    )
  }
)
Input.displayName = "Input"

export { Input }
