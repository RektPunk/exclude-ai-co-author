# exclude-ai-co-author
AI is your power tool, not your co-author. Reclaim your Git commit.

```yaml
# .pre-commit-config.yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/exclude-ai-co-author
    rev: v0.0.2
    hooks:
      - id: exclude-ai-co-author
```
