import 'package:flutter/material.dart';
import 'pages/home_page.dart';
import 'pages/service_page.dart';
import 'pages/product_page.dart';
import 'pages/portfolio_page.dart';
import 'pages/about_page.dart';
import 'pages/contact_page.dart';
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
      title: 'CobaltDev',
      theme: ThemeData(
        brightness: Brightness.dark,
        primaryColor: const Color(0xFFA855F7), // Purple 500
        scaffoldBackgroundColor: const Color(0xFF07070F), // Dark slate/purple
        fontFamily: 'Inter',
        textTheme: const TextTheme(
          headlineLarge: TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Colors.white, letterSpacing: -0.5),
          headlineMedium: TextStyle(fontSize: 36, fontWeight: FontWeight.bold, color: Colors.white, letterSpacing: -0.5),
          bodyLarge: TextStyle(fontSize: 16, color: Colors.white70),
        ),
        fontFamilyFallback: const ['Noto Color Emoji'],
      ),
      initialRoute: '/splash',
      onGenerateRoute: (settings) {
        Widget page;
        bool useLayout = true;

        switch (settings.name) {
          case '/splash':
            page = const SplashScreen();
            useLayout = false;
            break;
          case '/':
            page = const HomePage();
            break;
          case '/about':
            page = const AboutPage();
            break;
          case '/services':
            page = const ServicesPage();
            break;
          case '/products':
            page = const ProductsPage();
            break;
          case '/portfolio':
            page = const PortfolioPage();
            break;
          case '/contact':
            page = const ContactPage();
            break;
          default:
            page = const HomePage();
            break;
        }

        if (!useLayout) {
          return MaterialPageRoute(
            settings: settings,
            builder: (context) => page,
          );
        }

        return PageRouteBuilder(
          settings: settings,
          pageBuilder: (context, animation, secondaryAnimation) => MainLayout(child: page),
          transitionsBuilder: (context, animation, secondaryAnimation, child) {
            return FadeTransition(
              opacity: animation,
              child: child,
            );
          },
          transitionDuration: const Duration(milliseconds: 600),
        );
      },
    );
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
      drawer: Drawer(
        backgroundColor: const Color(0xFF0B091C),
        child: ListView(
          padding: EdgeInsets.zero,
          children: [
            DrawerHeader(
              decoration: const BoxDecoration(
                border: Border(bottom: BorderSide(color: Color(0x338B5CF6))),
              ),
              child: Align(
                alignment: Alignment.centerLeft,
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Image.asset('assets/images/cobalt_logo.png', width: 24, height: 24),
                    const SizedBox(width: 12),
                    const Text("CobaltDev", style: TextStyle(fontSize: 20, fontWeight: FontWeight.w600, color: Colors.white)),
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
        tileColor: isActive ? const Color(0xFFA855F7).withOpacity(0.15) : Colors.transparent,
        title: Text(
          title,
          style: TextStyle(
            color: isActive ? Colors.white : Colors.white70,
            fontWeight: isActive ? FontWeight.w600 : FontWeight.w400,
            fontSize: 15,
          ),
        ),
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