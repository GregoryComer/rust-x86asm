use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 10, 38, 225], OperandSize::Dword)
}

#[test]
fn vptestmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 162487689, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 15, 38, 20, 125, 137, 93, 175, 9], OperandSize::Dword)
}

#[test]
fn vptestmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 13, 6, 38, 204], OperandSize::Qword)
}

#[test]
fn vptestmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1248683107, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 5, 12, 38, 60, 197, 99, 100, 109, 74], OperandSize::Qword)
}

#[test]
fn vptestmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 44, 38, 224], OperandSize::Dword)
}

#[test]
fn vptestmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 42, 38, 26], OperandSize::Dword)
}

#[test]
fn vptestmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 85, 39, 38, 243], OperandSize::Qword)
}

#[test]
fn vptestmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1502821047, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 45, 38, 148, 192, 183, 58, 147, 89], OperandSize::Qword)
}

#[test]
fn vptestmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 77, 38, 248], OperandSize::Dword)
}

#[test]
fn vptestmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 75, 38, 22], OperandSize::Dword)
}

#[test]
fn vptestmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 68, 38, 253], OperandSize::Qword)
}

#[test]
fn vptestmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 330737265, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 21, 67, 38, 188, 95, 113, 166, 182, 19], OperandSize::Qword)
}

