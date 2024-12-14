"use client";

import { useTheme } from "@mui/material";
import { color2string, HighlightTarget } from "./frags/types";
import { useEffect } from "react";

interface OutputFieldProps {
  output: string;
  frags: HighlightTarget[];
}

const TAGNAME = "highlight-style";

export default function OutputField({output, frags}: OutputFieldProps) {
  const { typography } = useTheme();

  useEffect(() => {
    const existingStyleTag = document.getElementById(TAGNAME);
    if (existingStyleTag) {
      existingStyleTag.remove();
    }

    const styleTag = document.createElement("style");
    styleTag.id = TAGNAME;

    const css = frags
      .reverse()
      .flatMap(({name, is_target, style}: HighlightTarget) => {
        if (!is_target) {
          return [];
        }

        const {
          color,
          is_background,
        } = style;

        const color_str = color2string(color);

        const style_row = is_background ?
          "background-color: " + color_str + ";" :
          "color: " + color_str + ";";

        [`.${name} { ${style_row}}`];
      });
    styleTag.textContent = css.join("\n");

    console.log(css.join("\n"));
    console.log(styleTag);
    console.log(styleTag.textContent);

    document.head.appendChild(styleTag);
  }, [frags]);

  return (
    <pre dangerouslySetInnerHTML={{__html: output}} style={typography} />
  );
}