export interface ConversionSettings {
  fps: number;
  max_dimension: number;
  max_frames: number;
  quality: number;
  compression_level: number;
  overwrite: boolean;
  skip_frames: number;
}

export interface ConversionResult {
  input_path: string;
  output_path: string;
  output_size: number | null;
  success: boolean;
  error: string | null;
}

export interface FileItem {
  id: string;
  name: string;
  path: string;
  status: 'pending' | 'converting' | 'done' | 'error';
  outputPath?: string;
  outputSize?: string;
  error?: string;
  progress?: number;
}

export const defaultSettings: ConversionSettings = {
  fps: 10,
  max_dimension: 540,
  max_frames: 30,
  quality: 55,
  compression_level: 6,
  overwrite: false,
  skip_frames: 0,
};
