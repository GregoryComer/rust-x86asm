use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 208], OperandSize::Dword)
}

#[test]
fn vcvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 49], OperandSize::Dword)
}

#[test]
fn vcvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 255], OperandSize::Qword)
}

#[test]
fn vcvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RSI, 859079087, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 190, 175, 129, 52, 51], OperandSize::Qword)
}

#[test]
fn vcvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 227], OperandSize::Qword)
}

#[test]
fn vcvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1185884953, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 20, 181, 25, 43, 175, 70], OperandSize::Qword)
}

#[test]
fn vcvtss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 24, 45, 205], OperandSize::Dword)
}

#[test]
fn vcvtss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 56], OperandSize::Dword)
}

#[test]
fn vcvtss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 45, 248], OperandSize::Qword)
}

#[test]
fn vcvtss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 417109118, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 164, 250, 126, 148, 220, 24], OperandSize::Qword)
}

#[test]
fn vcvtss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 254, 56, 45, 245], OperandSize::Qword)
}

#[test]
fn vcvtss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1943084434, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 148, 146, 146, 29, 209, 115], OperandSize::Qword)
}

