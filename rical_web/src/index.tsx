/* @refresh reload */
import { render } from "solid-js/web";
import { Router, Route } from "@solidjs/router";
import "./index.css";
import Landing from "./pages/Landing.tsx";
import NotFound from "./pages/NotFound.tsx";
import Calendar from "./pages/Calendar.tsx";
import Login from "./pages/Login.tsx";
import { CalendarStateProvider } from "./util/StateProvider.tsx";

const root = document.getElementById("root");

render(
  () => (
    <CalendarStateProvider>
      <Router>
        <Route path="/" component={Landing} />
        <Route path="/cal" component={Calendar} />
        <Route path="/login" component={() => <Login signup={false} />} />
        <Route path="/signup" component={() => <Login signup={true} />} />
        <Route path="*404" component={NotFound} />
      </Router>
    </CalendarStateProvider>
  ),
  root!,
);
