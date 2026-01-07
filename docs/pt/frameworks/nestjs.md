# Suporte ao NestJS

O archlint entende a arquitetura modular do NestJS e fornece análise especializada para ele.

## Principais Recursos

- **Análise de Módulos**: Reconhece `@Module` como um ponto de coordenação e relaxa as regras de acoplamento para ele.
- **Pontos de Entrada**: Marca automaticamente Controllers e Providers como pontos de entrada.
- **Imposição de Camadas**: Funciona perfeitamente com arquiteturas de camadas no estilo NestJS (Controllers -> Services -> Repositories).
- **Sobrescritas de LCOM**: Ignora decoradores do NestJS na análise de coesão para focar na lógica real.

## Configuração Recomendada

```yaml
frameworks:
  - nestjs

layers:
  - name: presentation
    paths: ['**/*.controller.ts']
    can_import: ['application']

  - name: application
    paths: ['**/*.service.ts']
    can_import: ['domain']

  - name: domain
    paths: ['**/entities/**']
    can_import: []
```
