use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 243], OperandSize::Dword)
}

#[test]
fn vcvtss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EDI, 1364983722, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 167, 170, 255, 91, 81], OperandSize::Dword)
}

#[test]
fn vcvtss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 242], OperandSize::Qword)
}

#[test]
fn vcvtss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 262166307, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 164, 146, 35, 87, 160, 15], OperandSize::Qword)
}

#[test]
fn vcvtss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 232], OperandSize::Qword)
}

#[test]
fn vcvtss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 31], OperandSize::Qword)
}

#[test]
fn vcvtss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 120, 45, 232], OperandSize::Dword)
}

#[test]
fn vcvtss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1730539162, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 164, 65, 154, 238, 37, 103], OperandSize::Dword)
}

#[test]
fn vcvtss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 56, 45, 237], OperandSize::Qword)
}

#[test]
fn vcvtss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1401343529, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 45, 60, 141, 41, 206, 134, 83], OperandSize::Qword)
}

#[test]
fn vcvtss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 88, 45, 244], OperandSize::Qword)
}

#[test]
fn vcvtss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSS2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 45, 60, 95], OperandSize::Qword)
}

