import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button, Input, Form, Layout, Row, Col } from "antd";
import CustomTitleBar from "./CustomTitleBar";
import "./styles.css";

const { Header, Content } = Layout;

const { TextArea } = Input;

function InputApp() {
  const [form] = Form.useForm();
  const [text, setText] = useState("");
  const [end_trigger, setEndTrigger] = useState("");

  useEffect(() => {
    async function fetchSettings() {
      try {
        const settings = await invoke("load_settings");
        setEndTrigger(settings.end_trigger);
        //update the form
        form.setFieldsValue({
          end_trigger: settings.end_trigger,
        });
      } catch (error) {
        console.error("Error loading settings:", error);
      }
    }
    fetchSettings();
  }, []);

  async function sendToChatGpt() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("submit_text", { text });
    // Close the window
    await invoke("hide_it");
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
          Input
        </span>
      </Header>
      <Content style={{ padding: "40px" }}>
        <Form
          form={form}
          onSubmit={(e) => e.preventDefault()}
          onFinish={sendToChatGpt}
          style={{ width: "100%" }}
        >
          <Row gutter={16}>
            <Col flex="auto">
              <Form.Item>
                <TextArea
                  value={text}
                  rows={10}
                  placeholder="maxLength is 6"
                  onChange={(e) => setText(e.currentTarget.value)}
                />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={16}>
            <Col span={8}>
              <Form.Item>
                <Button type="primary" htmlType="submit">
                  Send
                </Button>
              </Form.Item>
            </Col>
          </Row>
        </Form>
      </Content>
    </Layout>
  );
}

export default InputApp;
