import TextField from "@mui/material/TextField";

interface InputFieldProps {
  input: string;
  setInput: (input: string) => void;
  minLines: number;
  maxLines: number;
}

export default function InputField({input, setInput, minLines, maxLines}: InputFieldProps) {
  return (
    <>
      <TextField
        label="Your Rust Code"
        multiline
        minRows={minLines}
        maxRows={maxLines}
        placeholder={"fn main() { println!(\"Hello, world!\"); }"}
        value={input}
        onChange={(e) => setInput(e.target.value)}
      />
    </>
  );
}
