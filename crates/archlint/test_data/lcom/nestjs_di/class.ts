import { Injectable } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';

@Injectable()
export class OrderService {
  constructor(
    private readonly configService: ConfigService,
    private readonly orderRepository: any,
  ) {}

  async getOrder(id: string) {
    const config = this.configService.get('orders');
    return this.orderRepository.findById(id);
  }

  async createOrder(data: any) {
    const maxOrders = this.configService.get('maxOrders');
    return this.orderRepository.create(data);
  }

  async updateOrder(id: string, data: any) {
    return this.orderRepository.update(id, data);
  }

  async deleteOrder(id: string) {
    return this.orderRepository.delete(id);
  }
}
