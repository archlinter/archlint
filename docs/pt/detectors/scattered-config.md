# Configuração Espalhada

**ID:** `scattered_config` | **Severity:** Low (default)

Identifica configurações (como acesso a variáveis de ambiente) que estão espalhadas por muitos arquivos diferentes em vez de serem centralizadas.

## Por que isso é um "smell"

Centralizar a configuração torna mais fácil:

- Ver todas as opções de configuração em um só lugar.
- Fornecer valores padrão.
- Validar a configuração na inicialização.
- Alterar a origem da configuração (por exemplo, de variáveis de ambiente para um arquivo ou um gerenciador de segredos).

## Como consertar

Crie um módulo `config` central que lê e valida todas as variáveis de ambiente e as exporta como um objeto tipado.
