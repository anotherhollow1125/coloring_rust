import { Box, IconButton, Typography } from "@mui/material";
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
          fontSize: "1.2em",
          whiteSpace: 'pre-wrap',
          wordBreak: 'break-word',
          ...(expanded ? {
            display: 'block',
            overflow: 'visible',
          } : {
            display: '-webkit-box',
            WebkitBoxOrient: 'vertical',
            WebkitLineClamp: maxLines,
            overflow: 'hidden',
            textOverflow: 'ellipsis',
          })
        }}
        className={hitTopFilter}
        dangerouslySetInnerHTML={{__html: output}}
      >
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