export interface NamedColor {
  type_: 'named';
  name: string;
}

export function namedColorNew(name: string): NamedColor {
  return {
    type_: 'named',
    name,
  };
}

export interface CustomColor {
  type_: 'custom';
  hex: string;
}

export type Color = NamedColor | CustomColor;

export function color2string(color: Color): string {
  if (color.type_ === 'named') {
    return color.name;
  } else {
    return color.hex;
  }
}

export interface HighlightStyle {
  color: Color;
  is_background: boolean;
}

export function style2css({color, is_background}: HighlightStyle) {
  const color_str = color2string(color);

  const css = is_background ?
`
background-color: ${color_str};
color: black;
` : 
`
background-color: transparent;
color: ${color_str};
`;

  return css;
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

export const initHighlightTargetArray = (is_dark_mode: boolean): HighlightTarget[] => {
  let block_color = is_dark_mode ? 'lightblue' : 'darkcyan';
  let item_color = is_dark_mode ? 'white' : 'darkblue';

  return [
    highlightTargetNew("literal", "lightcoral", !is_dark_mode),
    highlightTargetNew("ident", "orange", !is_dark_mode),
    highlightTargetNew("path", "lime", !is_dark_mode),
    highlightTargetNew("pat", "lightgreen", !is_dark_mode),
    highlightTargetNew("ty", "green", !is_dark_mode),
    highlightTargetNew("lifetime", "aquamarine", !is_dark_mode),
    highlightTargetNew("vis", "pink", !is_dark_mode),
    highlightTargetNew("expr", "cyan", !is_dark_mode),
    highlightTargetNew("stmt", "lightskyblue", !is_dark_mode),
    highlightTargetNew("meta", "violet", !is_dark_mode),
    highlightTargetNew("block", block_color, false),
    highlightTargetNew("item", item_color, false),
  ];
};

const TAGNAME = "highlight-style";

export function createFragsCss(frags: HighlightTarget[]) {
  const existingStyleTag = document.getElementById(TAGNAME);
  if (existingStyleTag) {
    existingStyleTag.remove();
  }

  const styleTag = document.createElement("style");
  styleTag.id = TAGNAME;

  const tagNames: string[] = [];
  const css = frags
    .toReversed()
    .flatMap(({name, is_target, style}: HighlightTarget) => {
      if (!is_target) {
        return [];
      }

      tagNames.push(name);

      const css = style2css(style);

      return [`@layer ${name} { *.${name} { ${css} } }`];
    });
  styleTag.textContent = `
  @layer ${tagNames.join(", ")};
  ${css.join("\n")}
  `;

  document.head.appendChild(styleTag);
} 