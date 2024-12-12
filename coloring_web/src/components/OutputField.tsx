import { useTheme } from "@mui/material";

interface OutputFieldProps {
  output: string;
}

export default function OutputField({output}: OutputFieldProps) {
  const { typography } = useTheme();

  return (
    <pre dangerouslySetInnerHTML={{__html: output}} style={typography} />
  );
}
