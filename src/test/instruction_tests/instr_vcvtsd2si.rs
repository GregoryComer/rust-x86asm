use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 255], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(EBX, 718368745, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 163, 233, 111, 209, 42], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 219], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 32], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 206], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSP)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 34], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 120, 45, 245], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1372277434, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 44, 253, 186, 74, 203, 81], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 177, 127, 56, 45, 212], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RSI, 561980055, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 150, 151, 34, 127, 33], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 255, 120, 45, 246], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1482859679, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 180, 208, 159, 164, 98, 88], OperandSize::Qword)
}

