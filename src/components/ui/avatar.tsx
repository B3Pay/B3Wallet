import * as AvatarPrimitive from "@radix-ui/react-avatar"
import * as React from "react"

import { VariantProps, cva } from "class-variance-authority"
import { cn } from "lib/utils"

const avatarVariants = cva(
  "ml-1px shadow relative flex shrink-0 overflow-hidden",
  {
    compoundVariants: [
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
      }
    ],
    variants: {
      color: {
        primary: "bg-primary/75 border-primary text-primary",
        secondary: "bg-secondary/75 border-secondary text-secondary",
        error: "bg-error/75 border-error text-error",
        success: "bg-success/75 border-success text-success",
        warning: "bg-warning/75 border-warning text-warning",
        info: "bg-info/75 border-info text-info",
        muted: "bg-gray-400/75 border-gray-500"
      },
      variant: {
        default: "border-2 shadow text-background",
        filled: "shadow text-background",
        outline: "border-2 shadow bg-transparent",
        ghost: "shadow bg-transparent"
      },
      size: {
        xs: "h-6 w-6 text-xs",
        sm: "h-8 w-8 text-sm",
        md: "h-9 w-9 text-base",
        lg: "h-10 w-10 text-lg",
        xl: "h-12 w-12 text-xl"
      },
      round: {
        both: "rounded-full",
        left: "rounded-l-full",
        right: "rounded-r-full",
        none: "rounded-none",
        top: "rounded-t",
        bottom: "rounded-b",
        topLeft: "rounded-tl",
        topRight: "rounded-tr",
        bottomLeft: "rounded-bl",
        bottomRight: "rounded-br"
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

export interface AvatarProps
  extends Omit<
      React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>,
      "color"
    >,
    VariantProps<typeof avatarVariants> {
  asChild?: boolean
  noShadow?: boolean
  height?: string
  asButton?: boolean
}

const Avatar = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Root>,
  AvatarProps
>(
  (
    {
      className,
      asButton,
      noShadow,
      color,
      height,
      size,
      round,
      variant,
      ...props
    },
    ref
  ) => (
    <AvatarPrimitive.Root
      ref={ref}
      className={cn(
        avatarVariants({
          variant,
          color,
          size,
          round
        }),
        `rounded-${round}-${size}`,
        noShadow && "shadow-none",
        asButton && "cursor-pointer",
        className,
        height
      )}
      {...props}
    />
  )
)

Avatar.displayName = AvatarPrimitive.Root.displayName
Avatar.defaultProps = {
  variant: "default",
  color: "primary",
  round: "none",
  size: "md"
}

const AvatarImage = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Image>,
  React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>
>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image
    ref={ref}
    className={cn("aspect-square h-full w-full", className)}
    {...props}
  />
))
AvatarImage.displayName = AvatarPrimitive.Image.displayName

const AvatarFallback = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Fallback>,
  React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>
>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback
    ref={ref}
    className={cn("flex h-full w-full items-center justify-center", className)}
    {...props}
  />
))
AvatarFallback.displayName = AvatarPrimitive.Fallback.displayName

export { Avatar, AvatarFallback, AvatarImage }
