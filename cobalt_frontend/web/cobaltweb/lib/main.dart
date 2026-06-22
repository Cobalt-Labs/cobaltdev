import 'package:flutter/material.dart';
import 'pages/home_page.dart' deferred as home_page;
import 'pages/service_page.dart' deferred as service_page;
import 'pages/product_page.dart' deferred as product_page;
import 'pages/portfolio_page.dart' deferred as portfolio_page;
import 'pages/about_page.dart' deferred as about_page;
import 'pages/contact_page.dart' deferred as contact_page;
import 'pages/splash_screen.dart';
import 'widgets/navbar.dart';

void main() {
  runApp(const CobaltDevApp());
}

class CobaltDevApp extends StatelessWidget {
  const CobaltDevApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Cobalt',
      // Smooth scrolling on web — removes the rubber-band/glow overscroll effect
      scrollBehavior: _SmoothScrollBehavior(),
      theme: ThemeData(
        brightness: Brightness.dark,
        primaryColor: const Color(0xFFA855F7),
        scaffoldBackgroundColor: const Color(0xFF07070F),
        fontFamily: 'Inter',
        textTheme: const TextTheme(
          headlineLarge: TextStyle(
            fontSize: 48,
            fontWeight: FontWeight.bold,
            color: Colors.white,
            letterSpacing: -0.5,
          ),
          headlineMedium: TextStyle(
            fontSize: 36,
            fontWeight: FontWeight.bold,
            color: Colors.white,
            letterSpacing: -0.5,
          ),
          bodyLarge: TextStyle(fontSize: 16, color: Colors.white70),
        ),
        fontFamilyFallback: const ['Noto Color Emoji'],
        // Remove splash/highlight on taps — feels more premium
        splashFactory: NoSplash.splashFactory,
        highlightColor: Colors.transparent,
      ),
      initialRoute: '/splash',
      onGenerateRoute: (settings) {
        Widget pageBuilder;
        bool useLayout = true;

        // Code Splitting Strategy: Load chunk dynamically
        Widget deferredLoader(Future<void> Function() loadLibrary, Widget Function() buildPage) {
          return FutureBuilder(
            future: loadLibrary(),
            builder: (context, snapshot) {
              if (snapshot.connectionState == ConnectionState.done) {
                return buildPage();
              }
              // Lightweight indicator while downloading the JS chunk
              return const Scaffold(
                backgroundColor: Color(0xFF07070F),
                body: Center(child: CircularProgressIndicator(color: Color(0xFFA855F7))),
              );
            },
          );
        }

        switch (settings.name) {
          case '/splash':
            pageBuilder = const SplashScreen(); // Critical path, no defer
            useLayout = false;
            break;
          case '/':
            pageBuilder = deferredLoader(home_page.loadLibrary, () => home_page.HomePage());
            break;
          case '/about':
            pageBuilder = deferredLoader(about_page.loadLibrary, () => about_page.AboutPage());
            break;
          case '/services':
            pageBuilder = deferredLoader(service_page.loadLibrary, () => service_page.ServicesPage());
            break;
          case '/products':
            pageBuilder = deferredLoader(product_page.loadLibrary, () => product_page.ProductsPage());
            break;
          case '/portfolio':
            pageBuilder = deferredLoader(portfolio_page.loadLibrary, () => portfolio_page.PortfolioPage());
            break;
          case '/contact':
            pageBuilder = deferredLoader(contact_page.loadLibrary, () => contact_page.ContactPage());
            break;
          default:
            pageBuilder = deferredLoader(home_page.loadLibrary, () => home_page.HomePage());
            break;
        }

        if (!useLayout) {
          return MaterialPageRoute(
            settings: settings,
            builder: (context) => pageBuilder,
          );
        }

        return PageRouteBuilder(
          settings: settings,
          pageBuilder: (context, animation, secondaryAnimation) =>
              MainLayout(child: pageBuilder),
          transitionsBuilder: (context, animation, secondaryAnimation, child) {
            return FadeTransition(opacity: animation, child: child);
          },
          // Snappy 300ms transition — 600ms felt sluggish
          transitionDuration: const Duration(milliseconds: 300),
        );
      },
    );
  }
}

/// Removes the overscroll glow/rubber-band effect for a clean web feel.
class _SmoothScrollBehavior extends ScrollBehavior {
  @override
  Widget buildOverscrollIndicator(
    BuildContext context,
    Widget child,
    ScrollableDetails details,
  ) {
    return child; // No glow indicator
  }

  @override
  ScrollPhysics getScrollPhysics(BuildContext context) {
    return const ClampingScrollPhysics();
  }
}

class MainLayout extends StatefulWidget {
  final Widget child;
  const MainLayout({super.key, required this.child});

  @override
  State<MainLayout> createState() => _MainLayoutState();
}

class _MainLayoutState extends State<MainLayout> {
  String currentRoute = '/';

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF07070F),
      drawer: Drawer(
        backgroundColor: const Color(0xFF0B091C),
        child: ListView(
          padding: EdgeInsets.zero,
          children: [
            DrawerHeader(
              decoration: const BoxDecoration(
                border: Border(
                  bottom: BorderSide(color: Color(0x33A855F7)),
                ),
              ),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Image.asset(
                      'assets/images/cobalt_logo.png',
                      width: 24,
                      height: 24,
                    ),
                    const SizedBox(width: 12),
                    const Text(
                      "Cobalt",
                      style: TextStyle(
                        fontSize: 20,
                        fontWeight: FontWeight.w600,
                        color: Colors.white,
                      ),
                    ),
                  ],
                ),
              ),
            ),
            _drawerItem("Home", "/"),
            _drawerItem("Services", "/services"),
            _drawerItem("Products", "/products"),
            _drawerItem("Portfolio", "/portfolio"),
            _drawerItem("About", "/about"),
            _drawerItem("Contact", "/contact"),
          ],
        ),
      ),
      body: Column(
        children: [
          const Navbar(),
          Expanded(child: widget.child),
        ],
      ),
    );
  }

  Widget _drawerItem(String title, String route) {
    final isActive = currentRoute == route;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 12.0, vertical: 2.0),
      child: ListTile(
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
        tileColor: isActive
            ? const Color(0xFFA855F7).withValues(alpha: 0.15)
            : Colors.transparent,
        title: Text(
          title,
          style: TextStyle(
            color: isActive ? Colors.white : Colors.white70,
            fontWeight: isActive ? FontWeight.w600 : FontWeight.w400,
            fontSize: 15,
          ),
        ),
        trailing: isActive
            ? Container(
                width: 4,
                height: 16,
                decoration: BoxDecoration(
                  color: const Color(0xFFA855F7),
                  borderRadius: BorderRadius.circular(2),
                ),
              )
            : null,
        onTap: () {
          Navigator.pop(context);
          if (!isActive) {
            Navigator.pushReplacementNamed(context, route);
          }
        },
      ),
    );
  }
}