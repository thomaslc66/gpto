import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import {
  Button,
  Input,
  Form,
  Checkbox,
  Layout,
  Row,
  Col,
  Spin,
  Slider,
  InputNumber,
  Tooltip,
} from "antd";
import { getCurrent } from "@tauri-apps/api/window";
import CustomTitleBar from "./CustomTitleBar";
import { relaunch } from "@tauri-apps/api/process";
import "./styles.css";

const { Header, Content } = Layout;

function Settings() {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(true);
  const [apiKey, setApiKey] = useState("");
  const [end_trigger, setEndTrigger] = useState("");
  const [start_trigger, setStartTrigger] = useState("");
  const [useInputBox, setUseInputBox] = useState(false);
  const [temperature, setTemperature] = useState(0.7);
  const [max_tokens, setMaxTokens] = useState(1024);

  useEffect(() => {
    async function fetchSettings() {
      try {
        setLoading(true);
        const settings = await invoke("load_settings");
        setApiKey(settings.api_key);
        setEndTrigger(settings.end_trigger);
        setStartTrigger(settings.start_trigger);
        setUseInputBox(settings.use_input_box);
        setMaxTokens(settings.max_tokens);
        setTemperature(settings.temperature);
        //update the form
        form.setFieldsValue({
          apiKey: settings.api_key,
          end_trigger: settings.end_trigger,
          start_trigger: settings.start_trigger,
          use_input_box: settings.use_input_box,
          temperature: settings.temperature,
          max_tokens: settings.max_tokens,
        });
        // Fields value have been loaded.
        setLoading(false);
      } catch (error) {
        console.error("Error loading settings:", error);
      }
    }
    fetchSettings();
  }, []);

  // In case relaunch is not working
  async function restartApp() {
    // Save the path to the current executable.
    const exePath = await app.getPath("exe");

    // Spawn a new instance of the application.
    await invoke("spawn", {
      command: exePath,
      args: [],
      stdout: "inherit",
      stderr: "inherit",
    });

    // Close the current instance of the application.
    app.exit(0);
  }

  async function saveSettings() {
    setLoading(true);
    await invoke("save_settings", {
      settings: {
        api_key: apiKey,
        start_trigger,
        end_trigger,
        use_input_box: useInputBox,
        temperature,
        max_tokens,
      },
    });
    setLoading(false);
    // Close the window
    const window = getCurrent();
    window.hide();
    let reload_status = await relaunch();
    console.log("reaload_status", reload_status);
    // await restartApp();
  }

  return (
    <Layout>
      <CustomTitleBar />
      <Header style={{ height: "40px", lineHeight: "40px" }}>
        <span
          style={{
            color: "white",
            height: "40px",
            fontSize: "30px",
            fontWeight: "bold",
          }}
        >
          Settings
        </span>
      </Header>
      {loading ? (
        <Content style={{ padding: "40px" }}>
          <Spin tip="Loading" size="large">
            <div className="content" />
          </Spin>
        </Content>
      ) : (
        <Content style={{ padding: "40px" }}>
          <Form
            form={form}
            onSubmit={(e) => e.preventDefault()}
            onFinish={saveSettings}
            style={{ width: "100%" }}
          >
            <Row gutter={16}>
              <Col flex="auto">
                <Form.Item
                  label={
                    <Tooltip title="You personal chat GPT api key">
                      <span>Api Key</span>
                    </Tooltip>
                  }
                  name="apiKey"
                >
                  <Input
                    value={apiKey}
                    onChange={(e) => setApiKey(e.target.value)}
                    placeholder="Enter API key"
                  />
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col flex="auto">
                <Form.Item
                  label={
                    <Tooltip title="Use special char and a word like: /askgpt or @gpto">
                      <span>Start Trigger word</span>
                    </Tooltip>
                  }
                  name="start_trigger"
                >
                  <Input
                    value={start_trigger}
                    onChange={(e) => setStartTrigger(e.target.value)}
                    placeholder="Enter a word starting with / or @ to start the trigger, default to /imagine"
                  />
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col flex="auto">
                <Form.Item
                  label={
                    <Tooltip title="Use special char like : \ & or / to detect the end of your prompt">
                      <span>End Trigger word</span>
                    </Tooltip>
                  }
                  name="end_trigger"
                >
                  <Input
                    value={end_trigger}
                    onChange={(e) => setEndTrigger(e.target.value)}
                    placeholder="Enter a special trigger which will end the prompt, default to \"
                  />
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col flex="auto">
                <Form.Item
                  label={
                    <Tooltip
                      title="0 to 0.3: More focused, coherent.
                    0.3 to 0.7: Balanced creativity and coherence.
                    0.7 to 1: Highly creative and diverse, but less coherent."
                    >
                      <span>Temperature</span>
                    </Tooltip>
                  }
                  name="temperature"
                >
                  <Row>
                    <Col span={12}>
                      <Slider
                        min={0}
                        max={1}
                        onChange={(e) => setTemperature(e)}
                        value={
                          typeof temperature === "number" ? temperature : 0
                        }
                        step={0.1}
                      />
                    </Col>
                    <Col span={4}>
                      <InputNumber
                        min={0}
                        max={1}
                        value={Number(temperature).toFixed(1)}
                        onChange={(value) => setTemperature(value)}
                        style={{ marginLeft: 16 }}
                        step={0.1}
                      />
                    </Col>
                  </Row>
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col span={6}>
                <Form.Item
                  name="max_tokens"
                  label={
                    <Tooltip title="Tokens = 4 chars or 0.75 words">
                      <span>Max Tokens</span>
                    </Tooltip>
                  }
                >
                  <InputNumber
                    min={1}
                    max={4096}
                    value={max_tokens}
                    onChange={(value) => setMaxTokens(value)}
                    style={{ marginLeft: 16 }}
                    step={1}
                  />
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col span={8}>
                <Form.Item
                  label={
                    <Tooltip title="Text input will open a windows to let you paste text for example">
                      <span>Use a Input for the text prompt</span>
                    </Tooltip>
                  }
                  name="use_input_box"
                  valuePropName="checked"
                >
                  <Checkbox
                    checked={useInputBox}
                    onChange={(e) => setUseInputBox(e.target.checked)}
                  ></Checkbox>
                </Form.Item>
              </Col>
            </Row>
            <Row gutter={16}>
              <Col span={8}>
                <Form.Item>
                  <Button type="primary" htmlType="submit">
                    Save
                  </Button>
                </Form.Item>
              </Col>
            </Row>
          </Form>
        </Content>
      )}
    </Layout>
  );
}

export default Settings;
