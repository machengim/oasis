import React from 'react';
import {Row, Form} from 'react-bootstrap';

export default function Step1() {
    return (
        <Row className="justify-content-md-center my-2">
            <Form>
                <Form.Group controlId="formBasicEmail">
                    <Form.Label>Username</Form.Label>
                    <Form.Control type="text" placeholder="Enter username" />
                    <Form.Text className="text-muted">
                    The username used for the admin account.
                    </Form.Text>
                </Form.Group>
                <Form.Group controlId="formBasicPassword">
                    <Form.Label>Password</Form.Label>
                    <Form.Control type="password" placeholder="Enter Password" />
                </Form.Group>
            </Form>
        </Row>
    );
}