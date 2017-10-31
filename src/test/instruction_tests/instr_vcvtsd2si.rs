use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 235], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 2085842158, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 180, 209, 238, 108, 83, 124], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 232], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 1720642424, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 188, 155, 120, 235, 142, 102], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 240], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectDisplaced(RSI, 2060513581, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 142, 45, 241, 208, 122], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 56, 45, 254], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 44, 247], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 127, 56, 45, 205], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RDX, 2147297116, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 154, 92, 39, 253, 127], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 255, 120, 45, 244], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 282333397, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 140, 153, 213, 16, 212, 16], OperandSize::Qword)
}

