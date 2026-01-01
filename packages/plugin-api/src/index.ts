// archlint plugin API types
export interface Detector {
  name: string;
  analyze: (context: any) => void;
}
