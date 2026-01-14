# Vazamento de Teste (Test Leakage)

**ID:** `test_leakage` | **Gravidade:** High (default)

Identifica código de produção que importa de arquivos de teste ou arquivos de mock.

## Por que isso é um smell

O código de produção nunca deve depender do código de teste. Isso pode levar ao aumento do tamanho do pacote, riscos de segurança e falhas na compilação se os arquivos de teste forem excluídos da compilação de produção.

## Como corrigir

- Mova a lógica compartilhada do arquivo de teste para um local seguro para produção.
- Certifique-se de que seus caminhos de importação estão corretos.
