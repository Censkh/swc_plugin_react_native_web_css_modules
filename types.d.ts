/**
 * Allow CSS module strings to be passed to RN components
 */
declare module "*.scss" {
    import {RegisteredStyle} from "react-native";
    const classes: { [key: string]: string & RegisteredStyle<any> };
    export default classes;
}
