import { CaretSortIcon, CheckIcon } from "@radix-ui/react-icons"
import * as SelectPrimitive from "@radix-ui/react-select"
import * as React from "react"

import { VariantProps, cva } from "class-variance-authority"
import { cn, focusRing } from "lib/utils"

const selectVariants = cva(
  "ml-1px flex items-center text-foreground justify-between whitespace-nowrap bg-transparent px-3 py-2 text-sm shadow placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1",
  {
    compoundVariants: [
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
        muted: "bg-gray-400/35 border-gray-500 hover:bg-gray-400/25"
      },
      variant: {
        default: "border-2",
        filled: "",
        outline: "border-2 bg-transparent",
        ghost: "bg-transparent hover:border-foreground"
      },
      size: {
        xs: "h-5 text-xs",
        sm: "h-6 text-sm",
        md: "h-8 text-base",
        lg: "h-10 text-lg",
        xl: "h-12 px-3 text-xl"
      },
      position: {
        popper:
          "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1"
      }
    },
    defaultVariants: {
      variant: "default",
      color: "primary",
      size: "md"
    }
  }
)

const Select = SelectPrimitive.Root

const SelectGroup = SelectPrimitive.Group

const SelectValue = SelectPrimitive.Value

export interface SelectProps
  extends Omit<
      React.ComponentPropsWithoutRef<typeof SelectPrimitive.Trigger>,
      "color" | "size" | "round"
    >,
    VariantProps<typeof selectVariants> {
  className?: string
  noShadow?: boolean
  icon?: React.ReactNode
  roundSize?: "xs" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl" | "full" | null
  roundSide?: "t" | "b" | "l" | "r" | "tl" | "tr" | "bl" | "br" | "none" | null
}

const SelectTrigger = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Trigger>,
  SelectProps
>(
  (
    {
      className,
      noShadow,
      icon,
      color,
      variant,
      roundSide,
      roundSize = "xl",
      size,
      children,
      ...props
    },
    ref
  ) => {
    const roundingClass = roundSide
      ? `rounded-${roundSide}-${roundSize}`
      : `rounded-${roundSize}`

    return (
      <SelectPrimitive.Trigger
        ref={ref}
        className={cn(
          focusRing,
          selectVariants({
            variant,
            color,
            size
          }),
          roundingClass,
          noShadow && "shadow-none",
          className
        )}
        {...props}
      >
        {children}
        <SelectPrimitive.Icon asChild>
          {icon || <CaretSortIcon className="h-4 w-4 opacity-50" />}
        </SelectPrimitive.Icon>
      </SelectPrimitive.Trigger>
    )
  }
)
SelectTrigger.displayName = SelectPrimitive.Trigger.displayName

const SelectContent = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Content>,
  React.ComponentPropsWithoutRef<typeof SelectPrimitive.Content>
>(({ className, children, position = "popper", ...props }, ref) => (
  <SelectPrimitive.Portal>
    <SelectPrimitive.Content
      ref={ref}
      className={cn(
        "relative z-50 min-w-[8rem] overflow-hidden rounded-md border border-slate-200 bg-white text-slate-950 shadow-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 dark:border-slate-800 dark:bg-slate-950 dark:text-slate-50",
        position === "popper" &&
          "data-[side=bottom]:translate-y-1 data-[side=left]:-translate-x-1 data-[side=right]:translate-x-1 data-[side=top]:-translate-y-1",
        className
      )}
      position={position}
      {...props}
    >
      <SelectPrimitive.Viewport
        className={cn(
          "p-1",
          position === "popper" &&
            "h-[var(--radix-select-trigger-height)] w-full min-w-[var(--radix-select-trigger-width)]"
        )}
      >
        {children}
      </SelectPrimitive.Viewport>
    </SelectPrimitive.Content>
  </SelectPrimitive.Portal>
))
SelectContent.displayName = SelectPrimitive.Content.displayName

const SelectLabel = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Label>,
  React.ComponentPropsWithoutRef<typeof SelectPrimitive.Label>
>(({ className, ...props }, ref) => (
  <SelectPrimitive.Label
    ref={ref}
    className={cn("px-2 py-1.5 text-sm font-semibold", className)}
    {...props}
  />
))
SelectLabel.displayName = SelectPrimitive.Label.displayName

const SelectItem = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Item>,
  React.ComponentPropsWithoutRef<typeof SelectPrimitive.Item>
>(({ className, children, ...props }, ref) => (
  <SelectPrimitive.Item
    ref={ref}
    className={cn(
      "relative flex w-full cursor-default select-none items-center rounded-sm py-1.5 pl-2 pr-8 text-sm outline-none focus:bg-slate-100 focus:text-slate-900 data-[disabled]:pointer-events-none data-[disabled]:opacity-50 dark:focus:bg-slate-800 dark:focus:text-slate-50",
      className
    )}
    {...props}
  >
    <span className="absolute right-2 flex h-3.5 w-3.5 items-center justify-center">
      <SelectPrimitive.ItemIndicator>
        <CheckIcon className="h-4 w-4" />
      </SelectPrimitive.ItemIndicator>
    </span>
    <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
  </SelectPrimitive.Item>
))
SelectItem.displayName = SelectPrimitive.Item.displayName

const SelectSeparator = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Separator>,
  React.ComponentPropsWithoutRef<typeof SelectPrimitive.Separator>
>(({ className, ...props }, ref) => (
  <SelectPrimitive.Separator
    ref={ref}
    className={cn("-mx-1 my-1 h-px bg-slate-100 dark:bg-slate-800", className)}
    {...props}
  />
))
SelectSeparator.displayName = SelectPrimitive.Separator.displayName

export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue
}
