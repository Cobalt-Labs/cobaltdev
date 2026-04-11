import 'package:flutter/material.dart';
import 'pages/home_page.dart';
import 'pages/service_page.dart';
import 'pages/product_page.dart';
import 'pages/portfolio_page.dart';
import 'pages/about_page.dart';
import 'pages/contact_page.dart';
import 'pages/cloud_page.dart';
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
      title: 'CobaltDev — Flutter + Rust',
      theme: ThemeData(
        brightness: Brightness.dark,
        primaryColor: const Color(0xFF10B981),
        scaffoldBackgroundColor: const Color(0xFF0A0A0A),
        fontFamily: 'Inter',
        textTheme: const TextTheme(
          headlineLarge: TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Colors.white),
          headlineMedium: TextStyle(fontSize: 36, fontWeight: FontWeight.bold, color: Colors.white),
          bodyLarge: TextStyle(fontSize: 18, color: Colors.white70),
        ),
        fontFamilyFallback: const ['Noto Color Emoji'],
      ),
      initialRoute: '/',
      routes: {
        '/': (context) => const MainLayout(child: HomePage()),
        '/about': (context) => const MainLayout(child: AboutPage()),
        '/services': (context) => const MainLayout(child: ServicesPage()),
        '/products': (context) => const MainLayout(child: ProductsPage()),
        '/cloud': (context) => const MainLayout(child: CloudPage()),
        '/portfolio': (context) => const MainLayout(child: PortfolioPage()),
        '/contact': (context) => const MainLayout(child: ContactPage()),
      },
    );
  }
}

class MainLayout extends StatelessWidget {
  final Widget child;
  const MainLayout({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: Drawer(
        backgroundColor: const Color(0xFF111111),
        child: ListView(
          children: [
            DrawerHeader(
              child: Row(
                children: [
                  Image.asset(
                    'assets/images/cobalt_logo.png',
                    height: 36,
                    errorBuilder: (context, error, stackTrace) => const SizedBox(),
                  ),
                  const SizedBox(width: 8),
                  const Text("CobaltDev", style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Color(0xFF10B981))),
                ],
              ),
            ),
            _drawerItem(context, "Home", "/"),
            _drawerItem(context, "Services", "/services"),
            _drawerItem(context, "Products", "/products"),
            _drawerItem(context, "Cloud", "/cloud"),
            _drawerItem(context, "Portfolio", "/portfolio"),
            _drawerItem(context, "About", "/about"),
            _drawerItem(context, "Contact", "/contact"),
          ],
        ),
      ),
      body: Column(
        children: [
          const Navbar(),
          Expanded(child: child),
        ],
      ),
    );
  }

  Widget _drawerItem(BuildContext context, String title, String route) {
    final currentRoute = ModalRoute.of(context)?.settings.name ?? '/';
    final isActive = currentRoute == route;

    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 4.0),
      child: ListTile(
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
        tileColor: isActive ? const Color(0xFF10B981).withOpacity(0.15) : Colors.transparent,
        leading: Icon(
          isActive ? Icons.chevron_right : Icons.circle,
          size: isActive ? 24 : 12,
          color: isActive ? const Color(0xFF10B981) : Colors.white24,
        ),
        title: Text(
          title,
          style: TextStyle(
            color: isActive ? const Color(0xFF10B981) : Colors.white70,
            fontWeight: isActive ? FontWeight.bold : FontWeight.w500,
            fontSize: 18,
          ),
        ),
        onTap: () {
          Navigator.pop(context);
          if (!isActive) Navigator.pushReplacementNamed(context, route);
        },
      ),
    );
  }
}