import { useMediaQuery } from "@mui/material";

export interface NamedColor {
  type_: 'named';
  name: string;
}

export interface CustomColor {
  type_: 'custom';
  r: number;
  g: number;
  b: number;
}

export type Color = NamedColor | CustomColor;

export function color2string(color: Color): string {
  if (color.type_ === 'named') {
    return color.name;
  } else {
    return `rgb(${color.r}, ${color.g}, ${color.b})`;
  }
}

export interface HighlightStyle {
  color: Color;
  is_background: boolean;
}

export interface HighlightTarget {
  name: string;
  is_target: boolean;
  style: HighlightStyle;
}

const highlightTargetNew = (
  name: string,
  color: string,
  is_background: boolean,
): HighlightTarget => {
  return {
    name,
    is_target: true,
    style: {
      color: {
        type_: 'named',
        name: color,
      },
      is_background,
    },
  };
};

export const initHighlightTargetArray = (): HighlightTarget[] => {
  const is_dark_mode = useMediaQuery('(prefers-color-scheme: dark)');

  let block_color = is_dark_mode ? 'darkcyan' : 'lightblue';
  let item_color = is_dark_mode ? 'darkblue' : 'white';

  return [
    highlightTargetNew("block", block_color, false),
    highlightTargetNew("expr", "cyan", !is_dark_mode),
    highlightTargetNew("ident", "orange", !is_dark_mode),
    highlightTargetNew("item", item_color, false),
    highlightTargetNew("lifetime", "aquamarine", !is_dark_mode),
    highlightTargetNew("literal", "lightcoral", !is_dark_mode),
    highlightTargetNew("meta", "violet", !is_dark_mode),
    highlightTargetNew("pat", "lightgreen", !is_dark_mode),
    highlightTargetNew("path", "lime", !is_dark_mode),
    highlightTargetNew("stmt", "lightskyblue", !is_dark_mode),
    highlightTargetNew("ty", "green", !is_dark_mode),
    highlightTargetNew("vis", "pink", !is_dark_mode),
  ];
};