import 'package:flutter/material.dart';
import 'pages/home_page.dart';
import 'pages/service_page.dart';
import 'pages/product_page.dart';
import 'pages/portfolio_page.dart';
import 'pages/about_page.dart';
import 'pages/contact_page.dart';
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
        primaryColor: const Color(0xFF10B981), // emerald-500
        scaffoldBackgroundColor: const Color(0xFF0A0A0A),
        fontFamily: 'Inter', // clean modern font
        textTheme: const TextTheme(
          headlineLarge: TextStyle(fontSize: 48, fontWeight: FontWeight.bold, color: Colors.white),
          headlineMedium: TextStyle(fontSize: 36, fontWeight: FontWeight.bold, color: Colors.white),
          bodyLarge: TextStyle(fontSize: 18, color: Colors.white70),
        ),
      ),
      initialRoute: '/',
      routes: {
        '/': (context) => const MainLayout(child: HomePage()),
        '/about': (context) => const MainLayout(child: AboutPage()),
        '/services': (context) => const MainLayout(child: ServicesPage()),
        '/products': (context) => const MainLayout(child: ProductsPage()),
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
            const DrawerHeader(
              child: Text("CobaltDev", style: TextStyle(fontSize: 28, fontWeight: FontWeight.bold, color: Color(0xFF10B981))),
            ),
            _drawerItem(context, "Home", "/"),
            _drawerItem(context, "Services", "/services"),
            _drawerItem(context, "Products", "/products"),
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
    return ListTile(
      title: Text(title, style: const TextStyle(color: Colors.white)),
      onTap: () {
        Navigator.pop(context);
        Navigator.pushReplacementNamed(context, route);
      },
    );
  }
}