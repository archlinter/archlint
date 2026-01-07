# archlint watch

O comando `watch` executa o archlint em segundo plano e reanalisa seu projeto toda vez que um arquivo muda.

## Uso

```bash
archlint watch [options]
```

## Opções

| Opção             | Padrão  | Descrição                                       |
| ----------------- | ------- | ----------------------------------------------- |
| `--debounce <ms>` | `300`   | Espera por mais mudanças antes de reexecutar    |
| `--clear`         | `false` | Limpa a tela do terminal antes de cada execução |

## Exemplos

### Feedback em tempo real durante o desenvolvimento

```bash
archlint watch --clear --debounce 500
```
