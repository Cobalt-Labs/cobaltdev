// import 'dart:ffi';
// import 'dart:io';
// import 'bridge_generated.dart';

// late final RustImpl api;

// void initRustApi() {
//   try {
//     DynamicLibrary dylib;
    
//     if (Platform.isAndroid) {
//       dylib = DynamicLibrary.open("librust.so");
//     } else if (Platform.isIOS) {
//       dylib = DynamicLibrary.process();
//     } else if (Platform.isLinux) {
//       String? path;
//       final List<String> possiblePaths = [
//         'librust.so',
//         './librust.so',
//         './target/release/librust.so',
//         '../rust/target/release/librust.so',
//       ];
      
//       for (final testPath in possiblePaths) {
//         if (File(testPath).existsSync()) {
//           path = testPath;
//           break;
//         }
//       }
      
//       if (path == null) {
//         throw Exception('Could not find librust.so in any of the expected locations');
//       }
      
//       dylib = DynamicLibrary.open(path);
//     } else if (Platform.isWindows) {
//       dylib = DynamicLibrary.open('rust.dll');
//     } else if (Platform.isMacOS) {
//       dylib = DynamicLibrary.open('librust.dylib');
//     } else {
//       throw Exception('Unsupported platform');
//     }
    
//     api = RustImpl(dylib);
//     print('Rust dynamic library loaded successfully.');
//   } catch (e) {
//     print('Failed to load Rust dynamic library: $e');
//     rethrow;
//   }
// }

