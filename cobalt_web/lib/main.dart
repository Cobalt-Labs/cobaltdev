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
      title: 'CobaltDev',
      theme: ThemeData.dark(),
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
      /// 🔥 MOBILE DRAWER
      drawer: Drawer(
        child: Container(
          color: Colors.black,
          child: ListView(
            children: [
              const DrawerHeader(
                child: Text(
                  "CobaltDev",
                  style: TextStyle(fontSize: 24),
                ),
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
      ),

      body: Column(
        children: [
          const Navbar(),
          Expanded(child: child),
        ],
      ),
    );
  }

  Widget _drawerItem(
      BuildContext context, String title, String route) {
    return ListTile(
      title: Text(title),
      onTap: () {
        Navigator.pop(context); // close drawer
        Navigator.pushReplacementNamed(context, route);
      },
    );
  }
}