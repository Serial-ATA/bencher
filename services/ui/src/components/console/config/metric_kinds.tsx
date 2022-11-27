import metricKindFieldsConfig from "../../fields/config/org/metricKindFieldsConfig";
import { BENCHER_API_URL } from "../../site/util";
import { Button, Card, Display, Field, Operation, Row } from "./types";
import { parentPath, addPath, viewSlugPath } from "./util";

const metricKindsConfig = {
  [Operation.LIST]: {
    operation: Operation.LIST,
    header: {
      title: "Metric Kinds",
      buttons: [
        {
          kind: Button.ADD,
          path: addPath,
        },
        { kind: Button.REFRESH },
      ],
    },
    table: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/metric-kinds`,
      add: {
        path: addPath,
        text: "Add a Metric Kind",
      },
      row: {
        key: "name",
        items: [
          {
            kind: Row.TEXT,
            key: "slug",
          },
          {},
          {},
          {},
        ],
        button: {
          text: "View",
          path: (pathname, datum) => viewSlugPath(pathname, datum),
        },
      },
    },
  },
  [Operation.ADD]: {
    operation: Operation.ADD,
    header: {
      title: "Add Metric Kind",
      path: parentPath,
    },
    form: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/metric-kinds`,
      fields: [
        {
          kind: Field.HIDDEN,
          key: "project",
          path_param: "project_slug",
        },
        {
          kind: Field.INPUT,
          label: "Name",
          key: "name",
          value: "",
          valid: null,
          validate: true,
          nullify: false,
          clear: false,
          config: metricKindFieldsConfig.name,
        },
        {
          kind: Field.INPUT,
          label: "Units",
          key: "units",
          value: "",
          valid: null,
          validate: false,
          nullify: false,
          clear: false,
          config: metricKindFieldsConfig.units,
        },
      ],
      path: parentPath,
    },
  },
  [Operation.VIEW]: {
    operation: Operation.VIEW,
    header: {
      key: "name",
      path: parentPath,
    },
    deck: {
      url: (path_params) =>
        `${BENCHER_API_URL()}/v0/projects/${
          path_params?.project_slug
        }/metric-kinds/${path_params?.metric_kind_slug}`,
      cards: [
        {
          kind: Card.FIELD,
          label: "Metric Kind Name",
          key: "name",
          display: Display.RAW,
        },
        {
          kind: Card.FIELD,
          label: "Metric Kind Slug",
          key: "slug",
          display: Display.RAW,
        },
        {
          kind: Card.FIELD,
          label: "Metric Kind UUID",
          key: "uuid",
          display: Display.RAW,
        },
        {
          kind: Card.FIELD,
          label: "Metric Kind Units",
          key: "units",
          display: Display.RAW,
        },
      ],
      buttons: false,
    },
  },
};

export default metricKindsConfig;