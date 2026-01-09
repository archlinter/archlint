# Suporte ao NestJS

O archlint entende a arquitetura modular do NestJS e fornece análise especializada para ele.

## Principais Recursos

- **Análise de Módulos**: Reconhece `@Module` como um ponto de coordenação e relaxa as regras de acoplamento para ele.
- **Pontos de Entrada**: Marca automaticamente Controllers e Providers como pontos de entrada.
- **Imposição de Camadas**: Funciona perfeitamente com arquiteturas de camadas no estilo NestJS (Controllers -> Services -> Repositories).
- **Sobrescritas de LCOM**: Ignora decoradores do NestJS na análise de coesão para focar na lógica real.

## Configuração Recomendada

```yaml
extends:
  - nestjs

rules:
  layer_violation:
    layers:
  - name: presentation
    path: ['**/*.controller.ts']
    allowed_imports: ['application']

  - name: application
    path: ['**/*.service.ts']
    allowed_imports: ['domain']

  - name: domain
    path: ['**/entities/**']
    allowed_imports: []
```
