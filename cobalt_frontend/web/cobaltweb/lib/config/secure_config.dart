// Secure configuration - NEVER hardcode secrets!
class SecureConfig {
  // Load from environment or secure storage
  static const String apiBaseUrl = String.fromEnvironment(
    'API_BASE_URL',
    defaultValue: 'https://localhost:8443', // Always HTTPS
  );
  
  // Use flutter_secure_storage for tokens
  static const String secureStorageKey = 'cobalt_auth_token';
  
  // Validate URLs before requests
  static bool isValidUrl(String url) {
    return url.startsWith('https://') && 
           !url.contains('localhost:3000') && // No dev ports in prod
           !url.contains(':8000');
  }
}
