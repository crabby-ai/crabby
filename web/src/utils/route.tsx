
import { lazily } from "react-lazily";
import { AcademicCapIcon, UserIcon } from "@heroicons/react/outline";



export const MAIN_ROUTES = [
  {
    path: "/login",
    component: () => {
      const { LoginPage } = lazily(() => import("../route/auth/login"));
      return LoginPage;
    },
    key: "login"
  },
  {
    path: "/",
    component: () => {
      const { HomeLayout } = lazily(() => import("../layouts/home"));
      return HomeLayout;
    },
    key: "home"
  }
]