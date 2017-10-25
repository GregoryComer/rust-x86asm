use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 245], OperandSize::Dword)
}

fn paddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(EDI, 1641982618, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 183, 154, 170, 222, 97], OperandSize::Dword)
}

fn paddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 225], OperandSize::Qword)
}

fn paddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(MM5)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 220, 47], OperandSize::Qword)
}

fn paddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 200], OperandSize::Dword)
}

fn paddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 360642404, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 148, 134, 100, 247, 126, 21], OperandSize::Dword)
}

fn paddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 201], OperandSize::Qword)
}

fn paddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1600325352, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 220, 156, 190, 232, 6, 99, 95], OperandSize::Qword)
}

