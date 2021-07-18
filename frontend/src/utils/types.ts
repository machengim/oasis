export interface INotification {
  type: 'success' | 'error';
  msg: string;
}

export interface ISetupRequest {
  username: string;
  password: string;
  storage: string;
}