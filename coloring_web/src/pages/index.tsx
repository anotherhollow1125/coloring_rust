import Head from "next/head";
import { Source_Code_Pro } from "next/font/google";
import styles from "@/styles/Home.module.css";
import InputField from "@/components/InputField";
import { Button, Checkbox, createTheme, Divider, ListItem, ListItemIcon, ThemeProvider } from "@mui/material";
import { useState } from "react";
import OutputField from "@/components/OutputField";
import { colored } from "coloring_wasm";
import Grid from "@mui/material/Grid2";
import FilterColumn, { initFilterList } from "@/components/filter/FilterColumn";

const SourceCodePro = Source_Code_Pro({
  weight: "400",
});

const theme = createTheme({
  typography: {
    fontFamily: SourceCodePro.style.fontFamily,
  },
  colorSchemes: {
    dark: true,
  },
});

export default function Home() {
  const [input, setInput] = useState("");
  const [filterList, setFilterList] = useState(initFilterList());

  const { hit_top_filter, hit_filters, colored: output } = colored({
    code: input,
    filters: filterList
      .flatMap(item => item.active ? [item.name] : []),
  });

  return (
    <ThemeProvider theme={theme}>
      <Head>
        <title>Rust フラグメント識別子判別器 Fragment Specifiers Highlighter</title>
        <meta name="description" content="Rustコードを解析してフラグメント識別子を判断。宣言マクロ作成に役立ちます。" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <div
        className={`${styles.page} ${SourceCodePro.className}`}
      >
        <main className={styles.main}>
          <h1>Rust フラグメント識別子判別器</h1>
          <InputField input={input} setInput={setInput} />
          <>Whole Match: {hit_top_filter}</>
          <OutputField output={output} />
          <Divider orientation="horizontal" />
          <Grid container spacing={2}>
          <Grid size={6}>
            <FilterColumn
              filterList={filterList}
              setFilterList={setFilterList}
              hit_top_filter={hit_top_filter}
              hit_filters={hit_filters}
            />
          </Grid>
          <Grid size={6}>
            preparing...
          </Grid>
        </Grid>
        </main>
        <footer className={styles.footer}>
          ©️ namnium 2024
        </footer>
      </div>
    </ThemeProvider>
  );
}
