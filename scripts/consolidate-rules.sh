#!/bin/bash
# Consolidate .cursor/rules into .github/copilot-instructions.md
echo "<instructions>" > .github/copilot-instructions.md
for file in $(find .cursor/rules -name "*.mdc" | sort); do
    echo "## Rule: $file" >> .github/copilot-instructions.md
    echo "" >> .github/copilot-instructions.md
    cat "$file" >> .github/copilot-instructions.md
    echo "" >> .github/copilot-instructions.md
    echo "---" >> .github/copilot-instructions.md
    echo "" >> .github/copilot-instructions.md
done
echo "</instructions>" >> .github/copilot-instructions.md
