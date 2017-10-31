use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 230], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 42, 31], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 42, 220], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 600419028, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 42, 36, 69, 212, 170, 201, 35], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 202, 42, 245], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 153678537, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 226, 42, 190, 201, 242, 40, 9], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 118, 120, 42, 222], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 42, 4, 248], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 102, 120, 42, 250], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 14, 0, 42, 58], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 174, 88, 42, 213], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1928393774, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 238, 0, 42, 132, 223, 46, 244, 240, 114], OperandSize::Qword)
}

