export const auth = () => {
  console.log('auth');
};
export const formatCurrency = (val: number) => val.toFixed(2);
export const validateEmail = (email: string) => email.includes('@');
export const parseQuery = (url: string) => ({});
export const logError = (err: any) => {
  console.error(err);
};
// These functions are unrelated
