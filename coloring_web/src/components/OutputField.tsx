import { Box, IconButton, Typography, useTheme } from "@mui/material";
import { createFragsCss, HighlightTarget } from "./frags/types";
import { useEffect, useState } from "react";
import { ExpandMore } from "@mui/icons-material";

interface OutputFieldProps {
  output: string;
  hitTopFilter: string;
  frags: HighlightTarget[];
  maxLines: number;
}

export default function OutputField({output, hitTopFilter, frags, maxLines}: OutputFieldProps) {
  const { typography } = useTheme();
  const [expanded, setExpanded] = useState(false);

  useEffect(() => {
    createFragsCss(frags);
  }, [frags]);

  return (
    <Box
      sx={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
      }}
    >
      <Typography
        sx={{
          width: '100%',
          display: '-webkit-box',
          WebkitBoxOrient: 'vertical',
          // 展開時は行数制限を解除
          WebkitLineClamp: expanded ? 'none' : maxLines,
          overflow: 'hidden',
          textOverflow: 'ellipsis',
        }}
      >
        <pre
          dangerouslySetInnerHTML={{__html: output}}
          style={{
            ...typography,
            fontSize: "1.2em",
            whiteSpace: 'pre-wrap',
            wordBreak: 'break-word',
          }}
          className={hitTopFilter}
        />
      </Typography>
      <IconButton
        onClick={() => setExpanded(!expanded)}
        aria-label={expanded ? "折りたたむ" : "展開"}
        size="small"
        sx={{
          transform: expanded ? 'rotate(180deg)' : 'rotate(0deg)',
          transition: 'transform 0.3s',
        }}
      >
        <ExpandMore />
      </IconButton>
    </Box>
  );
}