import './App.css';
import InputField from "@/components/InputField";
import { createTheme, Divider, FormControlLabel, FormGroup, Link, Switch, ThemeProvider, Typography, useMediaQuery } from "@mui/material";
import { useMemo, useState } from "react";
import OutputField from "@/components/OutputField";
import { colored } from "coloring_wasm";
import Grid from "@mui/material/Grid2";
import FilterColumn from "@/components/filter/FilterColumn";
import { initFilterArray } from "@/components/filter/types";
import { initHighlightTargetArray } from "@/components/frags/types";
import FragmentColumn from './components/frags/FragmentColumn';

function App() {
  const [input, setInput] = useState("");
  const [filterList, setFilterList] = useState(initFilterArray());
  const [isDarkMode, setIsDarkModeInner] = useState(useMediaQuery('(prefers-color-scheme: dark)'));
  const [highlightTargetList, setHighlightTargetList] = useState(initHighlightTargetArray(isDarkMode));

  const theme = useMemo(() => createTheme({
      typography: {
        fontFamily: ["Source Code Pro", "serif"].join(","),
      },
      palette: {
        mode: isDarkMode ? "dark" : "light",
      },
    }), [isDarkMode]);

  const setIsDarkMode = (newMode: boolean) => {
    setIsDarkModeInner(newMode);
    setHighlightTargetList(initHighlightTargetArray(newMode));
  };

  const { hit_top_filter, hit_filters, colored: output } = colored({
    code: input,
    filters: filterList
      .flatMap(item => item.active ? [item.name] : []),
  });

  return (
    <ThemeProvider theme={theme}>
      <div
        className={`page ${isDarkMode ? "dark-mode" : "light-mode"}`}
      >
        <main className="main">
          <h1>Rust フラグメント指定子判別器</h1>
          <FormGroup>
            <FormControlLabel 
              control = {<Switch checked={isDarkMode} onChange={() => setIsDarkMode(!isDarkMode)} />}
              label="Dark Mode"
            />
          </FormGroup>
          <InputField input={input} setInput={setInput} minLines={4} maxLines={16}/>
          <Typography className={hit_top_filter}>Whole Match: {hit_top_filter}</Typography>
          <OutputField output={output} frags={highlightTargetList} hitTopFilter={hit_top_filter} maxLines={16} />
          <Divider orientation="horizontal" />
          <Grid container spacing={2}>
          <Grid size={{ xs: 12, sm: 6 }}>
            <FragmentColumn
              fragmentList={highlightTargetList}
              setFragmentList={setHighlightTargetList}
              isDarkMode={isDarkMode}
            />
          </Grid>
          <Grid size={{ xs: 12, sm: 6 }}>
            <FilterColumn
              filterList={filterList}
              setFilterList={setFilterList}
              hit_top_filter={hit_top_filter}
              hit_filters={hit_filters}
            />
          </Grid>
          <Grid size={12} sx={{textAlign: "center"}}>
            <Divider orientation="horizontal" />
            <Typography variant='h4'>Links</Typography>
            <Link href="https://github.com/anotherhollow1125/coloring_rust" target="_blank">
              <Typography variant='h5'>GitHub</Typography>
            </Link>
            本ツールの開発リポジトリです。
            <Link href="https://qiita.com/namn1125/items/93461ddf93277b79a96f" target="_blank">
              <Typography variant='h5'>Qiita</Typography>
            </Link>
            本ツール開発の背景を書いています。
          </Grid>
        </Grid>
        </main>
        <footer className="footer">
          ©️ namnium 2024
        </footer>
      </div>
      <div className="github-corner">
        <div style={{backgroundColor: theme.palette.mode === "dark" ? "#ededed" : "gray"}}></div>
        <a href="https://github.com/anotherhollow1125/coloring_rust" target="_blank">
          <img src={theme.palette.mode === "dark" ? "./github-mark.png" : "./github-mark-white.png"}/>
        </a>
      </div>
    </ThemeProvider>
  );
}

export default App
