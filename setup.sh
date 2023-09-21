#!/bin/bash

# Check if zplug is installed, if not, install it.
if [ ! -d ~/.zplug ]; then
    echo "Installing zplug..."
    git clone https://github.com/zplug/zplug ~/.zplug
fi

# Create or overwrite the .zshrc file.
cat > ~/.zshrc <<EOF
# =====================
# General Configuration
# =====================

# Set the default shell to Zsh (if not already set)
[ -n "\$ZSH_VERSION" ] || exec zsh

# =====================
# Z-plug Plugin Manager
# =====================

# Source Z-plug
source ~/.zplug/init.zsh

# Plugins
zplug 'romkatv/powerlevel10k', as:theme, depth:1
zplug 'zsh-users/zsh-autosuggestions'
zplug 'zsh-users/zsh-history-substring-search'
zplug 'marlonrichert/zsh-autocomplete'
zplug 'hlissner/zsh-autopair'

# Check and install plugins if necessary
if ! zplug check --verbose; then
    printf "Install? [y/N]: "
    if read -q; then
        echo
        zplug install
    fi
fi

# Load plugins
zplug load

# =====================
# Keybindings and History
# =====================

# Keybindings for history substring search
bindkey "\$terminfo[kcuu1]" history-substring-search-up
bindkey "\$terminfo[kcud1]" history-substring-search-down

# History settings
SAVEHIST=1000
export HISTFILE=~/.zsh_history
setopt share_history

# =====================
# Custom Aliases and Functions
# =====================

alias rm='rm -r'
alias cp='cp -r'
alias ls='ls -hlF --color=auto'
alias ..='cd ../'
alias tree="tree -alI 'node_modules|.git'"
alias grep='grep --color=always'
alias grepFind='grep --exclude-dir=node_modules -nr . -e'
alias mkdir='mkdir -p'

# =====================
# Powerlevel10k Customization
# =====================

# Load Powerlevel10k instant prompt
if [[ -r "\${XDG_CACHE_HOME:-\$HOME/.cache}/p10k-instant-prompt-\${(%):-%n}.zsh" ]]; then
  source "\${XDG_CACHE_HOME:-\$HOME/.cache}/p10k-instant-prompt-\${(%):-%n}.zsh"
fi

# To customize the prompt, run \`p10k configure\` or edit ~/.p10k.zsh
[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh

# =====================
# Additional Customizations
# =====================

# Add your additional customizations below this section

# Example: Export environment variables
# export MY_VARIABLE="example_value"

EOF

# Inform the user
echo "Zsh configuration and dependencies installed. Please restart your shell."
