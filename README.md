# bridge-ls

An attempt to write a general purpose language server inspired by [null-ls](https://github.com/jose-elias-alvarez/null-ls.nvim) using [tower-lsp](https://github.com/ebkalderon/tower-lsp).

> [!NOTE]
> This is currently a work in progress and will probabaly remain experimental

## Features

- [x] Formatting
- [ ] Linting
- [ ] Snippets

## Installation

> [!IMPORTANT]
> You need the [rust toolchain](https://rustup.rs/) to build bridge-ls

You can use the following script to install it:

```bash
git clone https://github.com/luckasRanarison/bridge-ls
cd bridge-ls
cargo install --path .
```

## Configuration

After installing `bridge-ls` you need to set the `BRDIGE_LS_CONFIG` environment variable pointing to your configuration file path.

```bash
export BRDIGE_LS_CONFIG="~/.config/bridge-ls/config.json"
```

Here is the configuration schema:

```typescript
type Config = {
  builtins: {
    formatters: string[];
  };
  customs: {
    formatters: {
      command: string;
      args: string[];
      toStdin: boolean; // write the file content to stdin
      cleanup?: string; // command to run when the server shuts down
    };
  };
};
```

The following variables can be used with `args`:

- `$filename`: expands to the relative filename.
- `$filepath`: expands to the full filepath.

> [!TIP]
> See the [builtins](./builtins) for reference

## Usage

### Neovim

Using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig):

```lua
require("lspconfig").bridge_ls.setup({
  root_dir = function() return vim.fn.getcwd(0) end,
  name = "bridge_ls",
  cmd = { "bridge-ls" },
  autostart = true,
})
```
