import { driver } from "driver.js";
import "driver.js/dist/driver.css";
const OPEN_TUTORIAL_LOCAL_STORAGE = "resico.tutorial.open";

export const tutorial = driver({
    showProgress: true,
    allowClose: false,
    nextBtnText: "Siguiente",
    prevBtnText: "Atrás",
    doneBtnText: "Cerrar",
    steps: [
        {
            element: ".title",
            popover: {
                title: "Bienvenido",
                description: "Esta es una calculadora de facturas para RESICO.",
            },
        },
        {
            element: ".calc-options",
            popover: {
                description:
                    "Selecciona el modo de factura, dependiendo de tus necesidades.",
            },
        },
        {
            element: "#billButton",
            popover: {
                title: "Factura a empresa",
                description:
                    "Si tienes la cantidad que necesitas libre de impuestos, utiliza esta opción para calcularlos.",
            },
        },
        {
            element: "#reverseBillButton",
            popover: {
                title: "Desde factura a empresa",
                description:
                    "Si ya te hicieron el depósito o si quieres saber cuánto obtendrás libre por factura, \
                utiliza esta opción.",
            },
        },
        {
            element: ".principal-input",
            popover: {
                title: "Entrada de cantidad",
                description:
                    "Ingresa un número mayor a 0 para poder hacer el cálculo; puedes usar decimales. \
                Si ingresas letras o una cadena inválida será interpretada como 0.",
            },
        },
        {
            element: ".calc-btn",
            popover: {
                title: "Calcular",
                description:
                    "Presiona este botón para realizar el cálculo. Por favor presiónalo para seguir con el tutorial.",
            },
        },
        {
            element: ".Results",
            popover: {
                title: "Resultados",
                description: "Revisa la tabla de resultados desglosada.",
            },
        },
        {
            element: ".subtotal",
            popover: {
                title: "Subtotal",
                description:
                    "Esta es la cantidad que debes ingresar en el apartado 'Valor Unitario' en el portal del SAT.",
            },
        },
        {
            element: ".save-btn",
            popover: {
                title: "Guardar",
                description:
                    "Puedes guardar cada resultado. Presiona este botón para continuar con el tutorial.",
            },
        },
        {
            element: ".Card",
            popover: {
                title: "Resultado",
                description:
                    "Todos los resultados guardados se mostrarán de esta forma. \
                Podrás eliminarlos y acceder a sus detalles con los botones de cada uno.",
            },
        },
        {
            element: ".clear-btn",
            popover: {
                title: "Limpiar",
                description:
                    "Presiona este botón para borrar los resultados del cálculo y limpiar la cantidad.",
            },
        },
    ],
    onDestroyed: () => {
        localStorage.setItem(OPEN_TUTORIAL_LOCAL_STORAGE, "opened");
    },
});

export const dynStartTutorial = () => {
    const value = localStorage.getItem(OPEN_TUTORIAL_LOCAL_STORAGE);
    if (value == null) {
        tutorial.drive();
    }
};

export const startTutorial = () => {
    tutorial.drive();
};
