" Settings
set nocompatible		" Use vim mode, turn off vi compatible mode
filetype plugin indent on	" Turn on detection, plugin, and indent. See :help :filetype-overview
set number			" Show line numbers

" Vim8 packages

" git clone https://github.com/rust-lang/rust.vim ~/.vim/pack/plugins/start/rust.vim
let g:rustfmt_autosave = 1	" Enable automatic running of :RustFmt when you save a buffer