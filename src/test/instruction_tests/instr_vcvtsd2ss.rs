use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 90, 243], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 870414140, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 90, 191, 60, 119, 225, 51], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 90, 194], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1724135438, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 90, 36, 181, 14, 56, 196, 102], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 215, 154, 90, 239], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 559196555, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 247, 143, 90, 128, 139, 169, 84, 33], OperandSize::Dword)
}

#[test]
fn vcvtsd2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 239, 148, 90, 225], OperandSize::Qword)
}

#[test]
fn vcvtsd2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 672247339, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 175, 139, 90, 60, 221, 43, 174, 17, 40], OperandSize::Qword)
}

