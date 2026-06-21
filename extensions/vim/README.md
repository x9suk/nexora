# Nexora for Vim/Neovim

Nexora language support for Vim and Neovim.

## Features

- Syntax highlighting for `.nx` files
- Filetype detection
- Indentation support

## Installation

### Using vim-plug

```vim
Plug 'nexora-lang/nexora-vim'
```

### Using lazy.nvim

```lua
{
  "nexora-lang/nexora-vim",
  ft = "nx",
}
```

### Manual Installation

Copy the files to your Vim configuration directory:

```bash
cp -r ftdetect indent syntax ~/.vim/
```

## License

MIT
