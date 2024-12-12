import TextField from "@mui/material/TextField";

interface InputFieldProps {
  input: string;
  setInput: (input: string) => void;
}

export default function InputField({input, setInput}: InputFieldProps) {
  return (
    <>
      <TextField
        label="Your Rust Code"
        multiline
        minRows={4}
        maxRows={16}
        placeholder={"fn main() { println!(\"Hello, world!\"); }"}
        value={input}
        onChange={(e) => setInput(e.target.value)}
      />
    </>
  );
}
