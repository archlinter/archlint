# Abuso de Barrel Files (Barrel Abuse)

**ID:** `barrel_file_abuse` | **Gravidade:** Média (padrão)

Arquivos barrel (ex: arquivos `index.ts` que apenas reexportam outros arquivos) podem se tornar problemáticos quando crescem demais ou incluem muitas exportações não relacionadas.

## Por que isso é um smell

- **Dependências Circulares**: Arquivos barrel grandes são uma causa comum de dependências circulares indiretas.
- **Acoplamento Desnecessário**: Importar uma única coisa de um arquivo barrel grande pode fazer com que o bundler inclua muitos módulos não relacionados.
- **Performance**: Pode deixar mais lento tanto o desenvolvimento (indexação da IDE) quanto a produção (tamanho do bundle/tempo de carregamento).

## Como corrigir

- Evite arquivos barrel "pega-tudo" na raiz de diretórios grandes.
- Prefira imports diretos se um arquivo barrel estiver causando problemas.
- Agrupe exportações em arquivos barrel menores e mais específicos.
